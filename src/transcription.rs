use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};
use std::path::Path;
use chrono::Utc;

pub struct TranscriptionEvent {
    pub text: String,
    pub timestamp: f64,
}

pub struct WhisperState {
    ctx: WhisperContext,
}

impl WhisperState {
    pub fn new(model_path: &str) -> anyhow::Result<Self> {
        if !Path::new(model_path).exists() {
            return Err(anyhow::anyhow!("Whisper model not found at {}", model_path));
        }
        let ctx = WhisperContext::new_with_params(model_path, WhisperContextParameters::default())?;
        Ok(Self { ctx })
    }

    pub fn transcribe(&self, samples: &[f32]) -> anyhow::Result<Vec<TranscriptionEvent>> {
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_n_threads(4);
        params.set_language(Some("en"));
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        let mut state = self.ctx.create_state()?;
        state.full(params, samples)?;

        let mut events = Vec::new();
        let num_segments = state.full_n_segments()?;
        let now = Utc::now().timestamp_millis() as f64 / 1000.0;

        for i in 0..num_segments {
            if let Ok(segment) = state.full_get_segment_text(i) {
                events.push(TranscriptionEvent {
                    text: segment.trim().to_string(),
                    timestamp: now, // Simplification: use current time as transcription time
                });
            }
        }

        Ok(events)
    }
}
