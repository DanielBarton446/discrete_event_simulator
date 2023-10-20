use crate::genetic_learning::evolution::Evolvable;
use crate::genetic_learning::evolution::Population;
use crate::simulation::sim::Simulation;

use super::{
    bus::Bus,
    bus_environment::{BusEnvironment, BusEnvironmentSettings},
    bus_world_events::import_bus::ImportBusEvent,
};

pub fn create_bus_env(
    num_stops: usize,
    num_pass: usize,
    bus_env_settings: BusEnvironmentSettings,
) -> BusEnvironment {
    let mut env = BusEnvironment::new(bus_env_settings);
    env.create_bus_stops(num_stops);
    env.initialize_bus_stops_with_passengers(num_pass);
    env
}

pub fn add_bus_stops_from_env_to_buses(buses: &mut [Bus], env: &BusEnvironment) {
    for bus in buses.iter_mut() {
        for stop in env.bus_stops.iter() {
            bus.add_serviced_stop(stop.name.clone());
        }
    }
}

pub fn new_sim(runtime: usize, buses: &mut Vec<Bus>, env: BusEnvironment) -> Simulation {
    // println!("README: {:?}", buses);
    let sim_init_event = Box::new(ImportBusEvent::new(
        0,
        0,
        serde_json::to_string(&buses).unwrap(),
    ));
    Simulation::new(runtime, Box::new(env), sim_init_event)
}

pub fn create_n_buses_m_capacity(n: usize, capacity: usize) -> Vec<Bus> {
    let mut buses = Vec::new();

    for _ in 0..n {
        let bus = Bus::new(capacity);
        buses.push(bus);
    }

    buses
}

pub fn display_wait_time(sim: &Simulation) {
    let wait_time = match sim
        .statistics
        .get_series_by_name("Total Passenger Wait Time".to_string())
    {
        Some(series) => series,
        None => panic!("No series found"),
    };

    println!("{}", wait_time);
}

pub fn get_wait_time(sim: &Simulation) -> f64 {
    let wait_time = match sim
        .statistics
        .get_series_by_name("Total Passenger Wait Time".to_string())
    {
        Some(series) => series.get_last_value(),
        None => panic!("No series found"),
    };

    wait_time
}

pub fn evolve_buses_n_times(buses: &[Bus], n: usize) -> Population<Bus> {
    let mut population = Population::new(buses.to_vec());

    for _ in 0..n {
        match population.evolve() {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }
    }
    population
}
