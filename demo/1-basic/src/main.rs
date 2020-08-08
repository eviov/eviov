#![cfg_attr(debug_assertions, allow(unused_variables, dead_code, unreachable_code))]

use amethyst::{self, renderer, utils};

mod state;
use state::MainState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(amethyst::LoggerConfig {
        ..Default::default()
    });

    let root = utils::application_root_dir()?;

    let game_data = amethyst::GameDataBuilder::default().with_bundle(
        renderer::RenderingBundle::<renderer::types::DefaultBackend>::new()
            .with_plugin(
                renderer::RenderToWindow::from_config_path(root.join("config/display.ron"))?
                    .with_clear([0., 0., 0., 1.]),
            )
            .with_plugin(renderer::RenderFlat2D::default()),
    )?;

    let mut game = amethyst::Application::new(root.join("assets"), MainState, game_data)?;
    game.run();

    Ok(())
}
