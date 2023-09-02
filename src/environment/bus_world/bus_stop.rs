use std::fmt::{Display, Error, Formatter};

use super::{bus::Bus, passenger::Passenger};

pub struct BusStop {
    pub name: String,
    pub waiting_passengers: Vec<Passenger>,
    pub completed_passengers: Vec<Passenger>,
    pub buses_at_stop: Vec<Bus>,
}

impl BusStop {
    pub fn new(name: String) -> BusStop {
        BusStop {
            name,
            waiting_passengers: Vec::new(),
            completed_passengers: Vec::new(),
            buses_at_stop: Vec::new(),
        }
    }

    pub fn add_passenger(&mut self, passenger: Passenger) {
        self.waiting_passengers.push(passenger);
    }

    pub fn add_bus(&mut self, bus: Bus) {
        self.buses_at_stop.push(bus);
    }

    pub fn drain_bus(&mut self, bus_uid: usize) -> Bus {
        let bus_index = self
            .buses_at_stop
            .iter()
            .position(|b| b.uid == bus_uid)
            .unwrap();
        self.buses_at_stop.remove(bus_index)
    }
}

impl Display for BusStop {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let display_buses = self
            .buses_at_stop
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(
            f,
            "[{}] ({}|{}) \t{}",
            self.name,
            self.waiting_passengers.len(),
            self.completed_passengers.len(),
            display_buses
        )
    }
}
