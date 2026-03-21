//! SetObjectAcl operation - set access control list for an object in OBS.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the SetObjectAcl operation.
#[derive(Debug, Clone)]
pub struct SetObjectAclFluentBuilder {
    client: Client,
    inner: SetObjectAclInput,
}

impl SetObjectAclFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: SetObjectAclInput::default(),
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

    /// Set the owner ID.
    pub fn owner_id(mut self, owner_id: impl Into<String>) -> Self {
        self.inner.acl.owner.id = owner_id.into();
        self
    }

    /// Set whether the object ACL inherits from the bucket ACL.
    pub fn delivered(mut self, delivered: bool) -> Self {
        self.inner.acl.delivered = delivered;
        self
    }

    /// Add a grant to the ACL.
    pub fn grant(mut self, grant: Grant) -> Self {
        self.inner.acl.access_control_list.grants.push(grant);
        self
    }

    /// Set all grants.
    pub fn grants(mut self, grants: Vec<Grant>) -> Self {
        self.inner.acl.access_control_list.grants = grants;
        self
    }

    /// Set a canned ACL (predefined ACL).
    /// Common values: "private", "public-read", "public-read-write".
    pub fn canned_acl(mut self, canned_acl: impl Into<String>) -> Self {
        self.inner.canned_acl = Some(canned_acl.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<SetObjectAclOutput> {
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

        let mut headers = reqwest::header::HeaderMap::new();

        // If canned ACL is set, use x-obs-acl header
        if let Some(ref canned_acl) = self.inner.canned_acl {
            headers.insert(
                "x-obs-acl",
                reqwest::header::HeaderValue::from_str(canned_acl)
                    .map_err(|e| ObsError::InvalidInput(format!("Invalid canned ACL: {}", e)))?,
            );
        }

        // Build request body if not using canned ACL
        let body = if self.inner.canned_acl.is_none() {
            let acl_xml = crate::xml_utils::to_xml(&self.inner.acl)?;
            Some(acl_xml.into_bytes())
        } else {
            None
        };

        let resp = self
            .client
            .do_request(Method::PUT, Some(bucket), Some(key), Some(headers), Some(params), body)
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

        Ok(SetObjectAclOutput { version_id })
    }
}

/// Input for the SetObjectAcl operation.
#[derive(Debug, Clone, Default)]
pub struct SetObjectAclInput {
    bucket: String,
    key: String,
    version_id: Option<String>,
    canned_acl: Option<String>,
    acl: AccessControlPolicy,
}

/// Output for the SetObjectAcl operation.
#[derive(Debug, Clone)]
pub struct SetObjectAclOutput {
    version_id: Option<String>,
}

impl SetObjectAclOutput {
    /// Get the version ID.
    pub fn version_id(&self) -> Option<&str> {
        self.version_id.as_deref()
    }
}

/// Access control policy for an object.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "AccessControlPolicy")]
pub struct AccessControlPolicy {
    /// Owner information.
    #[serde(rename = "Owner")]
    pub owner: AclOwner,
    /// Whether the object ACL inherits from the bucket ACL.
    #[serde(rename = "Delivered", skip_serializing_if = "std::ops::Not::not")]
    pub delivered: bool,
    /// Access control list.
    #[serde(rename = "AccessControlList")]
    pub access_control_list: AccessControlList,
}

/// Owner information for object ACL.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AclOwner {
    /// Owner's domain ID.
    #[serde(rename = "ID")]
    pub id: String,
}

impl AclOwner {
    /// Get the owner's domain ID.
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Access control list.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccessControlList {
    /// List of grants.
    #[serde(rename = "Grant")]
    pub grants: Vec<Grant>,
}

/// A grant in the ACL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grant {
    /// Grantee information.
    #[serde(rename = "Grantee")]
    pub grantee: Grantee,
    /// Permission granted.
    #[serde(rename = "Permission")]
    pub permission: Permission,
}

impl Grant {
    /// Create a new grant.
    pub fn new(grantee: Grantee, permission: Permission) -> Self {
        Self { grantee, permission }
    }

    /// Get the grantee.
    pub fn grantee(&self) -> &Grantee {
        &self.grantee
    }

    /// Get the permission.
    pub fn permission(&self) -> &Permission {
        &self.permission
    }
}

/// Grantee information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grantee {
    /// Grantee's domain ID (for specific user).
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    id_value: Option<String>,
    /// Canned grantee (e.g., "Everyone" for public access).
    #[serde(rename = "Canned", skip_serializing_if = "Option::is_none")]
    canned_value: Option<CannedGrantee>,
}

impl Grantee {
    /// Get the grantee's domain ID.
    pub fn id(&self) -> Option<&str> {
        self.id_value.as_deref()
    }

    /// Get the canned grantee type.
    pub fn canned(&self) -> Option<&CannedGrantee> {
        self.canned_value.as_ref()
    }

    /// Create a grantee by domain ID.
    pub fn by_id(id: impl Into<String>) -> Self {
        Self {
            id_value: Some(id.into()),
            canned_value: None,
        }
    }

    /// Create a canned grantee (e.g., Everyone).
    pub fn from_canned(canned: CannedGrantee) -> Self {
        Self {
            id_value: None,
            canned_value: Some(canned),
        }
    }

    /// Create a grantee for everyone (public access).
    pub fn everyone() -> Self {
        Self::from_canned(CannedGrantee::Everyone)
    }
}

/// Canned grantee types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CannedGrantee {
    /// Everyone (public access).
    #[serde(rename = "Everyone")]
    Everyone,
}

/// Permission types for object ACL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    /// Read access to the object.
    #[serde(rename = "READ")]
    Read,
    /// Read access to the object ACL.
    #[serde(rename = "READ_ACP")]
    ReadAcp,
    /// Write access to the object ACL.
    #[serde(rename = "WRITE_ACP")]
    WriteAcp,
    /// Full control (READ + READ_ACP + WRITE_ACP).
    #[serde(rename = "FULL_CONTROL")]
    FullControl,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::Read => write!(f, "READ"),
            Permission::ReadAcp => write!(f, "READ_ACP"),
            Permission::WriteAcp => write!(f, "WRITE_ACP"),
            Permission::FullControl => write!(f, "FULL_CONTROL"),
        }
    }
}

/// Canned ACL constants.
pub mod canned_acl {
    /// Private (owner gets full control, no one else has access).
    pub const PRIVATE: &str = "private";
    /// Public read (owner gets full control, everyone gets read).
    pub const PUBLIC_READ: &str = "public-read";
    /// Public read/write (owner gets full control, everyone gets read and write).
    pub const PUBLIC_READ_WRITE: &str = "public-read-write";
    /// Authenticated read (owner gets full control, authenticated users get read).
    pub const AUTHENTICATED_READ: &str = "authenticated-read";
    /// Bucket owner read (object owner gets full control, bucket owner gets read).
    pub const BUCKET_OWNER_READ: &str = "bucket-owner-read";
    /// Bucket owner full control (object owner gets full control, bucket owner gets full control).
    pub const BUCKET_OWNER_FULL_CONTROL: &str = "bucket-owner-full-control";
}