//! Object operations module - AWS SDK style fluent builders for OBS object operations.
//!
//! This module provides the following operations:
//! - [`put_object`] - Upload an object to OBS
//! - [`get_object`] - Download an object from OBS
//! - [`delete_object`] - Delete a single object from OBS
//! - [`delete_objects`] - Batch delete multiple objects from OBS
//! - [`copy_object`] - Copy an object within OBS
//! - [`head_object`] - Get object metadata from OBS
//! - [`append_object`] - Append data to an object in OBS

mod append_object;
mod copy_object;
mod delete_object;
mod delete_objects;
mod get_object;
mod head_object;
mod put_object;

pub use append_object::*;
pub use copy_object::*;
pub use delete_object::*;
pub use delete_objects::*;
pub use get_object::*;
pub use head_object::*;
pub use put_object::*;