use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use futures::channel::oneshot;
use stdweb::web::{
    self,
    event::{self, IMessageEvent},
    IEventTarget, WebSocket,
};

use super::{Handler, WsClient};

pub struct StdwebWs<H: Handler> {
    ws: Arc<WebSocket>,
    handler: Arc<H>,
}

#[async_trait]
impl<H: Handler> WsClient<H> for StdwebWs<H> {
    async fn connect(server: &str, proto: &str, handler: H) -> Result<Self, String> {
        let ws = WebSocket::new_with_protocols(server, &[proto]).map_err(|err| err.to_string())?;
        ws.set_binary_type(web::SocketBinaryType::ArrayBuffer);
        let ws = Arc::new(ws);
        let handler = Arc::new(handler);
        {
            let handler = Arc::clone(&handler);
            let _ = ws.add_event_listener(move |event: event::SocketMessageEvent| {
                let data = event.data();
                let data = match data {
                    event::SocketMessageData::ArrayBuffer(buf) => buf,
                    _ => return, // we don't care about text messages and blobs
                };
                handler.on_message(data.into());
            });
        }
        {
            let handler = Arc::clone(&handler);
            let _ = ws.add_event_listener(move |event: event::SocketCloseEvent| {
                if event.code() != web::SocketCloseCode::NORMAL_CLOSURE {
                    handler.on_error(format!("Socket closed with code {:?}", event.code()));
                }
                handler.on_close(&event.reason());
            });
        }

        wait_open(&ws).await?;

        {
            let ws_clone = Arc::clone(&ws);
            let handler = Arc::clone(&handler);
            let _ = ws.add_event_listener(move |event: event::SocketErrorEvent| {
                handler.on_error(format!("{:?}", event));
                ws_clone.close();
            });
        }

        Ok(Self { ws, handler })
    }

    async fn maintain(&self) {}

    async fn send_bytes(&self, bytes: Vec<u8>) {
        if let Err(err) = self.ws.send_bytes(&bytes) {
            self.handler.on_error(format!("{:?}", err));
        }
    }

    async fn close(&self, message: &str) {
        let result = self
            .ws
            .close_with_status(web::SocketCloseCode::NORMAL_CLOSURE, message);
        if let Err(err) = result {
            self.handler.on_error(err.to_string());
        }
    }
}

async fn wait_open(ws: &WebSocket) -> Result<(), String> {
    let (sender, receiver) = oneshot::channel();
    let sender1 = Arc::new(Mutex::new(Some(sender)));
    let sender2 = Arc::clone(&sender1);

    let list1 = ws.add_event_listener(move |_: event::SocketOpenEvent| {
        let mut opt = sender1.lock().unwrap();
        match opt.take() {
            Some(sender) => {
                let _ = sender.send(Ok(()));
                // do nothing if socket future is dropped
            }
            None => (),
        }
    });
    let list2 = ws.add_event_listener(move |event: event::SocketOpenEvent| {
        let mut opt = sender2.lock().unwrap();
        match opt.take() {
            Some(sender) => {
                let _ = sender.send(Err(format!("{:?}", event)));
                // do nothing if socket future is dropped
            }
            None => panic!(),
        }
    });

    let ret = receiver.await.expect("Not cancelled anywhere");
    list1.remove();
    list2.remove();
    ret
}
