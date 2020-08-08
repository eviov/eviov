pub struct MainState;

impl amethyst::SimpleState for MainState {
    fn on_start(&mut self, data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {}
}

fn init_camera(world: &mut specs::World) {}
