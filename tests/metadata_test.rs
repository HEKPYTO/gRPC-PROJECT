// tests/metadata_test.rs
use bytes::Bytes;
use grpc_project::{Error, Metadata};

#[test]
fn test_metadata_creation() {
    let mut metadata = Metadata::new();
    assert!(metadata.insert("content-type", "application/grpc").is_ok());
    assert!(metadata.get("content-type").is_some());
}

#[test]
fn test_invalid_metadata_key() {
    let mut metadata = Metadata::new();
    assert!(matches!(
        metadata.insert("", "value"),
        Err(Error::InvalidMetadataKey(_))
    ));
}

#[test]
fn test_invalid_metadata_value() {
    let mut metadata = Metadata::new();
    assert!(matches!(
        metadata.insert("key", "invalid\0value"),
        Err(Error::InvalidMetadataValue(_))
    ));
}

#[test]
fn test_binary_metadata() {
    let mut metadata = Metadata::new();
    let binary_data = Bytes::from("binary data");
    assert!(metadata
        .insert_bin("binary-key", binary_data.clone())
        .is_ok());
}

#[test]
fn test_metadata_iteration() {
    let mut metadata = Metadata::new();
    metadata.insert("key1", "value1").unwrap();
    metadata.insert("key2", "value2").unwrap();

    let mut count = 0;
    for (key, value) in metadata.iter() {
        assert!(key.as_str().starts_with("key"));
        assert!(value.to_str().unwrap().starts_with("value"));
        count += 1;
    }
    assert_eq!(count, 2);
}

#[test]
fn test_metadata_case_sensitivity() {
    let mut metadata = Metadata::new();
    metadata.insert("Content-Type", "application/grpc").unwrap();
    assert!(metadata.get("content-type").is_some());
    assert!(metadata.get("CONTENT-TYPE").is_some());
}
