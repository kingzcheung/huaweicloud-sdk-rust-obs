use crate::{
    client::Client,
    error::{status_to_response, ObsError},
    model::bucket::{
        copy_object::CopyObjectResult, create_bucket::CreateBucketRequest,
        list_bucket::ListAllMyBuckets, list_object::ListBucketResult,
    },
    object::ObjectTrait,
};
use reqwest::Method;

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
    ) -> Result<ListBucketResult, ObsError>;
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

    pub async fn copy_object(&self, src: &str, dest: &str) -> Result<CopyObjectResult, ObsError> {
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
    ) -> Result<ListBucketResult, ObsError> {
        let mut uri = String::from("?");
        if let Some(p) = prefix {
            uri.push_str("prefix=");
            uri.push_str(p);
        }
        if let Some(m) = marker {
            if !uri.ends_with('?') {
                uri.push('&');
            }
            uri.push_str("marker=");
            uri.push_str(m);
        }

        if let Some(mk) = max_keys {
            if !uri.ends_with('?') {
                uri.push('&');
            }
            uri.push_str("key-marker=");
            uri.push_str(mk.to_string().as_str());
        }

        if uri.eq("?") {
            uri = String::new()
        }

        let resp = self
            .do_action(Method::GET, name, &uri, None, None::<String>)
            .await?;
        let status = resp.status();
        let text = resp.text().await?;

        status_to_response(status, text)
    }
}
