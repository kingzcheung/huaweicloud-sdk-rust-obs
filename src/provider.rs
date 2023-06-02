pub trait SecurityProvider {
    fn security(&self) -> SecurityHolder;
}

#[derive(Debug, Default,Clone)]
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

pub struct EnvSecurityProvider {
    sh: SecurityHolder,
    suffix: String,
}

impl EnvSecurityProvider {
    pub fn new(suffix: String) -> Self {
        let suffix = if !suffix.is_empty() {
            format!("_{}", suffix)
        } else {
            suffix
        };
        Self {
            sh: SecurityHolder::default(),
            suffix,
        }
    }
}

impl SecurityProvider for EnvSecurityProvider {
    fn security(&self) -> SecurityHolder {
        let ak = format!(
            "{}{}",
            std::env::var("OBS_ACCESS_KEY_ID").unwrap_or_default(),
            self.suffix.as_str()
        );
        let sk = format!(
            "{}{}",
            std::env::var("OBS_SECRET_ACCESS_KEY").unwrap_or_default(),
            self.suffix.as_str()
        );
        let security_token = format!(
            "{}{}",
            std::env::var("OBS_SECURITY_TOKEN").unwrap_or_default(),
            self.suffix.as_str()
        );
        SecurityHolder {
            ak,
            sk,
            security_token,
        }
    }
}
