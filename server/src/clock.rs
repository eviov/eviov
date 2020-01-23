use std::borrow::Cow;
use std::env;
use std::io;

use eviov::math::Time;
use eviov::{AlwaysZeroTimeSource, TimeSource};
use futures::compat::{Compat01As03, Compat01As03Sink};
use futures::future::Future;
use futures::sink::{Sink, SinkExt};
use futures::stream::{Stream, StreamExt};
use parking_lot::RwLock;
use tokio_tungstenite::connect_async;
use tungstenite::error::Error as WsError;
use tungstenite::handshake::client::Request;
use tungstenite::protocol::Message;
use url::Url;

pub type Clock = eviov::Clock<RwLock<eviov::ClockInner>>;

pub async fn create_clock() -> (Clock, Option<impl TimeSource>) {
    match env::var("TIME_SRC") {
        Ok(url) => {
            let url = Url::parse(&url).expect("The env var TIME_SRC is invalid");
            let mut request = Request::from(url);
            request.add_protocol(Cow::Borrowed("eviov_time"));
            let wss = Compat01As03::new(connect_async(request))
                .await
                .expect("Failed to connect to clock server")
                .0; // we don't need the response object
            let (sink, stream) = Compat01As03::new(wss).split();
            let src = UrlTimeSource(Compat01As03Sink::new(sink), Compat01As03::new(stream));
            let clock = Clock::new(&mut src)
                .await
                .expect("Failed to query clock server");
            (clock, Some(src))
        }
        Err(_) => (Clock::new(&mut AlwaysZeroTimeSource).await.unwrap(), None),
    }
}

pub struct UrlTimeSource<A, B>(A, B)
where
    A: Sink<Message, Error = WsError> + Unpin,
    B: Stream<Item = Result<Message, WsError>> + Unpin;

impl<A, B> TimeSource for UrlTimeSource<A, B>
where
    A: Sink<Message, Error = WsError> + Unpin,
    B: Stream<Item = Result<Message, WsError>> + Unpin,
{
    type FetchTime = Box<dyn Future<Output = Option<Time>>>;

    fn fetch_time(&mut self) -> Self::FetchTime {
        Box::new(async {
            let id = rand::random();
            let vec = rmp_serde::to_vec(&eviov::time_proto::Request { id })
                .expect("Failed to encode time_proto::Request");
            self.0.send(Message::Binary(vec)).await.ok()?;
            let resp = self.1.next().await?.ok()?;
            let resp = match resp {
                Message::Binary(vec) => vec,
                _ => return None,
            };
            let resp: eviov::time_proto::Response =
                rmp_serde::from_read(io::Cursor::new(resp)).ok()?;
            if resp.id != id {
                return None;
            }
            Some(resp.time)
        })
    }
}
