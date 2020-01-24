use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::{self, WebSocket, event, IEventTarget};

pub fn choose_server(server: &str) -> Result<(), Cow<'static, str>> {
    let ws = WebSocket::new_with_protocols(server, &["eviov"])
        .map_err(|_| format!("Failed to connect to server {}", server))?;
    ws.set_binary_type(web::SocketBinaryType::ArrayBuffer);

    let fsm = Rc::new(RefCell::new(Fsm::Connecting(ws)));
    {
        let clone = Rc::clone(&fsm);
        fsm.borrow()
            .ws()
            .unwrap()
            .add_event_listener(move |_: event::SocketOpenEvent| {
                clone.borrow_mut().connected();
            });
    }

    {
        let clone = Rc::clone(&fsm);
        fsm.borrow()
            .ws()
            .unwrap()
            .add_event_listener(move |_: event::SocketCloseEvent| {
                clone.borrow_mut().closed();
            });
    }

    {
        let clone = Rc::clone(&fsm);
        fsm.borrow()
            .ws()
            .unwrap()
            .add_event_listener(move |_: event::SocketErrorEvent| {
                clone.borrow_mut().errored();
            });
    }

    {
        use event::IMessageEvent;

        let clone = Rc::clone(&fsm);
        fsm.borrow()
            .ws()
            .unwrap()
            .add_event_listener(move |event: event::SocketMessageEvent| match event.data() {
                event::SocketMessageData::ArrayBuffer(buf) => {
                    let buf = Vec::<u8>::from(buf);
                    let data = match rmp_serde::from_read_ref(&buf) {
                        Ok(value) => value,
                        Err(err) => {
                            log::warn!("Failed decoding ws data: {}", err);
                            return;
                        }
                    };

                    clone.borrow_mut().message(data);
                }
                _ => {
                    log::warn!("Expected ArrayBuffer from ws, got {:?}", event.data());
                }
            });
    }
    Ok(())
}

enum Fsm {
    Connecting(web::WebSocket),
    Handshake(Handshake),
    Closed,
}

impl Fsm {
    fn ws(&self) -> Option<&web::WebSocket> {
        let ws = match self {
            Self::Connecting(ws) => ws,
            Self::Handshake(Handshake(ws)) => ws,
            Self::Closed => return None,
        };
        Some(ws)
    }

    fn connected(&mut self) {
        match self {
            Self::Connecting(ws) => {
                *self = Self::Handshake(Handshake(ws.clone()));
                unimplemented!("Perform handshake")
            }
            _ => {
                log::warn!("Socket connect dispatched again");
            }
        }
    }

    fn closed(&mut self) {
        let ws = match self {
            Self::Closed => {
                log::warn!("Socket close dispatched again");
                return;
            }
            _ => self.ws().unwrap(),
        };
        ws.close();
        *self = Self::Closed;
    }

    fn errored(&mut self) {
        if let Some(ws) = self.ws() {
            ws.close();
        }
        *self = Self::Closed;
        web::alert("WebSocket error encountered");
    }

    fn message(&mut self, data: eviov::proto::FromServer<'_>) {
        unimplemented!("Handle {:?}", data)
    }
}

struct Handshake(web::WebSocket);
