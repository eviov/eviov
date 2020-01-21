#![allow(dead_code, unused_variables)]

use std::io;

mod plugin;
pub use plugin::*;

pub mod universe;

mod ws;

#[actix_rt::main]
pub async fn start<X: Plugin>() -> io::Result<()> {
    pretty_env_logger::init();

    ws::start().await
}
