use mlua::prelude::*;

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "permission_from_string",
        lua.create_function(|lua, (perm_string,): (String,)| {
            let ps = kittycat::perms::Permission::from_string(&perm_string);
            lua.to_value(&ps)
        })?,
    )?;

    module.set(
        "permission_to_string",
        lua.create_function(|lua, (permission,): (LuaValue,)| {
            let perm: kittycat::perms::Permission = lua.from_value(permission)?;
            Ok(perm.to_string())
        })?,
    )?;

    module.set(
        "has_perm",
        lua.create_function(|lua, (permissions, permission): (LuaValue, LuaValue)| {
            let perm: kittycat::perms::Permission = lua.from_value(permission)?;
            let perms: Vec<kittycat::perms::Permission> = lua.from_value(permissions)?;
            Ok(kittycat::perms::has_perm(&perms, &perm))
        })?,
    )?;

    module.set(
        "has_perm_str",
        lua.create_function(|_, (permissions, permission): (Vec<String>, String)| {
            Ok(kittycat::perms::has_perm_str(&permissions, &permission))
        })?,
    )?;

    module.set(
        "staff_permissions_resolve",
        lua.create_function(|lua, sp: LuaValue| {
            let sp = lua.from_value::<kittycat::perms::StaffPermissions>(sp)?;
            let resolved = sp.resolve();
            lua.to_value(&resolved)
        })?,
    )?;

    module.set(
        "check_patch_changes",
        lua.create_function(
            |lua, (manager_perms, current_perms, new_perms): (LuaValue, LuaValue, LuaValue)| {
                let manager_perms: Vec<kittycat::perms::Permission> =
                    lua.from_value(manager_perms)?;
                let current_perms: Vec<kittycat::perms::Permission> =
                    lua.from_value(current_perms)?;
                let new_perms: Vec<kittycat::perms::Permission> = lua.from_value(new_perms)?;
                let changes = kittycat::perms::check_patch_changes(
                    &manager_perms,
                    &current_perms,
                    &new_perms,
                );

                match changes {
                    Ok(()) => Ok((true, LuaValue::Nil)),
                    Err(e) => match e {
                        kittycat::perms::CheckPatchChangesError::NoPermission { permission } => {
                            Ok((
                                false,
                                lua.to_value(&serde_json::json!({
                                    "type": "NoPermission",
                                    "permission": permission
                                }))?,
                            ))
                        }
                        kittycat::perms::CheckPatchChangesError::LacksNegatorForWildcard {
                            wildcard,
                            negator,
                        } => Ok((
                            false,
                            lua.to_value(&serde_json::json!({
                                "type": "LacksNegatorForWildcard",
                                "wildcard": wildcard,
                                "negator": negator
                            }))?,
                        )),
                    },
                }
            },
        )?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
