// src/transport/mod.rs
use crate::Result;
use bytes::Bytes;
use futures_util::future::BoxFuture;
use std::fmt::Debug;

pub mod connection;
pub mod http2;

pub use connection::Connection;
pub use http2::Http2Transport;

pub trait Transport: Debug {
    fn send(&mut self, data: Bytes) -> BoxFuture<'_, Result<()>>;
    fn receive(&mut self) -> BoxFuture<'_, Result<Option<Bytes>>>;
    fn close(&mut self) -> BoxFuture<'_, Result<()>>;
}
