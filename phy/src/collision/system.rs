use shrev::EventChannel;

use super::Event;

/// The system handling collisions.
pub struct System;

impl<'a> specs::System<'a> for System {
    type SystemData = (specs::Write<'a, EventChannel<Event>>, specs::Entities<'a>);

    fn run(&mut self, (col_events, entities): Self::SystemData) {
        todo!()
    }
}
