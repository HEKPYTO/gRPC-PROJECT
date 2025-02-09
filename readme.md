# grpc

A modular gRPC implementation in Rust with support for HTTP/2, compression, and streaming.

## Features

- **Full gRPC Support**: Implements the gRPC protocol specification
- **HTTP/2 Transport**: Built on top of the `h2` crate for HTTP/2 support
- **Compression**: Supports gzip compression with extensible compression framework
- **Streaming**: Supports unary, client streaming, and server streaming calls
- **Async/Await**: Built with modern Rust async/await syntax
- **Modular Design**: Clean separation of concerns with modular architecture
- **Error Handling**: Comprehensive error types and handling
- **Metadata Support**: Full support for gRPC metadata and headers

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
grpc_project = "0.1.0"
```

## Quick Start

Here's a simple example of using the client:

```rust
use grpc_project::{Client, ClientConfig, Metadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let mut client = Client::connect("http://localhost:50051", None).await?;
    
    // Create a request
    let request = MyRequest {
        message: "Hello".to_string(),
    };
    
    // Add optional metadata
    let mut metadata = Metadata::new();
    metadata.insert("custom-header", "value")?;
    
    // Make a unary call
    let response = client
        .unary::<MyRequest, MyResponse>("/service/method", request, Some(metadata))
        .await?;
        
    println!("Response: {:?}", response);
    Ok(())
}
```

## Architecture

The project is organized into several modules:

- `client`: gRPC client implementation
- `codec`: Message encoding/decoding and compression
- `transport`: HTTP/2 transport layer
- `metadata`: Metadata handling
- `error`: Error types and handling

## Configuration

The client can be configured with `ClientConfig`:

```rust
let config = ClientConfig {
    max_message_size: 4 * 1024 * 1024, // 4MB
    max_concurrent_streams: 100,
    enable_http2_keepalive: true,
    http2_keepalive_interval: Duration::from_secs(300),
    connect_timeout: Duration::from_secs(5),
};

let client = Client::connect("http://localhost:50051", Some(config)).await?;
```

## Examples

### Unary Call

```rust
let response = client
    .unary::<RequestType, ResponseType>("/service/method", request, None)
    .await?;
```

### With Metadata

```rust
let mut metadata = Metadata::new();
metadata.insert("authorization", "Bearer token")?;

let response = client
    .unary::<RequestType, ResponseType>("/service/method", request, Some(metadata))
    .await?;
```

## Error Handling

The library provides a comprehensive error type system:

```rust
#[derive(Error, Debug)]
pub enum Error {
    #[error("Transport error: {0}")]
    Transport(#[from] h2::Error),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Message too large: {0} bytes")]
    MessageTooLarge(usize),
    
    // ... other error variants
}
```

## Development

To run tests:

```bash
cargo test
```

To run specific test categories:

```bash
cargo test --test integration_test
cargo test --test compression_test
cargo test --test frame_test
```

## Requirements

- Rust 1.56 or higher (for async/await support)
- Dependencies:
  - h2: 0.3
  - tokio: 1.0
  - bytes: 1.0
  - prost: 0.12
  - And others as specified in Cargo.toml

## Acknowledgments

- [h2](https://github.com/hyperium/h2) - HTTP/2 implementation
- [tokio](https://tokio.rs/) - Async runtime
- [prost](https://github.com/tokio-rs/prost) - Protocol Buffers implementation