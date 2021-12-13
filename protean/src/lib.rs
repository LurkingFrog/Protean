//! Code to diff and patch fields on a structure
//!
//! A tool able to efficiently identify and report differences between structures.
//!
//! ## Use Cases
//!
//! - **Reducing network load** By sending a patch instead of a full object every time there is a
//!   change, we can minimize the amount of data sent for each transaction
//! - **Has changed** Allows for more granular equality testing rather than "Yes"/"No". Changes on
//!   fields considered unimportant can be ignored.
//!
//! TODO:
//! - Make derive in Protean

/*

workbook - try_into<Patch<Db>>

*/

pub mod error;

pub mod impls;

pub mod patch;

pub mod traits;

mod local {
  pub use crate::prelude::*;

  pub use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    fmt::{Debug, Display},
    hash::Hash,
  };
}

pub mod prelude {
  pub use super::*;

  pub use error::ProteanError;
  pub use patch::{Action, Patch, PatchAction, PatchOptions};
  pub use traits::{Patchwork, Patchworthy};
}
