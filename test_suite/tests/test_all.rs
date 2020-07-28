//! All the tests for Strain and its derive functions
//!
//! This will get split up later, but for now this will define the the project

use std::sync::Once;
static LOGGING: Once = Once::new();

use strain::{patch, Patchwork, StrainError};

/// Set up that should be run for each ea
fn init_test() {
  LOGGING.call_once(|| env_logger::init())
}

macro_rules! test {
  (fn $name:ident () $body:expr) => {
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
  use anyhow::Result;
  use rand::distributions::Alphanumeric;
  use rand::Rng;
  use serde::{Deserialize, Serialize};
  use strain::{Patch, Patchwork};

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

  impl Tester {
    pub fn random() -> Tester {
      let mut rng = rand::thread_rng();
      Tester {
        integer: rng.gen(),
        float: rng.gen(),
        string: {
          std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(15)
            .collect()
        },
      }
    }
  }

  // TODO: Convert this to a macro
  impl<'a> Patchwork<'a> for Tester {
    fn diff(&self, struct2: &Tester) -> Result<Patch> {
      let patch = self
        .new_patch()
        .merge("integer", self.integer.diff(&struct2.integer)?)?
        .merge("float", self.float.diff(&struct2.float)?)?
        .merge("string", self.string.diff(&struct2.string)?)?;
      Ok(patch)
    }
  }
}

test!(
  fn test_getters() {}
);

test!(
  fn test_apply_patch() {
    // Create an default tester
    let tester = tools::Tester::default();
    log::debug!("The initial tester is:\n{:#?}", tester);

    assert_eq!(tester.integer, 0);
    // let _patch = patch!(tester, (("integer", 1)));
    assert_eq!(tester.integer, 1);

    // // Create a new patch
    // let mut patch = tools::Tester::new_patch();
    // let _ = patch.add("integer".to_string(), "1".to_string());
    // log::debug!("Testing the Patch:\n{}", patch);
  }
);

test!(
  fn test_diff() {
    // Fill a tester with random data
    let test1 = tools::Tester::random();
    log::debug!("Test1: {:#?}", test1);

    // When compared to itself, a should return an empty patch
    let patch = test1.diff(&test1);
    log::debug!("Self Test:\n{:#?}", patch);

    let test2 = tools::Tester::random();
    log::debug!("Test2: {:#?}", test2);

    let patch = test1.diff(&test2);
    log::debug!("Test 2:\n{:#?}", patch);
    // for each field, update a clone with a new random integer and test that patch
  }
);

test!(
  fn test_primitives() {
    // Just so the Tester doesn't have to enumerate every combination, we want to validate all
    // primitives that have been implemented here
    // Docstring tests?
  }
);

test!(
  fn test_vec() {
    // Vectors and arrays are going to have order changes and we want to make sure they are handled properly
  }
);
