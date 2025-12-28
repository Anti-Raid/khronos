//! Single threaded khronos runtime struct/runner

pub mod runtime;

// Re-exports

pub use runtime::{KhronosRuntime, RuntimeCreateOpts};

// Re-export for convenience
pub use mluau;
pub use mluau as mlua;
pub use mlua_scheduler;
