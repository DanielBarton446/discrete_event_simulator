use super::event::Event;

pub struct BusEvent {
    uid: usize,
    timestamp: f64,
    data: String,
}

impl BusEvent {
    pub fn new(uid: usize, timestamp: f64, data: String) -> BusEvent {
        BusEvent {
            uid,
            timestamp,
            data,
        }
    }
}

impl Event for BusEvent {
    fn get_event_type(&self) -> &str {
        "BusEvent"
    }

    fn get_uid(&self) -> usize {
        self.uid
    }

    fn get_time_stamp(&self) -> f64 {
        self.timestamp
    }

    fn get_data(&self) -> Result<String, serde_json::Error> {
        Ok(self.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::event::{bus_event::BusEvent, event::Event};

    #[test]
    fn create_bus_event() {
        let bus_event = BusEvent::new(1, 0.0, String::from("Hello world!"));
        assert_eq!(bus_event.get_event_type(), "BusEvent");
        assert_eq!(bus_event.get_uid(), 1);
        assert_eq!(bus_event.get_time_stamp(), 0.0);
        assert_eq!(bus_event.get_data().unwrap(), "Hello world!");
    }
}
