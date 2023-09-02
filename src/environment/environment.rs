use std::fmt::Display;

use crate::{des::des::Scheduler, event::event::Event};

pub trait Environment: Display {
    fn apply_event(&mut self, scheduler: &mut Scheduler, event: Box<dyn Event>);
    fn get_state(&self) -> String;
}
