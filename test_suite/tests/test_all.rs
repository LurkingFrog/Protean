//! All the tests for Strain and its derive functions
//!
//! This will get split up later, but for now this will define the the project

use strain::{Patch, Patchwork, StrainError};

mod tools {
  use serde::{Deserialize, Serialize};
  use strain::Patchwork;

  /// A struct with all the data types that Patchwork should know how to handle
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Tester {
    pub integer: i32,
    pub float: f32,
    pub string: String,
  }

  impl Default for Tester {
    fn default() -> Tester {
      Tester {
        integer: 0,
        float: 0.0,
        string: "".to_string(),
      }
    }
  }

  // TODO: Convert this to a macro
  impl<'a> Patchwork<'a> for Tester {}
}

#[test]
fn test_hello_world() {
  println!("Testing works");
}

#[test]
fn test_apply_patch() {
  // Ugly to put the env logger init here, but there doesn't seem to be much other option
  env_logger::init();
  let mut patch = tools::Tester::new_patch();
  let _ = patch.add("integer".to_string(), "1".to_string());
  log::debug!("Testing the Patch:\n{}", patch)
}

#[test]
fn test_diff() {
  // Fill a tester with random data

  // for each field, update a clone with a new random number and test that patch
}
