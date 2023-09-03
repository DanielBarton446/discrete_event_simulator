use std::fmt::{Display, Error, Formatter};

use fake::{faker::name::en::Name, Fake};
use rand::{seq::SliceRandom, Rng};

#[derive(Clone)]
pub struct Passenger {
    pub uid: usize,
    pub name: String,
    pub source: String,
    pub destination: String,
    pub location: String,
    pub wait_time: u32,
}

impl Passenger {
    pub fn new(id: usize, name: String, source: String, destination: String) -> Passenger {
        Passenger {
            uid: id,
            name,
            source: source.clone(),
            destination,
            location: source,
            wait_time: 0,
        }
    }

    pub fn new_random_passenger(id: usize, bus_stops: &Vec<String>) -> Passenger {
        // Assuming bus_stops is a Vec<BusStop>
        let mut rng = rand::thread_rng();

        let source_index = rng.gen_range(0..bus_stops.len() - 1);

        let source = bus_stops[source_index].clone();
        let dest = bus_stops[source_index + 1..]
            .choose(&mut rng)
            .cloned()
            .unwrap(); // don't panic pls

        let name: String = Name().fake();
        Passenger::new(id, name, source, dest)
    }

    pub fn at_destination(&self) -> bool {
        self.location == self.destination
    }
}

impl Display for Passenger {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Passenger {}", self.uid)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn create_passenger() {
        let p = super::Passenger::new(0, "John".to_string(), "A".to_string(), "B".to_string());
        assert_eq!(p.uid, 0);
        assert_eq!(p.name, "John");
        assert_eq!(p.source, "A");
        assert_eq!(p.destination, "B");
        assert_eq!(p.location, "A");
        assert_eq!(p.wait_time, 0);
    }
}
