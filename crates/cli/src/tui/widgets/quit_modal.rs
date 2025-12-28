use crate::tui::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub struct QuitModal<'a> {
    app: &'a App<'a>,
}

impl<'a> QuitModal<'a> {
    pub fn new(app: &'a App<'a>) -> Self {
        Self { app }
    }
}

impl<'a> Widget for QuitModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.theme;

        // Small centered area
        let area = centered_rect(40, 20, area);
        
        Clear.render(area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(theme.destructive))
            .style(Style::default().bg(theme.background))
            .title(Span::styled(" Quit Confirmation ", Style::default().fg(theme.destructive).add_modifier(Modifier::BOLD)));

        let inner_area = block.inner(area);
        block.render(area, buf);

        let text = vec![
            Line::from("Are you sure you want to quit?"),
            Line::from(""),
            Line::from(vec![
                Span::styled("ENTER", Style::default().fg(theme.destructive).add_modifier(Modifier::BOLD)),
                Span::raw(" to confirm"),
            ]),
            Line::from(vec![
                Span::styled("ESC", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::BOLD)),
                Span::raw(" to cancel"),
            ]),
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
