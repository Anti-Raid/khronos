use mlua::prelude::*;

#[cfg(feature = "experiment_image_classification")]
mod image_classification;

#[allow(dead_code)] // Some feature combinations may not use this function
fn load_or_warn(
    lua: &Lua,
    tab: &LuaTable,
    tab_name: &str,
    f: impl FnOnce(&Lua) -> LuaResult<LuaTable>,
) -> LuaResult<()> {
    match f(lua) {
        Ok(exp_tab) => {
            tab.set(tab_name, exp_tab)?;
        }
        Err(e) => {
            eprintln!("Failed to load experiment {}: {}", tab_name, e);
        }
    }

    Ok(())
}

pub fn load_experiments(lua: &Lua, experiments: &[String]) -> LuaResult<LuaTable> {
    let experiments_table = lua.create_table()?;

    for experiment in experiments {
        match experiment.as_str() {
            #[cfg(feature = "experiment_image_classification")]
            "image_classification" => load_or_warn(
                lua,
                &experiments_table,
                "image_classification",
                image_classification::image_classification_experiment,
            )?,
            "sample_experiment" => {
                experiments_table.set("sample_experiment", lua.create_function(|_, ()| Ok(42))?)?
            }
            _ => {
                return Err(LuaError::external(format!(
                    "Unknown experiment: {}",
                    experiment
                )))
            }
        }
    }

    experiments_table.set_readonly(true);

    Ok(experiments_table)
}
