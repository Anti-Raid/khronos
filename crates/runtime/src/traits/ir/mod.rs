//! To avoid relying on AntiRaid settings here, provide our own cut-down IR of it here.
//!
//! We also use a second IR in the API itself. While this may seem like duplicated code, it allows for more flexibility

/// Object Storage IR
pub mod objectstorage;
/// Runtime IR
pub mod runtime;

pub use objectstorage::*;