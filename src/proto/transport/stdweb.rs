use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use futures::channel::oneshot;
use stdweb::web::{
    self,
    event::{self, IMessageEvent},
    IEventTarget, WebSocket,
};

use crate::proto::{Endpoint, Protocol};

pub struct StdwebWs<E: Endpoint> {
    ws: Arc<WebSocket>,
    _ph: PhantomData<E>,
}

impl<E: Endpoint> StdwebWs<E> {
    pub fn new(server: &str) -> Result<Self, String> {
        let ws = WebSocket::new_with_protocols(server, &[E::Protocol::name()])
            .map_err(|err| err.to_string())?;
        let ws = Arc::new(ws);
        ws.set_binary_type(web::SocketBinaryType::ArrayBuffer);

        Ok(Self {
            ws,
            _ph: PhantomData,
        })
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
