use crate::des::des::Scheduler;
use crate::environment::environment::Environment;
use crate::event::bus_event::BusEvent;
use crate::event::event::Event;

pub struct BusWorld {
    number_of_buses: usize,
}

impl BusWorld {
    pub fn new() -> BusWorld {
        BusWorld { number_of_buses: 0 }
    }
}

impl Environment for BusWorld {
    fn apply_event(&mut self, scheduler: &mut Scheduler, event: Box<dyn Event>) {
        match event.get_event_type() {
            "BusEvent" => {
                self.apply_bus_event(scheduler, event);
            }
            _ => {
                panic!("Error: Unknown event type {}", event.get_event_type())
            }
        }
    }
    fn get_state(&self) -> String {
        format!("Number of buses: {}", self.number_of_buses)
    }
}

impl BusWorld {
    fn apply_bus_event(&mut self, scheduler: &mut Scheduler, event: Box<dyn Event>) {
        println!("New Bus! (temporary representation of bus event)");
        println!("Bus event with UID: {}", event.get_uid());
        self.number_of_buses += 1;

        let dummy_event = Box::new(BusEvent::new(
            event.get_uid() + 1,
            event.get_time_stamp() + 1.0,
            String::from("dummy_bus_event"),
        ));

        scheduler.add_event(dummy_event);
    }
}

#[cfg(test)]
mod tests {
    use super::BusWorld;
    use crate::des::des::Scheduler;
    use crate::{environment::environment::Environment, event::bus_event::BusEvent};

    #[test]
    fn create_bus_world() {
        let mut bus_world = BusWorld::new();
        let mut scheduler = Scheduler::new(100.0);
        assert_eq!(bus_world.number_of_buses, 0);
        let event = Box::new(BusEvent::new(
            1,
            0.0,
            String::from("create_bus_world_event"),
        ));
        bus_world.apply_event(&mut scheduler, event);
        assert_eq!(bus_world.number_of_buses, 1);
    }
}
