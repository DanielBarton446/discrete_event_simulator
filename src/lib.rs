mod statistics {
    pub mod data_point;
    pub mod stats;
    mod timeseries;
}

pub mod event {
    pub mod event;
}

mod des {
    pub mod des;
}

pub mod simulation {
    pub mod sim;
}

pub mod environment {
    pub mod bus_world {
        pub mod bus;
        pub mod bus_environment;
        pub mod bus_scenario_traits;
        pub mod bus_stop;
        pub mod passenger;
        pub mod bus_world_events {
            pub mod import_bus;
            pub mod load_passengers;
            pub mod move_bus_to_stop;
            pub mod new_bus;
            pub mod unload_passengers;
            pub mod terminal_event;
        }
    }
    pub mod environment;
}

/// Genetic learning module
/// Contains the genetic learning algorithm
/// and the traits that are needed to implement
/// genetic learning for generic populations.
pub mod genetic_learning {
    /// Defines traits for generic evolution:
    /// - TODO: add a description of how this generally is used
    pub mod evolution;
}
