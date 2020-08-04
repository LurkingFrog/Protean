//! Become a node in a replicant cluster
//!
//! This is a project zygote. Keep all the info in memory with patch history. All queries/mutations
//! will flow through this. Diesel will subscribe and should never be accessed directly.
//!
//! # Key Ideas
//! - All data is kept in memory encoded into serde_json::Value. As all items are converted to json for
//!   transport, it doesn't add much cpu cost.
//! - Subscriptions are done with graphql. Why invent the wheel? This gives a spec and a set of introspection
//!   tools
//! -
//! # Think
//!
//! - Push registration: a secondary should be able to register an item in the cache remotely if it doesn't
//!   exist. This can let us make a stand alone replicant cache backed by memcached which does not need
//!   to know any of the object shapes.
//! - Extra filters on alerts (eg. only if change > amount)
//! - How to do batches
//! - How to handle race conditions when two publishers have different changes to the same u object
//! - Distributed - make multiple with each being primary for a subset of data
//! - Add a patchwork on_change_match: call function if it receives a change

use anyhow::{Context, Result};
use protean::Patchwork;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

pub mod error;
pub use error::ReplicantError;

#[macro_export]
macro_rules! register {
  () => {
    unimplemented!("'replicant::register!' still needs to be implemented")
  };
}

/// A struct that can be decomposed into a serialized cache
///
/// TODO: Functionality
/// - Rc like subscription pointers? Delete cache once all subscriptions to it are gone
/// - Subscription without explicit Replicant? Have it keyed to listen to a patch but
///   have it be ephemeral - do a callback on a patch but do not store
///
pub trait Replicant<'a>: Patchwork<'a> {
  /// Get a guid for this type. This should be generated repeatable hash, no matter which system it is
  /// compiled on, it should generate the same key.
  fn get_type_id() -> uuid::Uuid {
    // MD5 hash of
    // env!("CARGO_PKG_NAME");
    // env!("CARGO_PKG_VERSION");
    // get_type_name
    unimplemented!("Generic 'Replicant::get_type_id' still needs to be implemented")
  }

  /// Define whether the object implements
  fn key_type() -> KeyType {
    KeyType::Unkeyed
  }

  fn get_key() -> Result<uuid::Uuid> {
    unimplemented!("Generic 'Replicant::get_key' still needs to be implemented")
  }

  /// Get a pretty name for the replicant type.
  ///
  /// THINK: Should the name include the version number from the source code if available?
  /// This can be hardcoded in the derive macro

  fn get_type_name(&self) -> String {
    unimplemented!("Generic 'Replicant::get_type_name' still needs to be implemented")
  }
  // fn upsert(&mut self, cache_root: &str, patch: Patch) -> Result<()>;
}

// Placeholder magic for registering Replicants using their static methods
// THINK: Understand https://stackoverflow.com/questions/40252935/how-to-provide-type-only-argument-to-a-function
//  TODO: Clean this and convert it to a macro tied to Replicant

/// A placeholder trait used to register an struct that implements Replicant with a store
pub trait ReplicantContainer<'a> {
  type Item: Replicant<'a>;
}

/// The options for setting up the cache for the given replicant
#[derive(
  Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct Registrar {}
impl<'a> Patchwork<'a> for Registrar {
  fn diff(&self, _: &Registrar) -> Result<protean::Patch> {
    unreachable!("'ReplicantRegistrar::diff' is never used")
  }
}
impl<'a> Replicant<'a> for Registrar {}
impl Registrar {
  pub fn new() -> Result<Registrar> {
    Ok(Default::default())
  }
}

/// This is where we store all the local data.
///
/// THINK: How to add historic to this
/// THINK: Add Arc<Mutex>?

#[derive(Debug, Default)]
pub struct Store {
  /// The data store.
  /// HACK: This should likely be converted a trait so we can use memcached as a backend if desired
  cache: HashMap<uuid::Uuid, serde_json::Value>,

  ///
  root_map: HashMap<String, uuid::Uuid>,
}

impl std::fmt::Display for Store {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self.root_map.keys())
  }
}

impl Store {
  pub fn new() -> Store {
    Default::default()
  }

  pub fn register<'a, C: ReplicantContainer<'a>>(&mut self, item: C::Item) -> Result<&Store> {
    log::debug!("Registering an item to the store: {:#?}", item);

    match C::Item::key_type() {
      KeyType::Unkeyed => Err(ReplicantError::UnkeyedReplicantError).context(format!(
        "Could not register unkeyed replicant {:#?}",
        item.get_type_name()
      ))?,
      KeyType::Local => Ok(self),
      KeyType::Global => Ok(self),
    }
  }
}

// /// A mapping of where/how to send all the patches
// pub struct Subscriptions {
//   subscribers: HashMap<String, Rc<dyn Subscriber>>,
// }

// impl std::fmt::Display for Subscriptions {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "Subscribers {:#?}", self.subscribers.keys())
//   }
// }

// impl Subscriptions {
//   fn subscribe(&self, name: String, subscriber: Rc<dyn Subscriber>) -> Result<()> {
//     unimplemented!("'Subscriptions::subscribe' still needs to be implemented")
//   }
// }

/// Define if the replicant can be identified by a key
///
/// This defaults to none.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum KeyType {
  /// Does not have a key at all
  ///
  /// Items like primitives, Vec, and HashMap are generally repeatable everywhere.
  Unkeyed,

  /// A consistent key can be come up with from the Replicant
  ///
  /// HACK: Needs a much better description
  /// Local means that two items with the same key may or may not refer to the same type of object. It is
  /// on the user to make sure data is not overwritten.
  ///
  /// Having Address is a good example. It can be hashed into a unique ID, but the context of the key
  /// matters since being changed in one context doesn't mean it should change in all of them.
  Local,

  /// The key used is globally unique".to_string()
  ///
  /// Any time this is seen, we can assume that it referring to the same item
  Global,
}

impl Default for KeyType {
  fn default() -> KeyType {
    KeyType::Unkeyed
  }
}

/// Standard mess of boolean algebra for filtering
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FilterType {
  None,
  Fields(HashSet<String>),
}

impl Default for FilterType {
  fn default() -> FilterType {
    unimplemented!("Default is not implemented for FilterType")
  }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Filter {}

impl std::fmt::Display for Filter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl Filter {}

/// Define the options for the subscription
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct Subscription {}

impl std::fmt::Display for Subscription {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl Subscription {}
