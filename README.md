# Huawei Cloud OBS Rust SDK

[![Crates.io](https://img.shields.io/crates/v/huaweicloud-sdk-rust-obs.svg)](https://crates.io/crates/huaweicloud-sdk-rust-obs)
[![Documentation](https://docs.rs/huaweicloud-sdk-rust-obs/badge.svg)](https://docs.rs/huaweicloud-sdk-rust-obs)
[![License](https://img.shields.io/badge/license-Apache--2.0%2FMIT-blue.svg)](LICENSE-APACHE-2.0)

A Rust SDK for Huawei Cloud Object Storage Service (OBS), following **AWS SDK style API design**.

## Features

- 🚀 **AWS SDK Style API** - Fluent builder pattern for all operations
- ⚡ **Async/Await** - Built on Tokio for high-performance async I/O
- 🔒 **Type Safe** - Strong typing for all inputs and outputs
- 📦 **Comprehensive** - Support for bucket and object operations
- 🛠️ **Easy to Use** - Builder pattern for constructing requests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
huaweicloud-sdk-rust-obs = "1.0.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Create a Client

```rust
use huaweicloud_sdk_rust_obs::{Client, Config};

let config = Config::builder()
    .access_key("your-access-key", "your-secret-key")
    .region_name("cn-north-4")
    .build()?;

let client = Client::from_config(config)?;
```

### Bucket Operations

```rust
// List all buckets
let result = client.list_buckets().send().await?;
for bucket in result.buckets() {
    println!("Bucket: {}", bucket.name());
}

// Create a bucket
client.create_bucket()
    .bucket("my-bucket")
    .location_constraint("cn-north-4")
    .send()
    .await?;

// List objects in a bucket
let result = client.list_objects()
    .bucket("my-bucket")
    .prefix("photos/")
    .max_keys(100)
    .send()
    .await?;

for object in result.contents() {
    println!("Object: {} ({} bytes)", object.key(), object.size());
}
```

### Object Operations

```rust
// Upload an object
let data = b"Hello, World!";
let result = client.put_object()
    .bucket("my-bucket")
    .key("hello.txt")
    .body(data.to_vec())
    .content_type("text/plain")
    .send()
    .await?;

println!("ETag: {:?}", result.etag());

// Download an object
let result = client.get_object()
    .bucket("my-bucket")
    .key("hello.txt")
    .send()
    .await?;

let content = result.body();
println!("Content: {:?}", content);

// Delete an object
client.delete_object()
    .bucket("my-bucket")
    .key("hello.txt")
    .send()
    .await?;
```

### Multipart Upload

For large files, use multipart upload for better reliability and performance:

```rust
// 1. Initiate multipart upload
let initiate_result = client.initiate_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .content_type("application/zip")
    .send()
    .await?;

let upload_id = initiate_result.upload_id();
println!("Upload ID: {}", upload_id);

// 2. Upload parts
let mut completed_parts = Vec::new();
for part_num in 1..=3 {
    let part_data = vec![0u8; 1024 * 1024]; // 1MB per part
    
    let part_result = client.upload_part()
        .bucket("my-bucket")
        .key("large-file.zip")
        .upload_id(upload_id)
        .part_number(part_num)
        .body(part_data)
        .send()
        .await?;
    
    completed_parts.push(CompletedPart::new(part_num, part_result.etag()));
}

// 3. Complete multipart upload
let complete_result = client.complete_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .upload_id(upload_id)
    .parts(completed_parts)
    .send()
    .await?;

println!("ETag: {}", complete_result.etag());
```

You can also abort a multipart upload:

```rust
client.abort_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .upload_id(upload_id)
    .send()
    .await?;
```

### Streaming Upload

For large files or when you want to stream data directly without loading everything into memory:

```rust
use futures::stream;
use reqwest::Body;

// Create a stream from data chunks
let data1 = bytes::Bytes::from("Hello, ");
let data2 = bytes::Bytes::from("OBS!");
let total_size = data1.len() + data2.len();

let stream = stream::iter(vec![Ok(data1), Ok(data2)]);
let body = Body::wrap_stream(stream);

// Upload using streaming
let result = client.put_object()
    .bucket("my-bucket")
    .key("streaming.txt")
    .streaming_body(body)
    .content_length(total_size as u64)  // Required for streaming
    .content_type("text/plain")
    .send()
    .await?;

println!("ETag: {:?}", result.etag());
```

## API Reference

### Client Configuration

| Method | Description |
|--------|-------------|
| `Config::builder()` | Create a new configuration builder |
| `.access_key(ak, sk)` | Set access key credentials |
| `.region_name(name)` | Set region by name |
| `.endpoint(url)` | Set custom endpoint |
| `.timeout(duration)` | Set request timeout |
| `.secure(bool)` | Enable/disable HTTPS |

### Bucket Operations

| Method | Description |
|--------|-------------|
| `client.list_buckets()` | List all buckets |
| `client.create_bucket()` | Create a new bucket |
| `client.delete_bucket()` | Delete a bucket |
| `client.get_bucket_location()` | Get bucket location |
| `client.list_objects()` | List objects (v1) |
| `client.list_objects_v2()` | List objects (v2) |

### Object Operations

| Method | Description |
|--------|-------------|
| `client.put_object()` | Upload an object |
| `client.get_object()` | Download an object |
| `client.delete_object()` | Delete an object |
| `client.delete_objects()` | Delete multiple objects |
| `client.copy_object()` | Copy an object |
| `client.head_object()` | Get object metadata |
| `client.append_object()` | Append to an object |

### Multipart Upload Operations

| Method | Description |
|--------|-------------|
| `client.initiate_multipart_upload()` | Initialize a multipart upload |
| `client.upload_part()` | Upload a part |
| `client.copy_part()` | Copy a part from an existing object |
| `client.list_parts()` | List uploaded parts |
| `client.complete_multipart_upload()` | Complete a multipart upload |
| `client.abort_multipart_upload()` | Abort a multipart upload |
| `client.list_multipart_uploads()` | List in-progress multipart uploads |

## Examples

See the [`examples/`](examples/) directory for more examples:

- [`pub_object.rs`](examples/pub_object.rs) - Upload an object
- [`get_object.rs`](examples/get_object.rs) - Download an object
- [`streaming_upload.rs`](examples/streaming_upload.rs) - Streaming upload for large files

## Error Handling

```rust
use huaweicloud_sdk_rust_obs::{Client, ObsError};

match client.get_object().bucket("bucket").key("key").send().await {
    Ok(result) => println!("Got object: {:?}", result.content_length()),
    Err(ObsError::ServiceError { status, message, .. }) => {
        eprintln!("Service error: {} - {}", status, message);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Development

### Running Tests

```bash
# Set environment variables
export OBS_ACCESS_KEY_ID=your_access_key
export OBS_SECRET_ACCESS_KEY=your_secret_key
export OBS_BUCKET=your_bucket
export OBS_ENDPOINT=obs.cn-north-4.myhuaweicloud.com

# Run tests
cargo test
```

### Building

```bash
cargo build --release
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE-2.0](LICENSE-APACHE-2.0))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Changelog

### v1.1.0

- **New**: Multipart upload support for large files
  - `initiate_multipart_upload()` - Initialize a multipart upload
  - `upload_part()` - Upload a part
  - `copy_part()` - Copy a part from an existing object
  - `list_parts()` - List uploaded parts
  - `complete_multipart_upload()` - Complete a multipart upload
  - `abort_multipart_upload()` - Abort a multipart upload
  - `list_multipart_uploads()` - List in-progress multipart uploads
- Added comprehensive tests for multipart upload operations

### v1.0.0

- **Breaking Change**: Complete API redesign following AWS SDK style
- Added fluent builder pattern for all operations
- Added comprehensive type-safe input/output types
- Improved error handling with detailed error types
- Added support for all major bucket and object operations
- **New**: Streaming upload support for `put_object` operation
- Updated documentation and examples
