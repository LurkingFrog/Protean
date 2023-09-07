//! Common functions used for all the tests

use std::sync::Once;
static LOGGING: Once = Once::new();

/// Individual test setup
pub fn init_test() {
  LOGGING.call_once(env_logger::init)
}

/// Simple wrapper to initialize each test with logging to the screen
macro_rules! test_fn {
  (fn $name:ident () $body:expr) => {
    #[test]
    fn $name() {
      fn type_name_of<T>(_: T) -> &'static str {
        std::any::type_name::<T>()
      }

      crate::common::init_test();
      log::debug!("\n\n\n\n\t\t\tStarting to run test: {}\n\t<------------------------------------------------------------------------------------->\n", type_name_of($name));

      $body
    }
  };
}
pub(crate) use test_fn;

pub(crate) mod local {
  pub use protean::prelude::*;

  pub use rand::{distributions::Alphanumeric, prelude::*};
  pub use serde::{Deserialize, Serialize};
  pub use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
  };
  pub use uuid::Uuid;
}

/*
pub(crate) mod tester {
  use super::local::*;

  // pub use super::{Nested, Unkeyed};

  #[derive(Debug, Clone)]
  pub struct Tester {
    pub pk: Uuid,
    pub integer: i32,
    pub float: f32,
    pub string: String,
    // pub nested: Nested,
    // pub unkeyed: Unkeyed,
  }

  pub enum _TesterFields {
    Pk(Uuid),
    Integer(i32),
    Float(f32),
    String(String),
  }

  impl Default for Tester {
    fn default() -> Tester {
      Tester {
        pk: uuid::Uuid::new_v4(),
        integer: 0,
        float: 0.0,
        string: "".to_string(),
        // nested: Default::default(),
        // unkeyed: Default::default(),
      }
    }
  }

  impl Tester {
    pub fn _random() -> Tester {
      let mut rng = rand::thread_rng();
      Tester {
        pk: uuid::Uuid::new_v4(),
        integer: rng.gen(),
        float: rng.gen(),
        string: {
          std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(15)
            .map(char::from)
            .collect()
        },
        // nested: Nested::random(),
        // unkeyed: Unkeyed::random(),
      }
    }
  }
}
*/

pub(crate) mod database {
  use super::local;

  mod db {
    pub use super::{address::Address, local::*, organization::Organization};

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Db {
      pub organizations: HashMap<Uuid, Organization>,
      pub addresses: HashMap<Uuid, Address>,
    }

    impl Db {
      pub fn new() -> Db {
        Db::default()
      }
    }

    impl<'a> Patchwork<'a> for Db {
      type Accessor = ();
      type Element = DbField<'a>;

      /// Get an Id for the given object
      fn get_id() -> Option<String> {
        Some("TestDb".to_string())
      }

      fn values(&'a self) -> Vec<Self::Child> {
        vec![
          DbField::Organizations(&self.organizations),
          DbField::Addresses(&self.addresses),
        ]
      }

      // fn get_child(&'a self, name: &str) -> Result<Self::Child, ProteanError> {
      //   Ok(match name {
      //     "addresses" => DbField::Addresses(&self.addresses),
      //     "organizations" => DbField::Organizations(&self.organizations),
      //     _ => return Err(ProteanError::FieldNotFound),
      //   })
      // }
    }

    #[derive(Debug, Clone, Serialize)]
    pub enum DbField<'a> {
      Organizations(&'a HashMap<Uuid, Organization>),
      Addresses(&'a HashMap<Uuid, Address>),
    }

    impl<'a> Patchworthy<'a> for DbField<'a> {
      // fn get_child_name(&self) -> String {
      //   match self {
      //     DbField::Organizations(_) => "Organizations",
      //     DbField::Addresses(_) => "Addresses",
      //   }
      //   .to_string()
      // }

      fn as_json(&self) -> Result<serde_json::Value, ProteanError> {
        Ok(match self {
          DbField::Addresses(addrs) => serde_json::to_value(addrs),
          DbField::Organizations(orgs) => serde_json::to_value(orgs),
        }?)
      }
    }

    impl<'a> Hash for DbField<'a> {
      fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
          DbField::Organizations(_) => 0.hash(state),
          DbField::Addresses(_) => 1.hash(state),
        }
      }
    }

    impl<'a> Display for DbField<'a> {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
      }
    }
  }

  mod organization {
    use super::local::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Organization {
      pub org_id: Uuid,
      pub name: String,
    }

    impl Organization {
      pub fn new(name: String) -> Organization {
        Organization {
          org_id: Uuid::new_v4(),
          name,
        }
      }
    }

    impl Hash for Organization {
      fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.org_id.hash(state);
      }
    }
  }

  mod address {
    use super::local::*;

    #[derive(Debug, Default, Clone, Serialize, Deserialize)]
    pub struct Address {
      pub addr_id: Uuid,
      pub line1: String,
    }

    impl Address {
      pub fn new(line1: String) -> Address {
        Address {
          addr_id: Uuid::new_v4(),
          line1,
        }
      }
    }

    impl Hash for Address {
      fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.addr_id.hash(state);
      }
    }
  }

  pub use address::Address;
  pub use db::Db;
  pub use organization::Organization;
}
