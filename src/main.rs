use discrete_event_simulator::{
    environment::bus_world::bus_environment::BusEnvironment,
    environment::bus_world::bus_world_events::new_bus::NewBusEvent, simulation::sim::Simulation,
};

fn main() {
    println!("Create a bus world simulation!");
    let env = BusEnvironment::new();
    let init_event = Box::new(NewBusEvent::new(
        0,
        0.0,
        String::from("create the first bus!"),
    ));
    let mut sim = Simulation::new(10.0, Box::new(env), init_event);
    sim.run();
}
