use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

use crate::des::des::Scheduler;
use crate::environment::bus_world::bus::Bus;
use crate::environment::bus_world::bus_stop::BusStop;
use crate::environment::bus_world::bus_world_events::new_bus::NewBusesJson;
use crate::environment::bus_world::bus_world_events::{load_passengers::*, move_bus_to_stop::*};
use crate::environment::environment::Environment;
use crate::event::event::Event;

use super::bus_world_events::move_bus_to_stop::BusToStopMappingJson;
use super::bus_world_events::unload_passengers::{UnloadPassengersEvent, UnloadPassengersJson};
use super::passenger::Passenger;

enum BusEventTypes {
    NewBus,
    MoveBusToStop,
    LoadPassengers,
    UnloadPassengers,
}

impl FromStr for BusEventTypes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NewBus" => Ok(BusEventTypes::NewBus),
            "MoveBusToStop" => Ok(BusEventTypes::MoveBusToStop),
            "LoadPassengers" => Ok(BusEventTypes::LoadPassengers),
            "UnloadPassengers" => Ok(BusEventTypes::UnloadPassengers),
            _ => Err(()),
        }
    }
}

pub struct BusEnvironment {
    bus_stops: Vec<BusStop>,
}

impl BusEnvironment {
    pub fn new() -> BusEnvironment {
        BusEnvironment {
            bus_stops: Vec::new(),
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

    fn find_mut_stop_by_bus_uid(&mut self, uid: usize) -> Option<&mut BusStop> {
        for stop in self.bus_stops.iter_mut() {
            let bus = stop.buses_at_stop.iter_mut().find(|bus| bus.uid == uid);
            if bus.is_some() {
                return Some(stop);
            }
        }
        return None;
    }

    fn drain_bus_by_uid(&mut self, bus_uid: usize) -> Option<Bus> {
        for stop in &mut self.bus_stops {
            if stop
                .buses_at_stop
                .iter()
                .filter(|b| b.uid == bus_uid)
                .count()
                > 0
            {
                return Some(stop.drain_bus(bus_uid));
            }
        }
        return None;
    }
}

impl Environment for BusEnvironment {
    fn apply_event(&mut self, scheduler: &mut Scheduler, event: Box<dyn Event>) {
        match BusEventTypes::from_str(event.get_event_type()) {
            Ok(BusEventTypes::NewBus) => {
                self.apply_new_bus_event(scheduler, event);
            }
            Ok(BusEventTypes::MoveBusToStop) => {
                self.apply_move_bus_to_stop_event(scheduler, event);
            }
            Ok(BusEventTypes::LoadPassengers) => {
                self.apply_load_passengers_event(scheduler, event);
            }
            Ok(BusEventTypes::UnloadPassengers) => {
                self.apply_unload_passengers_event(scheduler, event);
            }
            Err(()) => {
                panic!("Error: Unknown event type {}", event.get_event_type())
            }
        }
    }
    fn get_state(&self) -> String {
        let mut number_of_buses = 0;
        for bus_stop in &self.bus_stops {
            number_of_buses += bus_stop.buses_at_stop.len();
        }
        format!("Number of buses: {}", number_of_buses)
    }
}

impl BusEnvironment {
    fn apply_new_bus_event(&mut self, scheduler: &mut Scheduler, event: Box<dyn Event>) {
        let bus_mapping = serde_json::from_str::<NewBusesJson>(&event.get_data().unwrap())
            .expect("Error: Could not deserialize bus mapping");

        for i in 0..bus_mapping.number_of_buses {
            let mut bus = Bus::new(i, bus_mapping.capacity);

            for stop in &mut self.bus_stops {
                bus.add_serviced_stop(stop.name.clone());
            }
            // again unwrapping bad
            let bus_routing = match bus.get_next_stop() {
                Some(next_stop) => BusToStopMappingJson::new(bus.uid, next_stop.to_string()),
                None => {
                    BusToStopMappingJson::new(bus.uid, bus.get_current_stop().unwrap().to_string())
                }
            };

            // Load initial bus passengers at first stop
            let schedule_load_passengers = Box::new(LoadPassengersEvent::new(
                event.get_uid() + 1,
                event.get_time_stamp() + 1,
                serde_json::to_string(&LoadPassengersJson::new(bus.uid)).unwrap(),
            ));
            scheduler.add_event(schedule_load_passengers);

            // Add bus to the stop
            self.bus_stops[0].add_bus(bus);

            // Schedule the bus to move to the next stop
            let start_bus_route = Box::new(MoveBusToStopEvent::new(
                event.get_uid() + 1,
                event.get_time_stamp() + 5, // This should be something long for moving between stops
                serde_json::to_string(&bus_routing).unwrap(),
            ));

            scheduler.add_event(start_bus_route);
        }
    }

    fn apply_move_bus_to_stop_event(&mut self, scheduler: &mut Scheduler, event: Box<dyn Event>) {
        // // This should be handled and not unwrapped, but whatever
        let bus_and_new_stop =
            serde_json::from_str::<BusToStopMappingJson>(&event.get_data().unwrap()).unwrap();
        // find and drain the bus we are looking for and do something with it later
        // unwrapping is bad
        let mut bus = self.drain_bus_by_uid(bus_and_new_stop.bus_uid).unwrap();
        // find the stop we are looking for and add the bus to it
        // unwrapping is bad
        let stop = self
            .find_mut_stop_by_name(&bus_and_new_stop.stop_name)
            .unwrap();

        // Need to schedule the bus unloading passengers at this stop
        let schedule_unload_passengers = Box::new(UnloadPassengersEvent::new(
            event.get_uid() + 1,
            event.get_time_stamp(),
            serde_json::to_string(&UnloadPassengersJson::new(bus.uid)).unwrap(),
        ));
        scheduler.add_event(schedule_unload_passengers);

        // Need to scheudle bus loading passengers at this stop
        let schedule_load_passengers = Box::new(LoadPassengersEvent::new(
            event.get_uid() + 1,
            event.get_time_stamp() + 1,
            serde_json::to_string(&LoadPassengersJson::new(bus.uid)).unwrap(),
        ));
        scheduler.add_event(schedule_load_passengers);

        // Schedule this bus to move to the next stop unless its at the end of the line
        if let Some(next_stop) = bus.get_next_stop() {
            let mapping = BusToStopMappingJson::new(bus.uid, next_stop.to_string());
            let next_event = Box::new(MoveBusToStopEvent::new(
                event.get_uid() + 1, // this is really bad....
                event.get_time_stamp() + 5,
                serde_json::to_string(&mapping).unwrap(),
            ));
            scheduler.add_event(next_event);
        }

        // Advance the bus to the next stop, as we have arrived to the current stop
        bus.advance_to_next_stop();

        // Finally, add the bus to the current stop.
        stop.add_bus(bus);
    }

    fn apply_load_passengers_event(&mut self, _scheduler: &mut Scheduler, event: Box<dyn Event>) {
        let bus_uid = serde_json::from_str::<LoadPassengersJson>(&event.get_data().unwrap())
            .expect("Error: Could not deserialize bus mapping")
            .bus_uid;
        let stop = self.find_mut_stop_by_bus_uid(bus_uid).unwrap();
        let bus_at_stop = stop
            .buses_at_stop
            .iter_mut()
            .find(|b| b.uid == bus_uid)
            .unwrap(); // unwrap bad.
        for key in &bus_at_stop.serviced_stop_names.clone() {
            if let Some(tentative_onboarders) = stop.waiting_passengers.get_mut(key) {
                while !tentative_onboarders.is_empty()
                    && bus_at_stop.current_passenger_count() < bus_at_stop.capacity
                {
                    bus_at_stop.add_passenger(tentative_onboarders.pop().unwrap());
                    // unwrap bad
                }
            }
        }
    }

    fn apply_unload_passengers_event(&mut self, _scheduler: &mut Scheduler, event: Box<dyn Event>) {
        let bus_uid = serde_json::from_str::<LoadPassengersJson>(&event.get_data().unwrap())
            .expect("Error: Could not deserialize bus mapping")
            .bus_uid;
        if let Some(stop) = self.find_mut_stop_by_bus_uid(bus_uid) {
            let bus_at_stop = stop
                .buses_at_stop
                .iter_mut()
                .find(|b| b.uid == bus_uid)
                .unwrap(); // unwrap bad.
            if let Some(passengers_getting_off) = bus_at_stop.passengers.get_mut(stop.name.as_str())
            {
                stop.completed_passengers.append(passengers_getting_off);
            }
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
    use crate::des::des::Scheduler;
    use crate::environment::bus_world::bus_world_events::new_bus::NewBusesJson;
    use crate::{
        environment::bus_world::bus_world_events::new_bus::NewBusEvent,
        environment::environment::Environment,
    };

    #[test]
    fn create_bus_world() {
        let mut bus_world = BusEnvironment::new();
        let mut scheduler = Scheduler::new(100);
        assert_eq!(bus_world.bus_stops.len(), 0);
        bus_world.create_bus_stops(1);
        let number_of_buses = NewBusesJson::new(1, 5);
        let event = Box::new(NewBusEvent::new(
            1,
            0,
            serde_json::to_string(&number_of_buses).unwrap(),
        ));
        bus_world.apply_event(&mut scheduler, event);
        assert_eq!(bus_world.bus_stops.len(), 1);
        assert_eq!(bus_world.bus_stops[0].buses_at_stop.len(), 1);
    }
}
