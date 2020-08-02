use amethyst::{ecs, shrev::EventChannel};

use super::Event;

/// The system handling collisions.
pub struct System;

impl<'a> ecs::System<'a> for System {
    type SystemData = (ecs::Write<'a, EventChannel<Event>>, ecs::Entities<'a>);

    fn run(&mut self, (col_events, entities): Self::SystemData) {
        unimplemented!()
    }
}
