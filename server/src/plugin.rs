use crate::universe::{Runtime, system};

pub trait Plugin : Sized + Send + Sync + 'static {
    type SystemExtra: system::Extra;

    fn init(runtime: Runtime<Self::SystemExtra>) -> Self;

    fn process_request(&mut self);
}
