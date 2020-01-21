//! A System refers to a "natural" celestial body and its satellites.
//!
//! Structurally, a System is the basic unit of distributed computation.
//! Each System maintains its own satellites, and only interacts with its child or parent system by
//! sending bodies between the systems.
//! Both artificial and natural bodies can be sent between systems.
//! A natural body that travels between systems is called a "comet system".

mod event;
pub use event::*;
mod extra;
pub use extra::*;
mod handle;
pub use handle::*;
#[path = "run.rs"]
mod _run;
pub use _run::*;
