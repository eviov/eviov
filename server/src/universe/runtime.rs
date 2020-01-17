use std::sync::atomic::{AtomicU32, Ordering};

use derive_more::Into;
use getset::*;
use legion::prelude::Universe;

#[derive(Getters, CopyGetters)]
pub struct Runtime {
    #[get_copy = "pub"]
    id: CurrentRuntimeId,
    counter: AtomicU32,
    #[get = "pub"]
    universe: Universe,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            id: CurrentRuntimeId(RuntimeId(rand::random())),
            counter: AtomicU32::default(),
            universe: Universe::new(),
        }
    }

    pub fn next_id(&self) -> u32 {
        self.counter.fetch_add(1, Ordering::SeqCst)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RuntimeId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Into)]
pub struct CurrentRuntimeId(RuntimeId);
