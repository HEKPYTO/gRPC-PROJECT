// src/metadata.rs
use crate::{Error, Result};
use bytes::Bytes;
use http::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub struct Metadata {
    headers: HeaderMap,
    binary: std::collections::HashMap<String, Bytes>,
}

impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, key: &str, value: &str) -> Result<()> {
        let key = normalize_key(key)?;
        let value = HeaderValue::from_str(value)
            .map_err(|_| Error::InvalidMetadataValue(value.to_string()))?;

        self.headers.insert(key, value);
        Ok(())
    }

    pub fn insert_bin(&mut self, key: &str, value: Bytes) -> Result<()> {
        let key = format!("{}-bin", normalize_key(key)?);
        self.binary.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<&HeaderValue> {
        normalize_key(key).ok().and_then(|k| self.headers.get(k))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&HeaderName, &HeaderValue)> {
        self.headers.iter()
    }
}

fn normalize_key(key: &str) -> Result<HeaderName> {
    if key.trim().is_empty() {
        return Err(Error::InvalidMetadataKey(key.to_string()));
    }
    HeaderName::from_str(key).map_err(|_| Error::InvalidMetadataKey(key.to_string()))
}
