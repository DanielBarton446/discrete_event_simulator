use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::event::schedulable::SchedulableEvent;

pub struct UnloadPassengersEvent {
    uid: usize,
    timestamp: usize,
    data: String,
}

#[derive(Deserialize, Serialize)]
pub struct UnloadPassengersJson {
    pub bus_uuid: String, // what else?
}

impl UnloadPassengersJson {
    pub fn new(bus_uuid: String) -> Self {
        Self { bus_uuid }
    }
}

impl UnloadPassengersEvent {
    pub fn new(uid: usize, timestamp: usize, data: String) -> UnloadPassengersEvent {
        UnloadPassengersEvent {
            uid,
            timestamp,
            data,
        }
    }
}

impl SchedulableEvent for UnloadPassengersEvent {
    fn get_event_type(&self) -> &str {
        "UnloadPassengers"
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

impl Display for UnloadPassengersEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "UnloadPassengersEvent: uid: {} for bus uid: {}",
            self.uid, self.data,
        )
    }
}
