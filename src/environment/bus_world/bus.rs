use std::fmt::{Display, Error, Formatter};

use crate::environment::bus_world::passenger::Passenger;

pub struct Bus {
    pub uid: usize,
    passengers: Vec<Passenger>,
    serviced_stop_names: Vec<String>,
    current_stop: usize,
}

impl Bus {
    pub fn new(uid: usize) -> Bus {
        Bus {
            uid,
            passengers: Vec::new(),
            serviced_stop_names: Vec::new(),
            current_stop: 0,
        }
    }

    pub fn add_passenger(&mut self, passenger: Passenger) {
        self.passengers.push(passenger);
    }

    pub fn add_serviced_stop(&mut self, stop: String) {
        self.serviced_stop_names.push(stop);
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
        let passenger_display = self
            .passengers
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "[Bus {}]", self.uid)
    }
}
