//! This module allows a simulation for any environment to be run.

use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use crossterm::{cursor, execute, terminal};

use crate::des::scheduler::Scheduler;
use crate::environment::env::Environment;
use crate::event::schedulable::SchedulableEvent;
use crate::statistics::data_point::DataPoint;
use crate::statistics::stats::Stats;

/// A simulation is a wrapper around a scheduler and an environment
/// in which the events in the scheduler are applied to.
/// The simulation also keeps track of statistics.
pub struct Simulation {
    scheduler: Scheduler,
    pub environment: Box<dyn Environment>,
    pub statistics: Stats,
}

impl Simulation {
    /// Creates a new simulation with the given runtime, environment, and initial event.
    /// The initial event is used to kick off the simulation. Note: it's important that
    /// events are self-scheduling, otherwise the simulation will stop after the initial event.
    ///
    /// ```
    /// use discrete_event_simulator::simulation::sim::Simulation;
    /// use discrete_event_simulator::environment::bus_world::bus_environment::*;
    /// use discrete_event_simulator::environment::bus_world::bus_world_events::*;
    /// use discrete_event_simulator::environment::bus_world::bus_world_events::new_bus::*;
    ///
    /// let number_of_buses = NewBusesJson::new(10, 15);
    /// let initial_event = Box::new(NewBusEvent::new(
    ///    1,
    ///    0,
    ///    serde_json::to_string(&number_of_buses).unwrap(),
    ///    ));
    /// let settings = BusEnvironmentSettings::default();
    /// let mut env = BusEnvironment::new(settings);
    /// // Env specific initialization
    /// env.create_bus_stops(3);
    ///
    /// // create and run our simulation
    /// let mut simulation = Simulation::new(100, Box::new(env), initial_event);
    /// simulation.run();
    ///
    /// // print out statistics
    /// println!("{}", simulation.statistics);
    /// ```
    pub fn new(
        runtime: usize,
        environment: Box<dyn Environment>,
        initial_event: Box<dyn SchedulableEvent>,
    ) -> Self {
        let mut sim = Simulation {
            scheduler: Scheduler::new(runtime),
            environment,
            statistics: Stats::new(),
        };
        sim.scheduler.add_event(initial_event);
        sim
    }

    fn terminal_event(&mut self) {
        let completion_event = self.environment.terminating_event(&mut self.scheduler);
        self.scheduler.add_event(completion_event);

        let last_event = self.scheduler.next_event().unwrap();
        self.environment
            .apply_event(&mut self.scheduler, &mut self.statistics, last_event);
    }

    pub fn run(&mut self) {
        let mut event_count = 0;
        while let Some(event) = self.scheduler.next_event() {
            self.environment
                .apply_event(&mut self.scheduler, &mut self.statistics, event);
            event_count += 1;
        }
        // Apply the terminating event
        self.terminal_event();

        let data_point = DataPoint::new(
            self.scheduler.current_time,
            event_count as f64,
            "Count".to_string(),
        );
        self.statistics
            .add_statistic(data_point, "Events Ran".to_string())
    }

    pub fn play_movie(&mut self, delay_millis: u64) {
        while let Some(event) = self.scheduler.next_event() {
            // Clear screen for animation
            let _ = execute!(io::stdout(), terminal::Clear(terminal::ClearType::All));
            // hide cursor
            let _ = execute!(io::stdout(), cursor::Hide);

            println!("Current Time: {}", self.scheduler.current_time);
            println!("Current Event: {}", event);
            print!("\r{}", self.environment);

            // apply the event
            self.environment
                .apply_event(&mut self.scheduler, &mut self.statistics, event);

            io::stdout().flush().unwrap();

            thread::sleep(Duration::from_millis(delay_millis));
        }
        // Apply the terminating event
        self.terminal_event();

        // Display statistics:
        self.statistics
            .all_series
            .sort_by(|a, b| a.statistic_label.cmp(&b.statistic_label));
        for series in self.statistics.all_series.iter() {
            println!("{}", series);
        }
    }

    pub fn add_arbitrary_event(&mut self, from: Box<dyn SchedulableEvent>) {
        self.scheduler.add_event(from);
    }
}

#[cfg(test)]
mod test {

    use crate::environment::bus_world::bus_environment::{BusEnvironment, BusEnvironmentSettings};
    use crate::environment::bus_world::bus_world_events::new_bus::{NewBusEvent, NewBusesJson};
    use crate::simulation::sim::Simulation;

    #[test]
    fn simulation_run() {
        let number_of_buses = NewBusesJson::new(10, 15);
        let initial_event = Box::new(NewBusEvent::new(
            1,
            0,
            serde_json::to_string(&number_of_buses).unwrap(),
        ));
        let settings = BusEnvironmentSettings::default();
        let mut env = BusEnvironment::new(settings);
        env.create_bus_stops(3);
        let mut simulation = Simulation::new(100, Box::new(env), initial_event);
        simulation.run();
        assert_eq!(1, 1);
    }
}
