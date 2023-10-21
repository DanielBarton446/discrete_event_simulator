use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

use crate::des::scheduler::Scheduler;
use crate::environment::bus_world::bus::Bus;
use crate::environment::bus_world::bus_scenario_traits::{
    AdvanceVehicleHandler, NewVehicleHandler, PassengerTransportHandler,
};
use crate::environment::bus_world::bus_stop::BusStop;
use crate::environment::bus_world::bus_world_events::new_bus::NewBusesJson;
use crate::environment::bus_world::bus_world_events::{load_passengers::*, move_bus_to_stop::*};
use crate::environment::environment::Environment;
use crate::event::schedulable::SchedulableEvent;
use crate::statistics::data_point::DataPoint;
use crate::statistics::stats::Stats;

use super::bus_world_events::import_bus::ImportBusesJson;
use super::bus_world_events::move_bus_to_stop::BusToStopMappingJson;
use super::bus_world_events::terminal_event::TerminalEvent;
use super::bus_world_events::unload_passengers::{UnloadPassengersEvent, UnloadPassengersJson};
use super::passenger::Passenger;

use serde::Serialize;

enum BusEventTypes {
    TerminalEvent,
    ImportBus,
    NewBus,
    MoveBusToStop,
    LoadPassengers,
    UnloadPassengers,
}

impl FromStr for BusEventTypes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Terminal" => Ok(BusEventTypes::TerminalEvent),
            "ImportBus" => Ok(BusEventTypes::ImportBus),
            "NewBus" => Ok(BusEventTypes::NewBus),
            "MoveBusToStop" => Ok(BusEventTypes::MoveBusToStop),
            "LoadPassengers" => Ok(BusEventTypes::LoadPassengers),
            "UnloadPassengers" => Ok(BusEventTypes::UnloadPassengers),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Copy, Clone)]
pub struct BusEnvironmentSettings {
    pickup_delay: usize,
    drop_off_delay: usize,
    next_stop_delay: usize,
    initial_delay: usize,
}

impl Display for BusEnvironmentSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "pickup_delay: {}, drop_off_delay: {}, next_stop_delay: {}, initial_delay: {}",
            self.pickup_delay, self.drop_off_delay, self.next_stop_delay, self.initial_delay
        )
    }
}

impl BusEnvironmentSettings {
    pub fn new(
        pickup_delay: usize,
        drop_off_delay: usize,
        next_stop_delay: usize,
        initial_delay: usize,
    ) -> Self {
        BusEnvironmentSettings {
            pickup_delay,
            drop_off_delay,
            next_stop_delay,
            initial_delay,
        }
    }
}

impl Default for BusEnvironmentSettings {
    /// Settings are in Seconds
    /// Default Settings are:
    /// * `pickup_delay`: 60,
    /// * `drop_off_delay`: 30,
    /// * `next_stop_delay`: 1200,
    /// * `initial_delay`: 600,
    fn default() -> Self {
        BusEnvironmentSettings {
            pickup_delay: 60,
            drop_off_delay: 30,
            next_stop_delay: 600,
            initial_delay: 600,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct BusEnvironment {
    pub bus_stops: Vec<BusStop>,
    settings: BusEnvironmentSettings,
}

impl BusEnvironment {
    pub fn new(settings: BusEnvironmentSettings) -> BusEnvironment {
        BusEnvironment {
            bus_stops: Vec::new(),
            settings,
        }
    }

    pub fn add_bus_to_start(&mut self, bus: Bus) -> Result<(), String> {
        if let Some(stop) = self.bus_stops.first_mut() {
            stop.add_bus(bus);
            Ok(())
        } else {
            Err("No bus stops exist".to_string())
        }
    }

    /// Create and Distribute `count` passengers among all bus stops
    /// * `count` - the number of passengers to create
    pub fn initialize_bus_stops_with_passengers(&mut self, count: usize) {
        if self.bus_stops.is_empty() {
            panic!("Error: No bus stops exist");
        }
        let mut bus_stop_names: Vec<String> = Vec::new();
        for stop in self.bus_stops.iter() {
            bus_stop_names.push(stop.name.clone());
        }
        for i in 0..count {
            let passenger = Passenger::new_random_passenger(i, &bus_stop_names);
            // this could be much better, but lets deal with efficiencies later
            if let Some(initial_stop) = self
                .bus_stops
                .iter_mut()
                .find(|stop| stop.name == passenger.source)
            {
                initial_stop.add_passenger(passenger);
            }
        }
    }

    pub fn create_bus_stops(&mut self, count: usize) {
        for i in 0..count {
            self.bus_stops.push(BusStop::new(
                char::from_u32('A' as u32 + i as u32).unwrap().to_string(),
            ));
        }
    }

    fn find_mut_stop_by_name(&mut self, stop_name: &str) -> Option<&mut BusStop> {
        self.bus_stops
            .iter_mut()
            .find(|stop| stop.name == stop_name)
    }

    fn find_mut_stop_by_bus_uuid(&mut self, uuid: String) -> Option<&mut BusStop> {
        for stop in self.bus_stops.iter_mut() {
            let bus = stop.buses_at_stop.iter_mut().find(|bus| bus.uuid == uuid);
            if bus.is_some() {
                return Some(stop);
            }
        }
        return None;
    }

    fn drain_bus_by_uuid(&mut self, bus_uuid: String) -> Option<Bus> {
        for stop in &mut self.bus_stops {
            if stop
                .buses_at_stop
                .iter()
                .filter(|b| b.uuid == bus_uuid)
                .count()
                > 0
            {
                return Some(stop.drain_bus(bus_uuid));
            }
        }
        return None;
    }

    fn completed_passengers_wait_time(&self, timestamp: usize, stat_recorder: &mut Stats) -> usize {
        // wait time of completed passengers
        let completed_passengers_wait = self.bus_stops.iter().fold(0, |acc, stop| {
            acc + stop
                .completed_passengers
                .iter()
                .fold(0, |cum, passenger| cum + passenger.wait_time as usize)
        });
        let num_completed = self
            .bus_stops
            .iter()
            .fold(0, |acc, stop| acc + stop.completed_passengers.len());
        stat_recorder.add_statistic(
            DataPoint::new(timestamp, num_completed as f64, "count".to_string()),
            "Completed Passengers".to_string(),
        );

        completed_passengers_wait
    }

    fn pending_passengers_wait_time(&self, timestamp: usize, stat_recorder: &mut Stats) -> usize {
        // Wait time of passengers still on buses
        let mut pending_passengers = 0;
        let mut pending_wait_time = 0;
        for stop in &self.bus_stops {
            for bus in &stop.buses_at_stop {
                // println!("Bus: {}", bus.uuid);
                // println!("Passengers: {:?}", bus.passengers);
                pending_passengers += bus
                    .passengers
                    .values()
                    .fold(0, |acc, passengers| acc + passengers.len());

                pending_wait_time += bus.passengers.values().fold(0, |acc, passengers| {
                    acc + passengers
                        .iter()
                        .fold(0, |cum, passenger| cum + passenger.wait_time as usize)
                });
            }
        }
        stat_recorder.add_statistic(
            DataPoint::new(timestamp, pending_passengers as f64, "count".to_string()),
            "Pending Passengers".to_string(),
        );
        pending_wait_time
    }

    fn forgotten_passengers_wait_time(&self, timestamp: usize, stat_recorder: &mut Stats) -> usize {
        // wait time of passengers who have not got on buses yet
        let mut forgotten_passengers = 0;
        let mut forgotten_passenger_wait_time = 0;
        for stop in &self.bus_stops {
            for passengers in stop.waiting_passengers.values() {
                forgotten_passenger_wait_time += passengers.len() * timestamp;
                forgotten_passengers += passengers.len();
            }
        }
        stat_recorder.add_statistic(
            DataPoint::new(timestamp, forgotten_passengers as f64, "count".to_string()),
            "Forgotten Passengers".to_string(),
        );
        forgotten_passenger_wait_time
    }

    pub fn record_total_wait_time(&self, timestamp: usize, stat_recorder: &mut Stats) {
        let mut total_wait_time = 0;
        // let completed_passengers_wait =
        //     self.completed_passengers_wait_time(timestamp, stat_recorder);
        let pending_passengers_wait = self.pending_passengers_wait_time(timestamp, stat_recorder);
        let forgotten_passengers = self.forgotten_passengers_wait_time(timestamp, stat_recorder);
        // total_wait_time +=
        //     completed_passengers_wait + pending_passengers_wait + forgotten_passengers;
        total_wait_time += pending_passengers_wait + forgotten_passengers;

        stat_recorder.add_statistic(
            DataPoint::new(timestamp, total_wait_time as f64, "seconds".to_string()),
            "Total Passenger Wait Time".to_string(),
        );
    }

    fn terminate_bus_sim(&mut self, stat_recorder: &mut Stats, event: Box<dyn SchedulableEvent>) {
        let timestamp = event.get_time_stamp();
        for stop in &mut self.bus_stops {
            for bus in &mut stop.buses_at_stop {
                for bus_load in bus.passengers.values_mut() {
                    for passenger in bus_load.iter_mut() {
                        passenger.wait_time += timestamp as u32;
                    }
                }
            }
        }

        self.record_total_wait_time(timestamp, stat_recorder);
    }
}

impl Default for BusEnvironment {
    fn default() -> Self {
        Self::new(BusEnvironmentSettings::default())
    }
}

impl Environment for BusEnvironment {
    fn apply_event(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    ) {
        match BusEventTypes::from_str(event.get_event_type()) {
            Ok(BusEventTypes::TerminalEvent) => {
                self.terminate_bus_sim(stat_recorder, event);
            }
            Ok(BusEventTypes::ImportBus) => {
                self.import_buses(scheduler, stat_recorder, event);
            }
            Ok(BusEventTypes::NewBus) => {
                self.create_new_bus(scheduler, stat_recorder, event);
            }
            Ok(BusEventTypes::MoveBusToStop) => {
                self.advance_bus_to_next_stop(scheduler, stat_recorder, event);
            }
            Ok(BusEventTypes::LoadPassengers) => {
                self.load_passengers(scheduler, stat_recorder, event);
            }
            Ok(BusEventTypes::UnloadPassengers) => {
                self.unload_passengers(scheduler, stat_recorder, event);
            }
            Err(()) => {
                panic!("Error: Unknown event type {}", event.get_event_type())
            }
        }
    }
    fn get_state(&self) -> String {
        if let Ok(serialized) = serde_json::to_string(&self.bus_stops) {
            return serialized;
        }
        String::new()
    }

    fn terminating_event(&self, scheduler: &mut Scheduler) -> Box<dyn SchedulableEvent> {
        Box::new(TerminalEvent::new(
            usize::MAX,
            scheduler.runtime,
            String::new(),
        ))
    }
}

impl PassengerTransportHandler for BusEnvironment {
    fn load_passengers(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    ) {
        let bus_uuid = serde_json::from_str::<LoadPassengersJson>(&event.get_data().unwrap())
            .expect("Error: Could not deserialize bus mapping")
            .bus_uuid;
        let passenger_pickup_delay = self.settings.pickup_delay;
        let stop = self.find_mut_stop_by_bus_uuid(bus_uuid.clone()).unwrap();
        let bus_at_stop = stop
            .buses_at_stop
            .iter_mut()
            .find(|b| b.uuid == bus_uuid)
            .unwrap(); // unwrap bad.

        let mut onboarded_passengers_count = 0;
        for key in &bus_at_stop.serviced_stop_names.clone() {
            if let Some(tentative_onboarders) = stop.waiting_passengers.get_mut(key) {
                while !tentative_onboarders.is_empty()
                    && bus_at_stop.current_passenger_count() < bus_at_stop.capacity
                {
                    bus_at_stop.add_passenger(tentative_onboarders.pop().unwrap());
                    onboarded_passengers_count += 1;
                    // unwrap bad
                }
            }
        }

        // Schedule advance to next stop if exists
        if let Some(next_stop) = bus_at_stop.get_next_stop() {
            let advance_to_next_stop_data = BusToStopMappingJson::new(bus_uuid, next_stop.clone());
            let advance_to_next_stop_event = Box::new(MoveBusToStopEvent::new(
                event.get_uid() + 1,
                event.get_time_stamp() + (onboarded_passengers_count * passenger_pickup_delay),
                serde_json::to_string(&advance_to_next_stop_data).unwrap(),
            ));
            scheduler.add_event(advance_to_next_stop_event);
        }

        // Stats, report the count of passengers onboarded
        let data_point = DataPoint::new(
            event.get_time_stamp(),
            onboarded_passengers_count as f64,
            "passengers loaded".to_string(),
        );
        stat_recorder.add_statistic(
            data_point,
            format!("Bus {}: Passengers Loaded", bus_at_stop.uuid),
        );
    }

    fn unload_passengers(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    ) {
        let bus_uuid = serde_json::from_str::<LoadPassengersJson>(&event.get_data().unwrap())
            .expect("Error: Could not deserialize bus mapping")
            .bus_uuid;
        let mut unloaded_passenger_count = 0;
        if let Some(stop) = self.find_mut_stop_by_bus_uuid(bus_uuid.clone()) {
            let bus_at_stop = stop
                .buses_at_stop
                .iter_mut()
                .find(|b| b.uuid == bus_uuid)
                .unwrap(); // unwrap bad.
            if let Some(passengers_getting_off) = bus_at_stop.passengers.get_mut(stop.name.as_str())
            {
                unloaded_passenger_count = passengers_getting_off.len();
                for p in passengers_getting_off.iter_mut() {
                    p.wait_time += event.get_time_stamp() as u32;
                }
                // println!(
                //     "Bus: {} | Unloading {} passengers",
                //     bus_at_stop.uuid,
                //     passengers_getting_off.len()
                // );
                stop.completed_passengers.append(passengers_getting_off);
                // println!(
                //     "Stop: {} | Completed Passengers: {}",
                //     stop.name,
                //     stop.completed_passengers.len()
                // );
            }

            // Schedule loading passengers after we unloaded passengers
            let load_passengers_data = LoadPassengersJson::new(bus_uuid);
            let load_bus_event = Box::new(LoadPassengersEvent::new(
                event.get_uid() + 1,
                event.get_time_stamp() + (unloaded_passenger_count * 4),
                serde_json::to_string(&load_passengers_data).unwrap(),
            ));
            scheduler.add_event(load_bus_event);

            // Report stats on how many passengers were unloaded
            let data_point = DataPoint::new(
                event.get_time_stamp(),
                unloaded_passenger_count as f64,
                String::from("Passengers (ct)"),
            );
            stat_recorder.add_statistic(
                data_point,
                format!("Bus {}: Passengers Unloaded", bus_at_stop.uuid),
            );
        }
    }
}

impl AdvanceVehicleHandler for BusEnvironment {
    fn advance_bus_to_next_stop(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    ) {
        // // This should be handled and not unwrapped, but whatever
        let bus_and_new_stop =
            serde_json::from_str::<BusToStopMappingJson>(&event.get_data().unwrap()).unwrap();

        // find and drain the bus we are looking for and do something with it later
        // unwrapping is bad
        let mut bus = self.drain_bus_by_uuid(bus_and_new_stop.bus_uuid).unwrap();

        // Advance the bus to the current stop(advanced by 1 stop)
        bus.advance_to_next_stop();

        // Schedule unloading passengers after we arrive at the next stop
        let unload_passengers_data = UnloadPassengersJson::new(bus.uuid.clone());
        let unload_passengers_event = Box::new(UnloadPassengersEvent::new(
            event.get_uid() + 1,
            event.get_time_stamp() + self.settings.next_stop_delay,
            serde_json::to_string(&unload_passengers_data).unwrap(),
        ));
        scheduler.add_event(unload_passengers_event);

        // find the stop we are looking for and add the bus to it
        // unwrapping is bad
        let stop = match self.find_mut_stop_by_name(&bus_and_new_stop.stop_name) {
            Some(stop) => stop,
            None => panic!("Error: Stop {} not found", bus_and_new_stop.stop_name),
        };

        // Finally, add the bus to the current stop.
        stop.add_bus(bus);

        // Stats, report the count of buses at this stop now
        let data_point = DataPoint::new(
            event.get_time_stamp(),
            stop.buses_at_stop.len() as f64,
            "bus_count".to_string(),
        );
        stat_recorder.add_statistic(data_point, format!("stop {}: buses", stop.name));
    }
}

impl NewVehicleHandler for BusEnvironment {
    fn create_new_bus(
        &mut self,
        scheduler: &mut Scheduler,
        _stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    ) {
        let bus_mapping = serde_json::from_str::<NewBusesJson>(&event.get_data().unwrap())
            .expect("Error: Could not deserialize bus mapping");

        for _ in 0..bus_mapping.number_of_buses {
            let mut bus = Bus::new(bus_mapping.capacity);

            for stop in &mut self.bus_stops {
                bus.add_serviced_stop(stop.name.clone());
            }

            let _bus_routing = match bus.get_next_stop() {
                Some(next_stop) => {
                    BusToStopMappingJson::new(bus.uuid.clone(), next_stop.to_string())
                }
                None => BusToStopMappingJson::new(
                    bus.uuid.clone(),
                    bus.get_current_stop().unwrap().to_string(),
                ),
            };

            // Start the Unload -> Load -> Advance Bus cycle
            let schedule_load_passengers = Box::new(UnloadPassengersEvent::new(
                event.get_uid() + 1,
                event.get_time_stamp() + self.settings.initial_delay,
                serde_json::to_string(&UnloadPassengersJson::new(bus.uuid.clone())).unwrap(),
            ));
            scheduler.add_event(schedule_load_passengers);

            // Add bus to the stop
            self.bus_stops[0].add_bus(bus);
        }
    }

    fn import_buses(
        &mut self,
        scheduler: &mut Scheduler,
        _stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    ) {
        let imported_buses = serde_json::from_str::<ImportBusesJson>(&event.get_data().unwrap())
            .expect("Error: Could not deserialize imported buses");

        for bus in imported_buses.buses {
            let _bus_routing = match bus.get_next_stop() {
                Some(next_stop) => {
                    BusToStopMappingJson::new(bus.uuid.clone(), next_stop.to_string())
                }
                None => BusToStopMappingJson::new(
                    bus.uuid.clone(),
                    match bus.get_current_stop() {
                        Some(stop) => stop.to_string(),
                        None => {
                            panic!("Error: Serviced Stops: {:?}", bus.serviced_stop_names)
                        }
                    },
                ),
            };

            // Start the Unload -> Load -> Advance Bus cycle
            let schedule_load_passengers = Box::new(UnloadPassengersEvent::new(
                event.get_uid() + 1,
                event.get_time_stamp() + self.settings.initial_delay,
                serde_json::to_string(&UnloadPassengersJson::new(bus.uuid.clone())).unwrap(),
            ));
            scheduler.add_event(schedule_load_passengers);

            // Add bus to the stop, based on what the bus' current stop
            self.bus_stops
                .iter_mut()
                .find(|stop| stop.name == *bus.get_current_stop().unwrap())
                .unwrap()
                .add_bus(bus);
            // self.bus_stops[0].add_bus(bus);
        }
    }
}

impl Display for BusEnvironment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Bus Stops:\tBuses:")?;
        for stop in self.bus_stops.iter() {
            write!(f, "{} ", &stop)?;
            writeln!(f)?;
        }

        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BusEnvironment;
    use crate::des::scheduler::Scheduler;
    use crate::environment::bus_world::bus_environment::BusEnvironmentSettings;
    use crate::environment::bus_world::bus_world_events::new_bus::NewBusesJson;
    use crate::{
        environment::bus_world::bus_world_events::new_bus::NewBusEvent,
        environment::environment::Environment, statistics::stats::Stats,
    };

    #[test]
    fn create_bus_world() {
        let mut bus_world = BusEnvironment::new(BusEnvironmentSettings::default());
        let mut scheduler = Scheduler::new(100);
        let mut stats_recorder = Stats::new();
        assert_eq!(bus_world.bus_stops.len(), 0);
        bus_world.create_bus_stops(1);
        let number_of_buses = NewBusesJson::new(1, 5);
        let event = Box::new(NewBusEvent::new(
            1,
            0,
            serde_json::to_string(&number_of_buses).unwrap(),
        ));
        bus_world.apply_event(&mut scheduler, &mut stats_recorder, event);
        assert_eq!(bus_world.bus_stops.len(), 1);
        assert_eq!(bus_world.bus_stops[0].buses_at_stop.len(), 1);
    }
}
