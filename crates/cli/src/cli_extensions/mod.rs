mod ext;

use std::{cell::RefCell, rc::Rc};

use khronos_runtime::rt::mlua::prelude::*;

pub fn load_extensions(
    cli_ext_state: Rc<RefCell<crate::cli::CliExtensionState>>,
    lua: &Lua,
    cli_table: &LuaTable,
) -> LuaResult<()> {
    cli_table.set("ext", ext::ext(cli_ext_state, lua)?)?;

    Ok(())
}
