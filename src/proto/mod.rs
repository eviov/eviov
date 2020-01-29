use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait Protocol: Send + Sync + 'static {
    type FromClient: Debug + Serialize + for<'de> Deserialize<'de>;
    type FromServer: Debug + Serialize + for<'de> Deserialize<'de>;

    fn name() -> &'static str;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QueryId(u32);

pub trait Message: Debug + Serialize + for<'de> Deserialize<'de> {
    type Protocol: Protocol;
}

pub trait Single: Message {}

pub trait ClientMessage: Message {
    fn to_enum(self) -> <Self::Protocol as Protocol>::FromClient;
}

pub trait ServerMessage: Message {
    fn to_enum(self) -> <Self::Protocol as Protocol>::FromServer;
}

pub trait Query {
    type Request: QueryRequest;
    type Response: QueryResponse;
}

pub trait QueryRequest: Message {
    type Query: Query;

    fn query_id(&self) -> QueryId;

    fn set_query_id(&mut self, id: QueryId);
}

pub trait QueryResponse: Message {
    type Query: Query;

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
