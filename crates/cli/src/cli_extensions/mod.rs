mod http;

use mlua::prelude::*;

pub fn load_extensions(lua: &Lua, cli_table: &LuaTable) -> LuaResult<()> {
    cli_table.set("http_client", http::http_client_experiment(lua)?)?;

    Ok(())
}
