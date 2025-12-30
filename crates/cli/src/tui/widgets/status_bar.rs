use crate::tui::app::{App, AppMode};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Status bar widget showing current mode, theme, and shortcuts
pub struct StatusBar<'a> {
    app: &'a App<'a>,
}

impl<'a> StatusBar<'a> {
    pub fn new(app: &'a App<'a>) -> Self {
        Self { app }
    }

    fn get_mode_text(&self) -> (&'static str, &'static str) {
        match self.app.mode {
            AppMode::Repl => ("REPL", "🔧"),
            AppMode::Script => ("SCRIPT", "📜"),
            AppMode::Idle => ("IDLE", "💤"),
        }
    }
}

impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.theme;
        let (mode_text, mode_icon) = self.get_mode_text();

        // Build status line with enhanced visuals
        let mut spans = vec![];

        // Left section: Title + Mode
        spans.push(Span::styled(
            " ",
            Style::default().fg(theme.primary),
        ));
        spans.push(Span::styled(
            format!(" 🔥 KHRONOS v{} ", env!("CARGO_PKG_VERSION")),
            Style::default()
                .fg(theme.primary_foreground)
                .bg(theme.primary)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            " ",
            Style::default().fg(theme.primary),
        ));

        spans.push(Span::styled(
            "",
            Style::default().fg(theme.accent),
        ));
        spans.push(Span::styled(
            format!(" {} {} ", mode_icon, mode_text),
            Style::default()
                .fg(theme.accent_foreground)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            " ",
            Style::default().fg(theme.accent),
        ));

        // Middle section: Theme + Stats
        spans.push(Span::styled(
            " ",
            Style::default().fg(theme.card),
        ));
        spans.push(Span::styled(
            format!(" 🎨 {} ", theme.name),
            Style::default()
                .fg(theme.foreground)
                .bg(theme.card)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            " ",
            Style::default().fg(theme.card),
        ));

        // History count
        spans.push(Span::styled(
            " ",
            Style::default().fg(theme.secondary),
        ));
        spans.push(Span::styled(
            format!(" 📖 {} ", self.app.command_history.len()),
            Style::default()
                .fg(theme.secondary_foreground)
                .bg(theme.secondary)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::styled(
            " ",
            Style::default().fg(theme.secondary),
        ));

        // Last Execution Time
        if let Some(duration) = self.app.last_execution_time {
            let color = if duration.as_millis() < 100 {
                theme.extra
            } else if duration.as_millis() < 1000 {
                theme.accent
            } else {
                theme.destructive
            };
            
            spans.push(Span::styled(
                " ",
                Style::default().fg(color),
            ));
            spans.push(Span::styled(
                format!(" ⚡ {:?} ", duration),
                Style::default()
                    .fg(theme.extra_foreground) // Assuming light foreground for these accents
                    .bg(color)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                " ",
                Style::default().fg(color),
            ));
        }

        // Available Width check for right-aligned bits
        let current_width: u16 = spans.iter().map(|s| s.width() as u16).sum();
        let available_width = area.width;

        // Right-aligned: Build Status (Badge style)
        let build_status = " 🛠️  READY ";
        let builder_badge_width = build_status.len() as u16 + 4; // prefix/suffix
        
        if current_width + builder_badge_width < available_width {
            let padding = available_width - current_width - builder_badge_width;
            spans.push(Span::raw(" ".repeat(padding as usize)));
            spans.push(Span::styled(
                "",
                Style::default().fg(theme.extra),
            ));
            spans.push(Span::styled(
                build_status,
                Style::default()
                    .fg(theme.extra_foreground)
                    .bg(theme.extra)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                "",
                Style::default().fg(theme.extra),
            ));
        }

        let line = Line::from(spans);
        let paragraph = Paragraph::new(line).style(Style::default().bg(theme.background));

        paragraph.render(area, buf);
    }
}
