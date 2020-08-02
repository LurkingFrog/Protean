//! All the tests for Protean and its derive functions
//!
//! This will get split up later, but for now this will define the the project
//!
//! # Milestone 1 (What is Done)
//!
//! Patchwork only!!
//! - Document Architecture (Book?)
//! - Document use cases
//!   - Minimal option messages in The Process Foundry (premature network optimization)
//!   - Partial update subscription (Similar to Apollo GraphQL, but for Postgres)
//!
//! After Milestone reached
//! - Code review from ADHD_Devs
//! - Include comments/PRs
//! - First, have finished first draft of personal landing page
//! - Ask for Code review on Rust
//! - Publish Crate

use std::sync::Once;
static LOGGING: Once = Once::new();

use protean::{patch, Patchwork, ProteanError};

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
  use protean::{Patch, Patchwork};
  use rand::distributions::Alphanumeric;
  use rand::Rng;
  use serde::{Deserialize, Serialize};

  /// A struct with all the data types that Patchwork should know how to handle
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Tester {
    pub integer: i32,
    pub float: f32,
    pub string: String,
    pub nested: Nested,
  }

  impl Default for Tester {
    fn default() -> Tester {
      Tester {
        integer: 0,
        float: 0.0,
        string: "".to_string(),
        nested: Default::default(),
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
        nested: Nested::random(),
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
        .merge("string", self.string.diff(&struct2.string)?)?
        .merge("nested", self.nested.diff(&struct2.nested)?)?;
      Ok(patch)
    }

    fn to_patch(&self) -> Result<Patch> {
      unimplemented!("'UnitTest Tester::to_patch' still needs to be implemented")
    }
  }

  /// A second struct to be nested inside the Tester
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Nested {
    level_2: u8,
  }

  impl Nested {
    pub fn random() -> Nested {
      let mut rng = rand::thread_rng();
      Nested { level_2: rng.gen() }
    }
  }

  impl Default for Nested {
    fn default() -> Nested {
      Nested { level_2: 0 }
    }
  }

  impl<'a> Patchwork<'a> for Nested {
    fn diff(&self, nested2: &Nested) -> Result<Patch> {
      let patch = self
        .new_patch()
        .merge("level_2", self.level_2.diff(&nested2.level_2)?)?;
      Ok(patch)
    }

    fn to_patch(&self) -> Result<Patch> {
      unimplemented!("'UnitTest Nested::to_patch' still needs to be implemented")
    }
  }
}

test!(
  fn test_apply_patch() {
    // Create an default tester
    let tester = tools::Tester::default();
    log::debug!("The initial tester is:\n{:#?}", tester);

    assert_eq!(tester.integer, 0);
    let _patch = patch!(tester, (("integer", 1)));
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
    assert!(patch.unwrap().is_empty(), true);

    let test2 = tools::Tester::random();
    log::debug!("Test2: {:#?}", test2);

    let patch = test1.diff(&test2);
    log::debug!("Test 2:\n{:#?}", patch);
  }
);

// Make sure we can apply a patch to a given struct
test!(
  fn test_apply() {
    // Create a default tester
    let base = tools::Tester::default();

    // Fill a tester with random data
    let random = tools::Tester::random();
    log::debug!("Test1: {:#?}", random);

    // Get an error from trying to apply a patch from the wrong type

    // match
  }
);

test!(
  fn test_vec() {
    // Vectors and arrays are going to have order changes and we want to make sure they are handled properly
  }
);

test!(
  fn test_hash() {
    // Vectors and arrays are going to have order changes and we want to make sure they are handled properly
  }
);
