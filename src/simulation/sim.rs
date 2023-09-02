use crate::des::des::Scheduler;
use crate::environment::environment::Environment;
use crate::event::event::Event;

pub struct Simulation {
    scheduler: Scheduler,
    pub environment: Box<dyn Environment>,
}

impl Simulation {
    pub fn new(
        runtime: f64,
        environment: Box<dyn Environment>,
        initial_event: Box<dyn Event>,
    ) -> Self {
        let mut sim = Simulation {
            scheduler: Scheduler::new(runtime),
            environment,
        };
        sim.scheduler.add_event(initial_event);
        return sim;
    }

    pub fn run(&mut self) {
        println!("Simulation::run()");

        while let Some(event) = self.scheduler.next_event() {
            self.environment.apply_event(&mut self.scheduler, event);
            println!("{}", self.environment);
        }
    }
}

#[cfg(test)]
mod test {

    use crate::environment::bus_world::bus_environment::BusEnvironment;
    use crate::environment::bus_world::bus_world_events::new_bus::NewBusEvent;
    use crate::simulation::sim::Simulation;

    #[test]
    fn simulation_run() {
        let initial_event = Box::new(NewBusEvent::new(1, 0.0, String::from("initial_bus_event")));
        let env = BusEnvironment::new();
        let mut simulation = Simulation::new(10.0, Box::new(env), initial_event);
        simulation.run();
        assert_eq!(simulation.environment.get_state(), "Number of buses: 11");
    }
}
