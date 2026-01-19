mod serde;
#[cfg(feature = "async")]
pub(crate) mod test_framework;

#[cfg(feature = "async")]
mod async_tests;
mod edge_cases;

// Sync and async is the same now
// #[cfg(feature = "sync")]
// mod blocking_tests;
