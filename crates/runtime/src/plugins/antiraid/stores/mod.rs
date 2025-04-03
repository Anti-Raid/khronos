use crate::{traits::context::KhronosContext, TemplateContextRef};
use mlua::prelude::*;

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "store",
        lua.create_function(|_, token: TemplateContextRef<T>| {
            Ok(token.context.runtime_shareable_data().store_table.clone())
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
