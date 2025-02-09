use mlua::prelude::*;

// Image classification experiments
use tch::{
    nn::VarStore,
    vision::imagenet,
    vision::resnet::{resnet101, resnet34, resnet50},
    vision::squeezenet::v1_1,
    vision::vgg::vgg16,
    Device, Kind,
};

fn resolve_image(
    vs: &VarStore,
    lua: &Lua,
    path: &String,
    model: &impl tch::nn::ModuleT,
) -> LuaResult<LuaTable> {
    let image = imagenet::load_image_and_resize224(path)
        .map_err(|e| LuaError::external(e.to_string()))?
        .to_device(vs.device());

    let output = image
        .unsqueeze(0)
        .apply_t(model, false)
        .softmax(-1, Kind::Float);

    let result_table = lua.create_table()?;
    for (probability, class) in imagenet::top(&output, 10).iter() {
        result_table.set(class.as_str(), *probability)?;
    }

    Ok(result_table)
}

pub fn image_classification_experiment(lua: &Lua) -> LuaResult<LuaTable> {
    let image_classification_exp = lua.create_table()?;

    let mut vs_resnet34 = VarStore::new(Device::cuda_if_available());
    let model_resnet34 = resnet34(&vs_resnet34.root(), imagenet::CLASS_COUNT);
    vs_resnet34
        .load("resnet34.bin")
        .map_err(|e| LuaError::external(e.to_string()))?;

    let mut vs_resnet50 = VarStore::new(Device::cuda_if_available());
    let model_resnet_50 = resnet50(&vs_resnet50.root(), imagenet::CLASS_COUNT);
    vs_resnet50
        .load("resnet50.bin")
        .map_err(|e| LuaError::external(e.to_string()))?;

    let mut vs_resnet101 = VarStore::new(Device::cuda_if_available());
    let model_resnet101 = resnet101(&vs_resnet101.root(), imagenet::CLASS_COUNT);
    vs_resnet101
        .load("resnet101.bin")
        .map_err(|e| LuaError::external(e.to_string()))?;

    let mut vs_squeezenet = VarStore::new(Device::cuda_if_available());
    let model_squeezenet = v1_1(&vs_squeezenet.root(), imagenet::CLASS_COUNT);
    vs_squeezenet
        .load("squeezenet1_1.bin")
        .map_err(|e| LuaError::external(e.to_string()))?;

    let mut vs_vgg16 = VarStore::new(Device::cuda_if_available());
    let model_vgg16 = vgg16(&vs_vgg16.root(), imagenet::CLASS_COUNT);
    vs_vgg16
        .load("vgg16.bin")
        .map_err(|e| LuaError::external(e.to_string()))?;

    image_classification_exp.set(
        "classify_image_path",
        lua.create_function(
            move |lua, (model, path): (String, String)| match model.as_str() {
                "resnet34" => resolve_image(&vs_resnet34, lua, &path, &model_resnet34),
                "resnet50" => resolve_image(&vs_resnet50, lua, &path, &model_resnet_50),
                "resnet101" => resolve_image(&vs_resnet101, lua, &path, &model_resnet101),
                "squeezenet1_1" => resolve_image(&vs_squeezenet, lua, &path, &model_squeezenet),
                "vgg16" => resolve_image(&vs_vgg16, lua, &path, &model_vgg16),
                _ => Err(LuaError::external("Unknown model")),
            },
        )?,
    )?;

    Ok(image_classification_exp)
}
