// TUI-specific entrypoint for the Khronos CLI
// This runs the full-screen Ratatui interface

use crate::cli::Cli;
use crate::tui::{app::App, events::EventHandler, ui};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

/// Run the TUI interface
pub async fn run_tui(cli: &mut Cli) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
    app.add_output("🔥 Khronos CLI Ready.".to_string());
    // app.add_output(format!("Theme: {} (Press Ctrl+T to change)", app.theme.name));
    // app.add_output("Press F1 for help, Ctrl+C to quit.".to_string());
    app.add_output("".to_string());

    // Create event handler
    let event_handler = EventHandler::new(Duration::from_millis(100));

    // Main loop
    loop {
        // Draw UI
        terminal.draw(|f| ui::render(f, &app))?;

        // Handle events
        match event_handler.next()? {
            crate::tui::events::Event::Key(key) => {
                // If theme switcher is open, it captures most input
                if app.show_theme_switcher {
                    match key.code {
                        KeyCode::Esc => {
                            app.show_theme_switcher = false;
                        }
                        KeyCode::Enter => {
                            app.apply_selected_theme();
                        }
                        KeyCode::Up => {
                            let filtered_count = app.filtered_themes().len();
                            if filtered_count > 0 {
                                app.selected_theme_index = (app.selected_theme_index + filtered_count - 1) % filtered_count;
                            }
                        }
                        KeyCode::Down => {
                            let filtered_count = app.filtered_themes().len();
                            if filtered_count > 0 {
                                app.selected_theme_index = (app.selected_theme_index + 1) % filtered_count;
                            }
                        }
                        KeyCode::Char(c) => {
                            app.theme_filter.push(c);
                            app.selected_theme_index = 0; // Reset selection on filter change
                        }
                        KeyCode::Backspace => {
                            app.theme_filter.pop();
                            app.selected_theme_index = 0;
                        }
                        _ => {}
                    }
                    continue; // Skip other handlers when switcher is open
                }

                // Check for quit
                if crate::tui::events::is_quit(&key) {
                    app.quit();
                }
                // Check for help
                else if crate::tui::events::is_help(&key) {
                    app.toggle_help();
                }
                // Check for theme switcher
                else if crate::tui::events::is_theme_switch(&key) {
                    app.open_theme_switcher();
                }
                // Check for clear
                else if crate::tui::events::is_clear(&key) {
                    app.clear_output();
                }
                // Handle input events
                else if !app.show_help {
                    // ANY non-function key starts interactivity
                    // if !app.is_interactive && matches!(key.code, KeyCode::Char(_) | KeyCode::Tab | KeyCode::Enter) {
                    //     app.is_interactive = true;
                    // }

                    match key.code {
                        KeyCode::Enter if key.modifiers.contains(KeyModifiers::SHIFT) => {
                            // Shift+Enter: new line
                            app.input.insert_newline();
                        }
                        KeyCode::Enter => {
                            // Enter: execute
                            let input = app.get_input();
                            if !input.trim().is_empty() {
                                // Check for slash commands
                                if input.starts_with('/') {
                                    app.save_to_history(); // Optional: save commands to history
                                    app.clear_input();
                                    
                                    let cmd = input.trim().to_lowercase();
                                    match cmd.as_str() {
                                        "/help" => app.toggle_help(),
                                        "/theme" => app.open_theme_switcher(),
                                        "/quit" | "/exit" => app.quit(),
                                        "/clear" => app.clear_output(),
                                        "/repl" => app.is_interactive = true, // Force REPL mode
                                        "/script" => app.is_interactive = true, // Same for now
                                        _ => app.add_output(format!("Unknown command: {}", cmd)),
                                    }
                                    
                                    // If we ran a command that doesn't switch mode, we prefer staying in dashboard 
                                    // unless it was a command that specifically acts on output (like clear, maybe?).
                                    // For now, slash commands keep you in dashboard unless stated otherwise.
                                } else {
                                    // Regular Lua Code -> Interactive Mode
                                    if !app.is_interactive {
                                        app.is_interactive = true;
                                        // On first switch, maybe clear the welcome screen?
                                        // app.clear_output(); 
                                    }

                                    app.save_to_history();
                                    app.add_output(format!("> {}", input));
                                    
                                    // Execute the Lua code
                                    let start = std::time::Instant::now();
                                    match cli.spawn_script("=repl", &input).await {
                                        Ok(values) => {
                                            if !values.is_empty() {
                                                let output = values
                                                    .iter()
                                                    .map(|value| {
                                                        match value {
                                                            khronos_runtime::rt::mlua::Value::String(s) => {
                                                                format!("\"{}\"", s.to_string_lossy())
                                                            }
                                                            _ => format!("{:#?}", value),
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                                    .join("\t");
                                                app.add_output(output);
                                            }
                                        }
                                        Err(e) => {
                                            app.add_output(format!("error: {}", e));
                                        }
                                    }
                                    let duration = start.elapsed();
                                    app.last_execution_time = Some(duration);
                                    
                                    app.clear_input();
                                }
                            }
                        }
                        KeyCode::Up if key.modifiers.contains(KeyModifiers::NONE) => {
                            app.history_prev();
                        }
                        KeyCode::Down if key.modifiers.contains(KeyModifiers::NONE) => {
                            app.history_next();
                        }
                        KeyCode::PageUp => {
                            app.scroll_up(10);
                        }
                        KeyCode::PageDown => {
                            app.scroll_down(10);
                        }
                        KeyCode::Home => {
                            app.scroll_to_top();
                        }
                        KeyCode::End => {
                            app.scroll_to_bottom();
                        }
                        KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            app.clear_input();
                        }
                        _ => {
                            // Pass other keys to the textarea
                            app.input.input(key);
                        }
                    }
                }
            }
            crate::tui::events::Event::Resize(_, _) => {
                // Terminal was resized, will redraw on next iteration
            }
            crate::tui::events::Event::Tick => {
                // Tick event for animations (not used yet)
            }
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
