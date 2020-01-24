#![allow(dead_code, unused_variables)]

use std::io;
use std::sync::Arc;
use std::thread;

mod clock;
pub use clock::*;

mod plugin;
pub use plugin::*;

pub mod universe;

mod ws;

#[actix_rt::main]
pub async fn start<X: Plugin>() -> io::Result<()> {
    pretty_env_logger::init();

    let (clock, src) = create_clock().await;
    if let Some(src) = src {
        let clock = clock.clone();
        thread::spawn(move || {
            use std::time::Duration;

            use eviov::LoopAction;
            use tokio::time::delay_for;

            async fn delay_fn() -> LoopAction {
                delay_for(Duration::from_secs(60)).await;
                LoopAction::Continue
            }

            futures::executor::block_on(clock.maintain(src, delay_fn));
        });
    }
    let runtime = universe::Runtime::<X::SystemExtra>::new(clock);
    let plugin = X::init(universe::Runtime::clone(&runtime));

    let plugin = Arc::new(plugin);
    ws::start(plugin).await?;

    Ok(())
}
