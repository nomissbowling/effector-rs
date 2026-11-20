//! effector
//!
//! Cargo test with [-- --nocapture] or with [-- --show-output]
//!
//! cargo test --bin effector --release -- --nocapture
//! cargo test --bin effector --release -- --show-output
//! cargo run --bin effector --release
//! cargo build --bin effector --release
//!

use effector::core::*;

use std::{result::Result, error::Error};

/// main
pub fn main() -> Result<(), Box<dyn Error>> {
  Ok(())
}

/// effector test
pub fn effector_test() -> Result<(), Box<dyn Error>> {
  assert_eq!(wx_test("test effector test")?, ()); // same as lib
  Ok(())
}

/// test with [-- --nocapture] or [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use effector::core::*; // lib
  use crate::*; // main

  /// test effector
  #[test]
  fn test_effector() -> Result<(), Box<dyn Error>> {
    assert_eq!(wx_test("test effector")?, ()); // same as lib
    assert_eq!(effector_test()?, ());
    Ok(())
  }
}
