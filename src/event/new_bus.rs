use super::super::environment::bus_world::bus::Bus;
use super::event::Event;

pub struct NewBusEvent {
    uid: usize,
    timestamp: f64,
    data: String,
}

impl NewBusEvent {
    pub fn new(uid: usize, timestamp: f64, data: String) -> NewBusEvent {
        NewBusEvent {
            uid,
            timestamp,
            data,
        }
    }
}

impl Event for NewBusEvent {
    fn get_event_type(&self) -> &str {
        "NewBus"
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
    use crate::event::{event::Event, new_bus::NewBusEvent};

    #[test]
    fn create_bus_event() {
        let bus_event = NewBusEvent::new(1, 0.0, String::from("Hello world!"));
        assert_eq!(bus_event.get_event_type(), "NewBus");
        assert_eq!(bus_event.get_uid(), 1);
        assert_eq!(bus_event.get_time_stamp(), 0.0);
        assert_eq!(bus_event.get_data().unwrap(), "Hello world!");
    }
}
