//! The core Patchwork trait and implementations

use super::local::*;

/// The core trait,
pub trait Patchwork<'a>: Clone + Sized {
  /// An enumeration of each field and a wrapper for the value
  ///
  /// This makes it generic without having to serialize to generate a patch
  /// THINK: How does this work for enums?
  type Field: Patchworthy<'a>;

  /// Get an Id for the given object, if one is defined
  fn get_id() -> Option<String> {
    None
  }

  /// Gets the version of the object.
  ///
  /// This is useful for ensuring patches can be applied at the destination. Items can change based
  /// on the code, so an optional level of documentation exists within the patch. This is especially
  /// important within MicroServices. A future feature may add the ability to migrate a patch to
  /// different versions. Some use cases are:
  /// - A struct where a field was renamed
  /// - An enum with variants added
  fn get_version() -> Option<String> {
    None
  }

  /// Create an empty patch
  fn new_patch() -> Patch<'a> {
    Patch::new()
  }

  /// Apply a given patch
  fn apply(
    &mut self,
    patch: Patch,
    options: Option<PatchOptions>,
  ) -> Result<Patch<'a>, ProteanError> {
    todo!(
      "Default Apply for {} is not yet written",
      std::any::type_name::<Self>()
    )
  }

  /// Export the full structure as a patch
  ///
  /// This is the same way that most databases will backup their data as a set of inserts instead of
  /// making a custom format.
  fn as_patch(&'a self) -> Patch<'a> {
    let mut patch = Patch::new();
    for field in self.values() {
      patch.add(Action::Set, field, None).unwrap();
    }
    patch
  }

  /// Return a Patchworthy list containing the value of each field
  fn values(&'a self) -> Vec<Self::Field>;

  // Leave for later. This should be its own project and allow versioned patches to
  // migrate/ignore/force data to match the object being applied to
  // fn get_version() -> Option<ModelVersion> {
  //   ModelVersionstd::any::type_name::<T>();

  //   unimplemented!("{} does not have versioning enabled", name)
  // }
}

/// Annotation that tells patchwork it is an enumeration of a values
///
/// There are optional option classes that can be customized based on the field, which can modify
/// how the patch can operate. An example would be for a Vec, the index to apply the operation to
/// matters. They should be optional and not appear in the serialized data if possible.
pub trait Patchworthy<'a>: Send + Sync + Debug + Display {
  /// Get the and id number correcsonding to the given field.
  /// Since a patchworthy object cannot directly require hash, we make sure that the user implements
  fn get_type_id(&self) -> u64;

  /// Export the wrapped value as a Create PatchAction
  ///
  /// A function used to create a patch
  fn as_patch(&self) -> Patch {
    todo!(
      "as_create for {} is not yet written",
      std::any::type_name::<Self>()
    )
  }
}

// A customizable set of actions that can be performed on a field.
//
// The default is simple CRUD (minus the R). Items like maps and vectors require more nuance, so a
// user may want to introduce items such as Swap, Insert, and Reverse.
// pub trait PatchAction: Clone + Send + Sync + Debug + Display {}
