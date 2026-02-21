use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc::Sender;
use std::thread;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct KeyEvent {
    pub key: String,
    pub timestamp: f64,
}

pub fn start_keyboard_listener(tx: Sender<KeyEvent>) {
    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            if let EventType::KeyPress(key) = event.event_type {
                let key_str = match key {
                    Key::KeyJ => Some("("),
                    Key::KeyK => Some(")"),
                    Key::KeyU => Some("["),
                    Key::KeyI => Some("]"),
                    Key::KeyM => Some("{"),
                    Key::Comma => Some("}"),
                    Key::KeyO => Some("^"),
                    Key::KeyL => Some("_"),
                    Key::Semicolon => Some("\\\\"),
                    _ => None,
                };

                if let Some(s) = key_str {
                    let timestamp = Utc::now().timestamp_millis() as f64 / 1000.0;
                    let _ = tx.send(KeyEvent {
                        key: s.to_string(),
                        timestamp,
                    });
                }
            }
        }) {
            eprintln!("Error listening to keyboard: {:?}", error);
        }
    });
}
