//! DeleteObjects operation - batch delete multiple objects from OBS.

use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};
use md5::{Digest, Md5};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the DeleteObjects operation.
#[derive(Debug, Clone)]
pub struct DeleteObjectsFluentBuilder {
    client: Client,
    inner: DeleteObjectsInput,
}

impl DeleteObjectsFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: DeleteObjectsInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Add an object to delete.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.objects.push(DeleteObjectRequest {
            key: key.into(),
            version_id: None,
        });
        self
    }

    /// Add an object with version to delete.
    pub fn key_with_version(mut self, key: impl Into<String>, version_id: impl Into<String>) -> Self {
        self.inner.objects.push(DeleteObjectRequest {
            key: key.into(),
            version_id: Some(version_id.into()),
        });
        self
    }

    /// Add multiple objects to delete.
    pub fn keys(mut self, keys: Vec<String>) -> Self {
        for key in keys {
            self.inner.objects.push(DeleteObjectRequest {
                key,
                version_id: None,
            });
        }
        self
    }

    /// Set quiet mode.
    /// When true, only returns deletion errors; when false, returns all deletion results.
    pub fn quiet(mut self, quiet: bool) -> Self {
        self.inner.quiet = quiet;
        self
    }

    /// Set encoding type for object names.
    /// Only "url" is supported.
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<DeleteObjectsOutput> {
        let bucket = &self.inner.bucket;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if self.inner.objects.is_empty() {
            return Err(ObsError::InvalidInput("at least one object key is required".to_string()));
        }
        if self.inner.objects.len() > 1000 {
            return Err(ObsError::InvalidInput("maximum 1000 objects can be deleted in one request".to_string()));
        }

        let mut params = HashMap::new();
        params.insert("delete".to_string(), String::new());

        let delete = DeleteRequest {
            quiet: self.inner.quiet,
            encoding_type: self.inner.encoding_type.clone(),
            object: self.inner.objects.clone(),
        };

        let body = crate::xml_utils::to_xml(&delete)?;

        let mut hasher = Md5::new();
        hasher.update(body.as_bytes());
        let result = hasher.finalize();
        let md5_value = general_purpose::STANDARD.encode(result);

        let mut headers = HeaderMap::new();
        headers.insert("Content-MD5", HeaderValue::from_str(&md5_value).unwrap());
        headers.insert(
            "Content-Length",
            HeaderValue::from_str(&body.len().to_string()).unwrap(),
        );

        let resp = self
            .client
            .do_request(
                Method::POST,
                Some(bucket),
                None,
                Some(headers),
                Some(params),
                Some(body.into_bytes()),
            )
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        // Parse the response
        let text = resp.text().await?;
        let result: DeleteResult = crate::xml_utils::from_xml(&text)?;

        Ok(DeleteObjectsOutput {
            deleted: result.deleted,
            errors: result.errors,
            encoding_type: result.encoding_type,
        })
    }
}

/// Input for the DeleteObjects operation.
#[derive(Debug, Clone, Default)]
pub struct DeleteObjectsInput {
    bucket: String,
    objects: Vec<DeleteObjectRequest>,
    quiet: bool,
    encoding_type: Option<String>,
}

/// Object to delete in the request.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteObjectRequest {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
    version_id: Option<String>,
}

impl DeleteObjectRequest {
    /// Create a new delete object request.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: None,
        }
    }

    /// Create a new delete object request with version.
    pub fn with_version(key: impl Into<String>, version_id: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            version_id: Some(version_id.into()),
        }
    }

    /// Get the object key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the version ID.
    pub fn version_id(&self) -> Option<&str> {
        self.version_id.as_deref()
    }
}

/// Output for the DeleteObjects operation.
#[derive(Debug, Clone)]
pub struct DeleteObjectsOutput {
    deleted: Vec<DeletedObject>,
    errors: Vec<DeleteError>,
    encoding_type: Option<String>,
}

impl DeleteObjectsOutput {
    /// Get the successfully deleted objects.
    pub fn deleted(&self) -> &[DeletedObject] {
        &self.deleted
    }

    /// Get the deletion errors.
    pub fn errors(&self) -> &[DeleteError] {
        &self.errors
    }

    /// Get the encoding type.
    pub fn encoding_type(&self) -> Option<&str> {
        self.encoding_type.as_deref()
    }

    /// Check if all objects were deleted successfully.
    pub fn is_all_success(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Successfully deleted object information.
#[derive(Debug, Clone, Deserialize)]
pub struct DeletedObject {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
    version_id: Option<String>,
    #[serde(rename = "DeleteMarker", skip_serializing_if = "Option::is_none")]
    delete_marker: Option<bool>,
    #[serde(rename = "DeleteMarkerVersionId", skip_serializing_if = "Option::is_none")]
    delete_marker_version_id: Option<String>,
}

impl DeletedObject {
    /// Get the object key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the version ID.
    pub fn version_id(&self) -> Option<&str> {
        self.version_id.as_deref()
    }

    /// Check if a delete marker was created/deleted.
    pub fn delete_marker(&self) -> Option<bool> {
        self.delete_marker
    }

    /// Get the delete marker version ID.
    pub fn delete_marker_version_id(&self) -> Option<&str> {
        self.delete_marker_version_id.as_deref()
    }
}

/// Deletion error information.
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteError {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
    version_id: Option<String>,
}

impl DeleteError {
    /// Get the object key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the error code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Get the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the version ID.
    pub fn version_id(&self) -> Option<&str> {
        self.version_id.as_deref()
    }
}

/// Result of DeleteObjects operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "DeleteResult")]
struct DeleteResult {
    #[serde(rename = "Deleted", default)]
    deleted: Vec<DeletedObject>,
    #[serde(rename = "Error", default)]
    errors: Vec<DeleteError>,
    #[serde(rename = "EncodingType", skip_serializing_if = "Option::is_none")]
    encoding_type: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename = "Delete")]
struct DeleteRequest {
    #[serde(rename = "Quiet", skip_serializing_if = "std::ops::Not::not")]
    quiet: bool,
    #[serde(rename = "EncodingType", skip_serializing_if = "Option::is_none")]
    encoding_type: Option<String>,
    #[serde(rename = "Object")]
    object: Vec<DeleteObjectRequest>,
}