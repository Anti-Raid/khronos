pub mod datastores;
pub mod discord;
pub mod httpclient;
pub mod img_captcha;
pub mod kv;
pub mod objectstorage;

use mluau::prelude::*;

// NOTE: These are options for going from other format -> lua ("serializing" lua values)
pub const LUA_SERIALIZE_OPTIONS: LuaSerializeOptions = LuaSerializeOptions::new()
    .set_array_metatable(true) // PATCH: Set array metatable to true as AntiRaid needs this anyways
    .serialize_none_to_null(false)
    .serialize_unit_to_null(false);

#[cfg(test)]
pub mod test_type_metamethod {
    #[test]
    fn test_type_metamethod() {
        use mluau::prelude::*;

        let lua = Lua::new();
        lua.sandbox(true).expect("failed to enable sandbox");

        pub struct A<B: Clone + 'static> {
            pub a: B,
        }

        impl LuaUserData for A<String> {
            fn add_fields<F: mluau::UserDataFields<Self>>(fields: &mut F) {
                fields.add_meta_field(LuaMetaMethod::Type, "MyType".to_string());
            }
            fn add_methods<M: mluau::UserDataMethods<Self>>(methods: &mut M) {
                methods.add_meta_method(LuaMetaMethod::ToString, |_, _this, ()| {
                    Ok("MyString".to_string())
                });
                methods.add_method("test", |_, this, ()| Ok(this.a.clone()));
            }
        }

        let a = A {
            a: "test".to_string(),
        };

        lua.load(
            r#"
            local a = ...
            assert(typeof(a) == "MyType", "typeof is not working, got " .. typeof(a));
            assert(tostring(a) == "MyString", "tostring is not working, got " .. tostring(a));
            "#,
        )
        .set_name("test")
        .call::<()>(a)
        .expect("test_type_metamethod");
    }
}
