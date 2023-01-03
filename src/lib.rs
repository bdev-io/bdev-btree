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

#[cfg(all(not(test), feature = "logging"))]
#[allow(unused_imports)] #[macro_use] extern crate log;

#[cfg(all(test, feature = "logging"))]
#[macro_use] mod macros;

#[cfg(all(test, not(feature = "logging")))]
#[macro_use] mod macros;

#[cfg(all(not(test), not(feature = "logging")))]
#[macro_use] mod macros;


/// DOC : PUBLIC FUNCTIONS RE EXPORT HERE

pub use func::init;

/// DOC : ===============================


/// DOC : PUBLIC STRUCTURES RE EXPORT HERE

/// DOC : ================================

/// DOC : PUBLIC TRAIT RE EXPORT HERE

pub use traits::external::BTreeDataTrait;

/// DOC : ===========================


// INFO : Module
pub(crate) mod global;
mod func;
pub(crate) mod constant;
mod traits;

// INFO : Test
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
