use std::{cmp::Ordering, fmt::Display};

pub trait Event: Display {
    fn get_event_type(&self) -> &str;
    fn get_uid(&self) -> usize;
    fn get_time_stamp(&self) -> f64;

    // Stringified JSON to use for arbitrary event handling
    fn get_data(&self) -> Result<String, serde_json::Error>;
}

impl Ord for dyn Event {
    fn cmp(&self, other: &Self) -> Ordering {
        let timestamp_ordering = self.get_time_stamp().partial_cmp(&other.get_time_stamp());

        match timestamp_ordering {
            Some(ordering) => ordering.reverse(),
            None => Ordering::Equal, // Handle the case where comparison fails
        }
    }
}

impl PartialOrd for dyn Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for dyn Event {
    fn eq(&self, other: &Self) -> bool {
        self.get_time_stamp() == other.get_time_stamp()
    }
}

impl Eq for dyn Event {}
