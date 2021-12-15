//! All the errors that can be returned from Protean

use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[cfg_attr(
  feature = "serde_support",
  derive(serde::Serialize, serde::Deserialize)
)]
pub enum ProteanError {
  #[error("The given patch doesn't match the type of struct it is being applied to")]
  InvalidPatchType,

  #[error("The patch did not set a key")]
  NoKeySet,

  #[error("Tried to add the second of the same key to a set")]
  DuplicateKey,

  #[error("Could not find the field with the given name")]
  FieldNotFound,

  #[error("Error (de)serializing the patch: {0}")]
  SerializationError(String),
}

impl serde::ser::Error for ProteanError {
  fn custom<T: Display>(msg: T) -> Self {
    ProteanError::SerializationError(msg.to_string())
  }
}

impl From<serde_json::Error> for ProteanError {
  fn from(err: serde_json::Error) -> ProteanError {
    ProteanError::SerializationError(err.to_string())
  }
}
