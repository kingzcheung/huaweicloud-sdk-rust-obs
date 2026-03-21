# 华为云 OBS Rust SDK

[![Crates.io](https://img.shields.io/crates/v/huaweicloud-sdk-rust-obs.svg)](https://crates.io/crates/huaweicloud-sdk-rust-obs)
[![Documentation](https://docs.rs/huaweicloud-sdk-rust-obs/badge.svg)](https://docs.rs/huaweicloud-sdk-rust-obs)
[![License](https://img.shields.io/badge/license-Apache--2.0%2FMIT-blue.svg)](LICENSE-APACHE-2.0)

华为云对象存储服务（OBS）的**非官方** Rust SDK。

[English](README.md) | 简体中文

## 特性

- 🚀 **流式构建器 API** - 所有操作采用流式构建器模式
- ⚡ **Async/Await** - 基于 Tokio 的高性能异步 I/O
- 🔒 **类型安全** - 所有输入输出均为强类型
- 📦 **功能完善** - 支持桶操作和对象操作
- 🛠️ **易于使用** - 构建器模式构造请求

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
huaweicloud-sdk-rust-obs = "1.0.0"
tokio = { version = "1", features = ["full"] }
```

## 快速开始

### 创建客户端

```rust
use huaweicloud_sdk_rust_obs::{Client, Config};

let config = Config::builder()
    .access_key("your-access-key", "your-secret-key")
    .region_name("cn-north-4")
    .build()?;

let client = Client::from_config(config)?;
```

### 桶操作

```rust
// 列出所有桶
let result = client.list_buckets().send().await?;
for bucket in result.buckets() {
    println!("桶: {}", bucket.name());
}

// 创建桶
client.create_bucket()
    .bucket("my-bucket")
    .location_constraint("cn-north-4")
    .send()
    .await?;

// 列出桶内对象
let result = client.list_objects()
    .bucket("my-bucket")
    .prefix("photos/")
    .max_keys(100)
    .send()
    .await?;

for object in result.contents() {
    println!("对象: {} ({} 字节)", object.key(), object.size());
}
```

### 对象操作

```rust
// 上传对象
let data = b"Hello, World!";
let result = client.put_object()
    .bucket("my-bucket")
    .key("hello.txt")
    .body(data.to_vec())
    .content_type("text/plain")
    .send()
    .await?;

println!("ETag: {:?}", result.etag());

// 下载对象
let result = client.get_object()
    .bucket("my-bucket")
    .key("hello.txt")
    .send()
    .await?;

let content = result.body();
println!("内容: {:?}", content);

// 删除对象
client.delete_object()
    .bucket("my-bucket")
    .key("hello.txt")
    .send()
    .await?;
```

### 分段上传

对于大文件，使用分段上传可以获得更好的可靠性和性能：

```rust
// 1. 初始化分段上传
let initiate_result = client.initiate_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .content_type("application/zip")
    .send()
    .await?;

let upload_id = initiate_result.upload_id();
println!("Upload ID: {}", upload_id);

// 2. 上传段
let mut completed_parts = Vec::new();
for part_num in 1..=3 {
    let part_data = vec![0u8; 1024 * 1024]; // 每段 1MB
    
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

// 3. 合并段
let complete_result = client.complete_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .upload_id(upload_id)
    .parts(completed_parts)
    .send()
    .await?;

println!("ETag: {}", complete_result.etag());
```

也可以取消分段上传：

```rust
client.abort_multipart_upload()
    .bucket("my-bucket")
    .key("large-file.zip")
    .upload_id(upload_id)
    .send()
    .await?;
```

### 流式上传

对于大文件或需要直接流式传输数据而不加载到内存的场景：

```rust
use futures::stream;
use reqwest::Body;

// 从数据块创建流
let data1 = bytes::Bytes::from("Hello, ");
let data2 = bytes::Bytes::from("OBS!");
let total_size = data1.len() + data2.len();

let stream = stream::iter(vec![Ok(data1), Ok(data2)]);
let body = Body::wrap_stream(stream);

// 使用流式上传
let result = client.put_object()
    .bucket("my-bucket")
    .key("streaming.txt")
    .streaming_body(body)
    .content_length(total_size as u64)  // 流式上传必须设置
    .content_type("text/plain")
    .send()
    .await?;

println!("ETag: {:?}", result.etag());
```

## API 参考

### 客户端配置

| 方法 | 描述 |
|------|------|
| `Config::builder()` | 创建配置构建器 |
| `.access_key(ak, sk)` | 设置访问密钥 |
| `.region_name(name)` | 按名称设置区域 |
| `.endpoint(url)` | 设置自定义终端节点 |
| `.timeout(duration)` | 设置请求超时 |
| `.secure(bool)` | 启用/禁用 HTTPS |

### 桶操作

| 方法 | 描述 |
|------|------|
| `client.list_buckets()` | 列出所有桶 |
| `client.create_bucket()` | 创建桶 |
| `client.delete_bucket()` | 删除桶 |
| `client.get_bucket_location()` | 获取桶位置 |
| `client.list_objects()` | 列出对象 (v1) |
| `client.list_objects_v2()` | 列出对象 (v2) |

### 对象操作

| 方法 | 描述 |
|------|------|
| `client.put_object()` | 上传对象 |
| `client.get_object()` | 下载对象 |
| `client.delete_object()` | 删除对象 |
| `client.delete_objects()` | 批量删除对象 |
| `client.copy_object()` | 复制对象 |
| `client.head_object()` | 获取对象元数据 |
| `client.append_object()` | 追加上传 |

### 分段上传操作

| 方法 | 描述 |
|------|------|
| `client.initiate_multipart_upload()` | 初始化分段上传 |
| `client.upload_part()` | 上传段 |
| `client.copy_part()` | 从现有对象复制段 |
| `client.list_parts()` | 列出已上传的段 |
| `client.complete_multipart_upload()` | 合并段 |
| `client.abort_multipart_upload()` | 取消分段上传 |
| `client.list_multipart_uploads()` | 列出进行中的分段上传 |

## 示例

查看 [`examples/`](examples/) 目录获取更多示例：

- [`pub_object.rs`](examples/pub_object.rs) - 上传对象
- [`get_object.rs`](examples/get_object.rs) - 下载对象
- [`streaming_upload.rs`](examples/streaming_upload.rs) - 流式上传大文件
- [`multipart_upload.rs`](examples/multipart_upload.rs) - 分段上传大文件

## 错误处理

```rust
use huaweicloud_sdk_rust_obs::{Client, ObsError};

match client.get_object().bucket("bucket").key("key").send().await {
    Ok(result) => println!("获取对象: {:?}", result.content_length()),
    Err(ObsError::ServiceError { status, message, .. }) => {
        eprintln!("服务错误: {} - {}", status, message);
    }
    Err(e) => eprintln!("错误: {}", e),
}
```

## 开发

### 运行测试

```bash
# 设置环境变量
export OBS_ACCESS_KEY_ID=your_access_key
export OBS_SECRET_ACCESS_KEY=your_secret_key
export OBS_BUCKET=your_bucket
export OBS_ENDPOINT=obs.cn-north-4.myhuaweicloud.com

# 运行测试
cargo test
```

### 构建

```bash
cargo build --release
```

## 许可证

根据以下任一许可证授权：

- Apache License, Version 2.0 ([LICENSE-APACHE-2.0](LICENSE-APACHE-2.0))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

任您选择。

## 贡献

欢迎贡献！请随时提交 Pull Request。