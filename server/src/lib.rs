#![feature(never_type)]
#![allow(dead_code, unused_variables, unreachable_code)]
#![warn(unused_results)]

use std::io;
use std::sync::Arc;

mod plugin;
pub use plugin::*;

pub mod universe;

mod ws;

fn create_clock() -> ! {
    unimplemented!()
}

#[actix_rt::main]
pub async fn start<X: Plugin>() -> io::Result<()> {
    pretty_env_logger::init();

    let runtime = universe::Runtime::<X::SystemExtra>::new(create_clock());
    let plugin = X::init(universe::Runtime::clone(&runtime));

    let plugin = Arc::new(plugin);
    ws::start(plugin).await?;

    Ok(())
}
