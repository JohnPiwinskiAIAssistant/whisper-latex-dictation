use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Transcribed { text: String, timestamp: f64 },
    KeyPressed { symbol: String, timestamp: f64 },
}

impl Event {
    pub fn timestamp(&self) -> f64 {
        match self {
            Event::Transcribed { timestamp, .. } => *timestamp,
            Event::KeyPressed { timestamp, .. } => *timestamp,
        }
    }
}

pub struct Timeline {
    events: Vec<Event>,
}

impl Timeline {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event);
        self.events.sort_by(|a, b| a.timestamp().partial_cmp(&b.timestamp()).unwrap());
    }

    pub fn generate_prompt(&self) -> String {
        let mut prompt = String::from(
            "Convert the following sequence of transcribed speech and keyboard-inserted symbols into a clean LaTeX document with inline ($...$) and display ($$...$$) mathematics. \
            The keyboard symbols like '(', ')', '[', ']', '{', '}', '^', '_', and '\\\\' were inserted manually by the user during dictation to clarify the structure. \n\
            IMPORTANT: The '\\\\' symbol (inserted via the semicolon key) indicates a line break or alignment point. Please use an 'aligned' environment for multiline formulas if this symbol is present.\n\n\
            Timeline of events:\n"
        );

        for event in &self.events {
            match event {
                Event::Transcribed { text, timestamp } => {
                    prompt.push_str(&format!("[{:.2}s] Speech: {}\n", timestamp, text));
                }
                Event::KeyPressed { symbol, timestamp } => {
                    prompt.push_str(&format!("[{:.2}s] Symbol: {}\n", timestamp, symbol));
                }
            }
        }

        prompt.push_str("\nOutput only the final LaTeX code.");
        prompt
    }

    pub fn get_events(&self) -> &[Event] {
        &self.events
    }
}
