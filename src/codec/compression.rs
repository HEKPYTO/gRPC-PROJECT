// src/codec/compressions.rs
use crate::Error;
use flate2::{read::GzDecoder, write::GzEncoder, Compression as GzCompression};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy)]
pub enum CompressionEncoding {
    None,
    Gzip,
    Deflate,
}

pub trait Compression {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Error>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Error>;
}

pub struct GzipCompression;

impl Compression for GzipCompression {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut encoder = GzEncoder::new(Vec::new(), GzCompression::default());
        encoder.write_all(data)?;
        Ok(encoder.finish()?)
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        let mut decoder = GzDecoder::new(data);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

pub struct NoCompression;

impl Compression for NoCompression {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(data.to_vec())
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(data.to_vec())
    }
}
