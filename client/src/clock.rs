use std::cell::RefCell;

use async_trait::async_trait;
use eviov::math::Time;
use eviov::TimeSource;
use stdweb::web::{self, WebSocket};

pub type Clock = eviov::Clock<RefCell<eviov::ClockInner>>;

pub async fn create_clock(
    url: impl Fn() -> Option<String> + Send + Sync,
) -> Result<(Clock, impl TimeSource), &'static str> {
    let mut source = UrlTimeSource::new(url)?;
    let clock = Clock::new(&mut source)
        .await
        .ok_or("Failed to query time server")?;
    Ok((clock, source))
}

pub struct UrlTimeSource<F: FnMut() -> Option<String> + Send + Sync> {
    url_src: F,
    ws: WebSocket,
}

impl<F: FnMut() -> Option<String> + Send + Sync> UrlTimeSource<F> {
    pub fn new(mut url: F) -> Result<Self, &'static str> {
        let ws = loop {
            let server = match url() {
                Some(server) => server,
                None => return Err("Failed to connect to any time server"),
            };
            let ws = match web::WebSocket::new_with_protocols(&server, &["eviov_time"]) {
                Ok(ws) => ws,
                Err(_) => continue,
            };
            ws.set_binary_type(web::SocketBinaryType::ArrayBuffer);
            break ws;
        };
        Ok(Self { url_src: url, ws })
    }
}

#[async_trait]
impl<F: Fn() -> Option<String> + Send + Sync> TimeSource for UrlTimeSource<F> {
    async fn fetch_time(&mut self) -> Option<Time> {
        let id = rand::random();
        self.ws
            .send_bytes(
                &rmp_serde::to_vec(&eviov::time_proto::Request { id })
                    .expect("Failed to encode time_proto::Request"),
            )
            .ok()?;

        unimplemented!("Receive message")
    }
}
