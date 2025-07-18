//! To avoid relying on AntiRaid settings here, provide our own cut-down IR of it here.
//!
//! We also use a second IR in the API itself. While this may seem like duplicated code, it allows for more flexibility

/// DataStore IR
pub mod datastores;
/// KV IR
pub mod kv;
/// Object Storage IR
pub mod objectstorage;

pub use datastores::*;
pub use kv::*;
pub use objectstorage::*;
