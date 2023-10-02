use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

use rand::{thread_rng, Rng};
use serde::Serialize;
use serde_json::json;

use crate::{
    environment::bus_world::passenger::Passenger,
    genetic_learning::bus_routing::evolution::{Dna, DnaString, Fitness, Reproductor},
};

#[derive(Serialize, Clone)]
pub struct Bus {
    pub dna_type: String,
    pub uid: usize,
    pub passengers: HashMap<String, Vec<Passenger>>,
    pub serviced_stop_names: Vec<String>,
    current_stop: usize,
    pub capacity: usize,
}

impl Bus {
    pub fn new(uid: usize, capacity: usize) -> Bus {
        Bus {
            dna_type: String::from("bus"),
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

impl<T> Reproductor<T> for Bus {
    fn reproduce(&self, second: &serde_json::Value) -> Result<Bus, String> {
        let binding = self.get_dna();
        let self_dna_type = binding.get("dna_type");
        let second_dna_type = second.get("dna_type");

        if self_dna_type.is_none() {
            return Err(format!("Self DNA type is None. UID: {}", self.uid));
        }

        if second_dna_type.is_none() {
            return Err(format!("Second DNA type is None. UID: {}", self.uid));
        }

        if let Some(dna_type) = self_dna_type {
            if dna_type != "bus" {
                return Err(format!(
                    "Self not the right DNA type for buses {}. UID: {}",
                    dna_type, self.uid
                ));
            }
        }

        if self_dna_type != second_dna_type {
            return Err(format!(
                "Cannot reproduce between different types of DNA: {} and {}",
                self_dna_type.unwrap(),
                second_dna_type.unwrap()
            ));
        }

        // println!("{}", self_dna_type.unwrap());
        let mut bus = Bus::new(69, 100);
        let rng = &mut thread_rng();
        let stop = &self.serviced_stop_names[rng.gen_range(0..self.serviced_stop_names.len())];
        bus.add_serviced_stop(stop.to_string());
        return Ok(bus);
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
}

impl<F> Fitness<F> for Bus {
    fn evaluate_fitness(&self) -> f32 {
        0.0
    }
}

impl Display for Bus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "[Bus {}]", self.uid,)
    }
}
