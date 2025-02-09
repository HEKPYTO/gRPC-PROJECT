// tests/integration_test.rs
use bytes::Bytes;
use h2::server::SendResponse;
use http::{Request, Response};
use prost::Message;
use std::time::Duration;
use tokio::net::TcpListener;

use grpc_project::{
    client::{Client, ClientConfig},
    codec::compression::{Compression, GzipCompression},
    Error, Metadata,
};

mod common;
use common::{TestRequest, TestResponse};

async fn handle_request<B>(_request: Request<B>, mut respond: SendResponse<Bytes>) {
    let response = Response::builder()
        .status(200)
        .header("content-type", "application/grpc")
        .header("grpc-encoding", "identity")
        .header("grpc-status", "0")
        .body(())
        .unwrap();

    let mut send_stream = respond.send_response(response, false).unwrap();

    let test_response = TestResponse {
        message: "test response".to_string(),
    };

    // Encode the response message
    let mut buf = Vec::new();
    test_response.encode(&mut buf).unwrap();
    let message_len = buf.len();

    // Create the gRPC frame
    let mut framed_data = Vec::with_capacity(message_len + 5);
    framed_data.push(0); // compression flag
    framed_data.extend_from_slice(&(message_len as u32).to_be_bytes());
    framed_data.extend_from_slice(&buf);

    send_stream
        .send_data(Bytes::from(framed_data), true)
        .unwrap();
}

async fn setup_test_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = format!("http://{}", listener.local_addr().unwrap());

    tokio::spawn(async move {
        let mut builder = h2::server::Builder::new();
        builder.max_concurrent_streams(100);

        while let Ok((socket, _)) = listener.accept().await {
            let mut connection = builder.handshake(socket).await.unwrap();

            while let Some(request) = connection.accept().await {
                if let Ok((request, respond)) = request {
                    tokio::spawn(handle_request(request, respond));
                }
            }
        }
    });

    // Small delay to ensure server is ready
    tokio::time::sleep(Duration::from_millis(100)).await;
    addr
}

#[tokio::test]
async fn test_client_connection() {
    let addr = setup_test_server().await;

    let config = ClientConfig::default();
    let mut client = Client::connect(&addr, Some(config)).await.unwrap();

    let request = TestRequest {
        message: "test request".to_string(),
    };

    let response = client
        .unary::<TestRequest, TestResponse>("/test.service/TestMethod", request, None)
        .await;
    assert!(
        response.is_ok(),
        "Expected successful response, got {:?}",
        response
    );

    if let Ok(response) = response {
        let body = response.into_body();
        assert_eq!(body.message, "test response");
    }
}

#[tokio::test]
async fn test_compression() {
    let compression = GzipCompression;
    let data = b"test compression data";

    let compressed = compression.compress(data).unwrap();
    let decompressed = compression.decompress(&compressed).unwrap();

    assert_eq!(data.to_vec(), decompressed);
}

#[tokio::test]
async fn test_client_error_handling() {
    // Test invalid URI format
    let result = Client::connect("invalid-uri", None).await;
    assert!(
        matches!(result, Err(Error::Protocol(_))),
        "Expected Protocol error for invalid URI, got {:?}",
        result
    );

    // Test connection timeout
    let config = ClientConfig {
        connect_timeout: Duration::from_millis(1),
        ..Default::default()
    };

    let result = Client::connect("http://127.0.0.1:65535", Some(config)).await;
    assert!(
        result.is_err(),
        "Expected connection timeout error, got {:?}",
        result
    );
}

#[tokio::test]
async fn test_metadata_handling() {
    let addr = setup_test_server().await;
    let mut client = Client::connect(&addr, None).await.unwrap();

    let request = TestRequest {
        message: "test".to_string(),
    };

    let mut metadata = Metadata::new();
    metadata.insert("custom-header", "test-value").unwrap();

    let response = client
        .unary::<TestRequest, TestResponse>("/test.service/TestMethod", request, Some(metadata))
        .await;
    assert!(
        response.is_ok(),
        "Expected successful response, got {:?}",
        response
    );
}
