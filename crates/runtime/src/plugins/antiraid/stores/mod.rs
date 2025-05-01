use crate::traits::context::KhronosContext;
use mlua::prelude::*;

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    let store = lua.app_data_ref::<crate::rt::runtime::RuntimeGlobalTable>().ok_or(
        mlua::Error::RuntimeError("No runtime global table found".to_string()),
    )?;

    module.set(
        "store",
        store.0.clone(),
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
