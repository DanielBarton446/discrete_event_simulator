use discrete_event_simulator::{
    environment::bus_world::bus_environment::BusEnvironment,
    environment::bus_world::bus_environment::BusEnvironmentSettings,
    environment::bus_world::{
        bus::Bus,
        bus_world_events::{
            import_bus::ImportBusEvent,
            new_bus::{NewBusEvent, NewBusesJson},
        },
    },
    genetic_learning::evolution::{Evolvable, Population},
    simulation::sim::Simulation,
};

// fn main() {
//     println!("Create a bus world simulation!");
//     let mut env = BusEnvironment::new(BusEnvironmentSettings::default());
//     let buses = NewBusesJson::new(5, 5);
//     let init_event = Box::new(NewBusEvent::new(
//         0,
//         0,
//         serde_json::to_string(&buses).unwrap(),
//     ));
//
//     env.create_bus_stops(5);
//     env.initialize_bus_stops_with_passengers(100);
//
//     let mut sim = Simulation::new(100, Box::new(env), init_event);
//     sim.play_movie(100);
//     // sim.run();
// }

fn main() {
    println!("Create a bus world simulation!");
    let mut buses = Vec::new();

    let mut env = BusEnvironment::new(BusEnvironmentSettings::default());
    env.create_bus_stops(5);
    env.initialize_bus_stops_with_passengers(100);

    for _ in 0..5 {
        let mut bus = Bus::new(5);
        for i in 0..5 {
            bus.add_serviced_stop(env.bus_stops[i].name.clone());
        }
        buses.push(bus);
    }

    let sim1_init_event = Box::new(ImportBusEvent::new(
        0,
        0,
        serde_json::to_string(&buses).unwrap(),
    ));
    let mut sim = Simulation::new(100, Box::new(env), sim1_init_event);
    // sim.play_movie(100);
    sim.run();

    let wait_time = match sim
        .statistics
        .get_series_by_name("Total Passenger Wait Time".to_string())
    {
        Some(series) => series,
        None => panic!("No series found"),
    };

    println!("{}", wait_time);

    let mut population = Population::new(buses.clone());
    match population.evolve() {
        Ok(_) => println!("Evolution successful!"),
        Err(e) => println!("Evolution failed: {}", e),
    }

    println!("Evolving 20 times because reasons");
    for _ in 0..20 {
        let _ = population.evolve();
    }

    let mut env2 = BusEnvironment::new(BusEnvironmentSettings::default());
    env2.create_bus_stops(5);
    env2.initialize_bus_stops_with_passengers(100);

    let sim2_init_event = Box::new(ImportBusEvent::new(
        0,
        0,
        serde_json::to_string(&buses).unwrap(),
    ));
    let mut sim2 = Simulation::new(100, Box::new(env2), sim2_init_event);
    // sim2.play_movie(100);
    sim2.run();

    let wait_time = match sim2
        .statistics
        .get_series_by_name("Total Passenger Wait Time".to_string())
    {
        Some(series) => series,
        None => panic!("No series found"),
    };

    println!("{}", wait_time);
}
