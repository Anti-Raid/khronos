use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RemoveGuildMember {
    pub user_id: serenity::all::UserId,
    pub reason: String,
}

impl ApiReq for RemoveGuildMember {
    type Resp = ();

    /// Removes a guild member after validating the reason and verifying bot permissions and hierarchy.
    ///
    /// Validates the provided `reason`, ensures the current bot user is available, checks that the bot has
    /// `KICK_MEMBERS` permission and the required hierarchy over the target `user_id`, then instructs the
    /// controller to remove the guild member with the given reason.
    ///
    /// # Returns
    ///
    /// `()` on success.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>) -> Result<(), crate::Error> {
    /// let req = crate::api::guilds::RemoveGuildMember {
    ///     user_id: serenity::all::UserId(123),
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

        this.check_permissions_and_hierarchy(
            bot_user.id,
            self.user_id,
            Permissions::KICK_MEMBERS,
        )
        .await?;

        this.controller()
            .remove_guild_member(self.user_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    /// Converts this request into the crate's `apilist::API` enum as the `RemoveGuildMember` variant.
    ///
    /// # Returns
    ///
    /// The `apilist::API::RemoveGuildMember` variant wrapping this request.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::guilds::remove_guild_member::RemoveGuildMember;
    /// use serenity::all::UserId;
    ///
    /// let req = RemoveGuildMember { user_id: UserId(1), reason: "violation".into() };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::RemoveGuildMember(r) => {
    ///         let _orig: RemoveGuildMember = r;
    ///     }
    ///     _ => panic!("expected RemoveGuildMember variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::RemoveGuildMember(self)
    }
}