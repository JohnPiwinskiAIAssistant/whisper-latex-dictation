mod audio;
mod transcription;
mod keyboard;
mod timeline;
mod gemini;
mod output;

use anyhow::{Result, Context};
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    println!("=== Multi-modal Math Dictation ===");
    println!("Instructions:");
    println!("- Dictate your math formula.");
    println!("- Use keys: j=(, k=), u=[, i=], m={{, ,=}}, o=^, l=_, ;=\\\\");
    println!("- Press Ctrl+C to finish and generate LaTeX.");

    let (key_tx, key_rx) = mpsc::channel();
    keyboard::start_keyboard_listener(key_tx);

    let audio_data = Arc::new(Mutex::new(Vec::new()));
    let _stream = audio::start_audio_capture(Arc::clone(&audio_data))
        .context("Failed to start audio capture")?;

    let model_path = std::env::var("WHISPER_MODEL_PATH")
        .unwrap_or_else(|_| "models/ggml-base.en.bin".to_string());
    let whisper = transcription::WhisperState::new(&model_path)
        .context("Failed to initialize Whisper")?;

    let gemini_key = std::env::var("GEMINI_API_KEY")
        .context("GEMINI_API_KEY environment variable not set")?;
    let gemini = gemini::GeminiClient::new(gemini_key);

    let output = output::OutputManager::new("dictation_log.txt")?;
    let mut timeline = timeline::Timeline::new();

    println!("Listening... Press Ctrl+C to stop.");

    let running = Arc::new(Mutex::new(true));
    let r = Arc::clone(&running);
    
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        println!("\nStopping... Processing results.");
        let mut running_lock = r.lock().unwrap();
        *running_lock = false;
    });

    while *running.lock().unwrap() {
        // Collect keyboard events
        while let Ok(key_event) = key_rx.try_recv() {
            let event = timeline::Event::KeyPressed {
                symbol: key_event.key,
                timestamp: key_event.timestamp,
            };
            timeline.add_event(event.clone());
            output.log_event(&event)?;
        }

        // Periodically transcribe audio (every 3 seconds of audio accumulated)
        let samples = {
            let mut data = audio_data.lock().unwrap();
            if data.len() >= 16000 * 3 {
                let s = data.clone();
                data.clear();
                Some(s)
            } else {
                None
            }
        };

        if let Some(s) = samples {
            match whisper.transcribe(&s) {
                Ok(events) => {
                    for e in events {
                        let event = timeline::Event::Transcribed {
                            text: e.text,
                            timestamp: e.timestamp,
                        };
                        timeline.add_event(event.clone());
                        output.log_event(&event)?;
                    }
                }
                Err(e) => eprintln!("Transcription error: {}", e),
            }
        }

        sleep(Duration::from_millis(100)).await;
    }

    // Final transcription of remaining audio
    let remaining_samples = {
        let mut data = audio_data.lock().unwrap();
        let s = data.clone();
        data.clear();
        s
    };
    if !remaining_samples.is_empty() {
        if let Ok(events) = whisper.transcribe(&remaining_samples) {
            for e in events {
                let event = timeline::Event::Transcribed {
                    text: e.text,
                    timestamp: e.timestamp,
                };
                timeline.add_event(event.clone());
                output.log_event(&event)?;
            }
        }
    }

    // Generate output
    let prompt = timeline.generate_prompt();
    output.log_message("GENERATING LATEX...")?;
    
    println!("Sending prompt to Gemini...");
    match gemini.convert_to_latex(&prompt).await {
        Ok(latex) => {
            println!("\n--- Generated LaTeX ---\n{}\n-----------------------", latex);
            output.log_message(&format!("FINAL LATEX:\n{}", latex))?;
        }
        Err(e) => {
            eprintln!("Gemini error: {}", e);
            output.log_message(&format!("GEMINI ERROR: {}", e))?;
        }
    }

    Ok(())
}
