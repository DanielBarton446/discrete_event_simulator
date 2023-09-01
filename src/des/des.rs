// Discrete Event Simulator
//
//
//

use crate::event::event::Event;
use std::collections::BinaryHeap;

pub struct Scheduler {
    pub current_time: f64,
    pub runtime: f64,
    pub event_queue: BinaryHeap<Box<dyn Event>>,
}

impl Scheduler {
    pub fn new(runtime: f64) -> Scheduler {
        Scheduler {
            current_time: 0.0,
            runtime,
            event_queue: BinaryHeap::<Box<dyn Event>>::new(),
        }
    }

    pub fn add_event(&mut self, event: Box<dyn Event>) {
        // somwhere we need to add logic for the delay of running the event vs
        // the absolute current time in the simulation that the event takes place.
        self.event_queue.push(event);
    }

    pub fn next_event(&mut self) -> Option<Box<dyn Event>> {
        let event = match self.event_queue.pop() {
            Some(event) => {
                if event.get_time_stamp() > self.runtime {
                    return None;
                }
                self.current_time = event.get_time_stamp();
                Some(event)
            }
            None => None,
        };
        return event;
    }
}
