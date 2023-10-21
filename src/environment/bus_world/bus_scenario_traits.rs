use crate::{des::scheduler::Scheduler, event::schedulable::SchedulableEvent, statistics::stats::Stats};

pub trait PassengerTransportHandler {
    fn load_passengers(
        &mut self,
        _scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    );

    fn unload_passengers(
        &mut self,
        _scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    );
}

pub trait AdvanceVehicleHandler {
    fn advance_bus_to_next_stop(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    );
}

pub trait NewVehicleHandler {
    fn create_new_bus(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    );

    fn import_buses(
        &mut self,
        scheduler: &mut Scheduler,
        stat_recorder: &mut Stats,
        event: Box<dyn SchedulableEvent>,
    );
}
