use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::channels::ModifyChannelPosition};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildChannelPositions {
    pub data: Vec<ModifyChannelPosition>,
}

impl ApiReq for ModifyGuildChannelPositions {
    type Resp = ();

    /// Modifies the ordering of channels in a guild after validating the bot's permissions.
    ///
    /// The method ensures the current bot user is available, verifies the bot has the
    /// `MANAGE_CHANNELS` permission, and applies the provided channel position changes
    /// via the controller. On success it returns `()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # async fn _example(ctx: &DiscordContext<impl DiscordProvider>) -> Result<(), crate::Error> {
    /// let req = ModifyGuildChannelPositions { data: vec![] };
    /// req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(bot_user.id, Permissions::MANAGE_CHANNELS)
            .await?;

        this.controller()
            .modify_guild_channel_positions(self.data.iter())
            .await?;

        Ok(())
    }

    /// Convert this request into the corresponding `crate::apilist::API` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = crate::api::guilds::modify_guild_channel_positions::ModifyGuildChannelPositions { data: Vec::new() };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::ModifyGuildChannelPositions(_) => {}
    ///     _ => panic!("unexpected API variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyGuildChannelPositions(self)
    }
}