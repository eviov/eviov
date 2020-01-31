use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait Protocol: Send + Sync + 'static {
    type FromClient: Endpoint;
    type FromServer: Endpoint;

    fn name() -> &'static str;
}

pub trait Endpoint: Debug + Serialize + for<'de> Deserialize<'de> {
    type Protocol: Protocol;
    type Peer: Endpoint<Protocol = Self::Protocol>;

    fn query_id(&self) -> Option<QueryId>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueryId(u32);

impl Default for QueryId {
    fn default() -> Self {
        QueryId(u32::max_value())
    }
}

pub trait Message: Debug + Serialize + for<'de> Deserialize<'de> {
    type Protocol: Protocol;
}

pub trait MessageFrom<E: Endpoint>: Message<Protocol = <E as Endpoint>::Protocol> {
    fn from_enum(e: E) -> Option<Self>;
    fn to_enum(self) -> E;
}

pub trait Single: Message {}

pub trait QueryRequest: Message {
    fn query_id(&self) -> QueryId;

    fn set_query_id(&mut self, id: QueryId);
}

pub trait QueryRequestFrom<E: Endpoint>: QueryRequest + MessageFrom<E> {
    type Response: QueryResponse + MessageFrom<E::Peer>;
}

pub trait QueryResponse: Message {
    fn query_id(&self) -> QueryId;

    fn set_query_id(&mut self, id: QueryId);
}

pub mod ch;
pub mod cs;
pub mod intra;
pub mod sh;
pub mod time;

mod transport;
pub use transport::*;
