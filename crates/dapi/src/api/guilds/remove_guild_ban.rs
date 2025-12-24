use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RemoveGuildBan {
    pub user_id: serenity::all::UserId,
    pub reason: String,
}

impl ApiReq for RemoveGuildBan {
    type Resp = ();

    /// Executes the RemoveGuildBan request, removing the guild ban for `user_id` with the supplied `reason`.
    ///
    /// Validates the reason, ensures the bot user is available and has BAN_MEMBERS permission, then delegates to the controller to remove the ban.
    ///
    /// # Returns
    ///
    /// `()` on success; an error when validation fails, the current bot user is missing, permission checks fail, or the controller operation returns an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn run<T: dapi::DiscordProvider>(ctx: &dapi::DiscordContext<T>) -> Result<(), dapi::Error> {
    /// use serenity::all::UserId;
    /// let req = dapi::api::guilds::remove_guild_ban::RemoveGuildBan {
    ///     user_id: UserId(123),
    ///     reason: "Violation of rules".into(),
    /// };
    /// req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(
            bot_user.id,
            Permissions::BAN_MEMBERS,
        )
        .await?;

        this.controller()
            .remove_guild_ban(
                self.user_id,
                Some(self.reason.as_str()),
            )
            .await?;

        Ok(())
    }

    /// Converts this `RemoveGuildBan` request into the global API enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = RemoveGuildBan { user_id: serenity::all::UserId(1), reason: "violation".into() };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::RemoveGuildBan(r) => assert_eq!(r.reason, "violation"),
    ///     _ => panic!("expected RemoveGuildBan variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::RemoveGuildBan(self)
    }
}