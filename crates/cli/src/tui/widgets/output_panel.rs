use crate::tui::app::App;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};

/// Output panel widget showing command output with scrolling
pub struct OutputPanel<'a> {
    app: &'a App<'a>,
}

impl<'a> OutputPanel<'a> {
    pub fn new(app: &'a App<'a>) -> Self {
        Self { app }
    }

    fn get_visible_lines(&self, height: usize) -> Vec<String> {
        let total_lines = self.app.output.len();
        if total_lines == 0 {
            return vec![];
        }

        let start_idx = total_lines.saturating_sub(height + self.app.scroll_offset);
        let end_idx = total_lines.saturating_sub(self.app.scroll_offset);

        self.app
            .output
            .iter()
            .skip(start_idx)
            .take(end_idx - start_idx)
            .cloned()
            .collect()
    }
}

impl<'a> Widget for OutputPanel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.theme;

        // Create block with minimal styling or just a basic container
        // To match OpenCode, we remove heavy borders. Maybe just padding?
        let block = Block::default()
            .style(Style::default().bg(theme.background));

        let inner = block.inner(area);
        block.render(area, buf);

        if self.app.output.is_empty() {
            // Show welcome ASCII art if no output
            let ascii = vec![
                "",
                "  _  ___                                ",
                " | |/ / |__  _ __ ___  _ __   ___  ___ ",
                " | ' /| '_ \\| '__/ _ \\| '_ \\ / _ \\/ __|",
                " | . \\| | | | | | (_) | | | | (_) \\__ \\",
                " |_|\\_\\_| |_|_|  \\___/|_| |_|\\___/|___/",
                "",
                " Ready to serve. Type a command or press F1 for help.",
            ];

            let mut lines = vec![];
            for (i, line) in ascii.iter().enumerate() {
                let color = if i < 6 { theme.primary } else { theme.muted_foreground };
                lines.push(Line::from(Span::styled(
                    *line,
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )));
            }

            let welcome = Paragraph::new(lines)
                .style(Style::default().bg(theme.background))
                .alignment(ratatui::layout::Alignment::Center);
            
            // Center the welcome message vertically
            let vertical_margin = (inner.height.saturating_sub(ascii.len() as u16)) / 2;
            let welcome_area = Rect {
                x: inner.x,
                y: inner.y + vertical_margin,
                width: inner.width,
                height: ascii.len() as u16,
            };
            welcome.render(welcome_area, buf);
            return;
        }

        // Get visible lines
        let height = inner.height as usize;
        let visible_lines = self.get_visible_lines(height);

        // Create lines with enhanced styling
        let lines: Vec<Line> = visible_lines
            .iter()
            .map(|line| {
                // Enhanced syntax highlighting
                if line.to_lowercase().contains("error") {
                    Line::from(Span::styled(
                        format!("❌ {}", line),
                        Style::default()
                            .fg(theme.destructive)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else if line.to_lowercase().contains("warn") {
                    Line::from(Span::styled(
                        format!("⚠️ {}", line),
                        Style::default()
                            .fg(theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else if line.starts_with(">") {
                    // User input echo
                    Line::from(Span::styled(
                        line.clone(),
                        Style::default()
                            .fg(theme.accent)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else if line.starts_with("🎨") || line.starts_with("🔥") || line.starts_with("🚀") {
                    // System messages
                    Line::from(Span::styled(
                        line.clone(),
                        Style::default()
                            .fg(theme.extra)
                            .add_modifier(Modifier::BOLD),
                    ))
                } else if line.contains("nil") || line.contains("true") || line.contains("false") {
                    // Lua values
                    Line::from(Span::styled(
                        line.clone(),
                        Style::default().fg(theme.primary),
                    ))
                } else {
                    Line::from(Span::styled(
                        line.clone(),
                        Style::default().fg(theme.foreground),
                    ))
                }
            })
            .collect();

        let paragraph = Paragraph::new(lines).style(Style::default().bg(theme.background));
        paragraph.render(inner, buf);

        // Native Scrollbar
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("▲"))
            .end_symbol(Some("▼"))
            .track_symbol(Some("│"))
            .thumb_symbol("█");

        let mut scrollbar_state = ScrollbarState::new(self.app.output.len())
            .position(self.app.output.len().saturating_sub(self.app.scroll_offset));

        scrollbar.render(area, buf, &mut scrollbar_state);
    }
}
