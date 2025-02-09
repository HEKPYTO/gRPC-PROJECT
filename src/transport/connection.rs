// src/transport/connection.rs
use crate::transport::Transport;
use crate::Result;
use bytes::Bytes;

#[derive(Debug)]
pub struct Connection<T> {
    transport: T,
    max_frame_size: usize,
}

impl<T: Transport> Connection<T> {
    pub fn new(transport: T) -> Self {
        Self {
            transport,
            max_frame_size: crate::client::DEFAULT_MAX_FRAME_SIZE,
        }
    }

    pub fn with_max_frame_size(mut self, size: usize) -> Self {
        self.max_frame_size = size;
        self
    }

    pub async fn send(&mut self, data: Bytes) -> Result<()> {
        if data.len() > self.max_frame_size {
            return Err(crate::Error::MessageTooLarge(data.len()));
        }
        self.transport.send(data).await
    }

    pub async fn receive(&mut self) -> Result<Option<Bytes>> {
        self.transport.receive().await
    }

    pub async fn close(&mut self) -> Result<()> {
        self.transport.close().await
    }
}
