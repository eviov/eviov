use async_trait::async_trait;

#[async_trait]
pub trait WsClient<H: Handler>: Sized {
    async fn connect(server: &str, proto: &str, handler: H) -> Result<Self, String>;

    async fn maintain(&self);

    async fn send_bytes(&self, bytes: Vec<u8>);

    async fn close(&self, message: &str);
}

pub trait Handler: Sized + Send + Sync + 'static {
    fn on_error(&self, error: String);

    fn on_close(&self, reason: &str);

    fn on_message(&self, bytes: Vec<u8>);
}

#[cfg(feature = "trait-tung")]
mod tung;
#[cfg(feature = "trait-tung")]
pub use tung::*;

#[cfg(feature = "trait-stdweb")]
mod stdweb;
#[cfg(feature = "trait-stdweb")]
pub use self::stdweb::*;
