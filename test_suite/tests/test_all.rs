//! All the tests for Strain and its derive functions
//!
//! This will get split up later, but for now this will define the the project

use strain::{Patch, StrainError};

mod tools {
  use strain::Patchwork;
  #[derive(Debug, Clone)]
  pub struct Tester {
    integer: i32,
    float: f32,
    string: String,
  }

  impl Patchwork for Tester {}
}

#[test]
fn test_hello_world() {
  println!("Testing works");
}

#[test]
fn test_diff() {
  // Fill a tester with random data

  // for each field, update a clone with a new random number and test that patch
}
