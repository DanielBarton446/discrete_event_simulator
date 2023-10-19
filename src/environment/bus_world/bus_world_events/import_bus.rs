use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::{environment::bus_world::bus::Bus, event::event::Event};

#[derive(Serialize)]
pub struct ImportBusesJson {
    pub buses: Vec<Bus>,
}

impl<'de> Deserialize<'de> for ImportBusesJson {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ImportBusesJson {
            buses: Vec::deserialize(deserializer)?,
        })
    }
}

pub struct ImportBusEvent {
    pub uid: usize,
    pub timestamp: usize,
    pub data: String,
}

impl ImportBusEvent {
    pub fn new(uid: usize, timestamp: usize, data: String) -> Self {
        Self {
            uid,
            timestamp,
            data,
        }
    }
}

impl Display for ImportBusEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "ImportBusEvent: uid: {}, data: {:?}",
            self.uid, self.data
        )
    }
}

impl Event for ImportBusEvent {
    fn get_event_type(&self) -> &str {
        "ImportBus"
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
