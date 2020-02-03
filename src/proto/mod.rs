//! Internal connection protocols.
//!
//! There are 5 types of connections, as in the submodules `time`, `cs`, `intra`, `ch` and `sh`.
//! Refer to the documentation of those submodules for details.
//!
//! This module also exposes a number of top-level traits, to be implemented by protocol
//! implementations.
//! These traits are intended to be implemented by the `codegen::proto!` macro;
//! do not try to implement them by hand.

use std::fmt::Debug;
use type_equals::TypeEquals;

use serde::{Deserialize, Serialize};

/// A type representing a connection protocol.
///
/// This trait is implemented by unit structs, which exist only for type inference.
pub trait Protocol: Send + Sync + 'static {
    type Client: Endpoint;
    type Server: Endpoint;

    fn name() -> &'static str;
}

/// An enum covering all possible messages from a protocol endpoint.
pub trait Endpoint: Debug + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static {
    /// The protocol that this endpoint belongs to.
    type Protocol: Protocol;
    /// The othe side of the protocol, i.e. Client => Server, Server => Client
    type Peer: Endpoint<Protocol = Self::Protocol, Peer = Self>;

    /// Retrieves the request query ID of the enum value.
    ///
    /// If the enum value represents a query request, this returns the query ID of the request.
    /// Otherwise, this returns `None`.
    fn request_query_id(&self) -> Option<QueryId>;

    /// Retrieves the response query ID of the enum value.
    ///
    /// If the enum value represents a query response, this returns the query ID of the response.
    /// Otherwise, this returns `None`.
    fn response_query_id(&self) -> Option<QueryId>;
}

/// A marker trait indicating that the endpoint is the client side of the protocol
pub trait ClientEndpoint {}
impl<T: Endpoint> ClientEndpoint for T where
    <<T as Endpoint>::Protocol as Protocol>::Client: TypeEquals<Other = T>
{
}

/// A marker trait indicating that the endpoint is the server side of the protocol
pub trait ServerEndpoint {}
impl<T: Endpoint> ServerEndpoint for T where
    <<T as Endpoint>::Protocol as Protocol>::Server: TypeEquals<Other = T>
{
}

/// Wraps the ID of a query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QueryId(u32);

impl Default for QueryId {
    fn default() -> Self {
        QueryId(u32::max_value())
    }
}

/// Represents a message that can be sent through the protocol.
pub trait Message: Debug + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static {
    /// The protocol that this message is sent through.
    type Protocol: Protocol;
}

/// A marker trait indicating that the message can be sent through a particular endpoint.
pub trait MessageFrom<E: Endpoint>: Message<Protocol = <E as Endpoint>::Protocol> {
    /// Converts an enum of message from the endpoint to this particular type,
    /// returning `None` if the enum value is incorrect.
    fn from_enum(e: E) -> Option<Self>;

    /// Converts this message to its enum representation for the endpoint.
    fn to_enum(self) -> E;
}

/// A marker trait indicating that the message is single, contrary to a request-response query
/// pair.
pub trait Single: Message {}

/// A message that expects a response value.
///
/// Implementors of this trait must also implement `QueryRequestFrom<E>` for at least one endpoint
/// `E`;
/// if a request type can be sent from both endpoints, it should implement `QueryRequestFrom<E>`
/// twice, each `E` corresponding to each endpoint.
pub trait QueryRequest: Message {
    /// The query ID of this message.
    ///
    /// This method is internal, and should only be called by the connection handler in the
    /// `transport` submodule.
    fn query_id(&self) -> QueryId;

    /// Sets the query ID of this message.
    ///
    /// This method is internal, and should only be called by the connection handler in the
    /// `transport` submodule.
    fn set_query_id(&mut self, id: QueryId);
}

/// A marker trait indicating that the message can be sent through a particular endpoint.
pub trait QueryRequestFrom<E: Endpoint>: QueryRequest + MessageFrom<E> {
    /// The corresponding response message type for this request.
    type Response: QueryResponse + MessageFrom<E::Peer>;
}

/// A message that responds to a QueryRequest message.
pub trait QueryResponse: Message {
    /// The query ID of this message.
    ///
    /// This method is internal, and should only be called by the connection handler in the
    /// `transport` submodule.
    fn query_id(&self) -> QueryId;

    /// Sets the query ID of this message.
    ///
    /// This method is internal, and should only be called by the connection handler in the
    /// `transport` submodule.
    fn set_query_id(&mut self, id: QueryId);
}

pub mod ch;
mod cs;
pub use cs::ctrl as cs_ctrl;
pub use cs::obs as cs_obs;
pub mod intra;
pub mod sh;
pub mod time;

pub mod transport;
