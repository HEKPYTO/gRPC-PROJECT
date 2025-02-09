// src/error.rs
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Transport error: {0}")]
    Transport(#[from] h2::Error),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Codec error: {0}")]
    Codec(String),

    #[error("Service error: {0}")]
    Service(Box<dyn std::error::Error + Send + Sync>),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] http::Error),

    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Send error")]
    SendError,

    #[error("Receive error")]
    ReceiveError,

    #[error("Invalid metadata key: {0}")]
    InvalidMetadataKey(String),

    #[error("Invalid metadata value: {0}")]
    InvalidMetadataValue(String),

    #[error("Encode error: {0}")]
    Encode(#[from] prost::EncodeError),

    #[error("Decode error: {0}")]
    Decode(#[from] prost::DecodeError),

    #[error("Elapsed error: {0}")]
    Elapsed(#[from] tokio::time::error::Elapsed),
}
