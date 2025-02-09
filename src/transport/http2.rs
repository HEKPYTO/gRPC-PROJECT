// src/transport/http2.rs
use crate::{Error, Result};
use crate::transport::Transport;
use bytes::Bytes;
use futures_util::future::BoxFuture;
use h2::client::SendRequest;
use http::Request;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Http2Transport {
    #[allow(dead_code)]
    send_request: SendRequest<Bytes>,
    receiver: mpsc::UnboundedReceiver<Result<Bytes>>,
    sender: mpsc::UnboundedSender<Result<Bytes>>,
}

impl Http2Transport {
    pub fn new(send_request: SendRequest<Bytes>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        Self {
            send_request,
            receiver,
            sender,
        }
    }
}

impl Transport for Http2Transport {
    fn send(&mut self, data: Bytes) -> BoxFuture<'_, Result<()>> {
        let sender = self.sender.clone();
        Box::pin(async move {
            let request = Request::builder()
                .method("POST")
                .body(())
                .map_err(Error::Http)?;

            let (response, mut send_stream) = self.send_request
                .send_request(request, false)
                .map_err(Error::Transport)?;

            send_stream.send_data(data, true)
                .map_err(Error::Transport)?;

            let response = response.await.map_err(Error::Transport)?;
            let (_parts, mut body) = response.into_parts();
            
            let mut bytes = Vec::new();
            while let Some(chunk) = body.data().await {
                let chunk = chunk.map_err(Error::Transport)?;
                bytes.extend_from_slice(&chunk);
            }

            sender.send(Ok(Bytes::from(bytes)))
                .map_err(|_| Error::SendError)?;

            Ok(())
        })
    }

    fn receive(&mut self) -> BoxFuture<'_, Result<Option<Bytes>>> {
        Box::pin(async move {
            match self.receiver.recv().await {
                Some(result) => result.map(Some),
                None => Ok(None),
            }
        })
    }

    fn close(&mut self) -> BoxFuture<'_, Result<()>> {
        Box::pin(async { Ok(()) })
    }
}