use std::{cmp::Ordering, fmt::Display};

pub trait SchedulableEvent: Display {
    fn get_event_type(&self) -> &str;
    fn get_uid(&self) -> usize;
    fn get_time_stamp(&self) -> usize;

    // Stringified JSON to use for arbitrary event handling
    fn get_data(&self) -> Result<String, serde_json::Error>;
}

impl Ord for dyn SchedulableEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_time_stamp().cmp(&other.get_time_stamp()).reverse()
    }
}

impl PartialOrd for dyn SchedulableEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for dyn SchedulableEvent {
    fn eq(&self, other: &Self) -> bool {
        self.get_time_stamp() == other.get_time_stamp()
    }
}

impl Eq for dyn SchedulableEvent {}
