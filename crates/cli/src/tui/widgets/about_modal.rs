use crate::tui::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct AboutModal<'a> {
    app: &'a App<'a>,
}

impl<'a> AboutModal<'a> {
    pub fn new(app: &'a App<'a>) -> Self {
        Self { app }
    }
}

impl<'a> Widget for AboutModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.theme;

        let area = centered_rect(50, 40, area);
        
        Clear.render(area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(theme.accent))
            .style(Style::default().bg(theme.background))
            .title(Span::styled(" About Khronos ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)));

        let inner_area = block.inner(area);
        block.render(area, buf);

        let version = env!("CARGO_PKG_VERSION");

        let text = vec![
            Line::from(Span::styled("Khronos CLI", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD | Modifier::ITALIC))),
            Line::from(format!("v{}", version)),
            Line::from(""),
            Line::from(Span::styled("Developed by AntiRaid Team", Style::default().fg(theme.foreground))),
            Line::from(""),
            Line::from(Span::styled("A specialized Lua runtime for", Style::default().fg(theme.muted_foreground))),
            Line::from(Span::styled("high-performance Discord bots.", Style::default().fg(theme.muted_foreground))),
            Line::from(""),
            Line::from(Span::styled("Press ESC to close", Style::default().fg(theme.muted_foreground))),
        ];

        Paragraph::new(text)
            .alignment(ratatui::layout::Alignment::Center)
            .render(inner_area, buf);
    }
}

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
