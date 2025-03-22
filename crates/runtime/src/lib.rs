pub mod plugins;
pub mod primitives;
pub mod rt;
pub mod traits;
pub mod utils;

pub use primitives::context::{TemplateContext, TemplateContextRef};

pub type Error = Box<dyn std::error::Error + Send + Sync>; // This is constant and should be copy pasted
