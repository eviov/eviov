use std::error::Error;
use std::fmt;

use futures::channel::mpsc::{self, UnboundedSender};
use futures::future::{self, Future};

use super::*;
use crate::universe::{Runtime, RuntimeId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tag {
    local: u32,
    runtime: RuntimeId,
}

pub fn run<X: Extra>(
    runtime: &Runtime<X>,
    extra: X,
) -> (Tag, Handle<X::Message>, impl Future<Output = ()>) {
    let tag = Tag {
        local: runtime.next_id(),
        runtime: runtime.id().into(),
    };
    let (messages, messages_recv) = mpsc::unbounded();
    let runtime: Runtime<X> = Clone::clone(&runtime);
    (
        tag,
        Handle { messages },
        run_impl(runtime, extra, messages_recv),
    )
}

#[derive(Debug)]
pub struct Handle<M> {
    messages: UnboundedSender<Message<M>>,
}

impl<M> Handle<M> {
    pub async fn send(&self, message: Message<M>) -> Result<(), Closed> {
        future::poll_fn(|ctx| self.messages.poll_ready(ctx))
            .await
            .map_err(|_| Closed)?;
        self.messages.unbounded_send(message).map_err(|_| Closed)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Closed;

impl fmt::Display for Closed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to send message because receiver is gone")
    }
}

impl Error for Closed {}

#[derive(Debug)]
pub enum Message<M> {
    Extra(M),
}
