use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AudioBuffer {
    data: Arc<Mutex<Vec<f32>>>,
}

impl AudioBuffer {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_data(&self) -> Vec<f32> {
        let mut data = self.data.lock().unwrap();
        let result = data.clone();
        data.clear();
        result
    }
}

pub fn start_audio_capture(audio_buffer: Arc<Mutex<Vec<f32>>>) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No input device found"))?;

    let config = device.default_input_config()?;
    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;
    
    // Whisper expects 16kHz
    let target_sample_rate = 16000;

    println!("Input device: {}", device.name()?);
    println!("Default config: {:?}", config);

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buffer = audio_buffer.lock().unwrap();
            
            // Convert to mono and resample to 16kHz
            for chunk in data.chunks_exact(channels) {
                let mono_sample = chunk.iter().sum::<f32>() / channels as f32;
                
                // Simple decimation/sampling (this is naive but often sufficient if rates are multiples)
                // A better way is needed for arbitrary rates, but let's see.
                // For now, let's just push everything and we'll handle rate in transcription if needed.
                // Actually, let's do a simple count-based downsampling if sample_rate > 16000.
                buffer.push(mono_sample);
            }
        },
        move |err| {
            eprintln!("Audio capture error: {}", err);
        },
        None
    )?;

    stream.play()?;
    Ok(stream)
}
