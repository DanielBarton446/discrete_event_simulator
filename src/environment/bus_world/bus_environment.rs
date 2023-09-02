use std::fmt::{format, Display, Error, Formatter};
use std::str::FromStr;

use crate::des::des::Scheduler;
use crate::environment::bus_world::bus::Bus;
use crate::environment::bus_world::bus_stop::BusStop;
use crate::environment::bus_world::bus_world_events::new_bus::NewBusesJson;
use crate::environment::bus_world::bus_world_events::{move_bus_to_stop::*, new_bus::NewBusEvent};
use crate::environment::environment::Environment;
use crate::event::event::Event;

use super::bus_world_events::move_bus_to_stop::BusToStopMappingJson;

enum BusEventTypes {
    NewBus,
    MoveBusToStop,
}

impl FromStr for BusEventTypes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NewBus" => Ok(BusEventTypes::NewBus),
            "MoveBusToStop" => Ok(BusEventTypes::MoveBusToStop),
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
        println!("New Bus! (temporary representation of bus event)");
        println!("Bus event with UID: {}", event.get_uid());

        let bus_mapping = serde_json::from_str::<NewBusesJson>(&event.get_data().unwrap())
            .expect("Error: Could not deserialize bus mapping");

        for i in 0..bus_mapping.number_of_buses {
            let mut bus = Bus::new(i);

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
            self.bus_stops[0].add_bus(bus);

            let start_bus_route = Box::new(MoveBusToStopEvent::new(
                event.get_uid() + 1,
                event.get_time_stamp() + 5.0, // This should be something long for moving between stops
                serde_json::to_string(&bus_routing).unwrap(),
            ));

            scheduler.add_event(start_bus_route);
        }
    }

    fn apply_move_bus_to_stop_event(&mut self, scheduler: &mut Scheduler, event: Box<dyn Event>) {
        // // This should be handled and not unwrapped, but whatever
        let mapping =
            serde_json::from_str::<BusToStopMappingJson>(&event.get_data().unwrap()).unwrap();
        // find and drain the bus we are looking for and do something with it later
        // unwrapping is bad
        let mut bus = self.drain_bus_by_uid(mapping.bus_uid).unwrap();
        // find the stop we are looking for and add the bus to it
        // unwrapping is bad
        let stop = self.find_mut_stop_by_name(&mapping.stop_name).unwrap();

        // Advance the bus to the next stop, as we have arrived at the new stop
        bus.advance_to_next_stop();
        // Schedule this bus to move to the next stop unless its at the end of the line
        if let Some(next_stop) = bus.get_next_stop() {
            let mapping = BusToStopMappingJson::new(bus.uid, next_stop.to_string());
            let next_event = Box::new(MoveBusToStopEvent::new(
                event.get_uid() + 1, // this is really bad....
                event.get_time_stamp() + 5.0,
                serde_json::to_string(&mapping).unwrap(),
            ));
            scheduler.add_event(next_event);
        }

        stop.add_bus(bus);
    }
}

impl Display for BusEnvironment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "Bus Stops:\tBuses:")?;
        for stop in self.bus_stops.iter() {
            write!(f, "{} ", &stop)?;
            // self.print_buses_at_stop(stop, f)?;
            writeln!(f)?;
        }

        writeln!(f)?;
        // self.print_bus_details(f)?;

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
        let mut scheduler = Scheduler::new(100.0);
        assert_eq!(bus_world.bus_stops.len(), 0);
        bus_world.create_bus_stops(1);
        let number_of_buses = NewBusesJson::new(1);
        let event = Box::new(NewBusEvent::new(
            1,
            0.0,
            serde_json::to_string(&number_of_buses).unwrap(),
        ));
        bus_world.apply_event(&mut scheduler, event);
        assert_eq!(bus_world.bus_stops.len(), 1);
        assert_eq!(bus_world.bus_stops[0].buses_at_stop.len(), 1);
    }
}
