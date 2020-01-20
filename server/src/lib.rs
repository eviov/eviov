#![allow(dead_code, unused_variables)]

use std::io;

dirmod::all!(default pub);

#[actix_rt::main]
async fn start<X: universe::system::Extra>() -> io::Result<()> {
    pretty_env_logger::init();

    ws::start().await
}
