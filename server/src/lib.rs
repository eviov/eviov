#![allow(dead_code, unused_variables)]

use std::io;

dirmod::all!(default file pub use; default dir pub);

#[actix_rt::main]
pub async fn start<X: Plugin>() -> io::Result<()> {
    pretty_env_logger::init();

    ws::start().await
}
