use discrete_event_simulator::{
    environment::bus_world::bus_environment::BusEnvironment,
    environment::bus_world::bus_environment::BusEnvironmentSettings,
    environment::bus_world::{bus::Bus, bus_world_events::import_bus::ImportBusEvent},
    genetic_learning::evolution::{Evolvable, Population},
    simulation::sim::Simulation,
};

fn new_basic_bus_sim_m_stops(runtime: usize, buses: &mut Vec<Bus>, num_stops: usize) -> Simulation {
    let mut env = BusEnvironment::new(BusEnvironmentSettings::default());
    env.create_bus_stops(num_stops);
    env.initialize_bus_stops_with_passengers(100);

    for bus in buses.iter_mut() {
        for stop in env.bus_stops.iter() {
            bus.add_serviced_stop(stop.name.clone());
        }
    }

    let sim_init_event = Box::new(ImportBusEvent::new(
        0,
        0,
        serde_json::to_string(&buses).unwrap(),
    ));
    Simulation::new(runtime, Box::new(env), sim_init_event)
}

fn create_n_buses(n: usize) -> Vec<Bus> {
    let mut buses = Vec::new();

    for _ in 0..n {
        let bus = Bus::new(5);
        buses.push(bus);
    }

    buses
}

fn display_wait_time(sim: &Simulation) {
    let wait_time = match sim
        .statistics
        .get_series_by_name("Total Passenger Wait Time".to_string())
    {
        Some(series) => series,
        None => panic!("No series found"),
    };

    println!("{}", wait_time);
}

fn evolve_buses_n_times(buses: &[Bus], n: usize) -> Population<Bus> {
    let mut population = Population::new(buses.to_vec());

    for _ in 0..n {
        match population.evolve() {
            Ok(_) => (),
            Err(e) => panic!("Error: {}", e),
        }
    }
    population
}

fn main() {
    println!("Create a bus world simulation!");
    let mut buses = create_n_buses(5);

    let mut sim = new_basic_bus_sim_m_stops(100, &mut buses, 10);
    // sim.play_movie(100);
    sim.run();
    display_wait_time(&sim);

    let population = evolve_buses_n_times(&buses, 20);
    let mut sim2 = new_basic_bus_sim_m_stops(100, &mut population.populace.clone(), 10);
    // sim2.play_movie(100);
    sim2.run();
    display_wait_time(&sim2);
}
