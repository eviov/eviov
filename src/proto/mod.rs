use std::fmt::Debug;

use serde::{Deserialize, Serialize};

pub trait Protocol {
    type FromClient: Debug + Serialize + for<'de> Deserialize<'de>;
    type FromServer: Debug + Serialize + for<'de> Deserialize<'de>;
}

pub trait Message: Debug + Serialize + for<'de> Deserialize<'de> {}

pub trait Query {
    type Request: Debug + Serialize + for<'de> Deserialize<'de>;
    type Response: Debug + Serialize + for<'de> Deserialize<'de>;
}

pub mod ch;
pub mod cs;
pub mod intra;
pub mod sh;
pub mod time;

mod transport;
pub use transport::*;
