pub mod statistics {
    //! Statistics module used to record custom
    //! timeseries metrics and events.
    pub mod data_point;
    pub mod stats;
    pub mod timeseries;
}

pub mod event {
    //! Event module used to define a generic event that
    //! can be utilized by the scheduler in the des module.
    pub mod schedulable;
}

pub mod des {
    //! Discrete event simulation module.
    pub mod scheduler;
}
pub mod simulation {
    //! Simulation module used to define a generic simulation
    //! for any environment.
    pub mod sim;
}

pub mod environment {
    //! Environment module used to define a generic environment
    //! that can be used by the simulation module.
    pub mod bus_world {
        //! Bus world environment module. This is an example environment
        //! that can be used to simulate a bus world.
        pub mod bus;
        pub mod bus_environment;
        pub mod bus_scenario_traits;
        pub mod bus_stop;
        pub mod passenger;
        pub mod utils;
        pub mod bus_world_events {
            pub mod import_bus;
            pub mod load_passengers;
            pub mod move_bus_to_stop;
            pub mod new_bus;
            pub mod terminal_event;
            pub mod unload_passengers;
        }
    }
    pub mod env;
}

pub mod genetic_learning {
    //! Genetic learning module.
    //! Contains the genetic learning algorithm
    //! and the traits that are needed to implement
    //! genetic learning for generic populations.
    pub mod charting;
    pub mod evolution;
}
