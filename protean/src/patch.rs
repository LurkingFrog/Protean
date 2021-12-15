//! A transferable set of transformations to update one structure to match another

use crate::local::*;

use serde::ser::{Error as SerdeError, SerializeMap, SerializeTuple};

/// A recursive patch designed to be applied to a given object
/// This is the root,
#[derive(Default, Debug, Serialize)]
pub struct Patch<'a> {
  /// A name that the patch is referenced by (usually the field name taken from Patchworthy)
  name: String,

  /// Version of the object represented, if available
  version: Option<String>,

  /// Settings for how the patch is handled. Settings are inherited if not configured
  options: Option<PatchOptions>,

  /// The actual operations done to transform the state into the desired result
  actions: PatchActions<'a>,
}

impl<'a> Patch<'a> {
  pub fn new(name: String) -> Patch<'a> {
    Patch {
      name,
      version: None,
      options: Some(PatchOptions::default()),
      actions: PatchActions::new(),
    }
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  /// Attempt to combine two patches, if there is no conflict.
  pub fn merge(&mut self, _patch: Patch) -> Result<(), ProteanError> {
    todo!("Work on merge")
  }

  /// Adds a patch to a child field
  pub fn add<T>(
    &mut self,
    action: Action,
    field: T,
    expected: Option<u64>,
  ) -> Result<(), ProteanError>
  where
    T: Patchworthy<'a> + 'a,
  {
    let act = PatchAction::new(action, field, expected);
    self.actions.add(act)
  }
}

// impl<'a> Deserialize<'a> for Patch<'a> {}

/// Specific settings that modify how a patch is applied
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PatchOptions {
  /// Default is true. Inserts will automatically be tried as upserts.
  allow_upsert: bool,
}

#[derive(Default, Debug)]
pub struct PatchActions<'a>(HashMap<String, PatchAction<'a>>);

impl<'a> PatchActions<'a> {
  pub fn new() -> PatchActions<'a> {
    PatchActions(HashMap::new())
  }

  fn add(&mut self, action: PatchAction<'a>) -> Result<(), ProteanError> {
    let entry = self.0.entry(action.get_name());
    match &entry {
      Entry::Vacant(_) => entry.or_insert(action),
      Entry::Occupied(_) => return Err(ProteanError::DuplicateKey),
      // "The item already exists in the table: {}",
      // item
      // ))
    };
    Ok(())
  }
}

// Manually create a patch serializer since derive doesn't work easily
impl<'a> Serialize for PatchActions<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut state = serializer.serialize_map(Some(self.0.len()))?;
    for (k, v) in &self.0 {
      match &v.action {
        Action::Null => continue,
        Action::List(_act) => todo!("No list actions yet"),
        Action::Map(_act) => todo!("No map actions yet"),
        Action::Reset => todo!("No Default action yet"),
        Action::Clear => todo!("No Clear action yet"),
        Action::Set => state.serialize_entry(k, v)?,
      };
    }
    state.end()
  }
}

#[derive(Debug)]
pub struct PatchAction<'a> {
  action: Action,

  /// The value to use when performing an action
  value: PatchValue<'a>,

  /// A hash of the original PatchValue
  ///
  /// An optional state check to make sure the patch is being applied to a specific value
  expected: Option<u64>,
}

impl<'a> PatchAction<'a> {
  pub fn new(
    action: Action,
    value: impl Patchworthy<'a> + 'a,
    expected: Option<u64>,
  ) -> PatchAction<'a> {
    PatchAction {
      action,
      value: PatchValue::Value(Box::new(value)),
      expected,
    }
  }

  pub fn get_name(&self) -> String {
    self.value.get_field_name()
  }
}

// Manually create a patch serializer since derive doesn't work easily
impl<'a> Serialize for PatchAction<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let size = match self.expected.is_some() {
      true => 2,
      false => 3,
    };
    let mut state = serializer.serialize_tuple(size)?;
    match &self.action {
      Action::Null => {
        unreachable!("Should never be trying to directly serialize a Null patch action")
      }
      Action::List(_act) => todo!("No list actions yet"),
      Action::Map(_act) => todo!("No map actions yet"),
      Action::Reset => todo!("No Default action yet"),
      Action::Clear => todo!("No Clear action yet"),
      Action::Set => {
        state.serialize_element("Set")?;
        state.serialize_element(&self.value)?;
      }
    };

    state.end()
  }
}

#[derive(Debug)]
pub enum PatchValue<'a> {
  Value(Box<dyn Patchworthy<'a> + 'a>),
  Patch(Patch<'a>),
}

impl<'a> PatchValue<'a> {
  pub fn get_field_name(&self) -> String {
    match self {
      PatchValue::Value(val) => val.get_field_name(),
      PatchValue::Patch(patch) => patch.get_name(),
    }
  }
}

impl<'a> Serialize for PatchValue<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    match self {
      PatchValue::Value(val) => match val.as_json() {
        Ok(val) => serializer.serialize_newtype_variant("PatchValue", 0, "Value", &val),
        Err(err) => Err(S::Error::custom(err.to_string())),
      },
      PatchValue::Patch(patch) => {
        serializer.serialize_newtype_variant("PatchValue", 1, "Patch", &patch)
      }
    }
  }
}

/// Actions that a patch can perform against a given target based upon its type.
// TODO: I'm not sure the function option this is worthwhile, as a function cannot be sent via API
// Func(Box<FnMut()>)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Action {
  /// No action to be performed
  Null,

  /// Reset the value to its default value
  Reset,

  /// Sets the value to its version of null. Will throw an error if not configured for the field.
  ///
  /// Numbers - set to 0
  /// String - set to ""
  /// Object - Run clear on each field
  ///
  Clear,

  /// Update the value of the target to a new one as a whole
  Set,

  /// An ordered set of values of the same type
  List(ListAction),

  /// An unordered set of key/value pairs
  Map(MapAction),
}

/// Actions specific to an ordered set of values
#[derive(Clone, Debug, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MapAction {
  /// Insert the value for the given key
  Insert(String),

  /// Change the value for the existing key
  Update(String),

  /// Remove the key from the map
  Delete(String),
}
