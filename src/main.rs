use discrete_event_simulator::environment::bus_world::utils::{
    create_n_buses_m_capacity, display_wait_time, evolve_buses_n_times, new_basic_bus_sim_m_stops,
};

fn main() {
    println!("Create a bus world simulation!");
    let mut buses = create_n_buses_m_capacity(5, 5);

    let mut sim = new_basic_bus_sim_m_stops(100, &mut buses, 10);
    // sim.play_movie(100);
    sim.run();
    display_wait_time(&sim);

    let population = evolve_buses_n_times(&buses, 20);
    let mut sim2 = new_basic_bus_sim_m_stops(100, &mut population.populace.clone(), 10);
    // sim2.play_movie(100);
    sim2.run();
    display_wait_time(&sim2);
}
