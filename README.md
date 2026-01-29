# Huaweicloud OBS SDK (unofficial)

[![Crates.io](https://img.shields.io/crates/l/huaweicloud-sdk-rust-obs/0.1.5?style=flat-square)](https://github.com/kingzcheung/huaweicloud-sdk-rust-obs) [![Crates.io](https://img.shields.io/crates/v/huaweicloud-sdk-rust-obs?style=flat-square)](https://crates.io/crates/huaweicloud-sdk-rust-obs) [![docs.rs](https://img.shields.io/docsrs/huaweicloud-sdk-rust-obs?style=flat-square)](https://docs.rs/huaweicloud-sdk-rust-obs/latest)

> 计划只支持基本的 obs 操作。如果你觉得功能有缺失，欢迎提交 issue 或 pull request。

## 基本使用

1. 添加 sdk 到项目中:

```
cargo add huaweicloud-sdk-rust-obs
```

2. 初始化客户端

```rust
use huaweicloud_sdk_rust_obs::{client, error::ObsError, bucket::BucketTrait, object::ObjectTrait};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // 从环境变量获取配置
    dotenvy::dotenv().ok(); // 可选：从 .env 文件加载
    
    let endpoint = std::env::var("ENDPOINT").unwrap_or_else(|_| "https://obs.ap-southeast-1.myhuaweicloud.com".to_string());
    let ak = std::env::var("OBS_AK").expect("OBS_AK must be set");
    let sk = std::env::var("OBS_SK").expect("OBS_SK must be set");
    let bucket = std::env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");

    let obs = client::Client::builder()
        .endpoint(&endpoint)
        .security_provider(&ak, &sk)
        .build()?;
    
    // 现在可以使用 obs 客户端执行各种操作
    // 示例见下面的不同功能部分
    Ok(())
}
```

### 列出所有存储桶

```rust
use huaweicloud_sdk_rust_obs::{client, error::ObsError, bucket::BucketTrait};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // ... 初始化客户端 (见上面) ...
    
    let buckets = obs.list_buckets().await?;
    for bucket in buckets.buckets.bucket {
        println!("Bucket: {} - Created: {}", bucket.name, bucket.creation_date);
    }
    
    Ok(())
}
```

### 创建存储桶

```rust
use huaweicloud_sdk_rust_obs::{client, error::ObsError, bucket::BucketTrait};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // ... 初始化客户端 (见上面) ...
    
    // 创建一个新的存储桶，指定区域
    obs.create_bucket("my-new-bucket", Some("cn-south-1")).await?;
    println!("Bucket created successfully!");
    
    Ok(())
}
```

### 上传对象

```rust
use huaweicloud_sdk_rust_obs::{client, error::ObsError, object::ObjectTrait};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // ... 初始化客户端 (见上面) ...
    
    // 上传文本数据
    let text_data = b"Hello, OBS!";
    obs.put_object(&bucket, "hello.txt", text_data).await?;
    
    // 上传二进制数据（如图片）
    let image_data = include_bytes!("path/to/image.jpg");
    obs.put_object(&bucket, "image.jpg", image_data).await?;
    
    println!("Objects uploaded successfully!");
    
    Ok(())
}
```

### 下载对象

```rust
use huaweicloud_sdk_rust_obs::{client, error::ObsError, object::ObjectTrait};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // ... 初始化客户端 (见上面) ...
    
    // 下载对象
    let data = obs.get_object(&bucket, "hello.txt").await?;
    let content = String::from_utf8_lossy(&data);
    println!("Downloaded content: {}", content);
    
    Ok(())
}
```

### 列出存储桶中的对象

```rust
use huaweicloud_sdk_rust_obs::{client, error::ObsError, bucket::BucketTrait};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // ... 初始化客户端 (见上面) ...
    
    // 列出存储桶中的所有对象
    let result = obs.list_objects(&bucket, None, None, Some(100)).await?;
    
    for object in result.contents.unwrap_or_default() {
        println!("Object: {} - Size: {} bytes - Modified: {}", 
                 object.key, object.size, object.last_modified);
    }
    
    Ok(())
}
```

### 获取存储桶位置

```rust
use huaweicloud_sdk_rust_obs::{client, error::ObsError, bucket::BucketTrait};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // ... 初始化客户端 (见上面) ...
    
    // 获取存储桶的位置信息
    let location = obs.bucket_location(&bucket).await?;
    println!("Bucket location: {}", location.location);
    
    Ok(())
}
```

### 复制对象

```rust
use huaweicloud_sdk_rust_obs::{client, error::ObsError, object::ObjectTrait};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // ... 初始化客户端 (见上面) ...
    
    // 复制同一个存储桶中的对象
    let result = obs.copy_object(&bucket, "source-object.txt", "destination-object.txt").await?;
    println!("Object copied. ETag: {}", result.etag);
    
    Ok(())
}
```
## 测试

1. 在项目根目录添加 `.env`文件，内容格式如下：

```
OBS_AK=XXXXXXXXXXXXXXX
OBS_SK=XXXXXXXXXXXXXXXXXXXX
OBS_BUCKET=your-bucket-name
ENDPOINT=obs.your-region.myhuaweicloud.com
```

2. 运行 `cargo test`
