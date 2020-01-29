use std::borrow::Cow;

use async_trait::async_trait;
use futures::lock::Mutex;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio::net::TcpStream;
use tungstenite::handshake::client::Request;
use tungstenite::protocol::{frame::coding::CloseCode, CloseFrame};
use tungstenite::Message;

use super::{Handler, WsClient};

struct TungWs<H: Handler> {
    wss: Mutex<tokio_tungstenite::WebSocketStream<TcpStream>>,
    handler: H,
}

#[async_trait]
impl<H: Handler> WsClient<H> for TungWs<H> {
    async fn connect(server: &str, proto: &str, handler: H) -> Result<Self, String> {
        let mut request = Request::from(url::Url::parse(server).map_err(|err| err.to_string())?);
        request.add_protocol(Cow::Owned(proto.to_string()));
        let (wss, _) = tokio_tungstenite::connect_async(request)
            .await
            .map_err(|err| err.to_string())?;
        let wss = Mutex::new(wss);
        Ok(TungWs { wss, handler })
    }

    async fn maintain(&self) {
        loop {
            let message = {
                let mut wss = self.wss.lock().await;
                wss.next().await // TODO is there a deadlock?
            };
            match message {
                Some(Ok(Message::Binary(bytes))) => {
                    self.handler.on_message(bytes);
                }
                Some(Ok(Message::Close(message))) => {
                    let message = match &message {
                        Some(frame) => {
                            match frame.code {
                                CloseCode::Normal => (),
                                code @ _ => {
                                    self.handler
                                        .on_error(format!("Socket closed with code {}", code));
                                    break;
                                }
                            }
                            frame.reason.as_ref()
                        }
                        None => "",
                    };
                    self.handler.on_close(message);
                }
                Some(Ok(_)) => continue,
                Some(Err(err)) => {
                    self.handler.on_error(err.to_string());
                    break;
                }
                None => break,
            }
        }
    }

    async fn send_bytes(&self, bytes: Vec<u8>) {
        let result = {
            let mut wss = self.wss.lock().await;
            wss.send(Message::Binary(bytes)).await
        }; // ignore errors at network layer
        if let Err(err) = result {
            self.handler.on_error(err.to_string());
        }
    }

    async fn close(&self, message: &str) {
        let result = {
            let mut wss = self.wss.lock().await;
            wss.close(Some(CloseFrame {
                code: CloseCode::Normal,
                reason: Cow::Borrowed(message),
            }))
            .await
        };
        if let Err(err) = result {
            self.handler.on_error(err.to_string());
        }
    }
}
