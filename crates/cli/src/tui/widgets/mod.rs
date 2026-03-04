// Custom widgets for the TUI

pub mod dashboard;
pub mod output_panel;
pub mod repl_input;
pub mod status_bar;
pub mod theme_switcher;
pub mod help_modal;
pub mod quit_modal;
pub mod about_modal;

pub use dashboard::Dashboard;
pub use output_panel::OutputPanel;
pub use repl_input::ReplInput;
pub use status_bar::StatusBar;
pub use theme_switcher::ThemeSwitcher;
pub use help_modal::HelpModal;
pub use quit_modal::QuitModal;
pub use about_modal::AboutModal;
