//! All the errors that can be returned from Protean

use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[cfg_attr(
  feature = "serde_support",
  derive(serde::Serialize, serde::Deserialize)
)]
pub enum ProteanError {
  #[error("There was an error attempting to convert from one type to another")]
  ConversionError,
}
