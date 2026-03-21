//! Configuration types for the OBS SDK.
//!
//! This module provides configuration options for the OBS client.

use std::time::Duration;

/// Signature type for OBS authentication.
#[derive(Debug, Clone, Copy, Default)]
pub enum SignatureType {
    /// OBS signature version 2
    V2,
    /// OBS signature version 4
    V4,
    /// OBS signature (Huawei Cloud specific)
    #[default]
    Obs,
}

/// Security credentials for OBS authentication.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// Access Key ID
    access_key_id: String,
    /// Secret Access Key
    secret_access_key: String,
    /// Security token (for temporary credentials)
    security_token: Option<String>,
}

impl Credentials {
    /// Create new credentials.
    pub fn new(access_key_id: impl Into<String>, secret_access_key: impl Into<String>) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            secret_access_key: secret_access_key.into(),
            security_token: None,
        }
    }

    /// Create new credentials with a security token.
    pub fn new_with_token(
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
        security_token: impl Into<String>,
    ) -> Self {
        Self {
            access_key_id: access_key_id.into(),
            secret_access_key: secret_access_key.into(),
            security_token: Some(security_token.into()),
        }
    }

    /// Get the access key ID.
    pub fn access_key_id(&self) -> &str {
        &self.access_key_id
    }

    /// Get the secret access key.
    pub fn secret_access_key(&self) -> &str {
        &self.secret_access_key
    }

    /// Get the security token.
    pub fn security_token(&self) -> Option<&str> {
        self.security_token.as_deref()
    }
}

/// Region configuration for OBS.
#[derive(Debug, Clone)]
pub struct Region {
    /// Region name (e.g., "cn-north-4")
    name: String,
    /// Endpoint for this region
    endpoint: String,
}

impl Region {
    /// Create a new region.
    pub fn new(name: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            endpoint: endpoint.into(),
        }
    }

    /// Create a region from a region name with default Huawei Cloud endpoint.
    pub fn from_name(name: &str) -> Self {
        let endpoint = format!("obs.{}.myhuaweicloud.com", name);
        Self {
            name: name.to_string(),
            endpoint,
        }
    }

    /// Get the region name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the endpoint.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Configuration for the OBS client.
#[derive(Debug, Clone)]
pub struct Config {
    /// Credentials for authentication
    credentials: Credentials,
    /// Region configuration
    region: Region,
    /// Signature type
    signature_type: SignatureType,
    /// Request timeout
    timeout: Duration,
    /// Connect timeout
    connect_timeout: Duration,
    /// Whether to use HTTPS
    secure: bool,
}

impl Config {
    /// Create a new configuration builder.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }

    /// Get the credentials.
    pub fn credentials(&self) -> &Credentials {
        &self.credentials
    }

    /// Get the region.
    pub fn region(&self) -> &Region {
        &self.region
    }

    /// Get the signature type.
    pub fn signature_type(&self) -> SignatureType {
        self.signature_type
    }

    /// Get the request timeout.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Get the connect timeout.
    pub fn connect_timeout(&self) -> Duration {
        self.connect_timeout
    }

    /// Check if HTTPS is enabled.
    pub fn is_secure(&self) -> bool {
        self.secure
    }

    /// Get the endpoint URL.
    pub fn endpoint_url(&self) -> String {
        let scheme = if self.secure { "https" } else { "http" };
        format!("{}://{}", scheme, self.region.endpoint)
    }
}

/// Builder for creating a Config.
#[derive(Debug, Clone)]
pub struct ConfigBuilder {
    credentials: Option<Credentials>,
    region: Option<Region>,
    signature_type: SignatureType,
    timeout: Duration,
    connect_timeout: Duration,
    secure: bool,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            credentials: None,
            region: None,
            signature_type: SignatureType::default(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            secure: true,
        }
    }
}

impl ConfigBuilder {
    /// Set the credentials.
    pub fn credentials(mut self, credentials: Credentials) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// Set the credentials from access key ID and secret access key.
    pub fn access_key(
        mut self,
        access_key_id: impl Into<String>,
        secret_access_key: impl Into<String>,
    ) -> Self {
        self.credentials = Some(Credentials::new(access_key_id, secret_access_key));
        self
    }

    /// Set the region.
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the region by name.
    pub fn region_name(mut self, name: impl Into<String>) -> Self {
        let name = name.into();
        self.region = Some(Region::from_name(&name));
        self
    }

    /// Set the endpoint directly.
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        let endpoint = endpoint.into();
        // Parse endpoint to extract region if possible
        let name = extract_region_from_endpoint(&endpoint).unwrap_or_else(|| "unknown".to_string());
        self.region = Some(Region::new(name, endpoint));
        self
    }

    /// Set the signature type.
    pub fn signature_type(mut self, signature_type: SignatureType) -> Self {
        self.signature_type = signature_type;
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the connect timeout.
    pub fn connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = connect_timeout;
        self
    }

    /// Enable or disable HTTPS.
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    /// Build the configuration.
    pub fn build(self) -> Result<Config, crate::error::ObsError> {
        let credentials = self.credentials.ok_or_else(|| {
            crate::error::ObsError::Credentials("credentials are required".to_string())
        })?;
        let region = self.region.ok_or_else(|| {
            crate::error::ObsError::InvalidInput("region or endpoint is required".to_string())
        })?;

        Ok(Config {
            credentials,
            region,
            signature_type: self.signature_type,
            timeout: self.timeout,
            connect_timeout: self.connect_timeout,
            secure: self.secure,
        })
    }
}

/// Extract region name from endpoint.
fn extract_region_from_endpoint(endpoint: &str) -> Option<String> {
    // Remove protocol prefix if present
    let endpoint = endpoint
        .strip_prefix("https://")
        .or_else(|| endpoint.strip_prefix("http://"))
        .unwrap_or(endpoint);

    // Try to extract region from endpoint like "obs.cn-north-4.myhuaweicloud.com"
    let parts: Vec<&str> = endpoint.split('.').collect();
    if parts.len() >= 2 && parts[0] == "obs" {
        Some(parts[1].to_string())
    } else {
        None
    }
}

lazy_static::lazy_static! {
    /// Sub-resources for OBS API
    pub static ref SUB_RESOURCES: Vec<&'static str> = vec![
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
