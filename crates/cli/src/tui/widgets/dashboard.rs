use crate::tui::theme::Theme;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

/// Dashboard widget showing keyboard shortcuts and brand logo
pub struct Dashboard {
    theme: Theme,
}

impl Dashboard {
    pub fn new(theme: Theme) -> Self {
        Self { theme }
    }
}

impl Widget for Dashboard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.theme;

        // Create a layout for the dashboard: Logo at top
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20), // Top Padding
                Constraint::Length(7),      // Logo
                Constraint::Min(10),        // Space for Input (handled by ui.rs) and commands
            ])
            .split(area);

        // 1. Render ASCII Logo (Centered)
        let logo = vec![
            Line::from(vec![Span::styled(" ██╗  ██╗██╗  ██╗██████╗  ██████╗ ███╗   ██╗ ██████╗ ███████╗", Style::default().fg(theme.primary))]),
            Line::from(vec![Span::styled(" ██║ ██╔╝██║  ██║██╔══██╗██╔═══██╗████╗  ██║██╔═══██╗██╔════╝", Style::default().fg(theme.primary))]),
            Line::from(vec![Span::styled(" █████╔╝ ███████║██████╔╝██║   ██║██╔██╗ ██║██║   ██║███████╗", Style::default().fg(theme.primary))]),
            Line::from(vec![Span::styled(" ██╔═██╗ ██╔══██║██╔══██╗██║   ██║██║╚██╗██║██║   ██║╚════██║", Style::default().fg(theme.primary))]),
            Line::from(vec![Span::styled(" ██║  ██╗██║  ██║██║  ██║╚██████╔╝██║ ╚████║╚██████╔╝███████║", Style::default().fg(theme.primary))]),
            Line::from(vec![Span::styled(" ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝ ╚═════╝ ╚══════╝", Style::default().fg(theme.primary))]),
        ];

        Paragraph::new(logo)
            .alignment(ratatui::layout::Alignment::Center)
            .render(chunks[1], buf);

        // Version is now part of the status bar or can be small
        
        // Spacer to push commands down below the input bar
        let commands_area_y = chunks[2].y + 5; // Leave room for input bar
        let commands_height = chunks[2].height.saturating_sub(5);
        
        let commands_area = Rect {
            x: chunks[2].x,
            y: commands_area_y,
            width: chunks[2].width,
            height: commands_height,
        };

        // 3. Render Command List (Multi-column) - Positioned lower
        let shortcuts_area = centered_rect(60, 60, commands_area);
        let shortcut_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Column 1
                Constraint::Percentage(50), // Column 2
            ])
            .split(shortcuts_area);

        let col1 = vec![
            Line::from(vec![
                Span::styled("  /help   ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Span::styled("show help       ", Style::default().fg(theme.foreground)),
                Span::styled("F1", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::DIM)),
            ]),
            Line::from(vec![
                Span::styled("  /theme  ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Span::styled("switch theme    ", Style::default().fg(theme.foreground)),
                Span::styled("ctrl+t", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::DIM)),
            ]),
            Line::from(vec![
                Span::styled("  /clear  ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Span::styled("clear output    ", Style::default().fg(theme.foreground)),
                Span::styled("ctrl+l", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::DIM)),
            ]),
        ];

        let col2 = vec![
            Line::from(vec![
                Span::styled("  /quit   ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Span::styled("exit app        ", Style::default().fg(theme.foreground)),
                Span::styled("ctrl+c", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::DIM)),
            ]),
            Line::from(vec![
                Span::styled("  /script ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)),
                Span::styled("run lua script  ", Style::default().fg(theme.foreground)),
                Span::styled("ctrl+s", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::DIM)),
            ]),
            Line::from(vec![
                Span::styled("  > any   ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("start repl      ", Style::default().fg(theme.foreground)),
                Span::styled("type...", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::DIM)),
            ]),
        ];

        Paragraph::new(col1)
            .render(shortcut_chunks[0], buf);
        
        Paragraph::new(col2)
            .render(shortcut_chunks[1], buf);
    }
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
