use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::event::event::Event;

pub struct LoadPassengersEvent {
    uid: usize,
    timestamp: usize,
    data: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoadPassengersJson {
    pub bus_uuid: String, // what else?
}

impl LoadPassengersJson {
    pub fn new(bus_uid: String) -> Self {
        Self { bus_uuid: bus_uid }
    }
}

impl LoadPassengersEvent {
    pub fn new(uid: usize, timestamp: usize, data: String) -> LoadPassengersEvent {
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

    fn get_time_stamp(&self) -> usize {
        self.timestamp
    }

    fn get_data(&self) -> Result<String, serde_json::Error> {
        Ok(self.data.clone())
    }
}

impl Display for LoadPassengersEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "LoadPassengersEvent: uid: {} for bus uid: {}",
            self.uid, self.data,
        )
    }
}
