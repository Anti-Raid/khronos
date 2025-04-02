use crate::{traits::context::KhronosContext, TemplateContextRef};
use mlua::prelude::*;

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    /*module.set(
        "store",
        lua.create_function(|_, token: TemplateContextRef<T>| {
            if !token.context.has_cap("store") {
                return Err(LuaError::runtime(
                    "You don't have permission to get the shared store in this template context",
                ));
            }

            Ok(token.context.isolate().inner().store_table().clone())
        })?,
    )?;*/

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
