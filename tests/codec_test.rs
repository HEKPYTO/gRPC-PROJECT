// tests/codec_test.rs
use bytes::{Bytes, BytesMut};
use grpc_project::{
    codec::{GrpcCodec, Frame},
    Error,
};

const TEST_MESSAGE_SIZE: usize = 1024; // 1KB for testing

#[test]
fn test_codec_encode_decode() {
    let mut codec = GrpcCodec::new()
        .with_max_message_size(TEST_MESSAGE_SIZE);
    let mut buf = BytesMut::new();
    
    let data = Bytes::from("test");
    let frame = Frame::new(data.clone(), false);
    
    assert!(codec.encode(frame, &mut buf).is_ok());
    let decoded = codec.decode(&mut buf).unwrap().unwrap();
    
    assert_eq!(decoded.data, data);
    assert!(!decoded.header.compressed);
}

#[test]
fn test_message_size_limit() {
    let mut codec = GrpcCodec::new()
        .with_max_message_size(10);
    let mut buf = BytesMut::new();
    
    let data = Bytes::from(vec![0; 20]);
    let frame = Frame::new(data, false);
    
    assert!(matches!(
        codec.encode(frame, &mut buf),
        Err(Error::MessageTooLarge(_))
    ));
}

#[test]
fn test_partial_decode() {
    let mut codec = GrpcCodec::new()
        .with_max_message_size(TEST_MESSAGE_SIZE);
    let mut buf = BytesMut::new();
    
    assert!(codec.decode(&mut buf).unwrap().is_none());
    
    buf.extend_from_slice(&[0]); // Just compression flag
    assert!(codec.decode(&mut buf).unwrap().is_none());
}

#[test]
fn test_multiple_frames() {
    let mut codec = GrpcCodec::new()
        .with_max_message_size(TEST_MESSAGE_SIZE);
    let mut buf = BytesMut::new();
    
    let frame1 = Frame::new(Bytes::from("frame1"), false);
    let frame2 = Frame::new(Bytes::from("frame2"), false);
    
    codec.encode(frame1, &mut buf).unwrap();
    codec.encode(frame2, &mut buf).unwrap();
    
    let decoded1 = codec.decode(&mut buf).unwrap().unwrap();
    let decoded2 = codec.decode(&mut buf).unwrap().unwrap();
    
    assert_eq!(decoded1.data, Bytes::from("frame1"));
    assert_eq!(decoded2.data, Bytes::from("frame2"));
}