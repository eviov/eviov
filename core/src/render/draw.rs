use amethyst::ecs;

use super::Camera;
use crate::phy;

/// Draws the camera scene on the window.
pub struct DrawSystem;

impl<'a> ecs::System<'a> for DrawSystem {
    type SystemData = (ecs::ReadExpect<'a, Camera>, ecs::ReadStorage<'a, phy::Star>);

    fn run(&mut self, (camera, star_store): Self::SystemData) {
        let star = match star_store.get(camera.star()).or_else(|| {
            use ecs::Join;
            star_store.join().next()
        }) {
            Some(star) => star,
            None => {
                log::warn!("No star entities; render aborted");
                // TODO handle
                return;
            }
        };
    }
}
