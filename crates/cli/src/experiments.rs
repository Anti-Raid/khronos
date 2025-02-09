use mlua::prelude::*;

// Image classification experiments
#[cfg(feature = "experiment_resnet101")]
use tch::vision::resnet::resnet101;
#[cfg(feature = "experiment_resnet34")]
use tch::vision::resnet::resnet34;
#[cfg(feature = "experiment_resnet50")]
use tch::vision::resnet::resnet50;
#[cfg(any(
    feature = "experiment_resnet34",
    feature = "experiment_resnet50",
    feature = "experiment_resnet101"
))]
use tch::{nn::VarStore, vision::imagenet, Device, Kind};

#[cfg(feature = "experiment_resnet34")]
fn resnet34_experiment(lua: &Lua) -> LuaResult<LuaTable> {
    let resnet34_exp = lua.create_table()?;

    let mut vs = VarStore::new(Device::cuda_if_available());

    let model = resnet34(&vs.root(), imagenet::CLASS_COUNT);
    vs.load("resnet34.bin")
        .map_err(|e| LuaError::external(e.to_string()))?;

    resnet34_exp.set(
        "classify_image_path",
        lua.create_function(move |lua, path: String| {
            let image = imagenet::load_image_and_resize224(path)
                .map_err(|e| LuaError::external(e.to_string()))?
                .to_device(vs.device());

            let output = image
                .unsqueeze(0)
                .apply_t(&model, false)
                .softmax(-1, Kind::Float);

            let result_table = lua.create_table()?;
            for (probability, class) in imagenet::top(&output, 10).iter() {
                result_table.set(class.as_str(), *probability)?;
            }

            Ok(result_table)
        })?,
    )?;

    Ok(resnet34_exp)
}

#[cfg(feature = "experiment_resnet50")]
fn resnet50_experiment(lua: &Lua) -> LuaResult<LuaTable> {
    let resnet50_exp = lua.create_table()?;

    let mut vs = VarStore::new(Device::cuda_if_available());
    let model = resnet50(&vs.root(), imagenet::CLASS_COUNT);
    vs.load("resnet50.bin")
        .map_err(|e| LuaError::external(e.to_string()))?;

    resnet50_exp.set(
        "classify_image_path",
        lua.create_function(move |lua, path: String| {
            let image = imagenet::load_image_and_resize224(path)
                .map_err(|e| LuaError::external(e.to_string()))?
                .to_device(vs.device());

            let output = image
                .unsqueeze(0)
                .apply_t(&model, false)
                .softmax(-1, Kind::Float);

            let result_table = lua.create_table()?;
            for (probability, class) in imagenet::top(&output, 10).iter() {
                result_table.set(class.as_str(), *probability)?;
            }

            Ok(result_table)
        })?,
    )?;

    Ok(resnet50_exp)
}

#[cfg(feature = "experiment_resnet101")]
fn resnet101_experiment(lua: &Lua) -> LuaResult<LuaTable> {
    let resnet101_exp = lua.create_table()?;

    let mut vs = VarStore::new(Device::cuda_if_available());
    let model = resnet101(&vs.root(), imagenet::CLASS_COUNT);
    vs.load("resnet101.bin")
        .map_err(|e| LuaError::external(e.to_string()))?;

    resnet101_exp.set(
        "classify_image_path",
        lua.create_function(move |lua, path: String| {
            let image = imagenet::load_image_and_resize224(path)
                .map_err(|e| LuaError::external(e.to_string()))?
                .to_device(vs.device());

            let output = image
                .unsqueeze(0)
                .apply_t(&model, false)
                .softmax(-1, Kind::Float);

            let result_table = lua.create_table()?;
            for (probability, class) in imagenet::top(&output, 10).iter() {
                result_table.set(class.as_str(), *probability)?;
            }

            Ok(result_table)
        })?,
    )?;

    Ok(resnet101_exp)
}

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
            #[cfg(feature = "experiment_resnet34")]
            "resnet34" => load_or_warn(lua, &experiments_table, "resnet34", resnet34_experiment)?,
            #[cfg(feature = "experiment_resnet50")]
            "resnet50" => load_or_warn(lua, &experiments_table, "resnet50", resnet50_experiment)?,
            #[cfg(feature = "experiment_resnet101")]
            "resnet101" => {
                load_or_warn(lua, &experiments_table, "resnet101", resnet101_experiment)?
            }
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

    Ok(experiments_table)
}
