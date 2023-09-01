use crate::environment::environment::Environment;
use crate::event::event::Event;

struct BusWorld {
    number_of_buses: usize,
}

impl BusWorld {
    pub fn new() -> BusWorld {
        BusWorld { number_of_buses: 0 }
    }
}

impl Environment for BusWorld {
    fn apply_event(&mut self, event: Box<dyn Event>) {
        match event.get_event_type() {
            "BusEvent" => {
                self.apply_bus_event(event);
            }
            _ => {
                panic!("Error: Unknown event type {}", event.get_event_type())
            }
        }
    }
}

impl BusWorld {
    fn apply_bus_event(&mut self, event: Box<dyn Event>) {
        println!("New Bus! (temporary representation of bus event)");
        println!("Bus event with UID: {}", event.get_uid());
        self.number_of_buses += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::{environment::environment::Environment, event::bus_event::BusEvent};

    use super::BusWorld;

    #[test]
    fn create_bus_world() {
        let mut bus_world = BusWorld::new();
        assert_eq!(bus_world.number_of_buses, 0);
        let event = Box::new(BusEvent::new(
            1,
            0.0,
            String::from("create_bus_world_event"),
        ));
        bus_world.apply_event(event);
        assert_eq!(bus_world.number_of_buses, 1);
    }
}
