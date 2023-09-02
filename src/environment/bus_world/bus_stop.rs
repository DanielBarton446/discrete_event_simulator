use super::passenger::Passenger;

pub struct BusStop {
    name: String,
    waiting_passengers: Vec<Passenger>,
}

impl BusStop {
    pub fn new(name: String) -> BusStop {
        BusStop {
            name,
            waiting_passengers: Vec::new(),
        }
    }

    pub fn add_passenger(&mut self, passenger: Passenger) {
        self.waiting_passengers.push(passenger);
    }
}
