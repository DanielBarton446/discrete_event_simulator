//! Responsible for managing time-sorted queue of events
//! and providing the next event to be processed.

use crate::event::schedulable::SchedulableEvent;
use std::collections::BinaryHeap;

/// Binary Heap implementation based queue for events.
/// Maintains current time and runtime.
/// This queue is able to support any event which
/// implements the [SchedulableEvent] trait.
pub struct Scheduler {
    pub current_time: usize,
    pub runtime: usize,
    pub event_queue: BinaryHeap<Box<dyn SchedulableEvent>>,
}

impl Scheduler {
    /// Create a new scheduler with a given runtime.
    ///
    /// ```
    /// use discrete_event_simulator::des::scheduler::Scheduler;
    /// // Create a scheduler with a runtime of 100
    /// let scheduler = Scheduler::new(100);
    /// ```
    pub fn new(runtime: usize) -> Scheduler {
        Scheduler {
            current_time: 0,
            runtime,
            event_queue: BinaryHeap::<Box<dyn SchedulableEvent>>::new(),
        }
    }

    /// Add an event to the scheduler.
    /// ```
    /// use discrete_event_simulator::des::scheduler::Scheduler;
    /// use discrete_event_simulator::event::schedulable::SchedulableEvent;
    /// use discrete_event_simulator::environment::bus_world::bus_world_events::terminal_event::*;
    /// let mut scheduler = Scheduler::new(100);
    /// // create a terminal even for the bus world environment and schedule it
    /// let event = TerminalEvent::new(0, 0, String::from("Terminal Event Data"));
    /// scheduler.add_event(Box::new(event));
    /// ```
    pub fn add_event(&mut self, event: Box<dyn SchedulableEvent>) {
        self.event_queue.push(event);
    }

    /// Get the next event from the scheduler.
    /// ```
    /// use discrete_event_simulator::des::scheduler::Scheduler;
    /// let mut scheduler = Scheduler::new(100);
    /// // get the next event from the scheduler (there is none)
    /// let event = scheduler.next_event();
    /// assert!(event.is_none());
    /// ```
    pub fn next_event(&mut self) -> Option<Box<dyn SchedulableEvent>> {
        match self.event_queue.pop() {
            Some(event) => {
                if event.get_time_stamp() > self.runtime {
                    return None;
                }
                self.current_time = event.get_time_stamp();
                Some(event)
            }
            None => None,
        }
    }
}
