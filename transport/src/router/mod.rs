use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use eviov_proto::{ClientEndpoint, Endpoint, Protocol};
use eviov_types::ObjectId;
use futures::channel::mpsc;
use futures::lock::Mutex;
use serde::{Deserialize, Serialize};

use super::WsClient;

mod receiver;
use receiver::AllReceivers;
pub use receiver::Receiver;

mod error;
pub use error::*;

mod handle;
pub use handle::Handle;

/// The main communications manager for a process.
pub struct Router<C: WsClient> {
    receivers: Mutex<AllReceivers>,
    out_pool: OutPool<C>,
}

impl<C: WsClient> Router<C> {
    /// Opens an internal communication within this process.
    ///
    /// # Return values
    /// Returns a handle to communicate with the peer node.
    ///
    /// # Errors
    /// This method errors if the specified node is not managed in this process.
    pub async fn open_internal<Me: Endpoint>(
        &self,
        id: ObjectId,
    ) -> Result<Handle<Me>, InternalOpenError> {
        let (send_me, recv_me) = mpsc::unbounded();
        let (send_peer, recv_peer) = mpsc::unbounded();

        {
            let receivers = self.receivers.lock().await;
            receivers
                .get::<Me::Peer>()
                .get(id)
                .ok_or(InternalOpenError::NoSuchObject)?
                .open(Handle::new(send_peer, recv_me));
        }

        Ok(Handle::new(send_me, recv_peer))
    }

    /// Opens an external communication with a node on another process.
    ///
    /// # Parameters
    /// - `server`: the socket address (with port) of the peer.
    /// - `id`: the object ID of the peer node.
    /// - `challenge_fn`: a function that takes an input byte slice and returns a byte vector,
    /// answering the login challenge.
    pub async fn open_external<Me>(
        &mut self,
        server: &str,
        id: ObjectId,
        challenge_fn: impl FnOnce(&[u8]) -> Vec<u8>,
    ) -> Result<Handle<Me>, OpenError>
    where
        Me: Endpoint + ClientEndpoint,
    {
        let (send_me, recv_me) = mpsc::unbounded();
        let (send_peer, recv_peer) = mpsc::unbounded();

        let peer = Handle::new(send_peer, recv_me);

        let client = self
            .out_pool
            .get_or_open::<Me, _>(server, challenge_fn)
            .await?;

        Ok(Handle::new(send_me, recv_peer))
    }
}

/// The message returned by the server after receiving the challenge.
#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeResult {
    /// Challenge is accepted.
    Ok,
    /// Challenge has failed. Socket should be closed immediately.
    Fail,
}

/// The pool of reusable external connections.
struct OutPool<C: WsClient> {
    clients: Mutex<HashMap<String, Arc<C>>>, // TODO figure out how to use (String, &'static str) as HashMap key without allocating a new string
}

impl<C: WsClient> OutPool<C> {
    /// Returns an external connection, opening a new one if absent.
    async fn get_or_open<Me, F>(&self, server: &str, challenge_fn: F) -> Result<Arc<C>, OpenError>
    where
        Me: Endpoint + ClientEndpoint,
        F: FnOnce(&[u8]) -> Vec<u8>,
    {
        let key = format!(
            "{}#{}",
            server,
            <<Me as Endpoint>::Protocol as Protocol>::name()
        );
        {
            let mut lock = self.clients.lock().await;
            if let Some(value) = lock.get(&key) {
                return Ok(Arc::clone(value));
            }

            // Do not drop the mutex lock, otherwise other routines might open connection for the
            // same server here
            let client = C::open(server).await?;

            async fn recv<C: WsClient>(client: &C) -> Result<Vec<u8>, OpenError> {
                match client.await_message(crate::OPEN_CONN_TIMEOUT).await {
                    Ok(Some(challenge)) => Ok(challenge),
                    Ok(None) => Err(OpenError::Timeout),
                    Err(err) => Err(OpenError::Io(err)),
                }
            }

            client
                .send_message(<<Me as Endpoint>::Protocol as Protocol>::name().as_bytes())
                .await?;
            let challenge = recv(&client).await?;
            let reply = challenge_fn(&challenge);
            client.send_message(&reply).await?;

            let result: ChallengeResult =
                rmp_serde::from_read(io::Cursor::new(recv(&client).await?))
                    .map_err(|err| err.to_string())?;
            match result {
                ChallengeResult::Ok => (),
                ChallengeResult::Fail => return Err(OpenError::ChallengeFailed),
            };

            lock.insert(key.clone(), Arc::new(client))
                .expect_none("Emptiness checked above, mutex locked");

            Ok(Arc::clone(lock.get(&key).expect("Just inserted")))
        }
    }
}
