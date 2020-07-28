//! All the tests for Strain and its derive functions
//!
//! This will get split up later, but for now this will define the the project

use std::sync::Once;

use strain::{patch, Patch, Patchwork, StrainError};

static LOGGING: Once = Once::new();

/// Set up that should be run for each ea
fn init_test() {
  LOGGING.call_once(|| env_logger::init())
}

macro_rules! test {
  (fn $name:ident ( $($arg:ident:$typ:ty),*) $body:expr) => {
    #[test]
    fn $name() {
      fn type_name_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
      }
      init_test();
      log::debug!("Starting to run test: {}", type_name_of($name));

      $body
    }
  };
}

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

test!(
  fn hello_world() {
    println!("Testing works");
  }
);

/// Tests
/// - test getters
/// - get value
/// - set value
/// - get diff

test!(
  fn test_getters() {}
);

test!(
  fn test_apply_patch() {
    init_test();

    // Create an default tester
    let mut tester = tools::Tester::default();
    log::debug!("The initial tester is:\n{:#?}", tester);

    assert_eq!(tester.integer, 0);
    let patch = patch!(tester, (("integer", 1)));
    assert_eq!(tester.integer, 1);

    // // Create a new patch
    // let mut patch = tools::Tester::new_patch();
    // let _ = patch.add("integer".to_string(), "1".to_string());
    // log::debug!("Testing the Patch:\n{}", patch);
  }
);

test!(
  fn test_diff() {
    init_test();
    // Fill a tester with random data

    // for each field, update a clone with a new random number and test that patch
  }
);
