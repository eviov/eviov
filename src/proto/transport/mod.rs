use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};

use futures::channel::oneshot;
use futures::future::Future;
use futures::lock::Mutex;

use super::{Endpoint, Message, MessageFrom, QueryId, QueryRequestFrom};

pub mod ws;

pub mod local;

pub struct Conn<A, E>
where
    A: Agent<E, <E as Endpoint>::Peer>,
    E: Endpoint,
{
    agent: A,
    query_id: AtomicU32,
    responses: Mutex<HashMap<QueryId, oneshot::Sender<Option<E::Peer>>>>,
    error: (
        Mutex<Option<oneshot::Sender<String>>>,
        Mutex<oneshot::Receiver<String>>,
    ),
}

impl<A, E> Conn<A, E>
where
    A: Agent<E, <E as Endpoint>::Peer>,
    E: Endpoint,
{
    pub fn new(agent: A) -> Self {
        let (sender, receiver) = oneshot::channel::<String>();
        Self {
            agent,
            query_id: AtomicU32::new(0),
            responses: Mutex::new(HashMap::new()),
            error: (Mutex::new(Some(sender)), Mutex::new(receiver)),
        }
    }

    pub fn send_single<M: Message + MessageFrom<E>>(&self, message: M) {
        self.agent.send_value(message.to_enum());
    }

    pub async fn send_query<M: QueryRequestFrom<E>>(&self, mut request: M) -> Option<M::Response> {
        let id = self.query_id.fetch_add(1, Ordering::AcqRel);
        let id = QueryId(id);
        request.set_query_id(id);

        let (sender, receiver) = oneshot::channel();
        {
            let mut lock = self.responses.lock().await;
            if lock.len() >= crate::hardcode::MAX_QUERY_POOL_SIZE {
                self.schedule_error("Exceeded max query pool size".to_string())
                    .await;
            }
            lock.insert(id, sender).expect_none("Duplicate query ID");
        }
        // ordering: make sure response handler is registered before the request is sent

        self.agent.send_value(request.to_enum());
        let response = receiver.await.ok()??;
        <<M as QueryRequestFrom<E>>::Response as MessageFrom<E::Peer>>::from_enum(response)
    }

    pub async fn schedule_error(&self, error: String) {
        let mut lock = self.error.0.lock().await;
        if let Some(sender) = lock.take() {
            let _ = sender.send(error); // if the receiver is dropped, drop the error too
        }
    }

    pub async fn heartbeat(&self, until: Instant) -> Result<(), String> {
        loop {
            {
                let mut recv = self.error.1.lock().await;
                if let Ok(Some(err)) = recv.try_recv() {
                    self.shutdown().await;
                    return Err(err);
                }
            }
            let message = self.agent.await_message(until - Instant::now()).await;
            if let Some(message) = message {
                if let Some(query_id) = message.query_id() {
                    let sender = {
                        let mut responses = self.responses.lock().await;
                        responses.remove(&query_id)
                    };
                    if let Some(sender) = sender {
                        let _ = sender.send(Some(message)); // dropped response handler should be skipped
                    } else {
                        self.schedule_error(
                            "Received response of nonexistent or stale Queryid".to_string(),
                        )
                        .await;
                    }
                }
            } else {
                break;
            }
        }
        {
            let mut recv = self.error.1.lock().await;
            if let Ok(Some(err)) = recv.try_recv() {
                self.shutdown().await;
                return Err(err);
            }
        }
        Ok(())
    }

    async fn shutdown(&self) {
        self.agent.close();
        let mut responses = self.responses.lock().await;
        for (_, sender) in responses.drain() {
            let _ = sender.send(None); // dropped response handler should be skipped
        }
    }
}

pub trait Agent<SendMsg: Endpoint, RecvMsg: Endpoint> {
    fn send_value(&self, message: SendMsg);

    type AwaitMessage: Future<Output = Option<RecvMsg>>;
    fn await_message(&self, time: Duration) -> Self::AwaitMessage;

    fn close(&self);
}
