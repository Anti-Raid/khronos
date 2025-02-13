use std::{cell::RefCell, rc::Rc};

use khronos_runtime::lua_promise;
use mlua::prelude::*;

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

    ext.set_readonly(true);

    Ok(ext)
}
