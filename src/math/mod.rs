use serde::{Deserialize, Serialize};

dirmod::all!();

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Time(pub i32);
