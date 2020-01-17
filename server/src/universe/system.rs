//! A System refers to a "natural" celestial body and its satellites.
//!
//! Structurally, a System is the basic unit of distributed computation.
//! Each System maintains its own satellites, and only interacts with its child or parent system by
//! sending bodies between the systems.
//! Both artificial and natural bodies can be sent between systems.
//! A natural body that travels between systems is called a "comet system".

use eviov::math::Eci;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemTag {
    local: u32,
    runtime: RuntimeId,
}

pub fn new_system(runtime: &Runtime, parent: Option<(SystemTag, Eci)>) -> SystemTag {
    let tag = SystemTag {
        local: runtime.next_id(),
        runtime: runtime.id().into(),
    };
    let mut world = runtime.universe().create_world();
    if let Some((parent, eci)) = parent {
        unimplemented!("Put link in parent");
    }
    let void: Vec<()> = vec![];
    world.insert((tag,), void);
    tag
}
