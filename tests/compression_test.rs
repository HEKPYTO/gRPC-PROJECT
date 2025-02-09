use grpc_project::codec::compression::{Compression, GzipCompression, NoCompression};

#[test]
fn test_gzip_compression() {
    let compression = GzipCompression;
    let data = b"test data";
    let compressed = compression.compress(data).unwrap();
    let decompressed = compression.decompress(&compressed).unwrap();
    assert_eq!(decompressed, data);
}

#[test]
fn test_no_compression() {
    let compression = NoCompression;
    let data = b"test data";
    let result = compression.compress(data).unwrap();
    assert_eq!(result, data);

    let decompressed = compression.decompress(&result).unwrap();
    assert_eq!(decompressed, data);
}

#[test]
fn test_large_data_compression() {
    let compression = GzipCompression;
    let data = vec![b'a'; 1024 * 1024];
    let compressed = compression.compress(&data).unwrap();
    let decompressed = compression.decompress(&compressed).unwrap();
    assert_eq!(decompressed, data);
}

#[test]
fn test_empty_data() {
    let compression = GzipCompression;
    let data = b"";
    let compressed = compression.compress(data).unwrap();
    let decompressed = compression.decompress(&compressed).unwrap();
    assert_eq!(decompressed, data);
}
