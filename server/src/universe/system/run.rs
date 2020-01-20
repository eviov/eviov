use std::collections::BTreeMap;

use eviov::math::Time;
use futures::channel::mpsc::UnboundedReceiver;

use super::*;
use crate::universe::Runtime;

pub(super) async fn run_impl<X: Extra>(
    runtime: Runtime<X>,
    mut extra: X,
    recv: UnboundedReceiver<Message<X::Message>>,
) {
    extra.setup_system().await;

    let mut event_queue = BTreeMap::<(Time, EventId), Event>::new();

    loop {}
    // TODO cleanup
}
