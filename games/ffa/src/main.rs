#![allow(dead_code, unused_variables)]

use std::io;

use eviov_server::universe::{self, system};
use futures::future;

fn main() -> io::Result<()> {
    eviov_server::start::<Plugin>()
}

struct Plugin {
    runtime: universe::Runtime<Extra>,
}

impl eviov_server::Plugin for Plugin {
    type SystemExtra = Extra;

    fn init(runtime: universe::Runtime<Extra>) -> Self {
        Self { runtime }
    }

    fn process_request(&mut self) {
        unimplemented!()
    }
}

struct Extra;

impl system::Extra for Extra {
    type Message = Message;

    type NextEvent = future::Pending<system::TickAction>;
    fn next_event(&mut self) -> Self::NextEvent {
        future::pending() // TODO
    }

    type SetupSystem = future::Ready<()>;
    fn setup_system(&mut self, add_entity: impl FnMut()) -> Self::SetupSystem {
        future::ready(()) // TODO
    }
}

#[derive(Debug)]
enum Message {}
