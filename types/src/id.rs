#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]

/// A network-unique runtime identifier for objects.
///
/// The identifier comprises a serial number and a generation number.
/// The serial number is unique across all objects in the current process.
/// The generation number is an identifier of the current process, unique in the connected network,
/// and hopefully unique all-time by probability.
pub struct ObjectId {
    serial: u32,
    generation: u32,
}

impl ObjectId {
    /// Creates a new object ID by data.
    pub fn new(serial: u32, generation: u32) -> Self {
        Self { serial, generation }
    }
}
