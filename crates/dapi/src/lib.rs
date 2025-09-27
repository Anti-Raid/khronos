pub mod context;
pub mod controller;
pub mod serenity_backports;
pub mod antiraid_check_permissions;
pub mod antiraid_check_channel_permissions;
pub mod antiraid_get_fused_member;
pub mod types;
pub mod api;
pub mod multioption;

pub type Error = Box<dyn std::error::Error + Send + Sync>; // This is constant and should be copy pasted