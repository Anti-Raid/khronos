use crate::tui::theme::Theme;
use std::collections::VecDeque;
use tui_textarea::TextArea;

/// Maximum number of output lines to keep in history
const MAX_OUTPUT_LINES: usize = 10000;

/// Maximum number of command history entries
const MAX_COMMAND_HISTORY: usize = 1000;

/// Application state for the TUI
pub struct App<'a> {
    /// Current theme
    pub theme: Theme,

    /// REPL input text area
    pub input: TextArea<'a>,

    /// Output history
    pub output: VecDeque<String>,

    /// Command history
    pub command_history: Vec<String>,

    /// Current position in command history (for up/down navigation)
    pub history_index: Option<usize>,

    /// Whether the help panel is visible
    pub show_help: bool,

    /// Whether to auto-scroll output to bottom
    pub auto_scroll: bool,

    /// Output scroll position (0 = bottom)
    pub scroll_offset: usize,

    /// Current mode (REPL, Script, etc.)
    pub mode: AppMode,

    /// Whether the app should quit
    pub should_quit: bool,

    /// Performance metrics
    pub last_execution_time: Option<std::time::Duration>,

    /// Whether the theme switcher is visible
    pub show_theme_switcher: bool,

    /// Index of the currently selected theme in the switcher
    pub selected_theme_index: usize,

    /// Filter text for the theme switcher
    pub theme_filter: String,

    /// Whether the user has started interacting with the REPL
    pub is_interactive: bool,
}

/// Application mode
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    /// REPL mode
    Repl,
    /// Running a script
    Script,
    /// Idle
    Idle,
}

impl<'a> App<'a> {
    /// Create a new app with the default theme
    pub fn new() -> Self {
        let mut input = TextArea::default();
        input.set_placeholder_text("Ask Khronos or type /help...");

        let mut app = Self {
            theme: Theme::default(),
            input,
            output: VecDeque::new(),
            command_history: Vec::new(),
            history_index: None,
            show_help: false,
            auto_scroll: true,
            scroll_offset: 0,
            mode: AppMode::Idle,
            should_quit: false,
            last_execution_time: None,
            show_theme_switcher: false,
            selected_theme_index: 0,
            theme_filter: String::new(),
            is_interactive: false,
        };

        app.add_output("🚀 Khronos CLI Started. Type a command or press F1 for help.".to_string());
        app
    }



    /// Open theme switcher
    pub fn open_theme_switcher(&mut self) {
        self.show_theme_switcher = true;
        self.theme_filter.clear();
        self.selected_theme_index = Theme::all()
            .iter()
            .position(|t| t.name == self.theme.name)
            .unwrap_or(0);
    }

    /// Apply the selected theme and close switcher
    pub fn apply_selected_theme(&mut self) {
        let themes = self.filtered_themes();
        if let Some(theme) = themes.get(self.selected_theme_index) {
            self.theme = *theme;
            // self.add_output(format!("🎨 Theme applied: {}", self.theme.name));
        }
        self.show_theme_switcher = false;
    }

    /// List themes matching the filter
    pub fn filtered_themes(&self) -> Vec<Theme> {
        let all = Theme::all();
        if self.theme_filter.is_empty() {
            all
        } else {
            all.into_iter()
                .filter(|t| t.name.to_lowercase().contains(&self.theme_filter.to_lowercase()))
                .collect()
        }
    }

    /// Add a line to the output
    pub fn add_output(&mut self, line: String) {
        self.output.push_back(line);
        if self.output.len() > MAX_OUTPUT_LINES {
            self.output.pop_front();
        }
        if self.auto_scroll {
            self.scroll_offset = 0;
        }
    }


    /// Clear the output
    pub fn clear_output(&mut self) {
        self.output.clear();
        self.scroll_offset = 0;
        self.add_output("Output cleared.".to_string());
    }

    /// Get the current input text
    pub fn get_input(&self) -> String {
        self.input.lines().join("\n")
    }

    /// Clear the input
    pub fn clear_input(&mut self) {
        self.input = TextArea::default();
        self.input.set_placeholder_text("Enter Lua code here...");
    }

    /// Add current input to command history
    pub fn save_to_history(&mut self) {
        let input = self.get_input();
        if !input.trim().is_empty() {
            self.command_history.push(input);
            if self.command_history.len() > MAX_COMMAND_HISTORY {
                self.command_history.remove(0);
            }
        }
        self.history_index = None;
    }

    /// Navigate to previous command in history
    pub fn history_prev(&mut self) {
        if self.command_history.is_empty() {
            return;
        }

        let new_index = match self.history_index {
            None => Some(self.command_history.len() - 1),
            Some(0) => Some(0),
            Some(i) => Some(i - 1),
        };

        if let Some(idx) = new_index {
            self.history_index = Some(idx);
            let cmd = self.command_history[idx].clone();
            self.input = TextArea::from(cmd.lines().map(|s| s.to_string()));
        }
    }

    /// Navigate to next command in history
    pub fn history_next(&mut self) {
        if self.command_history.is_empty() {
            return;
        }

        let new_index = match self.history_index {
            None => None,
            Some(i) if i >= self.command_history.len() - 1 => {
                self.clear_input();
                None
            }
            Some(i) => Some(i + 1),
        };

        if let Some(idx) = new_index {
            self.history_index = Some(idx);
            let cmd = self.command_history[idx].clone();
            self.input = TextArea::from(cmd.lines().map(|s| s.to_string()));
        }
    }

    /// Toggle help panel
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    /// Scroll output up
    pub fn scroll_up(&mut self, amount: usize) {
        self.auto_scroll = false;
        self.scroll_offset = self.scroll_offset.saturating_add(amount);
        let max_scroll = self.output.len().saturating_sub(1);
        if self.scroll_offset > max_scroll {
            self.scroll_offset = max_scroll;
        }
    }

    /// Scroll output down
    pub fn scroll_down(&mut self, amount: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
        if self.scroll_offset == 0 {
            self.auto_scroll = true;
        }
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.auto_scroll = false;
        self.scroll_offset = self.output.len().saturating_sub(1);
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.auto_scroll = true;
        self.scroll_offset = 0;
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new()
    }
}
