use crate::tui::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget},
};

/// Theme switcher modal widget
pub struct ThemeSwitcher<'a> {
    app: &'a App<'a>,
}

impl<'a> ThemeSwitcher<'a> {
    pub fn new(app: &'a App<'a>) -> Self {
        Self { app }
    }
}

impl<'a> Widget for ThemeSwitcher<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = &self.app.theme;
        let filtered_themes = self.app.filtered_themes();

        // Calculate a centered area for the switcher (slightly wider)
        let switcher_area = centered_rect(60, 50, area);
        
        // Clear the area before rendering to simulation floating
        Clear.render(switcher_area, buf);

        // Create main block with sleek borders (primary color)
        let main_block = Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(theme.primary))
            .style(Style::default().bg(theme.background))
            .title(Span::styled(" Theme Switcher ", Style::default().fg(theme.primary).add_modifier(Modifier::BOLD)));

        let inner_area = main_block.inner(switcher_area);
        main_block.render(switcher_area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Search bar (padded)
                Constraint::Min(0),    // List
                Constraint::Length(1), // Footer
            ])
            .split(inner_area);

        // 1. Render Search Input (Minimalist)
        let search_text = if self.app.theme_filter.is_empty() {
            Line::from(vec![
                Span::styled(" > ", Style::default().fg(theme.accent)),
                Span::styled("Type to search...", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::ITALIC)),
            ])
        } else {
            Line::from(vec![
                Span::styled(" > ", Style::default().fg(theme.accent)),
                Span::styled(&self.app.theme_filter, Style::default().fg(theme.foreground).add_modifier(Modifier::BOLD)),
                Span::styled("_", Style::default().fg(theme.primary).add_modifier(Modifier::RAPID_BLINK)),
            ])
        };

        Paragraph::new(search_text)
            .style(Style::default().bg(theme.background))
            .render(Rect { x: chunks[0].x + 1, y: chunks[0].y + 1, width: chunks[0].width - 2, height: 1 }, buf);
        
        // Separator
        let separator = Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(theme.muted));
        separator.render(Rect { x: chunks[0].x, y: chunks[0].y + 2, width: chunks[0].width, height: 1 }, buf);

        // 2. Render Theme List
        let list_items: Vec<ListItem> = filtered_themes
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let is_selected = i == self.app.selected_theme_index;
                let bg = if is_selected { theme.card } else { theme.background };
                let fg = if is_selected { theme.primary } else { theme.foreground };
                let prefix = if is_selected { "  ● " } else { "    " };
                
                let line = Line::from(vec![
                    Span::styled(prefix, Style::default().fg(fg)),
                    Span::styled(t.name, Style::default().fg(fg).add_modifier(if is_selected { Modifier::BOLD } else { Modifier::empty() })),
                ]);

                ListItem::new(line).style(Style::default().bg(bg))
            })
            .collect();

        List::new(list_items)
            .highlight_style(Style::default().bg(theme.card))
            .render(chunks[1], buf);

        // 3. Render Footer (Minimal)
        let footer_text = Line::from(vec![
            Span::styled("RET ", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::BOLD)),
            Span::raw("select  "),
            Span::styled("ESC ", Style::default().fg(theme.muted_foreground).add_modifier(Modifier::BOLD)),
            Span::raw("cancel"),
        ]);

        Paragraph::new(footer_text)
            .alignment(ratatui::layout::Alignment::Right)
            .style(Style::default().fg(theme.muted_foreground))
            .render(Rect { x: chunks[2].x, y: chunks[2].y, width: chunks[2].width - 2, height: 1 }, buf);
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
