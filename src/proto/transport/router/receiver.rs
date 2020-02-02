use std::collections::HashMap;
use std::fmt;

use super::Handle;
use crate::proto::{self, Endpoint};
use crate::ObjectId;

pub struct AllReceivers {
    map: typemap::ShareDebugMap,
}

impl Default for AllReceivers {
    fn default() -> Self {
        let mut map = typemap::ShareDebugMap::custom();

        macro_rules! protos {
            ($($proto:ident),* $(,)?) => {
                $(
                    map.insert::<typemap::K<Receivers<proto::$proto::FromClient>>>(Receivers::default())
                        .expect_none("Duplicate insert");
                    map.insert::<typemap::K<Receivers<proto::$proto::FromServer>>>(Receivers::default())
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
    pub fn get<Me: Endpoint>(&self) -> &Receivers<Me> {
        self.map
            .get::<typemap::K<Receivers<Me>>>()
            .expect("Missing insert")
    }

    pub fn get_mut<Me: Endpoint>(&mut self) -> &mut Receivers<Me> {
        self.map
            .get_mut::<typemap::K<Receivers<Me>>>()
            .expect("Missing insert")
    }
}

#[derive(Debug)]
pub struct Receivers<Me: Endpoint> {
    map: HashMap<ObjectId, Box<dyn Receiver<Me>>>,
}

impl<Me: Endpoint> Receivers<Me> {
    pub fn get(&self, id: ObjectId) -> Option<&dyn Receiver<Me>> {
        use std::ops::Deref;

        let boxed: &Box<_> = self.map.get(&id)?;
        Some(boxed.deref())
    }

    pub fn get_mut(&mut self, id: ObjectId) -> Option<&mut dyn Receiver<Me>> {
        use std::ops::DerefMut;

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

pub trait Receiver<Me: Endpoint>: fmt::Debug + Send + Sync {
    fn open(&self, handle: Handle<Me>);
}
