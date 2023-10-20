use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    environment::bus_world::passenger::Passenger,
    genetic_learning::evolution::{Breedable, Dna, Fitness},
};
use serde_with::serde_as;

#[serde_as]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Bus {
    pub uuid: String,
    #[serde_as(as = "Vec<(_, _)>")]
    pub passengers: HashMap<String, Vec<Passenger>>,
    pub serviced_stop_names: Vec<String>,
    current_stop: usize,
    pub capacity: usize,
}

impl Bus {
    pub fn new(capacity: usize) -> Bus {
        Bus {
            uuid: Uuid::new_v4().to_string(),
            passengers: HashMap::new(),
            serviced_stop_names: Vec::new(),
            current_stop: 0,
            capacity,
        }
    }

    pub fn reset(&mut self) {
        self.passengers.clear();
        self.serviced_stop_names.clear();
        self.current_stop = 0;
    }

    pub fn add_passenger(&mut self, passenger: Passenger) {
        self.passengers
            .entry(passenger.destination.clone())
            .or_default()
            .push(passenger);
    }

    pub fn add_serviced_stop(&mut self, stop: String) {
        self.serviced_stop_names.push(stop);
    }

    pub fn remove_services_stop(&mut self, stop: String) -> Result<(), String> {
        let index = self
            .serviced_stop_names
            .iter()
            .position(|s| s == &stop)
            .ok_or(format!("Stop {} not found", stop))?;
        self.serviced_stop_names.remove(index);
        Ok(())
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

impl<T> Breedable<T> for Bus
where
    T: Dna,
{
    fn reproduce(&self, other: &Bus) -> Result<Bus, String> {
        let larger_capacity = if self.capacity > other.capacity {
            self.capacity
        } else {
            other.capacity
        };
        let mut child = Bus::new(larger_capacity);
        let rng = &mut thread_rng();
        // Get the bus which has more stops serviced
        let mut longer = &self.serviced_stop_names;
        let mut shorter = &other.serviced_stop_names;
        if other.serviced_stop_names.len() > longer.len() {
            longer = &other.serviced_stop_names;
            shorter = &self.serviced_stop_names;
        }

        for stop in longer {
            #[allow(clippy::if_same_then_else)] // clippy doesn't realize % chance in else clause
            if shorter.contains(stop) {
                child.add_serviced_stop(stop.to_string());
            }
            // Give us a chance to add it if its not in both parents anyways
            else if rng.gen_bool(0.5) {
                child.add_serviced_stop(stop.to_string());
            }
        }
        Ok(child)
    }

    fn mutate(&mut self) {
        // we gonna remove a stop
        let rng = &mut thread_rng();
        let stop =
            self.serviced_stop_names[rng.gen_range(0..self.serviced_stop_names.len())].clone();
        self.remove_services_stop(stop)
            .expect("Error removing stop. This should never happen");
    }
}

impl Dna for Bus {
    fn get_dna(&self) -> serde_json::Value {
        let dna_string = serde_json::to_string(&self);
        match dna_string {
            Ok(dna_string) => {
                if let Ok(e) = serde_json::from_str(&dna_string) {
                    e
                } else {
                    panic!("Error serializing from json string");
                }
            }
            Err(e) => panic!("Error serializing bus: {}", e),
        }
    }
    fn get_species(&self) -> &'static str {
        "bus"
    }
}

impl<F> Fitness<F> for Bus {
    fn evaluate_fitness(&self) -> f32 {
        10.0
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[Bus {}]", self.uuid)
    }
}
