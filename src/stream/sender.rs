use crate::prelude::*;
use tokio::sync::mpsc;

/// The stream sender
pub struct StreamSender<T> {
    pub(super) tx: Option<mpsc::UnboundedSender<Result<T>>>,
}

impl<T> StreamSender<T> {
    /// Sends the chunk of data to reader
    pub fn send(&self, data: impl Into<T>) -> Result<()> {
        if let Some(ref tx) = self.tx {
            tx.send(Ok(data.into()))
                .map_err(|_| Error::StreamClosed.into())
        } else {
            Err(Error::StreamClosed.into())
        }
    }

    /// Send the error to reader
    pub fn send_err(&self, e: DynError) -> Result<()> {
        if let Some(ref tx) = self.tx {
            tx.send(Err(e)).map_err(|_| Error::StreamClosed.into())
        } else {
            Err(Error::StreamClosed.into())
        }
    }

    /// Closes the sender (the reader is will also be closed)
    pub fn close(&mut self) {
        self.tx.take();
    }

    /// Checks the channel for closed
    pub fn is_closed(&self) -> bool {
        self.tx.as_ref().map(|tx| tx.is_closed()).unwrap_or(true)
    }
}

impl<T> From<mpsc::UnboundedSender<Result<T>>> for StreamSender<T> {
    fn from(tx: mpsc::UnboundedSender<Result<T>>) -> Self {
        Self { tx: Some(tx) }
    }
}
