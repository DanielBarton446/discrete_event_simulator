use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

use serde::Serialize;

use crate::environment::bus_world::passenger::Passenger;

#[derive(Serialize)]
pub struct Bus {
    pub uid: usize,
    pub passengers: HashMap<String, Vec<Passenger>>,
    pub serviced_stop_names: Vec<String>,
    current_stop: usize,
    pub capacity: usize,
}

impl Bus {
    pub fn new(uid: usize, capacity: usize) -> Bus {
        Bus {
            uid,
            passengers: HashMap::new(),
            serviced_stop_names: Vec::new(),
            current_stop: 0,
            capacity,
        }
    }

    pub fn add_passenger(&mut self, passenger: Passenger) {
        self.passengers
            .entry(passenger.destination.clone())
            .or_insert_with(Vec::new)
            .push(passenger);
    }

    pub fn add_serviced_stop(&mut self, stop: String) {
        self.serviced_stop_names.push(stop);
    }

    pub fn current_passenger_count(&self) -> usize {
        self.passengers.values().map(|v| v.len()).sum()
    }

    pub fn get_current_stop(&self) -> Option<&String> {
        self.serviced_stop_names.get(self.current_stop)
    }

    pub fn advance_to_next_stop(&mut self) {
        self.current_stop += 1;
    }

    pub fn get_next_stop(&self) -> Option<&String> {
        self.serviced_stop_names.get(self.current_stop + 1)
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[Bus {}]", self.uid)
    }
}
