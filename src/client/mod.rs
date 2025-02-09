// src/client/mod.rs
use crate::transport::{Connection, Http2Transport};
use crate::{Error, Metadata, Result};
use bytes::Bytes;
use http::{Request, Response, Uri};
use std::convert::TryInto;
use std::str::FromStr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

pub const DEFAULT_MAX_FRAME_SIZE: usize = 4 * 1024 * 1024; // 4MB

#[derive(Debug)]
pub struct Client {
    connection: Connection<Http2Transport>,
    #[allow(dead_code)]
    config: ClientConfig,
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub max_message_size: usize,
    pub max_concurrent_streams: u32,
    pub enable_http2_keepalive: bool,
    pub http2_keepalive_interval: Duration,
    pub connect_timeout: Duration,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            max_message_size: DEFAULT_MAX_FRAME_SIZE,
            max_concurrent_streams: 100,
            enable_http2_keepalive: true,
            http2_keepalive_interval: Duration::from_secs(300),
            connect_timeout: Duration::from_secs(5),
        }
    }
}

impl Client {
    pub async fn connect<T: AsRef<str>>(addr: T, config: Option<ClientConfig>) -> Result<Self> {
        let config = config.unwrap_or_default();
        let addr_str = addr.as_ref();

        let uri = Uri::from_str(addr_str)
            .map_err(|_| Error::Protocol(format!("Invalid URI: {}", addr_str)))?;

        if !addr_str.starts_with("http://") && !addr_str.starts_with("https://") {
            return Err(Error::Protocol(
                "URI must start with http:// or https://".to_string(),
            ));
        }

        let host = uri
            .host()
            .ok_or_else(|| Error::Protocol("Missing host".to_string()))?;
        let port = uri.port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = timeout(config.connect_timeout, TcpStream::connect(&addr)).await??;

        let mut builder = h2::client::Builder::new();
        builder
            .initial_connection_window_size(
                config
                    .max_message_size
                    .try_into()
                    .map_err(|_| Error::Protocol("Size conversion failed".to_string()))?,
            )
            .initial_window_size(
                config
                    .max_message_size
                    .try_into()
                    .map_err(|_| Error::Protocol("Size conversion failed".to_string()))?,
            )
            .max_concurrent_streams(config.max_concurrent_streams);

        let (send_request, connection) =
            builder.handshake(stream).await.map_err(Error::Transport)?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        let transport = Http2Transport::new(send_request);
        let connection = Connection::new(transport).with_max_frame_size(config.max_message_size);

        Ok(Self { connection, config })
    }

    pub async fn unary<T, U>(
        &mut self,
        method: &str,
        request: T,
        metadata: Option<Metadata>,
    ) -> Result<Response<U>>
    where
        T: prost::Message,
        U: prost::Message + Default,
    {
        let mut request_builder = Request::builder()
            .method("POST")
            .uri(method)
            .header("content-type", "application/grpc")
            .header("te", "trailers");

        if let Some(metadata) = metadata {
            for (key, value) in metadata.iter() {
                request_builder = request_builder.header(key.as_str(), value);
            }
        }

        let mut buf = Vec::new();
        request.encode(&mut buf)?;

        let mut framed_data = Vec::with_capacity(buf.len() + 5);
        framed_data.push(0);
        framed_data.extend_from_slice(&(buf.len() as u32).to_be_bytes());
        framed_data.extend_from_slice(&buf);

        self.connection.send(Bytes::from(framed_data)).await?;

        if let Some(response_data) = self.connection.receive().await? {
            if response_data.len() < 5 {
                return Err(Error::Protocol("Invalid response frame".to_string()));
            }

            let _compressed = response_data[0] != 0;
            let length = u32::from_be_bytes([
                response_data[1],
                response_data[2],
                response_data[3],
                response_data[4],
            ]) as usize;

            if response_data.len() < 5 + length {
                return Err(Error::Protocol("Incomplete response frame".to_string()));
            }

            let message_data = &response_data[5..5 + length];
            let response = U::decode(message_data)?;
            Ok(Response::new(response))
        } else {
            Err(Error::Protocol("No response received".to_string()))
        }
    }
}
