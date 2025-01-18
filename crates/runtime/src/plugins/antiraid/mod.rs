pub mod datetime;
pub mod discord;
pub mod img_captcha;
pub mod interop;
pub mod kv;
pub mod lazy;
pub mod lockdowns;
pub mod permissions;
pub mod promise;
pub mod stings;
pub mod typesext;
pub mod userinfo;

#[cfg(test)]
pub mod test_type_metamethod {
    #[test]
    fn test_type_metamethod() {
        use mlua::prelude::*;

        let lua = Lua::new();
        lua.sandbox(true).expect("failed to enable sandbox");

        pub struct A<B: Clone + 'static> {
            pub a: B,
        }

        impl LuaUserData for A<String> {
            fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
                fields.add_meta_field(LuaMetaMethod::Type, "MyType".to_string());
            }
            fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
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
