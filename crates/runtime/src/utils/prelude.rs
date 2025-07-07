use mluau::prelude::*;

/// Disables potentially harmful functions in the Lua environment
pub fn disable_harmful(lua: &Lua) -> Result<(), LuaError> {
    // Ensure _G.print, _G.eprint and _G.require are nil
    lua.globals().set("print", LuaValue::Nil)?;
    lua.globals().set("eprint", LuaValue::Nil)?;
    lua.globals().set("require", LuaValue::Nil)?;
    Ok(())
}

/// Sets up the prelude for a Lua environment
pub fn setup_prelude(lua: &Lua, env: LuaTable) -> Result<(), LuaError> {
    // Prelude code providing some basic functions directly to the Lua VM
    let env_ref = env.clone();
    env.raw_set(
        "print",
        lua.create_function(move |lua, args: LuaMultiValue| {
            #[inline(always)]
            fn print_impl(stdout_table: &LuaTable, args: LuaMultiValue) -> LuaResult<()> {
                if args.is_empty() {
                    stdout_table.set(stdout_table.raw_len() + 1, "nil")?;
                } else {
                    let mut output = String::new();
                    for (i, arg) in args.iter().enumerate() {
                        output.push_str(&arg.to_string()?);
                        if i != args.len() - 1 {
                            output.push('\t');
                        }
                    }

                    stdout_table.set(stdout_table.raw_len() + 1, output)?;
                }
                Ok(())
            }

            let stdout_table = env_ref.get::<Option<LuaTable>>("stdout")?;
            if let Some(stdout_table) = stdout_table {
                print_impl(&stdout_table, args)?;
            } else {
                let stdout_table = lua.create_table()?;
                print_impl(&stdout_table, args)?;
                env_ref.set("stdout", stdout_table)?;
            }

            Ok(())
        })?,
    )?;

    Ok(())
}
