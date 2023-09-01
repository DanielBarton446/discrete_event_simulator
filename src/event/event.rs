use std::cmp::Ordering;

pub trait Event {
    fn get_event_type(&self) -> &str;
    fn get_uid(&self) -> usize;
    fn get_time_stamp(&self) -> f64;

    // Stringified JSON to use for arbitrary event handling
    fn get_data(&self) -> Result<String, serde_json::Error>;
}

impl Ord for dyn Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_time_stamp()
            .partial_cmp(&other.get_time_stamp())
            .unwrap()
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
