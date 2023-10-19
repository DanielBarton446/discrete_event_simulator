use std::fmt::Display;

use crate::{des::des::Scheduler, event::event::Event, statistics::stats::Stats};

pub trait Environment: Display {
    fn apply_event(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn Event>,
    );
    fn get_state(&self) -> String;
    fn terminating_event(&self) -> Box<dyn Event>;
}
