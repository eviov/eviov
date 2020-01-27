#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct SystemId {
    serial: u32,
    generation: u32,
}

impl SystemId {
    pub fn new(serial: u32, generation: u32) -> Self {
        Self { serial, generation }
    }
}
