//! Some preprocessor work for before running the tests
//!
//! Copied from Serde, as formatting the results of the macros seems like a good idea if its available
//! https://github.com/serde-rs/serde/blob/master/test_suite/build.rs

use std::process::{Command, ExitStatus, Stdio};
use toolchain_find;

fn has_cargo_expand() -> bool {
  let cargo_expand = if cfg!(windows) {
    "cargo-expand.exe"
  } else {
    "cargo-expand"
  };

  Command::new(cargo_expand)
    .arg("--version")
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .as_ref()
    .map(ExitStatus::success)
    .unwrap_or_else(
      |_| {
        println!(
          "cargo-expand is not installed. To run the expand based tests, install it with 'cargo install cargo-expand'"
        );
        false
      })
}

fn has_rustfmt() -> bool {
  match toolchain_find::find_installed_component("rustfmt").is_some() {
    true => true,
    false => {
      println!(
        "rustfmt is not installed. For prettier results, run 'rustup component add rustfmt'"
      );
      false
    }
  }
}

fn main() {
  if cfg!(feature = "expandtest") && has_cargo_expand() && has_rustfmt() {
    println!("cargo:rustc-cfg=expandtest");
  }
}
