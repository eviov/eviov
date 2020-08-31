#[derive(Debug)]
pub struct PhysicsBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PhysicsBundle {
    fn build(self, world: &mut World, builder: &mut DispatchBuilder<'a, 'b>) -> Result<(), Error> {
    }
}
