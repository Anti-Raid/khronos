use std::{cell::RefCell, io::Write, rc::Rc, str::FromStr};

use khronos_runtime::lua_promise;
use mlua::prelude::*;
use std::io::IsTerminal;
use termcolor::{Color, ColorChoice, WriteColor};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Colorize {
    fg_color: Option<String>,
    bg_color: Option<String>,
    bold: Option<bool>,
    dimmed: Option<bool>,
}

pub fn ext(
    cli_ext_state: Rc<RefCell<crate::cli::CliExtensionState>>,
    lua: &Lua,
) -> LuaResult<LuaTable> {
    let ext = lua.create_table()?;

    ext.set(
        "input",
        lua.create_function(move |_lua, (prompt,): (String,)| {
            Ok(lua_promise!(prompt, |_lua, prompt|, {
                let (tx, rx) = tokio::sync::oneshot::channel();

                std::thread::spawn(move || {
                    let mut editor = match rustyline::DefaultEditor::new() {
                        Ok(e) => e,
                        Err(e) => {
                            let _ = tx.send(Err(format!("Failed to create editor: {}", e)));
                            return;
                        }
                    };

                    let input = match editor.readline(&prompt) {
                        Ok(i) => i,
                        Err(e) => {
                            let _ = tx.send(Err(format!("Failed to read input: {}", e)));
                            return;
                        }
                    };

                    let _ = tx.send(Ok(input));
                });

                match rx.await {
                    Ok(Ok(input)) => Ok(input),
                    Ok(Err(e)) => Err(LuaError::external(e)),
                    Err(_) => Err(LuaError::external("Failed to receive input")),
                }
            }))
        })?,
    )?;

    ext.set(
        "request_next_entrypoint",
        lua.create_function(move |lua, entrypoint: LuaValue| {
            let value = lua.from_value::<Option<crate::cli::CliEntrypointAction>>(entrypoint)?;

            cli_ext_state.borrow_mut().requested_entrypoint = value;

            Ok(())
        })?,
    )?;

    ext.set(
        "color_print",
        lua.create_function(move |lua, (data, text): (LuaValue, String)| {
            let preference = std::env::var("COLOR").unwrap_or("auto".to_string());
            let mut choice = match preference.parse::<ColorChoice>() {
                Ok(c) => c,
                Err(_) => ColorChoice::Auto,
            };

            let data = match data {
                LuaValue::String(s) => {
                    let value = s.to_str()?;

                    if value == "bold" {
                        Colorize {
                            fg_color: None,
                            bg_color: None,
                            bold: Some(true),
                            dimmed: None,
                        }
                    } else if value == "dimmed" {
                        Colorize {
                            fg_color: None,
                            bg_color: None,
                            bold: None,
                            dimmed: Some(true),
                        }
                    } else if value.starts_with("bg:") {
                        Colorize {
                            fg_color: None,
                            bg_color: Some(value.trim_start_matches("bg:").to_string()),
                            bold: None,
                            dimmed: None,
                        }
                    } else if value.starts_with("fg:") {
                        Colorize {
                            fg_color: Some(value.trim_start_matches("fg:").to_string()),
                            bg_color: None,
                            bold: None,
                            dimmed: None,
                        }
                    } else {
                        Colorize {
                            fg_color: Some(value.to_string()),
                            bg_color: None,
                            bold: None,
                            dimmed: None,
                        }
                    }
                }
                LuaValue::Table(_) => lua.from_value(data)?,
                _ => return Err(mlua::Error::external("Invalid color data")),
            };

            if choice == ColorChoice::Auto && !std::io::stdin().is_terminal() {
                choice = ColorChoice::Never;
            }

            let mut stdout = termcolor::StandardStream::stdout(choice);
            stdout.set_color(
                termcolor::ColorSpec::new()
                    .set_fg(match data.fg_color {
                        Some(ref color) => Some(Color::from_str(color).map_err(|e| {
                            mlua::Error::external(format!(
                                "Failed to parse color for fg_color: {}",
                                e
                            ))
                        })?),
                        None => None,
                    })
                    .set_bg(match data.bg_color {
                        Some(ref color) => Some(Color::from_str(color).map_err(|e| {
                            mlua::Error::external(format!(
                                "Failed to parse color for bg_color: {}",
                                e
                            ))
                        })?),
                        None => None,
                    })
                    .set_bold(data.bold.unwrap_or(false))
                    .set_dimmed(data.dimmed.unwrap_or(false)),
            )?;
            write!(&mut stdout, "{}", text)?;
            stdout.reset()?;
            stdout.write_all(b"\n")?;
            Ok(())
        })?,
    )?;

    ext.set_readonly(true);

    Ok(ext)
}
