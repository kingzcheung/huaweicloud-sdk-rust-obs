use std::{time::Duration, collections::HashMap};
use lazy_static::lazy_static;

lazy_static! {
    static ref SUB_RESOURCES: Vec<&'static str> = vec![
        "CDNNotifyConfiguration",
        "acl",
        "append",
        "attname",
        "backtosource",
        "cors",
        "customdomain",
        "delete",
        "deletebucket",
        "directcoldaccess",
        "encryption",
        "inventory",
        "length",
        "lifecycle",
        "location",
        "logging",
        "metadata",
        "mirrorBackToSource",
        "modify",
        "name",
        "notification",
        "obscompresspolicy",
        "orchestration",
        "partNumber",
        "policy",
        "position",
        "quota",
        "rename",
        "replication",
        "response-cache-control",
        "response-content-disposition",
        "response-content-encoding",
        "response-content-language",
        "response-content-type",
        "response-expires",
        "restore",
        "storageClass",
        "storagePolicy",
        "storageinfo",
        "tagging",
        "torrent",
        "truncate",
        "uploadId",
        "uploads",
        "versionId",
        "versioning",
        "versions",
        "website",
        "x-image-process",
        "x-image-save-bucket",
        "x-image-save-object",
        "x-obs-security-token",
        "object-lock",
        "retention"
    ];
}

#[derive(Debug,Clone)]
pub enum SignatureType {
    V2,
    V4,
    OBS,
}

#[derive(Debug, Default, Clone)]
pub struct SecurityHolder {
    ak: String,
    sk: String,
    security_token: String,
}
impl SecurityHolder {
    pub fn new(ak: String, sk: String, security_token: String) -> Self {
        Self {
            ak,
            sk,
            security_token,
        }
    }

    pub fn ak(&self) -> &str {
        self.ak.as_ref()
    }

    pub fn sk(&self) -> &str {
        self.sk.as_ref()
    }

    pub fn security_token(&self) -> &str {
        self.security_token.as_ref()
    }
}

#[derive(Debug,Clone)]
pub struct Config {
    pub(crate) security_providers: Vec<SecurityHolder>,
    pub(crate) endpoint: String,
    pub(crate) is_secure: bool,
    pub(crate) region: String,
    pub(crate) timeout: Duration,
    pub(crate) signature_type: SignatureType,
}

pub type CanonicalizedResource = String;

impl Config {
    pub fn security_providers(&self) -> &[SecurityHolder] {
        self.security_providers.as_ref()
    }

    pub fn format_urls(&self, bucket_name: &str, object_key:&str, params: Option<HashMap<String,String>>) -> CanonicalizedResource{
        let mut canonicalized_resource: CanonicalizedResource = String::from("/");
        if !bucket_name.is_empty() {
            canonicalized_resource.push_str(bucket_name);
            canonicalized_resource.push('/');
            match self.signature_type {
                SignatureType::V2 | SignatureType::OBS => {
                    if !object_key.is_empty() {
                        canonicalized_resource.push_str(object_key);
                    }
                    if let Some(params) = params {
                        canonicalized_resource.push('?');
                        for (k,v) in &params {
                            if SUB_RESOURCES.contains(&k.as_str()) {
                                if !canonicalized_resource.ends_with('?') {
                                    canonicalized_resource.push('&');
                                }
                                canonicalized_resource.push_str(k);
                                canonicalized_resource.push('=');
                                canonicalized_resource.push_str(v);
                            }
                        }
                    }
                    
                    
                }
                SignatureType::V4 => canonicalized_resource.push('/')
            }
        }
        canonicalized_resource.trim_end_matches('?').into()
    }

    /// 暂时不支持自定义域名
    pub fn canonicalized_url(&self, bucket_name: &str, uri: &str) -> String {
        if bucket_name.is_empty() {
            String::from("/")
        } else {
            match self.signature_type {
                SignatureType::V2 | SignatureType::OBS => {
                    if uri.is_empty() {
                        format!("/{}/", bucket_name)
                    } else {
                        format!("/{}/{}", bucket_name, uri)
                    }
                }
                SignatureType::V4 => String::from("/"),
            }
        }
    }

    pub(crate) fn set_security_providers(&mut self, security_providers: Vec<SecurityHolder>) {
        self.security_providers = security_providers;
    }

    pub(crate) fn set_endpoint(&mut self, endpoint: String) {
        self.endpoint = endpoint;
    }

    pub(crate) fn set_is_secure(&mut self, is_secure: bool) {
        self.is_secure = is_secure;
    }

    pub(crate) fn set_region(&mut self, region: String) {
        self.region = region;
    }

    pub(crate) fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub(crate) fn set_signature_type(&mut self, signature_type: SignatureType) {
        self.signature_type = signature_type;
    }

    pub(crate) fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn endpoint(&self) -> &str {
        self.endpoint.as_ref()
    }
}
