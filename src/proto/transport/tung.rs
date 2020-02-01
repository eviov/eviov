use std::io::Cursor;
use std::marker::PhantomData;
use std::time::Duration;

use async_trait::async_trait;
use futures::future::{BoxFuture, FutureExt};
use futures::lock::Mutex;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    tungstenite::{error::Error, Message},
    WebSocketStream,
};

use crate::proto::{transport::Agent, Endpoint};

type Wss = WebSocketStream<TcpStream>;

pub struct TungClient<E: Endpoint> {
    wss: Mutex<Wss>,
    _ph: PhantomData<E>,
}

impl<E: Endpoint> TungClient<E> {
    #[inline]
    fn handle_write_err(
        wss: &mut Wss,
        err: Error,
        recursive: bool,
    ) -> BoxFuture<Result<(), String>> {
        async move {
            match err {
                Error::ConnectionClosed | Error::AlreadyClosed => {
                    Err("Connection closed".to_string())
                }
                Error::Capacity(_) => panic!("Attempt to write an oversized message"),
                Error::SendQueueFull(message) if !recursive => {
                    let result = wss.flush().await;
                    if let Err(err) = result {
                        Self::handle_write_err(wss, err, true).await?;
                    }
                    let result = wss.send(message).await;
                    if let Err(err) = result {
                        Self::handle_write_err(wss, err, true).await?;
                    }
                    Ok(())
                }
                err @ _ => Err(err.to_string()),
            }
        }
        .boxed()
    }
}

#[async_trait]
impl<E: Endpoint> Agent<E, E::Peer> for TungClient<E> {
    async fn send_value(&self, message: E) -> Result<(), String> {
        let bytes = rmp_serde::to_vec(&message).expect("Failed to rmp-encode message");
        {
            let mut wss = self.wss.lock().await;
            let result = wss.send(Message::Binary(bytes)).await;
            if let Err(err) = result {
                Self::handle_write_err(&mut wss, err, false).await?;
            }
        };
        Ok(())
    }

    async fn await_message(&self, time: Duration) -> Result<Option<E::Peer>, String> {
        loop {
            let result = {
                let mut wss = self.wss.lock().await;
                tokio::time::timeout(time, wss.next()).await
            };
            let message = match result {
                Ok(Some(Ok(message))) => message,
                Ok(None) => return Err("Socket closed by peer".to_string()), // end of stream
                Ok(Some(Err(err))) => return Err(err.to_string()),           // stream error
                Err(err) => return Ok(None),                                 // timeout
            };
            let message = match message {
                Message::Ping(buf) => {
                    let mut wss = self.wss.lock().await;
                    let ret = wss.send(Message::Pong(buf)).await;
                    if let Err(err) = ret {
                        Self::handle_write_err(&mut wss, err, false).await?;
                    }
                    continue;
                }
                Message::Pong(_) => {
                    // TODO update pong time
                    continue;
                }
                Message::Text(_) => {
                    // we do not support text messages
                    continue;
                }
                Message::Binary(buf) => buf,
                Message::Close(Some(frame)) => {
                    return Err(frame.reason.into_owned());
                }
                Message::Close(None) => {
                    return Err(String::new());
                }
            };
            let e: E::Peer =
                rmp_serde::from_read(Cursor::new(&message)).map_err(|err| err.to_string())?;
            break Ok(Some(e));
        }
    }

    fn close(&self) {
        unimplemented!()
    }
}
