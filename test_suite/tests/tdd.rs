//! Tests targeted against current development

mod common;

use common::test_fn;

test_fn!(
  fn import_to() {
    use crate::common::database::*;
    use protean::prelude::*;

    let db = Db::new();

    // Full export for an empty db
    let patch: Patch = db.as_patch();

    println!("{:#?}", patch);
  }
);

/*

Update:

{
  // Generated from
  id: Uuid
  type: Db
  version: "0.1.0"
}

*/
