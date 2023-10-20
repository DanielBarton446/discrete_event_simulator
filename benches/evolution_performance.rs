use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use discrete_event_simulator::{
    environment::bus_world::{
        bus::Bus,
        bus_environment::BusEnvironmentSettings,
        utils::{create_n_buses_m_capacity, evolve_buses_n_times, get_wait_time, new_sim},
    },
    genetic_learning::charting::draw_data,
};

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

fn evolve_then_run(config: &Configuration, buses: &[Bus], evolutions: usize) -> f64 {
    let population = evolve_buses_n_times(buses, evolutions);
    // let mut sim = new_basic_bus_sim_m_stops(
    //     config.runtime,
    //     &mut population.populace.clone(),
    //     config.bus_stops,
    // );
    // sim.run();
    // get_wait_time(&sim)
    0.0
}

fn run_sim_with_n_evolutions(c: &mut Criterion) {
    let mut binding = c.benchmark_group("incrementing_evolution_performance");
    let group = binding.sample_size(10);

    let settings = BusEnvironmentSettings::new(
        60,   // pickup_delay
        30,   // drop_off_delay
        1200, // next_stop_delay
        600,  // initial_delay
    );
    let config = Configuration {
        num_buses: 10,
        bus_stops: 10,
        bus_capacity: 50,
        passenger_count: 1000,
        runtime: 14400,
        settings,
    };

    let initial_buses = create_n_buses_m_capacity(config.num_buses, config.bus_capacity);

    let mut evolution_scores: HashMap<usize, Vec<f64>> = HashMap::new();
    for i in 0..=100 {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}", i)),
            &i,
            |b, &i| {
                b.iter(|| {
                    evolution_scores.entry(i).or_default().push(evolve_then_run(
                        &config,
                        &initial_buses,
                        i,
                    ))
                })
            },
        );
    }
    let averages = evolution_scores
        .iter()
        .map(|(evolutions, scores)| {
            (
                *evolutions,
                scores.iter().sum::<f64>() / scores.len() as f64,
            )
        })
        .collect::<Vec<(usize, f64)>>();
    let mut sorted_averages = averages.clone();
    sorted_averages.sort_by_key(|(evolutions, _)| *evolutions);
    let data = averages
        .iter()
        .map(|(evolutions, average)| (*evolutions as f64, *average))
        .collect::<Vec<(f64, f64)>>();
    draw_data("BenchmarkPerformance".to_string(), data, (1920, 1080)).unwrap();
    println!("Averages: {:?}", sorted_averages);
}

criterion_group!(benches, run_sim_with_n_evolutions);
criterion_main!(benches);
