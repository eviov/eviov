#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]

pub struct ObjectId {
    serial: u32,
    generation: u32,
}

impl ObjectId {
    pub fn new(serial: u32, generation: u32) -> Self {
        Self { serial, generation }
    }
}

pub type SystemId = ObjectId;
