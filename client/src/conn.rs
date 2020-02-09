use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use stdweb::web::{self, event, IEventTarget, WebSocket};

pub struct Conn {
    fsm: Rc<RefCell<Fsm>>,
}

impl Clone for Conn {
    fn clone(&self) -> Self {
        Self {
            fsm: Rc::clone(&self.fsm),
        }
    }
}

macro_rules! add_event_listener {
    ($fsm:ident, $method:ident ($event:ty)) => {{
        let fsm_clone = Rc::clone(&$fsm);
        let _ = $fsm
            .borrow()
            .ws()
            .expect("Fsm state should not be closed when setting up")
            .add_event_listener(move |event: $event| {
                fsm_clone.borrow_mut().$method(event);
            });
        // we won't need to cancel the handler; drop it
    }};
}

impl Conn {
    pub fn connect(server: &str) -> Result<(), Cow<'static, str>> {
        let ws = WebSocket::new_with_protocols(server, &["eviov"])
            .map_err(|_| format!("Failed to connect to server {}", server))?;
        ws.set_binary_type(web::SocketBinaryType::ArrayBuffer);

        let fsm = Rc::new(RefCell::new(Fsm::Connecting(ws)));
        add_event_listener!(fsm, connected(event::SocketOpenEvent));
        add_event_listener!(fsm, closed(event::SocketCloseEvent));
        add_event_listener!(fsm, errored(event::SocketErrorEvent));
        add_event_listener!(fsm, message(event::SocketMessageEvent));

        Ok(())
    }
}

enum Fsm {
    Connecting(WebSocket),
    Handshake(Handshake),
    Closed,
}

impl Fsm {
    fn ws(&self) -> Option<&WebSocket> {
        let ws = match self {
            Self::Connecting(ws) => ws,
            Self::Handshake(Handshake(ws)) => ws,
            Self::Closed => return None,
        };
        Some(ws)
    }

    fn connected(&mut self, _: event::SocketOpenEvent) {
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

    fn closed(&mut self, _: event::SocketCloseEvent) {
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

    fn errored(&mut self, _: event::SocketErrorEvent) {
        if let Some(ws) = self.ws() {
            ws.close();
        }
        *self = Self::Closed;
        web::alert("WebSocket error encountered");
    }

    fn message(&mut self, event: event::SocketMessageEvent) {
        use event::IMessageEvent;
        match event.data() {
            event::SocketMessageData::ArrayBuffer(_buf) => unimplemented!(),
            data @ _ => {
                log::warn!("Expected ArrayBuffer from ws, got {:?}", data);
            }
        }
    }
}

struct Handshake(WebSocket);
