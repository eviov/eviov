use std::collections::HashMap;
use std::fmt;

use super::Handle;
use eviov_proto::Endpoint;
use eviov_types::ObjectId;

/// A typemap for the protocol receivers registered on this process.
pub struct AllReceivers {
    map: typemap::ShareDebugMap,
}

impl Default for AllReceivers {
    fn default() -> Self {
        let mut map = typemap::ShareDebugMap::custom();

        macro_rules! protos {
            ($($proto:ident),* $(,)?) => {
                $(
                    map.insert::<typemap::K<Receivers<eviov_proto::$proto::Client>>>(Receivers::default())
                        .expect_none("Duplicate insert");
                    map.insert::<typemap::K<Receivers<eviov_proto::$proto::Server>>>(Receivers::default())
                        .expect_none("Duplicate insert");
                )*
            };
        }

        protos! {
            time,
            cs_obs,
            cs_ctrl,
            intra,
            ch,
            sh,
        }

        AllReceivers { map }
    }
}

impl AllReceivers {
    /// Retrieves the receiver list for a protocol endpoint.
    pub fn get<Me: Endpoint>(&self) -> &Receivers<Me> {
        self.map
            .get::<typemap::K<Receivers<Me>>>()
            .expect("Missing insert")
    }

    /// Mutably retrieves the receiver list for a protocol endpoint.
    pub fn get_mut<Me: Endpoint>(&mut self) -> &mut Receivers<Me> {
        self.map
            .get_mut::<typemap::K<Receivers<Me>>>()
            .expect("Missing insert")
    }
}

/// A mapping of receivers to nodes.
#[derive(Debug)]
pub struct Receivers<Me: Endpoint> {
    map: HashMap<ObjectId, Box<dyn Receiver<Me>>>,
}

impl<Me: Endpoint> Receivers<Me> {
    /// Retrieves a receiver by ID, or `None` if the object does not exist
    pub fn get(&self, id: ObjectId) -> Option<&dyn Receiver<Me>> {
        use std::ops::Deref;

        #[allow(clippy::borrowed_box)]
        let boxed: &Box<_> = self.map.get(&id)?;
        Some(boxed.deref())
    }

    /// Mutably retrieves a receiver by ID, or `None` if the object does not exist
    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut dyn Receiver<Me>> {
        use std::ops::DerefMut;

        #[allow(clippy::borrowed_box)]
        let boxed: &mut Box<_> = self.map.get_mut(&id)?;
        Some(boxed.deref_mut())
    }
}

impl<Me: Endpoint> Default for Receivers<Me> {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

/// The interface to open connections.
pub trait Receiver<Me: Endpoint>: fmt::Debug + Send + Sync {
    /// Opens a connection.
    fn open(&self, handle: Handle<Me>);
}
