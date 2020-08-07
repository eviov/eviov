use super::Camera;

/// Draws the camera scene on the window.
pub struct DrawSystem;

impl<'a> specs::System<'a> for DrawSystem {
    type SystemData = (
        specs::ReadExpect<'a, Camera>,
        specs::ReadStorage<'a, phy::Star>,
    );

    fn run(&mut self, (camera, star_store): Self::SystemData) {
        let star = match star_store.get(camera.star()).or_else(|| {
            use specs::Join;
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
