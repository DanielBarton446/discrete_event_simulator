use discrete_event_simulator::{
    environment::bus_world::bus_environment::BusEnvironment,
    environment::bus_world::bus_environment::BusEnvironmentSettings,
    environment::bus_world::{
        bus::Bus,
        bus_world_events::new_bus::{NewBusEvent, NewBusesJson},
    },
    genetic_learning::bus_routing::evolution::{Evolution, Population},
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

    for i in 0..5 {
        let mut bus = Bus::new(i, 5);
        for i in 0..5 {
            bus.add_serviced_stop("stop".to_string() + &i.to_string());
        }
        buses.push(bus);
    }

    let population = Population::new(buses);
    let mut evolution = Evolution::new(population);
    evolution.evolve();

    // let mut sim = Simulation::new(100, Box::new(env), init_event);
    // sim.play_movie(100);
}
