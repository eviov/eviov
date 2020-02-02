#[cfg(feature = "trait-stdweb")]
pub mod stdweb {
    use std::time::Duration;

    use futures::channel::oneshot;
    use futures::future::{self, Either, Future};

    pub fn delay(duration: Duration) -> oneshot::Receiver<()> {
        let (send, recv) = oneshot::channel();
        stdweb::web::set_timeout(
            move || {
                let _ = send.send(());
            },
            duration.as_millis() as u32,
        );

        recv
    }

    pub async fn timeout<T, F: Future<Output = T> + Unpin>(
        duration: Duration,
        fut: F,
    ) -> Result<T, F> {
        match future::select(delay(duration), fut).await {
            Either::Left((_, fut)) => Err(fut),
            Either::Right((t, _)) => Ok(t),
        }
    }
}

#[cfg(feature = "trait-tokio")]
pub mod tokio {
    use std::time::Duration;

    use futures::future::{self, Either, Future};

    pub fn delay(duration: Duration) -> tokio::time::Delay {
        tokio::time::delay_for(duration)
    }

    pub async fn timeout<T, F: Future<Output = T> + Unpin>(
        duration: Duration,
        fut: F,
    ) -> Result<T, F> {
        match future::select(delay(duration), fut).await {
            Either::Left((_, fut)) => Err(fut),
            Either::Right((t, _)) => Ok(t),
        }
    }
}
