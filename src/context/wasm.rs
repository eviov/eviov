use async_trait::async_trait;

use crate::context::ContextImpl;

pub struct WasmContext;

#[async_trait]
impl ContextImpl for WasmContext {
    fn spawn_future<F: Future<Output = ()> + Send + 'static>(&self, fut: F) {
        wasm_bindgen_futures::spawn_local(fut);
    }

    async fn sleep(&self, duration: Duration) {
        unimplemented!()
    }
}
