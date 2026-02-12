use crate::prelude::*;
use bytes::Bytes;
use tokio::sync::mpsc;

/// Server stream-response handler
#[derive(Clone)]
pub struct Stream {
    tx: mpsc::UnboundedSender<Result<Bytes>>,
}

impl Stream {
    /// Spawns stream handler & returns response body
    pub async fn spawn<H, P, FutH, FutP>(
        handler: H,
        processor: P,
    ) -> impl futures::Stream<Item = Result<Bytes>>
    where
        H: FnOnce(Stream) -> FutH + Send + 'static,
        FutH: Future<Output = ()> + Send + 'static,
        P: FnOnce(Result<Bytes>) -> FutP + Clone + Send + 'static,
        FutP: Future<Output = Result<Bytes>> + Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel::<Result<Bytes>>();

        // spawning handler with sender:
        tokio::spawn(async move {
            handler(Stream { tx }).await;
        });

        // creating unfold stream with processor:
        futures::stream::unfold(rx, move |mut rx| {
            let processor = processor.clone();
            async move {
                match rx.recv().await {
                    Some(msg) => {
                        let processed = processor(msg).await;
                        Some((processed, rx))
                    }
                    _ => None,
                }
            }
        })
    }

    /// Sends response chunk into stream
    pub fn send(&self, data: Result<Bytes>) -> Result<()> {
        self.tx.send(data).map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, "stream closed").into()
        })
    }

    /// Returns true if client is disconnected
    pub fn is_closed(&self) -> bool {
        self.tx.is_closed()
    }
}
