// src/codec/mod.rs
use crate::Error;
use bytes::BytesMut;

pub mod compression;
pub mod frame;

pub use compression::{Compression, CompressionEncoding};
pub use frame::Frame;

#[derive(Debug)]
pub struct GrpcCodec {
    max_message_size: usize,
}

impl Default for GrpcCodec {
    fn default() -> Self {
        Self {
            max_message_size: crate::client::DEFAULT_MAX_FRAME_SIZE,
        }
    }
}

impl GrpcCodec {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_message_size = size;
        self
    }

    pub fn encode(&mut self, frame: Frame, dst: &mut BytesMut) -> crate::Result<()> {
        if frame.len() > self.max_message_size {
            return Err(Error::MessageTooLarge(frame.len()));
        }
        frame.encode(dst);
        Ok(())
    }

    pub fn decode(&mut self, src: &mut BytesMut) -> crate::Result<Option<Frame>> {
        if let Some(frame) = Frame::decode(src) {
            if frame.len() > self.max_message_size {
                return Err(Error::MessageTooLarge(frame.len()));
            }
            Ok(Some(frame))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec_new() {
        let codec = GrpcCodec::new();
        assert_eq!(
            codec.max_message_size,
            crate::client::DEFAULT_MAX_FRAME_SIZE
        );
    }

    #[test]
    fn test_codec_with_max_message_size() {
        let codec = GrpcCodec::new().with_max_message_size(1024);
        assert_eq!(codec.max_message_size, 1024);
    }
}
