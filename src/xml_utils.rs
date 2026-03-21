//! XML serialization/deserialization utilities using quick-xml.

use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{ObsError, Result};

/// Serialize a value to XML string.
pub fn to_xml<T: Serialize>(value: &T) -> Result<String> {
    to_string(value).map_err(|e| ObsError::Serialization(e.to_string()))
}

/// Deserialize a value from XML string.
pub fn from_xml<T: DeserializeOwned>(xml: &str) -> Result<T> {
    from_str(xml).map_err(|e| ObsError::XmlParse(e.to_string()))
}