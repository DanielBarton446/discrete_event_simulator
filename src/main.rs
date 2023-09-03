use discrete_event_simulator::{
    environment::bus_world::bus_environment::BusEnvironment,
    environment::bus_world::bus_world_events::new_bus::{NewBusEvent, NewBusesJson},
    simulation::sim::Simulation,
};

fn main() {
    println!("Create a bus world simulation!");
    let mut env = BusEnvironment::new();
    let buses = NewBusesJson::new(5, 19);
    let init_event = Box::new(NewBusEvent::new(
        0,
        0.0,
        serde_json::to_string(&buses).unwrap(),
    ));

    env.create_bus_stops(5);
    env.initialize_bus_stops_with_passengers(100);

    let mut sim = Simulation::new(100.0, Box::new(env), init_event);
    sim.play_movie(300);
}
