mod ext;
mod http_client;
mod http_server;

use std::{cell::RefCell, rc::Rc};

use mlua::prelude::*;

pub fn load_extensions(
    cli_ext_state: Rc<RefCell<crate::cli::CliExtensionState>>,
    lua: &Lua,
    cli_table: &LuaTable,
) -> LuaResult<()> {
    cli_table.set("http_client", http_client::http_client(lua)?)?;
    cli_table.set("http_server", http_server::http_server(lua)?)?;
    cli_table.set("ext", ext::ext(cli_ext_state, lua)?)?;

    Ok(())
}
