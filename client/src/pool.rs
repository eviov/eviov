use std::cmp;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use eviov::ObjectId;

use crate::conn::Conn;

#[derive(Default)]
pub struct ConnPool {
    conns: HashMap<String, Conn>,
    systems: HashMap<ObjectId, SystemHandle>,
}

impl ConnPool {
    pub fn get(&self, id: ObjectId) -> Option<&SystemHandle> {
        self.systems.get(&id)
    }
}

pub struct SystemHandle {
    conn: Conn,
    id: ObjectId,
}

impl SystemHandle {
    pub async fn open(id: ObjectId, conn: Conn) -> Self {
        // TODO handshake logic
        Self { id, conn }
    }
}

impl PartialEq for SystemHandle {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SystemHandle {}

impl Ord for SystemHandle {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for SystemHandle {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for SystemHandle {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
    }
}
