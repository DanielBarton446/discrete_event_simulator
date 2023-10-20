use std::fmt::{Display, Error, Formatter};

use crate::event::event::Event;

pub struct TerminalEvent {
    uid: usize,
    timestamp: usize,
    data: String,
}

impl TerminalEvent {
    pub fn new(uid: usize, timestamp: usize, data: String) -> TerminalEvent {
        TerminalEvent {
            uid,
            timestamp,
            data,
        }
    }
}

impl Event for TerminalEvent {
    fn get_event_type(&self) -> &str {
        "Terminal"
    }

    fn get_uid(&self) -> usize {
        self.uid
    }

    fn get_time_stamp(&self) -> usize {
        self.timestamp
    }

    fn get_data(&self) -> Result<String, serde_json::Error> {
        Ok(self.data.clone())
    }
}

impl Display for TerminalEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "TerminalEvent: uid: {} ", self.uid)
    }
}
