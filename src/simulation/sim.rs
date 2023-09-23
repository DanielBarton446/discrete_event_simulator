use std::io::Write;
use std::time::Duration;
use std::{io, thread};

use crossterm::{cursor, execute, terminal};

use crate::des::des::Scheduler;
use crate::environment::environment::Environment;
use crate::event::event::Event;
use crate::statistics::data_point::DataPoint;
use crate::statistics::stats::Stats;

pub struct Simulation {
    scheduler: Scheduler,
    pub environment: Box<dyn Environment>,
    pub statistics: Stats,
}

impl Simulation {
    pub fn new(
        runtime: usize,
        environment: Box<dyn Environment>,
        initial_event: Box<dyn Event>,
    ) -> Self {
        let mut sim = Simulation {
            scheduler: Scheduler::new(runtime),
            environment,
            statistics: Stats::new(),
        };
        sim.scheduler.add_event(initial_event);
        return sim;
    }

    pub fn run(&mut self) {
        let mut event_count = 0;
        while let Some(event) = self.scheduler.next_event() {
            self.environment
                .apply_event(&mut self.scheduler, &mut self.statistics, event);
            event_count += 1;
        }
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
        // Display statistics:
        self.statistics
            .all_series
            .sort_by(|a, b| a.statistic_label.cmp(&b.statistic_label));
        for series in self.statistics.all_series.iter() {
            println!("{}", series);
        }
    }

    pub fn add_arbitrary_event(&mut self, from: Box<dyn Event>) {
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
        assert_eq!(simulation.environment.get_state(), "[{\"name\":\"A\",\"waiting_passengers\":{},\"completed_passengers\":[],\"buses_at_stop\":[]},{\"name\":\"B\",\"waiting_passengers\":{},\"completed_passengers\":[],\"buses_at_stop\":[]},{\"name\":\"C\",\"waiting_passengers\":{},\"completed_passengers\":[],\"buses_at_stop\":[{\"uid\":0,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":6,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":8,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":7,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":2,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":9,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":3,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":5,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":4,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15},{\"uid\":1,\"passengers\":{},\"serviced_stop_names\":[\"A\",\"B\",\"C\"],\"current_stop\":2,\"capacity\":15}]}]");
    }
}
