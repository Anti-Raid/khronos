use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

/// TUI events
#[derive(Debug, Clone)]
pub enum Event {
    /// Key press event
    Key(KeyEvent),
    /// Terminal resize event
    #[allow(dead_code)]
    Resize(u16, u16),
    /// Tick event for animations/updates
    Tick,
}

/// Event handler for the TUI
pub struct EventHandler {
    /// Tick rate for animations
    tick_rate: Duration,
}

impl EventHandler {
    /// Create a new event handler
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    /// Poll for the next event
    pub fn next(&self) -> std::io::Result<Event> {
        if event::poll(self.tick_rate)? {
            match event::read()? {
                CrosstermEvent::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        Ok(Event::Key(key))
                    } else {
                        Ok(Event::Tick)
                    }
                }
                CrosstermEvent::Resize(w, h) => Ok(Event::Resize(w, h)),
                _ => Ok(Event::Tick),
            }
        } else {
            Ok(Event::Tick)
        }
    }
}

/// Check if a key event matches the given key code and modifiers
pub fn key_match(event: &KeyEvent, code: KeyCode, modifiers: KeyModifiers) -> bool {
    event.code == code && event.modifiers == modifiers
}

/// Check if Ctrl+C was pressed
pub fn is_quit(event: &KeyEvent) -> bool {
    key_match(event, KeyCode::Char('c'), KeyModifiers::CONTROL)
        || key_match(event, KeyCode::Char('q'), KeyModifiers::CONTROL)
}

/// Check if Ctrl+T was pressed (theme switcher)
pub fn is_theme_switch(event: &KeyEvent) -> bool {
    key_match(event, KeyCode::Char('t'), KeyModifiers::CONTROL)
}


/// Check if Ctrl+L was pressed (clear output)
pub fn is_clear(event: &KeyEvent) -> bool {
    key_match(event, KeyCode::Char('l'), KeyModifiers::CONTROL)
}

/// Check if F1 was pressed (help)
pub fn is_help(event: &KeyEvent) -> bool {
    event.code == KeyCode::F(1)
}
