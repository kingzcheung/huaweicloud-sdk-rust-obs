//! Authentication module for OBS SDK.
//!
//! This module handles signature generation and request authentication
//! following Huawei Cloud OBS API specification.
//!
//! # Signature Algorithm
//!
//! The signature is calculated as follows:
//! ```text
//! StringToSign = HTTP-Verb + "\n" +
//!                Content-MD5 + "\n" +
//!                Content-Type + "\n" +
//!                Date + "\n" +
//!                CanonicalizedHeaders + "\n" +
//!                CanonicalizedResource
//!
//! Signature = Base64(HMAC-SHA1(SecretKey, UTF8Encode(StringToSign)))
//! ```

use crate::client::Client;
use crate::config::SignatureType;
use crate::error::Result;
use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Utc};
use hmac_sha1::hmac_sha1;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::str::FromStr;

/// RFC 1123 date format for HTTP headers
const RFC1123: &str = "%a, %d %b %Y %H:%M:%S GMT";

/// Authorization trait for signature generation.
pub trait Authorization {
    /// Generate signature for the request.
    fn signature(
        &self,
        method: &str,
        headers: HashMap<String, Vec<String>>,
        canonicalized_resource: String,
    ) -> Result<String>;

    /// Generate authorization headers for the request.
    fn auth(
        &self,
        method: &str,
        bucket: &str,
        params: HashMap<String, String>,
        headers: HashMap<String, Vec<String>>,
        canonicalized_resource: String,
    ) -> Result<HeaderMap>;
}

impl Authorization for Client {
    /// Generate signature for the request.
    ///
    /// This method constructs the StringToSign and calculates the HMAC-SHA1 signature.
    fn signature(
        &self,
        method: &str,
        headers: HashMap<String, Vec<String>>,
        canonicalized_resource: String,
    ) -> Result<String> {
        // Build the string to sign
        let string_to_sign =
            build_string_to_sign(method, headers.clone(), canonicalized_resource, true);

        // Calculate signature using HMAC-SHA1
        let credentials = self.config().credentials();
        let signature = calculate_signature(&string_to_sign, credentials.secret_access_key())?;

        Ok(signature)
    }

    /// Generate authorization headers for the request.
    ///
    /// This method:
    /// 1. Adds the Host header
    /// 2. Adds the Date header
    /// 3. Calculates the signature
    /// 4. Builds the Authorization header
    fn auth(
        &self,
        method: &str,
        bucket: &str,
        params: HashMap<String, String>,
        mut headers: HashMap<String, Vec<String>>,
        canonicalized_resource: String,
    ) -> Result<HeaderMap> {
        let signature_type = self.config().signature_type();
        let endpoint = self.config().region().endpoint();

        // Add Host header
        if !bucket.is_empty() {
            headers.insert("Host".into(), vec![format!("{}.{}", bucket, endpoint)]);
        } else {
            headers.insert("Host".into(), vec![endpoint.to_string()]);
        }

        // Prepare Date header
        prepare_date_header(&mut headers, signature_type);

        // Build canonicalized resource with query parameters
        let full_canonicalized_resource =
            build_canonicalized_resource(canonicalized_resource, &params);

        // Calculate signature
        let sign = self.signature(method, headers.clone(), full_canonicalized_resource)?;

        // Build Authorization header
        let credentials = self.config().credentials();
        let auth_value = format!("OBS {}:{}", credentials.access_key_id(), sign);

        // Build final headers
        let mut result_headers = HeaderMap::new();
        result_headers.insert("Authorization", HeaderValue::from_str(&auth_value).unwrap());

        for (key, values) in headers.iter() {
            let header_name = HeaderName::from_str(key).unwrap();
            let header_value = HeaderValue::from_str(&values.join(",")).unwrap();
            result_headers.insert(header_name, header_value);
        }

        Ok(result_headers)
    }
}

/// Build the StringToSign for signature calculation.
///
/// # Format
/// ```text
/// StringToSign = HTTP-Verb + "\n" +
///                Content-MD5 + "\n" +
///                Content-Type + "\n" +
///                Date + "\n" +
///                CanonicalizedHeaders + "\n" +
///                CanonicalizedResource
/// ```
fn build_string_to_sign(
    method: &str,
    headers: HashMap<String, Vec<String>>,
    canonicalized_resource: String,
    is_obs: bool,
) -> String {
    let prefix = if is_obs { "x-obs-" } else { "x-amz-" };

    // Build canonicalized headers (x-obs-* headers)
    let canonicalized_headers = build_canonicalized_headers(&headers, prefix);

    // Check if x-obs-date or x-amz-date header exists
    let has_special_date = headers.keys().any(|k| {
        let k_lower = k.to_lowercase();
        k_lower == "x-obs-date" || k_lower == "x-amz-date"
    });

    // Build the string to sign
    // Format: VERB\n + MD5\n + Content-Type\n + Date\n + canonicalizedHeaders + canonicalizedResource
    // Note: If x-obs-date/x-amz-date exists, Date should be empty in StringToSign
    let mut string_to_sign = String::new();

    // HTTP-Verb
    string_to_sign.push_str(method);
    string_to_sign.push('\n');

    // Content-MD5 (empty if not present)
    let content_md5 = get_header_value(&headers, "content-md5");
    string_to_sign.push_str(&content_md5);
    string_to_sign.push('\n');

    // Content-Type (empty if not present)
    let content_type = get_header_value(&headers, "content-type");
    string_to_sign.push_str(&content_type);
    string_to_sign.push('\n');

    // Date - empty if x-obs-date/x-amz-date exists, otherwise use actual date
    if has_special_date {
        string_to_sign.push('\n');
    } else {
        let date = get_header_value(&headers, "date");
        string_to_sign.push_str(&date);
        string_to_sign.push('\n');
    }

    // CanonicalizedHeaders (x-obs-* headers, sorted)
    // Note: If there are canonicalized headers, add a newline after them
    if !canonicalized_headers.is_empty() {
        string_to_sign.push_str(&canonicalized_headers);
        string_to_sign.push('\n');
    }

    // CanonicalizedResource
    string_to_sign.push_str(&canonicalized_resource);

    // Debug output
    tracing::debug!("StringToSign:\n{}", string_to_sign);
    tracing::debug!("CanonicalizedHeaders: {}", canonicalized_headers);
    tracing::debug!("CanonicalizedResource: {}", canonicalized_resource);

    string_to_sign
}

/// Get header value from headers map (case-insensitive lookup).
fn get_header_value(headers: &HashMap<String, Vec<String>>, key: &str) -> String {
    let key_lower = key.to_lowercase();
    headers
        .iter()
        .find(|(k, _)| k.to_lowercase() == key_lower)
        .and_then(|(_, v)| v.first())
        .map(|s| s.as_str())
        .unwrap_or("")
        .to_string()
}

/// Build canonicalized headers (x-obs-* headers).
///
/// Rules:
/// 1. Convert header names to lowercase
/// 2. Sort headers by name in dictionary order
/// 3. For x-obs-meta-* headers, trim the value
/// 4. Format as "header-name:header-value\n"
fn build_canonicalized_headers(headers: &HashMap<String, Vec<String>>, prefix: &str) -> String {
    let mut obs_headers: Vec<(String, String)> = headers
        .iter()
        .filter(|(key, _)| {
            let key_lower = key.to_lowercase();
            key_lower.starts_with(prefix)
        })
        .map(|(key, values)| {
            let key_lower = key.to_lowercase();
            let value = if key_lower.starts_with(&format!("{}meta-", prefix)) {
                // For meta headers, trim each value
                values
                    .iter()
                    .map(|v| v.trim())
                    .collect::<Vec<_>>()
                    .join(",")
            } else {
                values.join(",")
            };
            (key_lower, value)
        })
        .collect();

    // Sort by header name
    obs_headers.sort_by(|a, b| a.0.cmp(&b.0));

    // Build the canonicalized headers string
    obs_headers
        .iter()
        .map(|(key, value)| format!("{}:{}", key, value))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Build canonicalized resource.
///
/// # Format
/// ```text
/// CanonicalizedResource = "/" + bucket + "/" + object + "?" + sub-resources
/// ```
fn build_canonicalized_resource(mut resource: String, params: &HashMap<String, String>) -> String {
    // Process query parameters (sub-resources)
    let sub_resources = crate::config::SUB_RESOURCES.as_slice();

    let mut resource_params: Vec<(&String, &String)> = params
        .iter()
        .filter(|(key, _)| sub_resources.contains(&key.as_str()) || key.starts_with("x-obs-"))
        .collect();

    // Sort by parameter name
    resource_params.sort_by(|a, b| a.0.cmp(b.0));

    if !resource_params.is_empty() {
        resource.push('?');
        for (i, (key, value)) in resource_params.iter().enumerate() {
            if i > 0 {
                resource.push('&');
            }
            resource.push_str(key);
            if !value.is_empty() {
                resource.push('=');
                resource.push_str(value);
            }
        }
    }

    resource
}

/// Prepare the Date header.
///
/// If x-obs-date header is present, the Date header is set to empty.
/// Otherwise, the current time in RFC 1123 format is used.
fn prepare_date_header(headers: &mut HashMap<String, Vec<String>>, signature_type: SignatureType) {
    let is_v4 = matches!(signature_type, SignatureType::V4);

    // Find x-obs-date or x-amz-date header (case-insensitive)
    let obs_date_key = headers.keys().find(|k| k.to_lowercase() == "x-obs-date");
    let amz_date_key = headers.keys().find(|k| k.to_lowercase() == "x-amz-date");

    let date_key = obs_date_key.or(amz_date_key).cloned();

    if let Some(key) = date_key {
        if let Some(date_values) = headers.get(&key) {
            if date_values.len() == 1 {
                if is_v4 {
                    // V4 signature uses different date format
                    if let Ok(t) = DateTime::parse_from_str(&date_values[0], "%Y%m%dT%H%M%SZ") {
                        headers.insert("Date".into(), vec![t.format(RFC1123).to_string()]);
                        return;
                    }
                } else if date_values[0].ends_with("GMT") {
                    headers.insert("Date".into(), vec![date_values[0].clone()]);
                    return;
                }
            }
        }
        // Remove invalid x-obs-date header
        headers.remove(&key);
    }

    // Set current date if not present (case-insensitive check)
    let has_date = headers.keys().any(|k| k.to_lowercase() == "date");
    if !has_date {
        let now = Utc::now();
        let date_str = now.format(RFC1123).to_string();
        headers.insert("Date".into(), vec![date_str]);
    }
}

/// Calculate signature using HMAC-SHA1.
///
/// # Algorithm
/// ```text
/// Signature = Base64(HMAC-SHA1(SecretKey, UTF8Encode(StringToSign)))
/// ```
fn calculate_signature(string_to_sign: &str, secret_key: &str) -> Result<String> {
    let hash = hmac_sha1(secret_key.as_bytes(), string_to_sign.as_bytes());
    let signature = general_purpose::STANDARD.encode(hash);
    Ok(signature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_string_to_sign() {
        let mut headers = HashMap::new();
        headers.insert(
            "content-md5".into(),
            vec!["d41d8cd98f00b204e9800998ecf8427e".into()],
        );
        headers.insert("content-type".into(), vec!["text/plain".into()]);
        headers.insert("date".into(), vec!["Sat, 12 Oct 2015 08:12:38 GMT".into()]);
        headers.insert("x-obs-acl".into(), vec!["public-read".into()]);

        let string_to_sign =
            build_string_to_sign("PUT", headers, "/bucket-test/object-test?acl".into(), true);

        assert!(string_to_sign.contains("PUT"));
        assert!(string_to_sign.contains("d41d8cd98f00b204e9800998ecf8427e"));
        assert!(string_to_sign.contains("text/plain"));
        assert!(string_to_sign.contains("Sat, 12 Oct 2015 08:12:38 GMT"));
        assert!(string_to_sign.contains("x-obs-acl:public-read"));
        assert!(string_to_sign.contains("/bucket-test/object-test?acl"));
    }

    #[test]
    fn test_calculate_signature() {
        // Test signature calculation
        let string_to_sign = "PUT\n\ntext/plain\nSat, 12 Oct 2015 08:12:38 GMT\nx-obs-acl:public-read\n/bucket-test/object-test?acl";
        let result = calculate_signature(string_to_sign, "test-secret-key");
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_build_canonicalized_headers() {
        let mut headers = HashMap::new();
        headers.insert("x-obs-acl".into(), vec!["public-read".into()]);
        headers.insert("x-obs-meta-key1".into(), vec!["value1".into()]);
        headers.insert(
            "x-obs-meta-key2".into(),
            vec!["value2".into(), "value3".into()],
        );

        let result = build_canonicalized_headers(&headers, "x-obs-");

        // Headers should be sorted
        assert!(result.contains("x-obs-acl:public-read"));
        assert!(result.contains("x-obs-meta-key1:value1"));
        assert!(result.contains("x-obs-meta-key2:value2,value3"));
    }

    #[test]
    fn test_build_canonicalized_resource() {
        let mut params = HashMap::new();
        params.insert("acl".into(), "".into());
        params.insert("versionId".into(), "xxx".into());

        let result = build_canonicalized_resource("/bucket/object".into(), &params);

        // Parameters should be sorted
        assert!(result.starts_with("/bucket/object?"));
        assert!(result.contains("acl"));
        assert!(result.contains("versionId=xxx"));
    }
}
