use serde::{Deserialize, Serialize};

use crate::event::event::Event;

pub struct LoadPassengersEvent {
    uid: usize,
    timestamp: f64,
    data: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoadPassengersJson {
    pub bus_uid: usize, // what else?
}

impl LoadPassengersJson {
    pub fn new(bus_uid: usize) -> Self {
        Self { bus_uid }
    }
}

impl LoadPassengersEvent {
    pub fn new(uid: usize, timestamp: f64, data: String) -> LoadPassengersEvent {
        LoadPassengersEvent {
            uid,
            timestamp,
            data,
        }
    }
}

impl Event for LoadPassengersEvent {
    fn get_event_type(&self) -> &str {
        "LoadPassengers"
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
