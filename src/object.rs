use std::collections::HashMap;

use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use urlencoding::encode;

use crate::{
    client::Client,
    error::{status_to_response, ObsError},
    model::{bucket::copy_object::CopyObjectResult, object::ObjectMeta},
};

#[async_trait]
pub trait ObjectTrait {
    /// PUT上传
    async fn put_object<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
        object: &[u8],
    ) -> Result<(), ObsError>;

    /// 复制对象
    async fn copy_object<S1, S2, S3>(
        &self,
        bucket: S1,
        src: S2,
        dest: S3,
    ) -> Result<CopyObjectResult, ObsError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        S3: AsRef<str> + Send;

    /// 删除对象
    async fn delete_object<S: AsRef<str> + Send>(&self, bucket: S, key: S) -> Result<(), ObsError>;

    /// 获取对象内容
    async fn get_object<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
    ) -> Result<bytes::Bytes, ObsError>;

    /// 获取对象元数据
    async fn get_object_metadata<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
    ) -> Result<ObjectMeta, ObsError>;
}

#[async_trait]
impl ObjectTrait for Client {
    /// PUT上传
    async fn put_object<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
        object: &[u8],
    ) -> Result<(), ObsError> {
        let mut with_headers = HeaderMap::new();
        with_headers.insert(
            "Content-Length",
            HeaderValue::from_str(format!("{}", object.len()).as_str()).unwrap(),
        );
        let resp = self
            .do_action(
                Method::PUT,
                bucket,
                key,
                Some(with_headers),
                Some(object.to_owned()),
            )
            .await?;
        let _ = resp.text().await?;

        Ok(())
    }

    /// 复制对象
    async fn copy_object<S1, S2, S3>(
        &self,
        bucket: S1,
        src: S2,
        dest: S3,
    ) -> Result<CopyObjectResult, ObsError>
    where
        S1: AsRef<str> + Send,
        S2: AsRef<str> + Send,
        S3: AsRef<str> + Send,
    {
        let mut with_headers = HeaderMap::new();
        let dest = dest.as_ref().trim_start_matches('/');
        let src = src.as_ref().trim_start_matches('/');
        let src = encode(src);
        let copy_source = format!("/{}/{}", bucket.as_ref(), src);
        with_headers.insert(
            "x-obs-copy-source",
            HeaderValue::from_str(&copy_source).unwrap(),
        );

        let resp = self
            .do_action(
                Method::PUT,
                bucket,
                dest,
                Some(with_headers),
                None::<String>,
            )
            .await?;
        let status = resp.status();
        let text = resp.text().await?;
        status_to_response::<CopyObjectResult>(status, text)
        // match status {
        //     StatusCode::OK => {
        //         let r: CopyObjectResponse = serde_xml_rs::from_str(&text)?;
        //         Ok(r)
        //     }
        //     StatusCode::FORBIDDEN => {
        //         let er: ErrorResponse = serde_xml_rs::from_str(&text)?;
        //         Err(ObsError::Response {
        //             status: StatusCode::FORBIDDEN,
        //             message: er.message,
        //         })
        //     }
        //     _ => Err(ObsError::Unknown),
        // }
    }

    /// 删除对象
    async fn delete_object<S: AsRef<str> + Send>(&self, bucket: S, key: S) -> Result<(), ObsError> {
        let _resp = self
            .do_action(Method::DELETE, bucket, key, None, None::<String>)
            .await?;
        Ok(())
    }

    ///获取对象内容
    async fn get_object<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
    ) -> Result<bytes::Bytes, ObsError> {
        let resp = self
            .do_action(Method::GET, bucket, key, None, None::<String>)
            .await?
            .bytes()
            .await?;

        Ok(resp)
    }

    /// 获取对象元数据
    async fn get_object_metadata<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
    ) -> Result<ObjectMeta, ObsError> {
        let resp = self
            .do_action(Method::HEAD, bucket, key, None, None::<String>)
            .await?;
        let headers = resp.headers();
        let mut data = HashMap::with_capacity(headers.len());
        for (key, val) in headers {
            data.insert(key.as_str(), val.to_str().unwrap());
        }

        let header_str = serde_json::to_string(&data).map_err(|_e| ObsError::ParseOrConvert)?;

        let data: ObjectMeta =
            serde_json::from_str(&header_str).map_err(|_e| ObsError::ParseOrConvert)?;

        Ok(data)
    }
}
