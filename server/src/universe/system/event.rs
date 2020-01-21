#[derive(Debug)]
pub struct Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventId(u32);

pub enum TickAction {
    Continue,
    Stop,
}

pub fn handle_event(event: Event) -> TickAction {
    unimplemented!()
}
