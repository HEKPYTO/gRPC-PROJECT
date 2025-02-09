// src/lib.rs
pub mod client;
pub mod codec;
pub mod error;
pub mod metadata;
pub mod transport;

pub use client::Client;
pub use codec::{Frame, GrpcCodec};
pub use error::Error;
pub use metadata::Metadata;
pub use transport::{Connection, Transport};

pub type Result<T> = std::result::Result<T, Error>;

pub const GRPC_VERSION: &str = "1.0.0";
pub const DEFAULT_MAX_MESSAGE_SIZE: usize = 4 * 1024 * 1024;
pub const DEFAULT_WINDOW_SIZE: u32 = 65_535;
pub const DEFAULT_CONNECT_TIMEOUT_SECS: u64 = 5;
