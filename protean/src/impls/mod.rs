//! Patchwork implementations for various primitives and common types
//!
//! Wrappers for each of the major genre of data object around specific implementations, with
//! Patch and PatchAction implementations for each. This is very similar to how SerdeJson does it,
//! as any data structure can be described with these.

pub use crate::local::*;

pub mod list;
pub mod map;
pub mod object;
pub mod primitives;

pub mod json;
