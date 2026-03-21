//! Operations module - AWS SDK style fluent builders for OBS operations.
//!
//! This module provides fluent builders for each OBS operation, following
//! the AWS SDK pattern where each operation has:
//! - A fluent builder for constructing the request
//! - An input type for the request parameters
//! - An output type for the response data
//! - A `.send()` method to execute the request

mod bucket;
mod multipart;
mod object;

pub use bucket::*;
pub use multipart::*;
pub use object::*;

use crate::client::Client;
use crate::error::Result;

/// Trait for all fluent builders.
pub trait FluentBuilder {
    /// The input type for this operation.
    type Input;
    /// The output type for this operation.
    type Output;

    /// Create a new fluent builder.
    fn new(client: Client) -> Self;

    /// Send the request and receive the response.
    fn send(&self) -> impl std::future::Future<Output = Result<Self::Output>> + Send;
}

/// Trait for operations that can be customized.
pub trait RequestBuilder {
    /// Customize the request before sending.
    fn customize<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut reqwest::RequestBuilder) -> &mut reqwest::RequestBuilder;
}