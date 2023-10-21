use std::fmt::{Display, Formatter};

use crate::event::schedulable::SchedulableEvent;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BusToStopMappingJson {
    pub bus_uuid: String,
    pub stop_name: String,
}

impl BusToStopMappingJson {
    pub fn new(bus_uuid: String, stop_name: String) -> Self {
        Self {
            bus_uuid,
            stop_name,
        }
    }
}

pub struct MoveBusToStopEvent {
    uid: usize,
    timestamp: usize,
    data: String,
}

impl MoveBusToStopEvent {
    pub fn new(uid: usize, timestamp: usize, data: String) -> MoveBusToStopEvent {
        MoveBusToStopEvent {
            uid,
            timestamp,
            data,
        }
    }
}

impl SchedulableEvent for MoveBusToStopEvent {
    fn get_event_type(&self) -> &str {
        "MoveBusToStop"
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

impl Display for MoveBusToStopEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "MoveBusToStopEvent: event uid: {}, Data: {}",
            self.uid, self.data
        )
    }
}
