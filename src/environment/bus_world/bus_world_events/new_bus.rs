use std::fmt::{Display, Error, Formatter};

use serde::{Deserialize, Serialize};

use crate::event::schedulable::SchedulableEvent;

#[derive(Deserialize, Serialize)]
pub struct NewBusesJson {
    pub number_of_buses: usize,
    pub capacity: usize,
}

impl NewBusesJson {
    pub fn new(number_of_buses: usize, capacity: usize) -> Self {
        Self {
            number_of_buses,
            capacity,
        }
    }
}

pub struct NewBusEvent {
    uid: usize,
    timestamp: usize,
    data: String,
}

impl NewBusEvent {
    pub fn new(uid: usize, timestamp: usize, data: String) -> NewBusEvent {
        NewBusEvent {
            uid,
            timestamp,
            data,
        }
    }
}

impl SchedulableEvent for NewBusEvent {
    fn get_event_type(&self) -> &str {
        "NewBus"
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

impl Display for NewBusEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "NewBusEvent: uid: {} for bus uid: {}",
            self.uid, self.data,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::environment::bus_world::bus_world_events::new_bus::NewBusEvent;
    use crate::event::schedulable::SchedulableEvent;

    #[test]
    fn create_bus_event() {
        let bus_event = NewBusEvent::new(1, 0, String::from("Hello world!"));
        assert_eq!(bus_event.get_event_type(), "NewBus");
        assert_eq!(bus_event.get_uid(), 1);
        assert_eq!(bus_event.get_time_stamp(), 0);
        assert_eq!(bus_event.get_data().unwrap(), "Hello world!");
    }
}
