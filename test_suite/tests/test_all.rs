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

use protean::{patch, Patchwork};
use replicant::register;

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

  use replicant::{Registrar, ReplicantContainer};

  /// A struct with all the data types that Patchwork should know how to handle
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Tester {
    pub pk: uuid::Uuid,
    pub integer: i32,
    pub float: f32,
    pub string: String,
    pub nested: Nested,
    pub unkeyed: Unkeyed,
  }

  impl Default for Tester {
    fn default() -> Tester {
      Tester {
        pk: uuid::Uuid::new_v4(),
        integer: 0,
        float: 0.0,
        string: "".to_string(),
        nested: Default::default(),
        unkeyed: Default::default(),
      }
    }
  }

  impl Tester {
    pub fn random() -> Tester {
      let mut rng = rand::thread_rng();
      Tester {
        pk: uuid::Uuid::new_v4(),
        integer: rng.gen(),
        float: rng.gen(),
        string: {
          std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(15)
            .collect()
        },
        nested: Nested::random(),
        unkeyed: Unkeyed::random(),
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
        .merge("nested", self.nested.diff(&struct2.nested)?)?
        .merge("unkeyed", self.unkeyed.diff(&struct2.unkeyed)?)?;
      Ok(patch)
    }

    fn to_patch(&self) -> Result<Patch> {
      unimplemented!("'UnitTest Tester::to_patch' still needs to be implemented")
    }
  }
  impl<'a> ReplicantContainer<'a> for Tester {
    type Item = Registrar;
  }

  /// A second struct to be nested inside the Tester
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Nested {
    pk: uuid::Uuid,
    level_2: u8,
  }

  impl Nested {
    pub fn random() -> Nested {
      let mut rng = rand::thread_rng();
      Nested {
        pk: uuid::Uuid::new_v4(),
        level_2: rng.gen(),
      }
    }
  }

  impl Default for Nested {
    fn default() -> Nested {
      Nested {
        pk: uuid::Uuid::new_v4(),
        level_2: 0,
      }
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

  /// An unkeyed struct for testing some Replicant functionality
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Unkeyed {
    pk: uuid::Uuid,
    something: f64,
  }

  impl Unkeyed {
    pub fn random() -> Unkeyed {
      let mut rng = rand::thread_rng();
      Unkeyed {
        pk: uuid::Uuid::new_v4(),
        something: rng.gen(),
      }
    }
  }

  impl Default for Unkeyed {
    fn default() -> Unkeyed {
      Unkeyed {
        pk: uuid::Uuid::new_v4(),
        something: 0.0,
      }
    }
  }

  impl<'a> Patchwork<'a> for Unkeyed {
    fn diff(&self, unkeyed2: &Unkeyed) -> Result<Patch> {
      let patch = self
        .new_patch()
        .merge("something", self.something.diff(&unkeyed2.something)?)?;
      Ok(patch)
    }

    fn to_patch(&self) -> Result<Patch> {
      unimplemented!("'UnitTest Unkeyed::to_patch' still needs to be implemented")
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
    let _base = tools::Tester::default();

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

test!(
  fn test_replicant_full() {
    // Make sure stores stay in sync based on subscriptions
    use tools::*;

    // Make some random seed data
    let _tests: Vec<Tester> = [0..3].iter().map(|_| tools::Tester::random()).collect();

    test_replicant_register();
    // test_replicant_subscribe();
    // test_replicant_trigger();
    // Create some empty caches
    let _primary = replicant::Store::new();

    // Register Nested
    // Create a new root in

    // Register Tester
    // See that we create two nodes

    // Upsert tests into the primary cache
    // assert in primary
    // assert not in either of the others

    // Add subscription of primary from secondary
    // See that all items sync over tos
  }
);

test!(
  fn test_replicant_register() {
    // Options - Historic vs Patchwork?
    // Cannot add an Unkeyed item as a root
    // All Local keyed items create their own root
    let mut primary = replicant::Store::new();

    // Test the different methods of registering a Replicant
    // Use the default options
    // let result = register!(primary, tools::Tester);

    // Use custom options
    let opts = replicant::Registrar::new().unwrap();
    // let result = register!(primary, tools::Tester, opts);

    // Direct call
    let result = primary.register::<tools::Tester>(opts);

    log::debug!("result:\n{:#?}", result);

    // Register Unkeyed
    // Assert Error(UnkeyedReplicantError)
    // let result = primary.register::<tools::Tester>(ReplicantType);

    // Assert there is a root of Tester.
    // Assert that there is a guid for Tester

    // Assert there is a root for Nested
    // Assert there is a guid in the map for Nested

    // Make a dup Tester in a different module
    // Register it
    // Assert it has a root, different from the original

    // Register Nested
    // Assert nothing new happened
  }
);

test!(
  fn test_replicant_node() {
    // Check items that we need to store about each field
    // - Root Node Only
    //   - Number of patches applied (versioning). Root node only?
    //   - Historic instance for each root item?
    // - All Nodes
    //   - Vec<Subscriber_IDs>
    //   -
  }
);

test!(
  fn test_replicant_crud() {
    // CRUD for the store data, testing the resulting patches
    // THINK: Am I recreating a relational database wheel?
    // THINK: How to separate the data from the rules?
    // THINK: Delete vs. Retire
    // - Does caching
    // THINK: Can we separate data from subscriptions to store in memcached/redis?
    // -
    // THINK: Link counter like Rc?
    // - +1 on direct insert (this way it exists after all references are deleted)
    // - +1 each parent using
    // - Always cascade retire calls: -1 retire parent

    // Insert new Tested
    // Assert one tester node, one nested node
    // Insert Same tested - Error

    // Update previously inserted error

    // Retire value
    // How does this cascade?

    // Retire value (cache expire)
  }
);

test!(
  fn test_replicant_subscribe() {
    // THINK:
    // See data move between caches
    // Replicant to graphql format?
    // List subscribers
    // Tag watcher

    // FUTURE ITEMS
    // Update subscription
  }
);

test!(
  fn test_replicant_transform() {
    // THINK: How/Can we synchronize between different shapes of data?
    // - This is the core of choreography - take a sync event and convert it to an action/callback
  }
);

test!(
  fn test_replicant_trigger() {
    // Test patch update triggering sends to all subscribers
    // Test patch update triggers all matching callbacks
  }
);

test!(
  fn test_replicant_ack() {
    // Test all subcribers return and ack that they all applied a patch or an error
  }
);
