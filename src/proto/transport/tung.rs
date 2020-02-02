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

use super::WsClient;

type Wss = WebSocketStream<TcpStream>;

#[derive(Debug)]
pub struct TungClient {
    wss: Mutex<Wss>,
}

impl TungClient {
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
impl WsClient for TungClient {
    async fn open(server: &str) -> Result<Self, String> {
        let (wss, _) = tokio_tungstenite::connect_async(server)
            .await
            .map_err(|err| err.to_string())?;
        let wss = Mutex::new(wss);
        Ok(Self { wss })
    }

    async fn send_message(&self, bytes: &[u8]) -> Result<(), String> {
        {
            let mut wss = self.wss.lock().await;
            let result = wss.send(Message::Binary(bytes.to_vec())).await;
            if let Err(err) = result {
                Self::handle_write_err(&mut wss, err, false).await?;
            }
        };
        Ok(())
    }

    async fn await_message(&self, time: Duration) -> Result<Option<Vec<u8>>, String> {
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
            break Ok(Some(message));
        }
    }

    async fn close(&self) {
        let mut lock = self.wss.lock().await;
        let _ = lock.close(None).await; // the close frame is not used for communication
                                        // even if an IO error occurred during close, we don't care because we're already gone
    }
}
