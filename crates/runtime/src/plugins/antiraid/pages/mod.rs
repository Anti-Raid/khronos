// To avoid over-relying on Settings
mod settings_ir;

use super::LUA_SERIALIZE_OPTIONS;
use crate::primitives::create_userdata_iterator_with_fields;
use crate::traits::context::KhronosContext;
use crate::traits::pageprovider::PageProvider;
use crate::TemplateContext;
use mlua::prelude::*;
use mlua_scheduler::LuaSchedulerAsyncUserData;
use settings_ir::Setting;

const MAX_TITLE_LENGTH: usize = 256;
const MAX_DESCRIPTION_LENGTH: usize = 4096;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
/// A intermediary representation of a template page
pub struct Page {
    title: String,
    description: String,
    settings: Vec<Setting>,
}

impl Page {
    pub fn validate(&self) -> LuaResult<()> {
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
        methods.add_scheduler_async_method("get", async move |lua, this, _g: ()| {
            this.check_action("get".to_string())?;

            let page = match this.page_provider.get_page().await {
                Some(page) => page,
                None => {
                    return Ok(LuaValue::Nil);
                }
            };

            let page = Page {
                title: page.title,
                description: page.description,
                settings: page.settings.into_iter().map(|e| e.into()).collect(),
            };

            let page = lua.to_value_with(&page, LUA_SERIALIZE_OPTIONS)?;

            Ok(page)
        });

        methods.add_scheduler_async_method("save", async move |lua, this, page: LuaValue| {
            this.check_action("save".to_string())?;

            let page: Page = lua.from_value(page)?;

            page.validate()?;

            // Convert to khronos PageProviderPage raw struct IR
            let page = crate::traits::ir::Page {
                title: page.title,
                description: page.description,
                settings: page.settings.into_iter().map(|e| e.into()).collect(),
            };

            this.page_provider
                .set_page(page)
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_scheduler_async_method("delete", async move |_, this, _g: ()| {
            this.check_action("delete".to_string())?;

            this.page_provider
                .delete_page()
                .await
                .map_err(|e| LuaError::external(e.to_string()))?;

            Ok(())
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<PageExecutor<T>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Methods
                    "get", "save", "delete",
                ],
            )
        });
    }

    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "PageExecutor");
    }
}

// WIP: not yet enabled
pub fn init_plugin<T: KhronosContext>(
    lua: &Lua,
    token: &TemplateContext<T>,
) -> LuaResult<LuaValue> {
    let Some(page_provider) = token.context.page_provider() else {
        return Err(LuaError::external(
            "The pages plugin is not supported in this context",
        ));
    };

    let executor = PageExecutor {
        context: token.context.clone(),
        page_provider,
    }
    .into_lua(lua)?;

    Ok(executor)
}
