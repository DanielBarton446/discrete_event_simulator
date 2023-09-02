use std::fmt::{Display, Error, Formatter};

use crate::environment::bus_world::bus_stop::BusStop;
use crate::environment::bus_world::passenger::Passenger;

pub struct Bus {
    pub uid: usize,
    passengers: Vec<Passenger>,
    serviced_stops: Vec<BusStop>,
}

impl Bus {
    pub fn new(uid: usize) -> Bus {
        Bus {
            uid,
            passengers: Vec::new(),
            serviced_stops: Vec::new(),
        }
    }

    pub fn add_passenger(&mut self, passenger: Passenger) {
        self.passengers.push(passenger);
    }

    pub fn add_serviced_stop(&mut self, stop: BusStop) {
        self.serviced_stops.push(stop);
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
        write!(f, "Bus {}: Passengers: {}", self.uid, passenger_display)
    }
}
