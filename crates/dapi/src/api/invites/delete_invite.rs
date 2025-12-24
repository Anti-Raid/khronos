use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteInvite {
    pub code: String,
    pub reason: String,
}

impl ApiReq for DeleteInvite {
    type Resp = serde_json::Value;

    /// Deletes the invite identified by `self.code` after validating inputs and permissions.
    ///
    /// Validates the provided reason, ensures the invite belongs to the current guild, verifies the bot has either the Manage Server (Manage Guild) permission or Manage Channels permission for the invite's channel, deletes the invite using the controller, and returns the deleted invite as JSON.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dapi::api::invites::delete_invite::DeleteInvite;
    /// # use dapi::DiscordContext;
    /// let req = DeleteInvite { code: "abc".into(), reason: "cleanup".into() };
    /// // `ctx` must be a valid `DiscordContext` implementation
    /// let deleted = futures::executor::block_on(async { req.execute(&ctx).await }).unwrap();
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };    

        // Call get_invite to find the channel id
        let invite_json = this.controller()
            .get_invite(&self.code, false, false, None)
            .await?;

        let invite = serde_json::from_value::<serenity::all::Invite>(invite_json)?;

        if let Some(guild) = invite.guild {
            if guild.id != this.guild_id() {
                return Err("Invite does not belong to the current guild".into());
            }
        }

        let (_partial_guild, _bot_member, _channel, perms) = this.check_channel_permissions(bot_user.id, invite.channel.id.widen(), Permissions::empty())
            .await?;

        let has_perms = perms.manage_guild() || perms.manage_channels();

        if !has_perms {
            return Err("Bot does not have permission to manage channels (either Manage Server globally or Manage Channels on the channel level)".into());
        }

        let invite = this.controller()
        .delete_invite(&self.code, Some(self.reason.as_str()))
        .await?;

        Ok(invite)
    }

    /// Wraps this request value in the crate's API enum as the `API::DeleteInvite` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::invites::delete_invite::DeleteInvite;
    ///
    /// let req = DeleteInvite { code: "abc123".into(), reason: "cleanup".into() };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::DeleteInvite(_) => {},
    ///     _ => panic!("expected API::DeleteInvite variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteInvite(self)
    }
}