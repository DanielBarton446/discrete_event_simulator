use std::fmt::Display;

use crate::{
    des::scheduler::Scheduler, event::schedulable::SchedulableEvent, statistics::stats::Stats,
};

pub trait Environment: Display {
    fn apply_event(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    );
    fn get_state(&self) -> String;
    fn terminating_event(&self, scheduler: &mut Scheduler) -> Box<dyn SchedulableEvent>;
}
