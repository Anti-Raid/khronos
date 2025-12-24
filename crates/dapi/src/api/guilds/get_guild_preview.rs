use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildPreview;

impl ApiReq for GetGuildPreview {
    type Resp = serde_json::Value;

    /// Fetches the guild preview from the provided Discord context.
    ///
    /// # Returns
    ///
    /// The guild preview as a `serde_json::Value`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dapi::api::guilds::GetGuildPreview;
    /// # async fn example(this: &dapi::DiscordContext<impl dapi::DiscordProvider>) {
    /// let req = GetGuildPreview;
    /// let preview = req.execute(this).await.unwrap();
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let guild_preview = this
            .controller()
            .get_guild_preview()
            .await?;

        Ok(guild_preview)
    }

    /// Converts this request into the corresponding `crate::apilist::API` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = GetGuildPreview;
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::GetGuildPreview(_) => {},
    ///     _ => panic!("unexpected variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildPreview(self)
    }
}