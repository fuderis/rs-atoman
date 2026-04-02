pub mod reader;
pub use reader::StreamReader;
pub mod sender;
pub use sender::StreamSender;

use crate::prelude::*;
use futures::{Stream as FuturesStream, StreamExt};
use serde::de::DeserializeOwned;

/// The stream read/send manager
pub struct Stream;

impl Stream {
    /// Creates a typed pair (Internal Channel)
    pub fn new<T>() -> (StreamSender<T>, StreamReader<T>) {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        (StreamSender::from(tx), StreamReader::from(rx))
    }

    /// Reads an external byte stream (reqwest) and spawns a worker for parsing in T
    pub fn read<T, S>(mut source: S) -> StreamReader<T>
    where
        T: DeserializeOwned + Send + 'static,
        S: FuturesStream<Item = Result<Bytes>> + Send + Unpin + 'static,
    {
        let (tx, rx) = Stream::new::<T>();

        // let's create a "sender" that will parse bytes and send them to StreamReader:
        tokio::spawn(async move {
            let mut buffer = Vec::new();

            while let Some(res) = source.next().await {
                match res {
                    Ok(bytes) => {
                        buffer.extend_from_slice(&bytes);

                        // parsing all accumulated complete JSON objects:
                        let mut de = serde_json::Deserializer::from_slice(&buffer).into_iter::<T>();
                        let mut offset = 0;

                        while let Some(Ok(item)) = de.next() {
                            if tx.send(item).is_err() {
                                return; // the reader is closed, we're exiting
                            }
                            offset = de.byte_offset();
                        }
                        buffer.drain(..offset);
                    }
                    Err(e) => {
                        tx.tx.as_ref().map(|s| s.send(Err(e)));
                        return;
                    }
                }
            }

            // check if there is anything there besides spaces and line breaks:
            if !buffer.iter().all(|b| b.is_ascii_whitespace()) {
                tx.tx
                    .as_ref()
                    .map(|s| s.send(Err(Error::UnexpectedEOF.into())));
            }
        });

        rx
    }

    /// Server stream (HTTP body)
    pub fn body<H, Fut>(handler: H) -> impl FuturesStream<Item = Result<Bytes>>
    where
        H: FnOnce(StreamSender<Bytes>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<Result<Bytes>>();

        // spawning stream handler:
        tokio::spawn(async move {
            handler(StreamSender { tx: Some(tx) }).await;
        });

        // create the stream body:
        futures::stream::unfold(
            rx,
            |mut rx| async move { rx.recv().await.map(|res| (res, rx)) },
        )
    }
}
