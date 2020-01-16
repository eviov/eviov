use serde::{Deserialize,Serialize};

use crate::math::*;

dirmod::all!(default file pub use; default dir pub);

macro_rules! group {
    ($mod:ident, $enum:ident<$lt:lifetime> {
        $($name:ident { $($fields:tt)* })*
    }) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub enum $enum<$lt> {
            $(
                #[serde(borrow)]
                $name($mod::$name<$lt>),
            )*
        }

        pub mod $mod {
            use super::*;

            $(
                #[derive(Debug, Serialize, Deserialize)]
                pub struct $name<'t> {
                    _ph: std::marker::PhantomData<&$lt ()>,
                    $($fields)*
                }
            )*
        }
    };
}

group!(server, FromServer<'t> {
    Ping {
        pub time: Time,
    }
    Shutdown {
        pub message: &'t str,
    }
});

group!(client, FromClient<'t> {
    Pong {
        pub send_time: Time,
        pub recv_time: Time,
    }
    Login {
        pub name: &'t str,
    }
});
