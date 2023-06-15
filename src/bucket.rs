use std::collections::HashMap;

use crate::{
    client::Client,
    error::{status_to_response, ObsError},
    model::bucket::{
        copy_object::CopyObjectResult, create_bucket::CreateBucketRequest,
        list_bucket::ListAllMyBucketsResult, list_object::ListBucketResult,
        location::Location,
    },
    object::ObjectTrait,
};
use reqwest::Method;

#[async_trait::async_trait]
pub trait BucketTrait {
    /// 获取桶列表
    /// # Example
    ///
    /// ```
    /// let obs = client::Client::builder()
    ///  .endpoint("https://obs.ap-southeast-1.myhuaweicloud.com")
    ///  .security_provider(&ak, &sk)
    ///  .build()?;
    /// let _res = obs.list_buckets().await?;
    /// ```
    async fn list_buckets(&self) -> Result<ListAllMyBucketsResult, ObsError>;
    /// 创建桶
    ///
    /// - name: 桶名
    /// - location: 桶地区
    ///
    /// # Example
    ///
    /// Basic usage:
    ///
    /// ```
    /// let obs = client::Client::builder()
    ///  .endpoint("https://obs.ap-southeast-1.myhuaweicloud.com")
    ///  .security_provider(&ak, &sk)
    ///  .build()?;
    /// let _res = obs.create_bucket("bucket", "cn-north-4").await?;
    /// ```
    async fn create_bucket<S1, S2>(&self, name: S1, location: Option<S2>) -> Result<(), ObsError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send;

    /// 列举桶内对象
    ///
    /// - `name`: 桶名
    /// - `prefix`: 列举以指定的字符串prefix开头的对象。
    /// - `marker`: 列举桶内对象列表时，指定一个标识符，从该标识符以后按字典顺序返回对象列表。该字段仅用于非多版本列举。
    /// - `max-keys`: 指定返回的最大对象数，返回的对象列表将是按照字典顺序的最多前max-keys个对象，范围是[1，1000]，超出范围时，按照默认的1000进行处理。
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let obs = client::Client::builder()
    ///  .endpoint("https://obs.ap-southeast-1.myhuaweicloud.com")
    ///  .security_provider(&ak, &sk)
    ///  .build()?;
    /// let _res = obs.list_objects('bucket', None, None, None).await?;
    /// ```
    async fn list_objects<S1>(
        &self,
        name: S1,
        prefix: Option<&str>,
        marker: Option<&str>,
        max_keys: Option<usize>,
    ) -> Result<ListBucketResult, ObsError>
    where
        S1: AsRef<str> + Send;

    /// 获取桶区域位置
    async fn bucket_location<S1>(&self, name: S1) -> Result<Location, ObsError>
    where
        S1: AsRef<str> + Send;
}

pub struct Bucket<'a> {
    name: &'a str,
    client: &'a Client,
}

impl<'a> Bucket<'a> {
    pub fn new(name: &'a str, client: &'a Client) -> Self {
        Self { name, client }
    }

    pub async fn put_object<S>(&self, key: S, object: &[u8]) -> Result<(), ObsError>
    where
        S: AsRef<str> + Send,
    {
        self.client
            .put_object(self.name, key.as_ref(), object)
            .await
    }

    pub async fn copy_object<S1, S2>(&self, src: S1, dest: S2) -> Result<CopyObjectResult, ObsError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
    {
        self.client.copy_object(self.name, src, dest).await
    }

    pub async fn list_objects(
        &self,
        prefix: Option<&str>,
        marker: Option<&str>,
        max_keys: Option<usize>,
    ) -> Result<ListBucketResult, ObsError> {
        self.client
            .list_objects(self.name, prefix, marker, max_keys)
            .await
    }

    pub async fn location(&self) -> Result<Location, ObsError> {
        self.client.bucket_location(self.name).await
    }
}

#[async_trait::async_trait]
impl BucketTrait for Client {
    async fn list_buckets(&self) -> Result<ListAllMyBucketsResult, ObsError> {
        let resp = self
            .do_action_without_bucket_name(Method::GET, "", None, None, None::<String>)
            .await?;
        let status = resp.status();
        let text = resp.text().await?;
        status_to_response::<ListAllMyBucketsResult>(status, text)
    }

    async fn create_bucket<S1, S2>(&self, name: S1, location: Option<S2>) -> Result<(), ObsError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
    {
        let body = if let Some(loc) = location {
            let xml = CreateBucketRequest::new(loc.as_ref());
            serde_xml_rs::to_string(&xml)?
        } else {
            String::new()
        };

        let _res = self
            .do_action(Method::PUT, name, "", None, None, Some(body))
            .await?;

        Ok(())
    }

    async fn list_objects<S1>(
        &self,
        name: S1,
        prefix: Option<&str>,
        marker: Option<&str>,
        max_keys: Option<usize>,
    ) -> Result<ListBucketResult, ObsError>
    where
        S1: AsRef<str> + Send,
    {
        let mut params = HashMap::new();
        params.insert("delimiter".into(), "/".to_string());

        if let Some(m) = marker {
            params.insert("marker".into(), m.into());
        }

        if let Some(mk) = max_keys {
            params.insert("max-keys".into(), mk.to_string());
        }

        if let Some(p) = prefix {
            params.insert("prefix".into(), p.into());
        }

        let resp = self
            .do_action(Method::GET, name, "", None, Some(params), None::<String>)
            .await?;
        let status = resp.status();
        let text = resp.text().await?;
        // println!("{}",&text);
        status_to_response::<ListBucketResult>(status, text)
    }

    async fn bucket_location<S1>(&self, name: S1) -> Result<Location, ObsError>
    where
        S1: AsRef<str> + Send,
    {
        let mut params = HashMap::new();
        params.insert("location".to_string(), "".to_string());

        let resp = self
            .do_action(
                Method::GET,
                name,
                "",
                None,
                Some(params),
                None::<String>,
            )
            .await?;
        let status = resp.status();
        let text = resp.text().await?;

        status_to_response::<Location>(status, text)
    }
}
