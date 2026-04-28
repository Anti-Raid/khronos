use mluau::prelude::*;

pub struct PartialStaffPosition(pub kittycat::perms::PartialStaffPosition);
impl LuaUserData for PartialStaffPosition {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("id", |lua, this| lua.create_string(&this.0.id));
        fields.add_field_method_get("index", |_, this| Ok(this.0.index));
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}