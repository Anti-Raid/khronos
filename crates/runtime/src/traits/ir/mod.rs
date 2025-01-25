//! To avoid relying on AntiRaid settings here, provide our own cut-down IR of it here.
//!
//! We also use a second IR in the API itself. While this may seem like duplicated code, it allows for more flexibility

/// KV IR
pub mod kv;
/// Lockdowns IR
pub mod lockdowns;
/// Settings IR
pub mod settings_ir;

pub use kv::*;
pub use lockdowns::*;
pub use settings_ir::*;
