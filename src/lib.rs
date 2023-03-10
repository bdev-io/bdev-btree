//! # Hackable B-Tree Package
//!
//! This package provides a B-Tree implementation that can be used as a library.
//! It is not a general purpose On-Memory B-Tree implementation,
//! but a B-Tree implementation that is optimized for the use case of
//! the [Hackable B-Tree](https://github.com/bdev-io/btree) project.
//!
//! ## Features
//!
//! IF `logging` Feature is not enabled, all logging will be disabled -> no overhead.
//!
//! - [x] On-Disk Optimize
//! - [x] Asynchronous B-Tree
//! - [x] B-Tree with a controllable degree
//! - [x] Translated to
//! - [x] Enable Logging Feature
//!
//! ## Must Do
//! 
//! You Must Implement [`BTreeGeneralTypeTrait`] for your key/data structure.
//! Look at [`BTreeGeneralTypeTrait`] for more information.
//!
//! [`BTreeGeneralTypeTrait`]: crate::traits::external::BTreeGeneralTypeTrait
//! 
//! To Start Using This Package, Using [`init`] functions.
//!
//! ## B-Tree Data-Structure
//! Look at [`BTree`] for more information
//!
//! [`BTree`]: crate::tree::BTree
//! [`init`]: crate::func::init

// DOC : CONDITIONAL MACROS
  #[cfg(all(not(test), feature = "logging"))]
  #[allow(unused_imports)] #[macro_use] extern crate log;

  #[cfg(all(test, feature = "logging"))]
  #[macro_use] mod macros;

  #[cfg(all(test, not(feature = "logging")))]
  #[macro_use] mod macros;

  #[cfg(all(not(test), not(feature = "logging")))]
  #[macro_use] mod macros;

// ==========================


// DOC : RE-EXPORTS

  pub use func::init;

  pub use traits::external::BTreeGeneralTypeTrait;

// ================






// DOC : MODULES
  pub(crate) mod global;
  mod func;
  pub(crate) mod constant;
  mod traits;
  mod tree;
// ==============





#[cfg(test)]
mod tests {
  use super::*;

  // HACK : This is a hack to make sure test success.
  #[allow(clippy::assertions_on_constants)]
  #[tokio::test(flavor = "current_thread")]
  async fn test_logging() {
    #[cfg(feature = "logging")]
    {
      info!("Logging is enabled");
    }
    #[cfg(not(feature = "logging"))]
    {
      info!("Logging is disabled");
    }
    assert!(true);
  }
}

// TODO : Trait Unit Testing
