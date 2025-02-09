// src/codec/frame.rs
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct Frame {
    pub header: FrameHeader,
    pub data: Bytes,
}

#[derive(Debug)]
pub struct FrameHeader {
    pub compressed: bool,
    pub length: usize,
}

impl Frame {
    pub fn new(data: Bytes, compressed: bool) -> Self {
        let length = data.len();
        Self {
            header: FrameHeader { compressed, length },
            data,
        }
    }

    pub fn into_data(self) -> Bytes {
        self.data
    }

    pub fn len(&self) -> usize {
        self.header.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn encode(&self, dst: &mut BytesMut) {
        dst.put_u8(if self.header.compressed { 1 } else { 0 });
        dst.put_u32(self.header.length as u32);
        dst.extend_from_slice(&self.data);
    }

    pub fn decode(src: &mut BytesMut) -> Option<Self> {
        if src.len() < 5 {
            return None;
        }

        let compressed = src[0] != 0;
        let length = u32::from_be_bytes([src[1], src[2], src[3], src[4]]) as usize;

        if src.len() < 5 + length {
            return None;
        }

        src.advance(5);
        let data = src.split_to(length).freeze();

        Some(Frame::new(data, compressed))
    }
}
