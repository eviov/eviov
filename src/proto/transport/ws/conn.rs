use std::marker::PhantomData;

use super::WsClient;
use crate::proto;

pub struct Conn<C, P>
where
    C: WsClient<Handler>,
    P: proto::Protocol,
{
    client: C,
    _ph: PhantomData<P>,
}

impl<C, P> Conn<C, P>
where
    C: WsClient<Handler>,
    P: proto::Protocol,
{
    pub async fn new(server: &str) -> Result<Self, String> {
        let future = C::connect(server, P::name(), Handler);
        let result = future.await;
        let client: C = result?;
        Ok(Self {
            client,
            _ph: PhantomData,
        })
    }

    pub fn post_message(&self, message: impl proto::Message + proto::Single) {
        unimplemented!()
    }

    pub async fn post_query<Q: proto::QueryRequest>(&self, query: Q) -> Result<<<Q as proto::QueryRequest>::Query as proto::Query>::Response, QueryError> {
        unimplemented!()
    }
}

pub struct Handler;

impl super::Handler for Handler
{
    fn on_error(&self, error: String) {
        unimplemented!()
    }

    fn on_close(&self, error: &str) {
        unimplemented!()
    }

    fn on_message(&self, bytes: Vec<u8>) {
        unimplemented!()
    }
}

pub enum QueryError {
    SocketClosed,
}
