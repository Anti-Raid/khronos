use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::KhronosContext;
use crate::{plugins::antiraid::promise::UserDataLuaPromise, TemplateContextRef};
use mlua::prelude::*;

#[derive(Clone)]
/// An lockdown executor is used to manage AntiRaid lockdowns from Lua
/// templates
pub struct Chunk<T: KhronosContext> {
    context: T,
    code: String,
    chunk_name: Option<String>,
    environment: Option<LuaTable>,
    optimization_level: Option<u8>,
}

impl<T: KhronosContext> Chunk<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("luau:{}", action)) && !self.context.has_cap("luau:*") {
            return Err(LuaError::runtime(
                "Luau action is not allowed in this template context",
            ));
        }

        Ok(())
    }

    pub fn setup_chunk(&self, lua: &Lua) -> LuaResult<LuaChunk<'_>> {
        let mut compiler = mlua::Compiler::new();
        if let Some(level) = self.optimization_level {
            compiler = compiler.set_optimization_level(level);
        }

        let bytecode = compiler.compile(&self.code)?;

        let mut chunk = lua.load(bytecode);
        chunk = chunk.set_mode(mlua::ChunkMode::Binary); // We've compiled it anyways so

        if let Some(name) = &self.chunk_name {
            chunk = chunk.set_name(name);
        }

        if let Some(env) = &self.environment {
            chunk = chunk.set_environment(env.clone());
        } else {
            chunk = chunk.set_environment(lua.globals());
        }

        chunk = chunk.set_compiler(compiler);

        Ok(chunk)
    }
}

impl<T: KhronosContext> LuaUserData for Chunk<T> {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "Chunk");
        fields.add_field_method_get("environment", |_, this| Ok(this.environment.clone()));
        fields.add_field_method_set("environment", |_, this, args: LuaTable| {
            this.check_action("eval.set_environment".to_string())?;
            this.environment = Some(args);
            Ok(())
        });
        fields.add_field_method_get("optimization_level", |_, this| Ok(this.optimization_level));
        fields.add_field_method_set("optimization_level", |_, this, level: u8| {
            this.check_action("eval.set_optimization_level".to_string())?;
            if ![0, 1, 2].contains(&level) {
                return Err(LuaError::runtime(
                    "Invalid optimization level. Must be 0, 1 or 2",
                ));
            }

            this.optimization_level = Some(level);
            Ok(())
        });
        fields.add_field_method_get("code", |_, this| Ok(this.code.clone()));
        fields.add_field_method_set("code", |_, this, code: String| {
            this.check_action("eval.modify_set_code".to_string())?;
            this.code = code;
            Ok(())
        });
        fields.add_field_method_get("chunk_name", |_, this| Ok(this.chunk_name.clone()));
        fields.add_field_method_set("chunk_name", |_, this, name: Option<String>| {
            this.check_action("eval.set_chunk_name".to_string())?;
            this.chunk_name = name;
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("call", |lua, this, args: LuaMultiValue| {
            this.check_action("eval.call".to_string())?;

            let chunk = this.setup_chunk(lua)?;
            let res = chunk.call::<LuaMultiValue>(args)?;

            Ok(res)
        });

        methods.add_promise_method("call_async", async move |lua, this, args: LuaMultiValue| {
            this.check_action("eval.call_async".to_string())?;

            let func = this.setup_chunk(&lua)?
            .into_function()?;


            let th = lua.create_thread(func)?;

            let scheduler = mlua_scheduler_ext::Scheduler::get(&lua);
            let output = scheduler
                .spawn_thread_and_wait("Eval", th, args)
                .await?;

            match output {
                Some(result) => result,
                None => {
                    Ok(LuaMultiValue::new())
                }
            }
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Chunk<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "environment",
                    "optimization_level",
                    "code",
                    "chunk_name",
                    // Methods
                    "call",
                    "call_async",
                ],
            )
        });
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "load",
        lua.create_function(|_, (token, code): (TemplateContextRef<T>, String)| {
            if !token.context.has_cap("luau:eval") && !token.context.has_cap("luau:*") {
                return Err(LuaError::runtime(
                    "You don't have permission to evaluate Luau code in this template context",
                ));
            }

            let chunk = Chunk {
                context: token.context.clone(),
                code,
                chunk_name: None,
                environment: None,
                optimization_level: None,
            };

            Ok(chunk)
        })?,
    )?;

    module.set(
        "format",
        lua.create_function(|_, values: LuaMultiValue| {
            if !values.is_empty() {
                Ok(values
                    .iter()
                    .map(|value| format!("{:#?}", value))
                    .collect::<Vec<_>>()
                    .join("\t"))
            } else {
                Ok("nil".to_string())
            }
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
