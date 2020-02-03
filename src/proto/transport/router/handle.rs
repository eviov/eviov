use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;

use futures::channel::{mpsc, oneshot};
use futures::lock::Mutex;
use futures::sink::SinkExt;
use futures::stream::StreamExt;

use crate::proto::{Endpoint, Handler, MessageFrom, QueryId, QueryRequestFrom, Single};

#[derive(Debug)]
pub struct Handle<SendMsg: Endpoint> {
    send: Arc<Mutex<mpsc::UnboundedSender<SendMsg>>>,
    recv: Mutex<mpsc::UnboundedReceiver<SendMsg::Peer>>,

    error: Mutex<Option<String>>,
    next_query_id: AtomicU32,
    query_handlers: Mutex<HashMap<QueryId, oneshot::Sender<SendMsg::Peer>>>,
}

impl<SendMsg: Endpoint> Handle<SendMsg> {
    pub fn new(
        send: mpsc::UnboundedSender<SendMsg>,
        recv: mpsc::UnboundedReceiver<SendMsg::Peer>,
    ) -> Self {
        Self {
            send: Arc::new(Mutex::new(send)),
            recv: Mutex::new(recv),

            error: Mutex::new(None),
            next_query_id: AtomicU32::new(0),
            query_handlers: Mutex::new(HashMap::new()),
        }
    }

    pub async fn send_single<M>(&self, message: M)
    where
        M: MessageFrom<SendMsg> + Single,
    {
        let mut send = self.send.lock().await;
        let _ = send.send(message.to_enum()).await; // send error is not significant
    }

    pub async fn send_query<M>(&self, mut query: M) -> Result<M::Response, String>
    where
        M: QueryRequestFrom<SendMsg>,
    {
        let query_id = self.next_query_id.fetch_add(1, Ordering::SeqCst);
        let query_id = QueryId(query_id);

        query.set_query_id(query_id);

        let (sender, receiver) = oneshot::channel();
        {
            let mut query_handlers = self.query_handlers.lock().await;
            query_handlers
                .insert(query_id, sender)
                .expect_none("Duplicate query ID");
        }

        {
            let mut send = self.send.lock().await;
            if let Err(err) = send.send(query.to_enum()).await {
                self.schedule_error(err.to_string()).await;
                self.check_error().await?;
            }
        }

        let msg = receiver.await; // if schedule_error() is called during this await, this will interrupt with an Err(oneshot::Canceled)
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => {
                self.check_error().await?;
                unreachable!("oneshot senders are dropped without sending a value only when the connection is closed")
            }
        };

        if let Some(msg) = M::Response::from_enum(msg) {
            Ok(msg)
        } else {
            self.schedule_error("Query response has incompatible type")
                .await;
            self.check_error().await?;
            unreachable!("check_error() should break after calling schedule_error()")
        }
    }

    pub async fn heartbeat<H>(
        &self,
        until: Instant,
        handler: Arc<H>,
        context: crate::Context<impl crate::ContextImpl>,
    ) -> Result<(), String>
    where
        H: Handler<Endpoint = SendMsg>,
    {
        let mut recv = match self.recv.try_lock() {
            Some(guard) => guard,
            None => {
                panic!("Race condition: two routines tried to receive the same connection handle")
            }
        };

        loop {
            self.check_error().await?;

            // no problem that we lock recv for a long time, since it is a bug if there are two
            // routines trying to receive from the same connection handle simultaneously.

            let now = Instant::now(); // store the duration, to avoid race conditions
            if until <= now {
                return Ok(()); // timeout
            }
            let msg: Result<Option<SendMsg::Peer>, _> =
                context.timeout(until - now, recv.next()).await;

            let msg = match msg {
                Ok(Some(msg)) => msg,
                Ok(None) => {
                    self.schedule_error("End of receive stream").await;
                    self.check_error().await?;
                    unreachable!("check_error() should break after calling schedule_error()")
                }
                Err(_) => return Ok(()), // timeout reached
            };

            if let Some(query_id) = msg.response_query_id() {
                let sender = {
                    let mut query_handlers = self.query_handlers.lock().await;
                    query_handlers.remove(&query_id)
                };
                if let Some(sender) = sender {
                    // do nothing if the receiver stopped awaiting
                    // (although this shouldn't happen right now)
                    let _ = sender.send(msg);
                } else {
                    self.schedule_error(
                        "Received response message with unassociated or obsolete query ID",
                    )
                    .await;
                    self.check_error().await?;
                }
            } else {
                let send = Arc::clone(&self.send);
                let handler = Arc::clone(&handler);
                let future = async move {
                    if let Some(resp) = handler.handle_message(msg).await {
                        let mut send = send.lock().await;
                        let _ = send.send(resp).await; // send error is not significant
                    }
                };
                context.spawn(future);
            }
        }
        // this loop always breaks at the `timeout` function
    }

    async fn schedule_error(&self, err: impl Into<String>) {
        {
            let mut lock = self.error.lock().await;
            if lock.is_none() {
                *lock = Some(err.into());
            }
        }
        // do not update error again, because later invocations are most likely induced errors
        // TODO FIXME: what if race condition makes the induced error come first?

        // now cleanup the channels
        {
            let mut lock = self.send.lock().await;
            lock.disconnect();
        }
        {
            let mut lock = self.recv.lock().await;
            lock.close();
        }
        {
            let mut lock = self.query_handlers.lock().await;
            lock.clear(); // this will drop all handlers, interrupting their respective receivers
        }
    }

    async fn check_error(&self) -> Result<(), String> {
        let guard = self.error.lock().await;
        match &*guard {
            Some(err) => Err(err.clone()),
            None => Ok(()),
        }
    }
}
