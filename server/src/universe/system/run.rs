use std::collections::BTreeMap;

use eviov::math::Time;
use eviov::ObjectId;
use futures::channel::mpsc::UnboundedReceiver;
use legion::world::World;

use super::*;
use crate::universe::{Runtime, RuntimeId};

pub async fn run_impl<X: Extra>(
    runtime: Runtime<X>,
    mut extra: X,
    recv: UnboundedReceiver<Message<X::Message>>,
) {
    let runtime_ref = &runtime;
    let mut world = World::new();
    let world_ref = &mut world;

    extra
        .setup_system(move || {
            world_ref.insert(
                (),
                vec![(ObjectId::new(
                    RuntimeId::from(runtime_ref.id()).into_u32(),
                    runtime_ref.next_id(),
                ),)],
            );
            // TODO more logic
        })
        .await;

    let mut event_queue = BTreeMap::<(Time, EventId), Event>::new();

    loop {
        let key = event_queue.keys().next();
        if let Some(&(t, evid)) = key {
            runtime
                .time_future(t, async {
                    handle_event(event_queue.remove(&(t, evid)).unwrap())
                })
                .await; // TODO other events
        }
    }

    // TODO cleanup
}
