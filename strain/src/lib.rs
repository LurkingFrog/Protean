//! Traits applying and tracking struct mutations
//!
//! This set of tools is to remove the boilerplate when equality is not enough, but we want to know
//! what is different between two structs. As an outgrowth of this, keeping a historical record of the
//! mutations can help in transactional applications, able to roll back changes to a prior state as well
//! as generate better error logs.

pub use serde::{Deserialize, Serialize};

pub mod error;
pub use error::StrainError;

/// A method of creating and detecting mutations between structs
pub trait Patchwork {}

/// Keeps an internal record of mutations to the struct
pub trait Historic {}

///
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct Patch {}

impl std::fmt::Display for Patch {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl Patch {}
