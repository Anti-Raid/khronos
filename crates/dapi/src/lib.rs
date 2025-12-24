use crate::{context::DiscordContext, controller::DiscordProvider};

pub mod context;
pub mod controller;
pub mod serenity_backports;
pub mod antiraid_check_permissions;
pub mod antiraid_check_channel_permissions;
pub mod antiraid_get_fused_member;
pub mod types;
pub mod api;
pub mod apilist;
pub mod multioption;

pub type Error = Box<dyn std::error::Error + Send + Sync>; // This is constant and should be copy pasted

#[allow(async_fn_in_trait)]
pub trait ApiReq {
    type Resp: 'static + serde::Serialize + for<'de> serde::Deserialize<'de> + Send;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, Error>;

    // Convert req to ApiList
    fn to_apilist(self) -> apilist::API;
}

#[inline(always)]
pub async fn exec_api<A: ApiReq, T: DiscordProvider>(
    this: &DiscordContext<T>,
    req: A,
) -> Result<A::Resp, crate::Error> {
    req.execute(this).await
}



/// Helper function to extract image format from a data URL
pub fn get_format_from_image_data<'a>(data: &'a str) -> Result<&'a str, crate::Error> {
    if !data.starts_with("data:image/") {
        return Err("Image must be a data URL".into());
    }

    let Some(format) = data.split(";").next() else {
        return Err("Image is not a valid data URL".into());
    };

    let Some(format) = format.split("/").nth(1) else {
        return Err("No format found in data URL".into());
    };

    Ok(format)
}
