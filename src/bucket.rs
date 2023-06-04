use reqwest::Method;

use crate::{
    client::Client,
    error::ObsError,
    model::bucket::{
        create_bucket::CreateBucketRequest, list_bucket::ListAllMyBuckets,
        list_object::ListObjectsResponse,
    },
    object::ObjectTrait,
};

#[async_trait::async_trait]
pub trait BucketTrait {
    async fn list_buckets(&self) -> Result<ListAllMyBuckets, ObsError>;
    async fn create_bucket(&self, name: &str, location: Option<&str>) -> Result<(), ObsError>;
    async fn list_objects(
        &self,
        name: &str,
        prefix: Option<&str>,
        marker: Option<&str>,
        max_keys: Option<usize>,
    ) -> Result<ListObjectsResponse, ObsError>;
}

pub struct Bucket<'a> {
    name: &'a str,
    client: &'a Client,
}

impl<'a> Bucket<'a> {
    pub fn new(name: &'a str, client: &'a Client) -> Self {
        Self { name, client }
    }

    pub async fn put_object(&self, key: &str, object: &'static [u8]) -> Result<(), ObsError> {
        self.client.put_object(self.name, key, object).await
    }

    pub async fn copy_object(&self, src: &str, dest: &str) -> Result<(), ObsError> {
        self.client.copy_object(self.name, src, dest).await
    }
}

#[async_trait::async_trait]
impl BucketTrait for Client {
    async fn list_buckets(&self) -> Result<ListAllMyBuckets, ObsError> {
        let resp: ListAllMyBuckets = self
            .do_action_without_bucket_name(Method::GET, "", None, None::<String>)
            .await?
            .json()
            .await?;

        Ok(resp)
    }
    async fn create_bucket(&self, name: &str, location: Option<&str>) -> Result<(), ObsError> {
        let body = if let Some(loc) = location {
            let xml = CreateBucketRequest::new(loc);
            serde_xml_rs::to_string(&xml)?
        } else {
            String::new()
        };

        let _res = self
            .do_action(Method::PUT, name, "", None, Some(body))
            .await?;

        Ok(())
    }

    async fn list_objects(
        &self,
        name: &str,
        prefix: Option<&str>,
        marker: Option<&str>,
        max_keys: Option<usize>,
    ) -> Result<ListObjectsResponse, ObsError> {
        unimplemented!()
    }
}
