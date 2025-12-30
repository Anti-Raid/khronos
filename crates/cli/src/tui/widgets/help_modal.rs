use crate::tui::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Widget},
};

pub struct HelpModal<'a> {
    app: &'a App<'a>,
}

impl<'a> HelpModal<'a> {
    pub fn new(app: &'a App<'a>) -> Self {
        Self { app }
    }
}

impl<'a> Widget for HelpModal<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.theme;

        // Centered area
        let area = centered_rect(60, 60, area);
        
        Clear.render(area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(theme.primary))
            .style(Style::default().bg(theme.background))
            .title(Span::styled(" Help ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));

        let inner_area = block.inner(area);
        block.render(area, buf);

        let items = vec![
            ("Available Global Keys:", ""),
            ("Context", "Action"),
            ("-------", "------"),
            ("F1 / ?", "Toggle Help"),
            ("F2", "About Khronos"),
            ("Ctrl+T", "Theme Switcher"),
            ("Ctrl+L", "Clear Output"),
            ("Ctrl+C / Ctrl+Q", "Quit"),
            ("", ""),
            ("Navigation:", ""),
            ("Up / Down", "History Navigation"),
            ("PgUp / PgDown", "Scroll Output"),
            ("Home / End", "Scroll to Top/Bottom"),
        ];

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|(k, v)| {
                let content = if v.is_empty() {
                    // Section Header
                    Line::from(vec![
                         Span::styled(format!("{}", k), Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                    ])
                } else {
                     Line::from(vec![
                        Span::styled(format!("{:<20}", k), Style::default().fg(theme.primary)),
                        Span::styled(format!("{}", v), Style::default().fg(theme.foreground)),
                    ])
                };
                ListItem::new(content)
            })
            .collect();

        List::new(list_items)
            .style(Style::default().bg(theme.background))
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
