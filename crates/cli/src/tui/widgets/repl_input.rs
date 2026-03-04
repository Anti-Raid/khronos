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

        // Render suggestion overlay
        if let Some(suggestion) = self.app.get_active_suggestion() {
            let input = self.app.get_input();
            let trimmed_input = input.trim();
            
            // Calculate where to draw the suggestion
            // This is tricky with TUI-textarea but for single line we can estimate.
            // We assume single line input for commands.
            
            let cursor_x = area.x + 1 + (trimmed_input.len() as u16); 
            // Note: This simple calculation assumes no scrolling and simple layout.
            // For a robust solution, we'd need to know the cursor position from textarea, 
            // but tui-textarea encapsulates that.
            // However, since we are auto-completing potentially empty prefixes (/help vs /h),
            // we should render the *rest* of the command.
            
            let suggestion_text = &suggestion[trimmed_input.len()..];
            
            if !suggestion_text.is_empty() && cursor_x < area.x + area.width {
                 let ghost_text = ratatui::text::Span::styled(
                    suggestion_text, 
                    Style::default().fg(theme.muted_foreground).add_modifier(ratatui::style::Modifier::ITALIC)
                );
                
                // Render ghost text right after the input
                buf.set_string(cursor_x, area.y + 1, ghost_text.content, ghost_text.style);

                // Render "Tab to complete" hint on the right
                let hint = "Tab to complete";
                let hint_len = hint.len() as u16;
                // Ensure we have space
                if area.width > hint_len + 5 {
                     let hint_x = area.x + area.width - hint_len - 1;
                     let hint_span = ratatui::text::Span::styled(
                        hint, 
                        Style::default().fg(theme.primary).add_modifier(ratatui::style::Modifier::BOLD)
                    );
                    buf.set_string(hint_x, area.y + 1, hint_span.content, hint_span.style);
                }
            }
        }
    }
}
