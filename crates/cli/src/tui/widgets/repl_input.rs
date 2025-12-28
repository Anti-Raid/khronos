use crate::tui::app::App;
use ratatui::{buffer::Buffer, layout::Rect, style::Style, widgets::Widget};

/// REPL input widget (wraps tui-textarea)
pub struct ReplInput<'a> {
    app: &'a App<'a>,
}

impl<'a> ReplInput<'a> {
    pub fn new(app: &'a App<'a>) -> Self {
        Self { app }
    }
}

impl<'a> Widget for ReplInput<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.theme;

        // Clone the textarea and apply enhanced theme
        let mut textarea = self.app.input.clone();
        
        // Set styles based on theme with enhanced visuals
        textarea.set_cursor_line_style(
            Style::default()
                .bg(theme.accent)
                .add_modifier(ratatui::style::Modifier::BOLD)
        );
        textarea.set_cursor_style(
            Style::default()
                .bg(theme.primary)
                .fg(theme.primary_foreground)
                .add_modifier(ratatui::style::Modifier::REVERSED)
        );
        textarea.set_style(Style::default().fg(theme.foreground).bg(theme.background));
        
        let prompt = if self.app.is_interactive { "> " } else { "  " };

        let block = if !self.app.is_interactive {
            // Dashboard Style: Search bar look
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(theme.primary))
                .style(Style::default().bg(theme.background))
                // .title(" Ask anything... or type /help ") // Removed for cleaner look
        } else {
            // REPL Style: Minimal top separator
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::TOP)
                .border_style(Style::default().fg(theme.border))
                .style(Style::default().bg(theme.background))
                .title(ratatui::text::Span::styled(
                    format!(" {} ", prompt),
                    Style::default()
                        .fg(theme.primary)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ))
        };

        textarea.set_block(block);

        textarea.render(area, buf);
    }
}
