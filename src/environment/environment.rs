use crate::event::event::Event;

pub trait Environment {
    fn apply_event(&mut self, event: Box<dyn Event>);
}
