use std::time::Duration;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Config {
    pub(crate) security_providers: Vec<SecurityHolder>,
    pub(crate) endpoint: String,
    pub(crate) is_secure: bool,
    pub(crate) region: String,
    pub(crate) timeout: Duration,
    pub(crate) signature_type: SignatureType,
}

impl Config {
    pub fn security_providers(&self) -> &[SecurityHolder] {
        self.security_providers.as_ref()
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
