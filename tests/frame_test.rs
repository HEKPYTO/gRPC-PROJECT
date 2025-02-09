// tests/frame_test.rs
use bytes::{Bytes, BytesMut};
use grpc_project::codec::frame::Frame;

#[test]
fn test_frame_creation() {
    let data = Bytes::from("test");
    let frame = Frame::new(data.clone(), false);
    assert_eq!(frame.data, data);
    assert_eq!(frame.header.compressed, false);
    assert_eq!(frame.len(), 4);
}

#[test]
fn test_frame_encode_decode() {
    let data = Bytes::from("test");
    let frame = Frame::new(data.clone(), false);
    let mut buf = BytesMut::new();

    frame.encode(&mut buf);
    let decoded = Frame::decode(&mut buf).unwrap();

    assert_eq!(decoded.data, data);
    assert_eq!(decoded.header.compressed, false);
    assert_eq!(decoded.len(), frame.len());
}

#[test]
fn test_empty_frame() {
    let data = Bytes::from("");
    let frame = Frame::new(data, false);
    assert!(frame.is_empty());
}

#[test]
fn test_partial_decode() {
    let mut buf = BytesMut::new();
    assert!(Frame::decode(&mut buf).is_none());

    buf.extend_from_slice(&[0]);
    assert!(Frame::decode(&mut buf).is_none());

    buf.extend_from_slice(&[0, 0, 0, 1]);
    assert!(Frame::decode(&mut buf).is_none());
}

#[test]
fn test_compressed_frame() {
    let data = Bytes::from("test");
    let frame = Frame::new(data.clone(), true);
    let mut buf = BytesMut::new();

    frame.encode(&mut buf);
    let decoded = Frame::decode(&mut buf).unwrap();

    assert_eq!(decoded.data, data);
    assert_eq!(decoded.header.compressed, true);
}
