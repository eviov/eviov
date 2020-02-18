use std::collections::HashMap;

use eviov_proto::Endpoint;
use eviov_types::ObjectId;
use futures::lock::Mutex;

pub struct Router {
    out_conn: HashMap<
}

/// An object that can receive and send messages.
///
/// # Type parameters
/// `E` is the endpoint enum for the messages sent from this object.
pub trait Node<E: Endpoint> {
    fn recv(&self, message: <E as Endpoint>::Peer);
}
