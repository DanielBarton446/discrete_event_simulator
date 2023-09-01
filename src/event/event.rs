pub trait Event {
    fn get_event_type(&self) -> &str;
    fn get_uid(&self) -> usize;
    fn get_time_stamp(&self) -> f64;

    // Stringified JSON to use for arbitrary event handling
    fn get_data(&self) -> Result<String, serde_json::Error>;
}
