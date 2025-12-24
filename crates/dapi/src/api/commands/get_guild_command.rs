use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildCommand {
    pub command_id: serenity::all::CommandId,
}

impl ApiReq for GetGuildCommand {
    type Resp = serde_json::Value;

    /// Fetches the stored guild command from the controller as JSON.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::api::commands::GetGuildCommand;
    /// # async fn example<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>, id: serenity::all::CommandId) -> Result<(), crate::Error> {
    /// let req = GetGuildCommand { command_id: id };
    /// let value = req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Returns
    ///
    /// The command as `serde_json::Value`.
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let resp = this.controller()
            .get_guild_command(self.command_id)
            .await?;

        Ok(resp)
    }

    /// Convert the request into the `crate::apilist::API` enum variant for a GetGuildCommand.
    ///
    /// # Examples
    ///
    /// ```
    /// use serenity::all::CommandId;
    /// use crate::api::commands::get_guild_command::GetGuildCommand;
    ///
    /// let req = GetGuildCommand { command_id: CommandId(123) };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::GetGuildCommand(inner) => {
    ///         assert_eq!(inner.command_id, CommandId(123));
    ///     }
    ///     _ => panic!("expected GetGuildCommand variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildCommand(self)
    }
}