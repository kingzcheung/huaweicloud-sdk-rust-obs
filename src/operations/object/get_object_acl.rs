//! GetObjectAcl operation - get access control list for an object in OBS.

use std::collections::HashMap;

use reqwest::Method;
use serde::Deserialize;

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the GetObjectAcl operation.
#[derive(Debug, Clone)]
pub struct GetObjectAclFluentBuilder {
    client: Client,
    inner: GetObjectAclInput,
}

impl GetObjectAclFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: GetObjectAclInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Set the object key.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = key.into();
        self
    }

    /// Set the version ID.
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<GetObjectAclOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }

        let mut params = HashMap::new();
        params.insert("acl".to_string(), String::new());

        if let Some(ref version_id) = self.inner.version_id {
            params.insert("versionId".to_string(), version_id.clone());
        }

        let resp = self
            .client
            .do_request(Method::GET, Some(bucket), Some(key), None, Some(params), None)
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        // Get version ID from response header
        let version_id = resp
            .headers()
            .get("x-obs-version-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        // Parse the response body
        let text = resp.text().await?;
        let policy: AccessControlPolicy = crate::xml_utils::from_xml(&text)?;

        Ok(GetObjectAclOutput {
            version_id,
            owner: policy.owner,
            delivered: policy.delivered,
            grants: policy.access_control_list.grants,
        })
    }
}

/// Input for the GetObjectAcl operation.
#[derive(Debug, Clone, Default)]
pub struct GetObjectAclInput {
    bucket: String,
    key: String,
    version_id: Option<String>,
}

/// Output for the GetObjectAcl operation.
#[derive(Debug, Clone)]
pub struct GetObjectAclOutput {
    version_id: Option<String>,
    owner: AclOwner,
    delivered: bool,
    grants: Vec<Grant>,
}

impl GetObjectAclOutput {
    /// Get the version ID.
    pub fn version_id(&self) -> Option<&str> {
        self.version_id.as_deref()
    }

    /// Get the owner information.
    pub fn owner(&self) -> &AclOwner {
        &self.owner
    }

    /// Check if the object ACL inherits from the bucket ACL.
    pub fn delivered(&self) -> bool {
        self.delivered
    }

    /// Get the list of grants.
    pub fn grants(&self) -> &[Grant] {
        &self.grants
    }
}

/// Access control policy response from OBS.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "AccessControlPolicy")]
struct AccessControlPolicy {
    #[serde(rename = "Owner")]
    owner: AclOwner,
    #[serde(rename = "Delivered", default)]
    delivered: bool,
    #[serde(rename = "AccessControlList")]
    access_control_list: AccessControlList,
}

/// Access control list (internal for deserialization).
#[derive(Debug, Clone, Deserialize)]
struct AccessControlList {
    #[serde(rename = "Grant", default)]
    grants: Vec<Grant>,
}

// Re-export types from set_object_acl module
pub use super::set_object_acl::{AclOwner, Grant};