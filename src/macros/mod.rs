//! ## HACK : Hacky way to enable logging

#[cfg(all(test, feature = "logging"))]
#[macro_use] mod test_log;

#[cfg(all(test, not(feature = "logging")))]
#[macro_use] mod disabled_log;

#[cfg(all(not(test), not(feature = "logging")))]
#[macro_use] mod disabled_log;
