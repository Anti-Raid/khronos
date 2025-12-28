use crate::tui::{
    app::App,
    widgets::{Dashboard, OutputPanel, ReplInput, StatusBar, ThemeSwitcher},
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

/// Render the main UI
pub fn render(frame: &mut Frame, app: &App) {
    let theme = &app.theme;

    // Create main vertical layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(10),     // Content (Output / Dashboard)
            Constraint::Length(3),   // Input panel (minimal)
            Constraint::Length(1),   // Status bar (bottom)
        ])
        .split(frame.area());


    // Render status bar
    // Render status bar at the bottom
    frame.render_widget(StatusBar::new(app), main_chunks[2]);

    if !app.is_interactive && !app.show_help {
        // --- DASHBOARD MODE ---
        // 1. Render Dashboard content (Logo + Hints)
        frame.render_widget(Dashboard::new(theme.clone()), main_chunks[0]);

        // 2. Render Centered Input
        // Calculate a centered area for the input bar
        let area = main_chunks[0];
        let center_y = area.height / 2;
        let input_area = Rect {
            x: area.x + (area.width.saturating_sub(60)) / 2, // 60 char width
            y: area.y + center_y,
            width: 60.min(area.width),
            height: 3,
        };
        
        // Clear the area behind the input for popup effect
        let block = ratatui::widgets::Block::default().style(ratatui::style::Style::default().bg(theme.background));
        frame.render_widget(block, input_area);
        
        frame.render_widget(ReplInput::new(app), input_area);

    } else if !app.show_help {
        // --- REPL MODE ---
        // Render Output centered
        frame.render_widget(OutputPanel::new(app), main_chunks[0]);
        
        // Render Input at bottom (above status bar)
        frame.render_widget(ReplInput::new(app), main_chunks[1]);
    }

    // If help is shown, overlay it (using Dashboard/Help widget)
    if app.show_help {
        let help_area = centered_rect(80, 80, frame.area());
        frame.render_widget(Dashboard::new(theme.clone()), help_area);
    }

    // Render Theme Switcher modal on top of everything
    if app.show_theme_switcher {
        frame.render_widget(ThemeSwitcher::new(app), frame.area());
    }
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
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
