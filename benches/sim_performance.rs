use std::fmt::{Display, Formatter};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

use discrete_event_simulator::{
    environment::bus_world::{
        bus_environment::{BusEnvironment, BusEnvironmentSettings},
        bus_world_events::new_bus::{NewBusEvent, NewBusesJson},
    },
    simulation::sim::Simulation,
};

// Set some meaningful values:
//  runtime unit: IRL Seconds
//
//  Real world configuration for times
//      Runtime: 4 hours (14400 seconds),
//      pickup_delay: 1 minute (60),
//      drop_off_delay: 30 seconds (30),
//      next_stop_delay: 20 minutes (1200),
//      initial_delay: 10 minutes (600),
//      bus_stops: 10 (takes 3:20 h:m to get to last stop)
//      bus_capacity: 50
//      passengers = 5(half of serviced buses) * 10(stops) * 50(capacity) = 2500
//
//

#[derive(Copy, Clone)]
struct Configuration {
    num_buses: usize,
    bus_stops: usize,
    bus_capacity: usize,
    passenger_count: usize,
    runtime: usize,
    settings: BusEnvironmentSettings,
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "buses: {}, stops: {}, capacity: {}, passengers: {}, runtime: {}, settings: {}",
            self.num_buses,
            self.bus_stops,
            self.bus_capacity,
            self.passenger_count,
            self.runtime,
            self.settings
        )
    }
}

fn incrementing_configs(c: &mut Criterion) {
    let mut binding = c.benchmark_group("incrementing_configs");
    let group = binding.sample_size(10);

    let settings = BusEnvironmentSettings::new(
        60,   // pickup_delay
        30,   // drop_off_delay
        1200, // next_stop_delay
        600,  // initial_delay
    );

    for i in 1..=100 {
        let config = Configuration {
            num_buses: 10 * (i) as usize,
            bus_stops: 10 * (i) as usize,
            bus_capacity: 10,
            passenger_count: 500 * i as usize,
            runtime: 14400,
            settings,
        };
        println!(
            "Config: {} \n {}",
            config,
            run_sim_from_config(config).unwrap()
        );
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}", i * 10)),
            &i,
            |b, &i| b.iter(|| run_sim_from_config(config)),
        );
    }
}

fn run_sim_from_config(config: Configuration) -> Option<String> {
    let mut env = BusEnvironment::new(config.settings);
    let buses = NewBusesJson::new(config.num_buses, config.bus_capacity);
    let init_event = Box::new(NewBusEvent::new(
        0, // uid
        0, // timestamp for first event
        serde_json::to_string(&buses).unwrap(),
    ));

    env.create_bus_stops(config.bus_stops);
    env.initialize_bus_stops_with_passengers(config.passenger_count);

    let mut sim = Simulation::new(config.runtime, Box::new(env), init_event);
    sim.run();
    if let Some(event_count_series) = sim
        .statistics
        .get_series_by_name(String::from("Events Ran"))
    {
        return Some(format!("{}", event_count_series));
    }
    None
}

criterion_group!(benches, incrementing_configs);
criterion_main!(benches);
