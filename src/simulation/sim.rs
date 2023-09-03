use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use crossterm::{cursor, execute, terminal};

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
        while let Some(event) = self.scheduler.next_event() {
            self.environment.apply_event(&mut self.scheduler, event);
        }
    }

    pub fn play_movie(&mut self, delay_millis: u64) {
        while let Some(event) = self.scheduler.next_event() {
            self.environment.apply_event(&mut self.scheduler, event);
            // Clear screen for animation
            let _ = execute!(io::stdout(), terminal::Clear(terminal::ClearType::All));
            // hide cursor
            let _ = execute!(io::stdout(), cursor::Hide);

            println!("Current Time: {}", self.scheduler.current_time);
            print!("\r{}", self.environment);

            io::stdout().flush().unwrap();

            thread::sleep(Duration::from_millis(delay_millis));
        }
    }

    pub fn add_arbitrary_event(&mut self, from: Box<dyn Event>) {
        self.scheduler.add_event(from);
    }
}

#[cfg(test)]
mod test {

    use crate::environment::bus_world::bus_environment::BusEnvironment;
    use crate::environment::bus_world::bus_world_events::new_bus::{NewBusEvent, NewBusesJson};
    use crate::simulation::sim::Simulation;

    #[test]
    fn simulation_run() {
        let number_of_buses = NewBusesJson::new(10, 15);
        let initial_event = Box::new(NewBusEvent::new(
            1,
            0.0,
            serde_json::to_string(&number_of_buses).unwrap(),
        ));
        let mut env = BusEnvironment::new();
        env.create_bus_stops(1);
        let mut simulation = Simulation::new(10.0, Box::new(env), initial_event);
        simulation.run();
        assert_eq!(simulation.environment.get_state(), "Number of buses: 10");
    }
}
