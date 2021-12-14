//! A transferable set of transformations to update one structure to match another

use crate::local::*;

/// A recursive patch designed to be applied to a given object
/// This is the root,
#[derive(Default, Debug)]
pub struct Patch<'a> {
  /// Version of the object represented, if available
  version: Option<String>,

  /// Settings for how the patch is handled. Settings are inherited if not configured
  options: Option<PatchOptions>,

  /// The actual operations done to transform the state into the desired result
  actions: HashMap<String, PatchAction<'a>>,
}

impl<'a> Patch<'a> {
  pub fn new() -> Patch<'a> {
    Patch {
      version: None,
      options: Some(PatchOptions::default()),
      actions: HashMap::new(),
    }
  }

  /// Attempt to combine two patches, if there is no conflict.
  pub fn merge(&mut self, patch: Patch) -> Result<(), ProteanError> {
    todo!("Work on merge")
  }

  /// Adds a patch to a child field
  pub fn add<T>(
    &mut self,
    action: Action,
    field: T,
    expected: Option<T>,
  ) -> Result<(), ProteanError>
  where
    T: Patchworthy<'a> + 'a,
  {
    let entry = self.actions.entry(field.get_field_name());
    println!("The Entry is: {:?}", entry);
    match &entry {
      Entry::Vacant(_) => {
        let act = PatchAction::new(
          action,
          field,
          expected.map(|x| Box::new(x) as Box<dyn Patchworthy<'a> + 'a>),
        );
        entry.or_insert(act);
      }
      Entry::Occupied(_) => return Err(ProteanError::DuplicateKey),
      // "The item already exists in the table: {}",
      // item
      // ))
    }
    Ok(())
  }
}

/// Specific settings that modify how a patch is applied
#[derive(Default, Debug)]
pub struct PatchOptions {
  /// Default is true. Inserts will automatically be tried as upserts.
  allow_upsert: bool,
}

#[derive(Debug)]
pub struct PatchAction<'a> {
  action: Action,

  /// The value to use when performing an action
  value: PatchValue<'a>,

  /// The expected value of the item before the patch is applied
  expected: Option<Box<dyn Patchworthy<'a> + 'a>>,
}

impl<'a> PatchAction<'a> {
  pub fn new(
    action: Action,
    value: impl Patchworthy<'a> + 'a,
    expected: Option<Box<dyn Patchworthy<'a> + 'a>>,
  ) -> PatchAction<'a> {
    PatchAction {
      action,
      value: PatchValue::Value(Box::new(value)),
      expected,
    }
  }
}

#[derive(Debug)]
pub enum PatchValue<'a> {
  Value(Box<dyn Patchworthy<'a> + 'a>),
  Patch(Patch<'a>),
}

/// Actions that a patch can perform against a given target based upon its type.
// TODO: I'm not sure if this is worthwhile, as a function cannot be sent via API
// Func(Box<FnMut()>)
#[derive(Clone, Debug)]
pub enum Action {
  /// No action to be performed
  Null,

  /// Reset the value to its default value
  Default,

  /// Sets the value to its version of null. Will throw an error if not configured for the field.
  ///
  /// Numbers - set to 0
  /// String - set to ""
  /// Object - Run clear on each field
  ///
  Clear,

  /// Update the value of the target to a new one as a whole
  Set,

  /// An organized data structure with name based access.
  Object,

  /// An ordered set of values of the same type
  List(ListAction),

  /// An unordered set of key/value pairs
  Map(MapAction),
}

/// Actions specific to an ordered set of values
#[derive(Clone, Debug)]
pub enum ListAction {
  /// Swap two values
  Swap(usize, usize),

  /// Delete the value, shifting the remaining items left
  Remove(usize),

  /// Add a new item before the given index, Shifting items right
  Insert(usize),

  /// Add the value(s) on to the end of the list
  Append(),
}

/// Actions specific to a set of key/value pairs
#[derive(Clone, Debug)]
pub enum MapAction {
  /// Insert the value for the given key
  Insert(String),

  /// Change the value for the existing key
  Update(String),

  /// Remove the key from the map
  Delete(String),
}
