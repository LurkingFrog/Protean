//! All the errors that can be returned from Protean

use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[cfg_attr(
  feature = "serde_support",
  derive(serde::Serialize, serde::Deserialize)
)]
pub enum ProteanError {
  #[error("The given patch doesn't match the type of struct it is being applied to")]
  InvalidPatchType,
}
