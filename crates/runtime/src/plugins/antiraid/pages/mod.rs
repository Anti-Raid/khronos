// To avoid over-relying on Settings
mod settings_ir;

use crate::traits::context::KhronosContext;
use crate::traits::pageprovider::PageProvider;
use crate::utils::executorscope::ExecutorScope;
use crate::{lua_promise, TemplateContextRef};
use mlua::prelude::*;
use settings_ir::Setting;

const MAX_ID_LENGTH: usize = 100;
const MAX_TITLE_LENGTH: usize = 256;
const MAX_DESCRIPTION_LENGTH: usize = 4096;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
/// A intermediary representation of a template page
pub struct Page {
    id: String,
    title: String,
    description: String,
    settings: Vec<Setting>,
}

impl Page {
    pub fn validate(&self) -> LuaResult<()> {
        if self.id.len() > MAX_ID_LENGTH {
            return Err(LuaError::external("ID is too long"));
        }
        if self.title.len() > MAX_TITLE_LENGTH {
            return Err(LuaError::external("Title is too long"));
        }
        if self.description.len() > MAX_DESCRIPTION_LENGTH {
            return Err(LuaError::external("Description is too long"));
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct PageExecutor<T: KhronosContext> {
    context: T,
    page_provider: T::PageProvider,
}

impl<T: KhronosContext> PageExecutor<T> {
    pub fn check_action(&self, action: String) -> LuaResult<()> {
        if !self.context.has_cap(&format!("page:{}", action)) {
            return Err(LuaError::runtime(
                "Page action is not allowed in this template context",
            ));
        }

        Ok(())
    }
}

impl<T: KhronosContext> LuaUserData for PageExecutor<T> {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get", |_lua, this, _g: ()| {
            Ok(lua_promise!(this, _g, |lua, this, _g|, {
                this.check_action("get".to_string())?;

                let page = match this.page_provider.get_page().await {
                    Some(page) => page,
                    None => {
                        return Ok(LuaValue::Nil);
                    }
                };

                let page = Page {
                    id: page.id,
                    title: page.title,
                    description: page.description,
                    settings: page.settings.into_iter().map(|e| e.into()).collect(),
                };

                let page = lua.to_value(&page)?;

                Ok(page)
            }))
        });

        methods.add_method("save", |_lua, this, page: LuaValue| {
            Ok(lua_promise!(this, page, |lua, this, page|, {
                this.check_action("save".to_string())?;

                let page: Page = lua.from_value(page)?;

                page.validate()?;

                // Convert to khronos PageProviderPage raw struct IR
                let page = crate::traits::ir::Page {
                    id: page.id,
                    title: page.title,
                    description: page.description,
                    settings: page.settings.into_iter().map(|e| e.into()).collect(),
                };

                this.page_provider.set_page(page).await
                .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        });

        methods.add_method("delete", |_, this, _g: ()| {
            Ok(lua_promise!(this, _g, |_lua, this, _g|, {
                this.check_action("delete".to_string())?;

                this.page_provider.delete_page().await
                .map_err(|e| LuaError::external(e.to_string()))?;

                Ok(())
            }))
        })
    }

    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "PageExecutor");
    }
}

pub fn init_plugin<T: KhronosContext>(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "new",
        lua.create_function(
            |_, (token, scope): (TemplateContextRef<T>, Option<String>)| {
                let scope = ExecutorScope::scope_str(scope)?;
                let Some(page_provider) = token.context.page_provider(scope) else {
                    return Err(LuaError::external(
                        "The pages plugin is not supported in this context",
                    ));
                };

                let executor = PageExecutor {
                    context: token.context.clone(),
                    page_provider,
                };

                Ok(executor)
            },
        )?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table

    Ok(module)
}
