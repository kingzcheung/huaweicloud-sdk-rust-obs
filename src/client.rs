use std::{collections::HashMap, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Body, Method, Response,
};

use crate::{
    auth::Authorization,
    bucket::Bucket,
    config::{Config, SecurityHolder, SignatureType},
    error::ObsError,
    model::object::ObjectMeta,
};

#[derive(Debug)]
pub struct Client {
    config: Config,
    http_client: reqwest::Client,
}

impl Client {
    /// endpoint 格式: https[http]://obs.cn-north-4.myhuaweicloud.com
    pub fn new<S: ToString>(
        access_key_id: S,
        secret_access_key: S,
        endpoint: &str,
    ) -> Result<Client, ObsError> {
        ClientBuilder::new()
            .security_provider(access_key_id, secret_access_key)
            .endpoint(endpoint)
            .build()
    }

    pub fn security(&self) -> Option<SecurityHolder> {
        if !self.config().security_providers().is_empty() {
            for sh in self.config().security_providers() {
                if !sh.sk().is_empty() && !sh.ak().is_empty() {
                    return Some(sh.clone());
                }
            }
            None
        } else {
            None
        }
    }
    pub fn bucket<'a>(&'a self, name: &'a str) -> Bucket {
        Bucket::new(name, self)
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn do_action<T>(
        &self,
        method: Method,
        bucket_name: &str,
        uri: &str,
        with_headers: Option<HeaderMap>,
        body: Option<T>,
    ) -> Result<Response, ObsError>
    where
        T: Into<Body>,
    {
        let url = format!(
            "https://{}.{}/{}",
            bucket_name,
            self.config().endpoint(),
            uri
        );

        let canonicalized_url = self.config().canonicalized_url(bucket_name, uri);
        let mut headers = self.auth(
            method.as_str(),
            bucket_name,
            HashMap::new(),
            HashMap::new(),
            canonicalized_url,
        )?;
        if let Some(wh) = with_headers {
            headers.extend(wh);
        }

        let mut req = self.http_client.request(method, url).headers(headers);

        if let Some(body) = body {
            req = req.body(body);
        }
        let res = req.send().await?;
        Ok(res)
    }

}

#[derive(Debug)]
pub struct ClientBuilder {
    config: Config,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ClientBuilder {
    fn new() -> ClientBuilder {
        ClientBuilder {
            config: Config {
                security_providers: vec![],
                endpoint: "".into(),
                is_secure: false,
                region: "".into(),
                timeout: Duration::from_secs(3),
                signature_type: SignatureType::V2,
            },
        }
    }
    pub fn signature_type(mut self, st: SignatureType) -> ClientBuilder {
        self.config.set_signature_type(st);
        self
    }

    pub fn security_providers(mut self, sps: Vec<SecurityHolder>) -> ClientBuilder {
        self.config.set_security_providers(sps);
        self
    }

    /// 节点，支持以下三种格式:
    ///
    /// 1. http://your-endpoint
    /// 2. https://your-endpoint
    /// 3. your-endpoint
    pub fn endpoint<S: ToString>(mut self, value: S) -> ClientBuilder {
        let mut value = value.to_string();
        if value.starts_with("https://") {
            value = value.replace("https://", "");
        } else if value.starts_with("http://") {
            value = value.replace("http://", "");
        }
        self.config.set_endpoint(value);
        self
    }

    pub fn security_provider<S: ToString>(mut self, ak: S, sk: S) -> ClientBuilder {
        self.config.security_providers.push(SecurityHolder::new(
            ak.to_string(),
            sk.to_string(),
            "".to_string(),
        ));
        self
    }

    pub fn timeout(mut self, duration: Duration) -> ClientBuilder {
        self.config.set_timeout(duration);
        self
    }

    pub fn region<S: ToString>(mut self, value: S) -> ClientBuilder {
        self.config.set_region(value.to_string());
        self
    }

    pub fn is_secure(mut self, value: bool) -> ClientBuilder {
        self.config.set_is_secure(value);
        self
    }

    pub fn build(self) -> Result<Client, ObsError> {
        let req_client = reqwest::ClientBuilder::new()
            .timeout(self.config.timeout())
            .build();
        Ok(Client {
            config: self.config,
            http_client: req_client?,
        })
    }
}
