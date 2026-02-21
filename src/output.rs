use std::fs::OpenOptions;
use std::io::Write;
use crate::timeline::Event;

pub struct OutputManager {
    file_path: String,
}

impl OutputManager {
    pub fn new(file_path: &str) -> anyhow::Result<Self> {
        // Create or truncate the file
        let _ = std::fs::File::create(file_path)?;
        Ok(Self {
            file_path: file_path.to_string(),
        })
    }

    pub fn log_event(&self, event: &Event) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file_path)?;
        
        let line = match event {
            Event::Transcribed { text, timestamp } => format!("[{:.2}] SPEECH: {}\n", timestamp, text),
            Event::KeyPressed { symbol, timestamp } => format!("[{:.2}] KEY: {}\n", timestamp, symbol),
        };
        
        file.write_all(line.as_bytes())?;
        Ok(())
    }

    pub fn log_message(&self, message: &str) -> anyhow::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.file_path)?;
        
        file.write_all(format!(">>> {}\n", message).as_bytes())?;
        Ok(())
    }
}
