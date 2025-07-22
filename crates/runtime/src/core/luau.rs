use mlua_scheduler::LuaSchedulerAsyncUserData;
use mluau::prelude::*;

#[derive(Clone)]
/// An lockdown executor is used to manage AntiRaid lockdowns from Lua
/// templates
pub struct Chunk {
    code: String,
    chunk_name: Option<String>,
    environment: Option<LuaTable>,
    optimization_level: Option<u8>,
}

impl Chunk {
    pub fn setup_chunk(&self, lua: &Lua) -> LuaResult<LuaChunk<'_>> {
        let mut compiler = mluau::Compiler::new();
        if let Some(level) = self.optimization_level {
            compiler = compiler.set_optimization_level(level);
        }

        let bytecode = compiler.compile(&self.code)?;

        let mut chunk = lua.load(bytecode);
        chunk = chunk.set_mode(mluau::ChunkMode::Binary); // We've compiled it anyways so

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

impl LuaUserData for Chunk {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "Chunk");
        fields.add_field_method_get("environment", |_, this| Ok(this.environment.clone()));
        fields.add_field_method_set("environment", |_, this, args: LuaTable| {
            this.environment = Some(args);
            Ok(())
        });
        fields.add_field_method_get("optimization_level", |_, this| Ok(this.optimization_level));
        fields.add_field_method_set("optimization_level", |_, this, level: u8| {
            if ![0, 1, 2].contains(&level) {
                return Err(LuaError::runtime(
                    "Invalid optimization level. Must be 0, 1 or 2",
                ));
            }

            this.optimization_level = Some(level);
            Ok(())
        });
        fields.add_field_method_get("code", |lua, this| lua.create_string(&this.code));
        fields.add_field_method_set("code", |_, this, code: String| {
            this.code = code;
            Ok(())
        });
        fields.add_field_method_get("chunk_name", |lua, this| {
            if let Some(name) = &this.chunk_name {
                Ok(LuaValue::String(lua.create_string(name)?))
            } else {
                Ok(LuaValue::Nil)
            }
        });
        fields.add_field_method_set("chunk_name", |_, this, name: Option<String>| {
            this.chunk_name = name;
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("call", |lua, this, args: LuaMultiValue| {
            let chunk = this.setup_chunk(lua)?;
            let res = chunk.call::<LuaMultiValue>(args)?;

            Ok(res)
        });

        methods.add_scheduler_async_method(
            "call_async",
            async move |lua, this, args: LuaMultiValue| {
                let func = this.setup_chunk(&lua)?.into_function()?;

                let th = lua.create_thread(func)?;

                let scheduler = mlua_scheduler::taskmgr::get(&lua);
                let output = scheduler.spawn_thread_and_wait(th, args).await?;

                match output {
                    Some(result) => result,
                    None => Ok(LuaMultiValue::new()),
                }
            },
        );
    }

    #[cfg(feature = "repl")]
    fn register(registry: &mut LuaUserDataRegistry<Self>) {
        Self::add_fields(registry);
        Self::add_methods(registry);
        let fields = registry.fields(false).iter().map(|x| x.to_string()).collect::<Vec<_>>();
        registry.add_meta_field("__ud_fields", fields);
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "load",
        lua.create_function(|_, code: String| {
            let chunk = Chunk {
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
                    .map(|value| format!("{value:#?}"))
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
