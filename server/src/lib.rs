#![allow(dead_code, unused_variables)]

use std::io;
use std::sync::Arc;
use std::thread;

mod clock;
pub use clock::*;

mod plugin;
pub use plugin::*;

mod util;

pub mod universe;

mod ws;

#[actix_rt::main]
pub async fn start<X: Plugin>() -> io::Result<()> {
    pretty_env_logger::init();

    let (clock, src) = create_clock().await;
    if let Some(src) = src {
        let clock_ref = &clock;
        thread::spawn(move || {
            async fn delay_fn() {
                use std::time::Duration;

                use tokio::time::delay_for;

                delay_for(Duration::from_secs(60));
            }

            futures::executor::block_on(clock_ref.maintain(src, delay_fn));
        });
    }
    let runtime = universe::Runtime::<X::SystemExtra>::new(clock);
    let plugin = X::init(universe::Runtime::clone(&runtime));

    let plugin = Arc::new(plugin);
    ws::start(plugin).await?;

    Ok(())
}
