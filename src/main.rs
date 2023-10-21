use discrete_event_simulator::{
    environment::{
        bus_world::{
            bus::Bus,
            bus_environment::BusEnvironmentSettings,
            utils::{
                add_bus_stops_from_env_to_buses, create_bus_env, create_n_buses_m_capacity,
                evolve_buses_n_times, get_wait_time, new_sim,
            },
        },
        env::Environment,
    },
    genetic_learning::charting::draw_data,
    statistics::stats::Stats,
};

/// Settings are in seconds
struct EvolveBusSimSettings {
    runtime: usize,
    num_buses: usize,
    bus_capacity: usize,
    num_stops: usize,
    total_passengers: usize,
    num_evolutions: usize,
    env_settings: BusEnvironmentSettings,
}
impl Default for EvolveBusSimSettings {
    /// Note, all values are in seconds.
    /// Default values are:
    /// `runtime`: 14400,
    /// `num_buses`: 10,
    /// `bus_capacity`: 50,
    /// `num_stops`: 10,
    /// `total_passengers`: 1000,
    /// `num_evolutions`: 100,
    /// `env_settings`: [BusEnvironmentSettings::default()],
    fn default() -> Self {
        Self {
            runtime: 14400,
            num_buses: 10,
            bus_capacity: 100,
            num_stops: 10,
            total_passengers: 1500,
            num_evolutions: 100,
            env_settings: BusEnvironmentSettings::default(),
        }
    }
}

fn print_top_info(
    n: usize,
    scores: &[(f64, f64)],
    configurations: &[Vec<Bus>],
    environments: &Vec<Box<dyn Environment>>,
    stats: &Vec<Stats>,
) {
    let mut top_pairs = Vec::<(f64, usize)>::new();
    for (i, score) in scores.iter().enumerate() {
        top_pairs.push((score.1, i));
    }
    top_pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for i in 0..n {
        let index = top_pairs[i].1;
        let val = top_pairs[i].0;
        println!("Top {} number of evolutions:", i + 1);
        println!("{}\n", index);
        println!("Top {} score(total waiting time):", i + 1);
        println!("{}\n", val);
        println!("Top {} configuration:", i + 1);
        for bus in &configurations[index] {
            println!("{:?}", bus);
        }
        println!();
        println!("Top {} environment:", i + 1,);
        println!("{}", environments[index]);
        println!("{}", stats[index]);
    }
}

fn chart_evolutions(settings: EvolveBusSimSettings) {
    let mut buses = create_n_buses_m_capacity(settings.num_buses, settings.bus_capacity);
    let mut evolution_scores = Vec::<(f64, f64)>::new();
    let env = create_bus_env(
        settings.num_stops,
        settings.total_passengers,
        settings.env_settings,
    );
    // Need to initialize the first set of bus stops the buses service
    add_bus_stops_from_env_to_buses(&mut buses, &env);

    let mut configurations = Vec::<Vec<Bus>>::new();
    let mut environments = Vec::<Box<dyn Environment>>::new();
    let mut statistics_all = Vec::<Stats>::new();
    for evolution_count in 0..settings.num_evolutions {
        // need clone for consistency from starting serviced stops
        let mut population = evolve_buses_n_times(&buses.clone(), evolution_count);
        let cloned_env = env.clone();
        let mut sim = new_sim(settings.runtime, &mut population.populace, cloned_env);

        sim.run();

        evolution_scores.push((evolution_count as f64, get_wait_time(&sim)));
        configurations.push(population.populace);
        environments.push(sim.environment);
        statistics_all.push(sim.statistics);
    }

    // print initial env
    println!("Initial environment:");
    println!("{}", env);
    print_top_info(
        1,
        &evolution_scores,
        &configurations,
        &environments,
        &statistics_all,
    );

    match draw_data(
        format!("{}_Evolutions", settings.num_evolutions),
        evolution_scores,
        (1920, 1080),
    ) {
        Ok(_) => println!("Charted {} evolutions!", settings.num_evolutions),
        Err(e) => println!("Error charting: {}", e),
    }
}

fn main() {
    let settings = EvolveBusSimSettings::default();
    chart_evolutions(settings);
}
