use crate::event::event::Event;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BusToStopMappingJson {
    pub bus_uid: usize,
    pub stop_name: String,
}

impl BusToStopMappingJson {
    pub fn new(bus_uid: usize, stop_name: String) -> Self {
        Self { bus_uid, stop_name }
    }
}

pub struct MoveBusToStopEvent {
    uid: usize,
    timestamp: f64,
    data: String,
}

impl MoveBusToStopEvent {
    pub fn new(uid: usize, timestamp: f64, data: String) -> MoveBusToStopEvent {
        MoveBusToStopEvent {
            uid,
            timestamp,
            data,
        }
    }
}

impl Event for MoveBusToStopEvent {
    fn get_event_type(&self) -> &str {
        "MoveBusToStop"
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
