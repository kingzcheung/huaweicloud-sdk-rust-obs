use async_trait::async_trait;
use base64::{engine::general_purpose, Engine};
use md5::{Digest, Md5};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use std::collections::HashMap;
use urlencoding::encode;

use crate::{
    client::Client,
    error::{status_to_response, ObsError},
    model::{
        bucket::copy_object::CopyObjectResult,
        delete_object::{Boolean, Delete, Item, Object, ResponseMode},
        object::{NextPosition, ObjectMeta},
    },
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

    /// 批量删除对象
    async fn delete_objects<S: AsRef<str> + Send, K: IntoIterator<Item = S> + Send>(
        &self,
        bucket: S,
        keys: K,
        response_mode: ResponseMode,
    ) -> Result<(), ObsError>;

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

    /// 追加写对象
    async fn append_object<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
        appended: &[u8],
        position: u64,
    ) -> Result<NextPosition, ObsError>;
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
                None,
                Some(object.to_owned()),
            )
            .await?;
        let status = resp.status();
        let text = resp.text().await?;

        if status.is_success() {
            Ok(())
        } else {
            // 尝试解析错误响应
            match serde_xml_rs::from_str::<crate::model::ErrorResponse>(&text) {
                Ok(error_response) => Err(crate::error::ObsError::Response {
                    status,
                    message: error_response.message,
                }),
                Err(_) => {
                    // 如果无法解析错误响应，则返回原始文本
                    Err(crate::error::ObsError::Response {
                        status,
                        message: text,
                    })
                }
            }
        }
    }

    async fn append_object<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
        appended: &[u8],
        position: u64,
    ) -> Result<NextPosition, ObsError> {
        let mut params = HashMap::new();
        params.insert("append".to_string(), "".into());
        params.insert("position".into(), position.to_string());
        let mut with_headers = HeaderMap::new();
        with_headers.insert(
            "Content-Length",
            HeaderValue::from_str(format!("{}", appended.len()).as_str()).unwrap(),
        );
        let resp = self
            .do_action(
                Method::POST,
                bucket,
                key,
                Some(with_headers),
                Some(params),
                Some(appended.to_owned()),
            )
            .await?;
        let status = resp.status();
        let headers = resp.headers().clone();
        if status.is_success() {
            let next_position = if let Some(next) = headers.get("x-obs-next-append-position") {
                let next = String::from_utf8_lossy(next.as_bytes()).to_string();
                next.parse::<u64>().ok()
            } else {
                None
            };
            Ok(next_position as NextPosition)
        } else {
            Err(ObsError::Response {
                status,
                message: "response error".into(),
            })
        }
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
                None,
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
            .do_action(Method::DELETE, bucket, key, None, None, None::<String>)
            .await?;
        Ok(())
    }
    /// 批量删除对象
    async fn delete_objects<S: AsRef<str> + Send, K: IntoIterator<Item = S> + Send>(
        &self,
        bucket: S,
        keys: K,
        response_mode: ResponseMode,
    ) -> Result<(), ObsError> {
        let mut with_headers = HeaderMap::new();
        let mut params = HashMap::new();
        params.insert("delete".to_string(), "".to_string());

        let body = Delete {
            quiet: response_mode.to_bool(),
            item: keys
                .into_iter()
                .map(|key| {
                    Item::Object(Object {
                        key_name: key.as_ref().to_owned(),
                        version_id: None,
                    })
                })
                .collect::<Vec<Item>>(),
        };
        let body = serde_xml_rs::to_string(&body)?;
        let mut hasher = Md5::new();
        hasher.update(body.as_bytes());
        let result = hasher.finalize();

        let val = general_purpose::STANDARD.encode(result);

        with_headers.insert("Content-MD5", HeaderValue::from_str(val.as_str()).unwrap());

        with_headers.insert(
            "Content-Length",
            HeaderValue::from_str(format!("{}", body.len()).as_str()).unwrap(),
        );
        let _resp = self
            .do_action(
                Method::POST,
                bucket,
                "",
                Some(with_headers),
                Some(params),
                Some(body),
            )
            .await?;
        // dbg!(_resp.text().await?);
        Ok(())
    }

    ///获取对象内容
    async fn get_object<S: AsRef<str> + Send>(
        &self,
        bucket: S,
        key: S,
    ) -> Result<bytes::Bytes, ObsError> {
        let resp = self
            .do_action(Method::GET, bucket, key, None, None, None::<String>)
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
            .do_action(Method::HEAD, bucket, key, None, None, None::<String>)
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
