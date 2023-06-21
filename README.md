# Huaweicloud OBS SDK (unofficial)

[![Crates.io](https://img.shields.io/crates/l/huaweicloud-sdk-rust-obs/0.1.5?style=flat-square)](https://github.com/kingzcheung/huaweicloud-sdk-rust-obs) [![Crates.io](https://img.shields.io/crates/v/huaweicloud-sdk-rust-obs?style=flat-square)](https://crates.io/crates/huaweicloud-sdk-rust-obs) [![docs.rs](https://img.shields.io/docsrs/huaweicloud-sdk-rust-obs?style=flat-square)](https://docs.rs/huaweicloud-sdk-rust-obs/latest)

> WIP. 计划只支持基本的 obs 操作。

## 基本使用

1. 添加 sdk 到项目中:

```
cargo add huaweicloud-sdk-rust-obs
```

2.示例

```rust
#[tokio::main]
async fn main() -> Result<(), ObsError> {
    const DEFAULT_BUCKET_NAME:&str = "test_bucket";
    let endpoint = "https://obs.ap-southeast-1.myhuaweicloud.com";
    // see: https://support.huaweicloud.com/api-obs/obs_04_0116.html
    let ak = "xxx"; 
    let sk = "xxxxxx";
    let obs = client::Client::builder()
        .endpoint(endpoint)
        .security_provider(ak, sk)
        .build()?;

    // put object
    let object = include_bytes!("testdata/test.jpeg");
    obs.put_object(DEFAULT_BUCKET_NAME, "obs-client-key.jpeg", object)
        .await?;

    Ok(())

}
```
## 测试

1. 在项目根目录添加 `.env`文件，内容格式如下：

```
OBS_AK=XXXXXXXXXXXXXXX
OBS_SK=XXXXXXXXXXXXXXXXXXXX
```

2. `tests/common/mod.rs` 下的 `DEFAULT_BUCKET_NAME` 替换为自己的 bucket 名称
3. 运行 `cargo test`