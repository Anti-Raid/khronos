use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, multioption::MultiOption};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditChannelPermissions {
    pub channel_id: serenity::all::GenericChannelId,
    pub target_id: serenity::all::TargetId,
    pub allow: MultiOption<Permissions>,
    pub deny: MultiOption<Permissions>,
    #[serde(rename = "type")]
    pub kind: u8,
    pub reason: String,
}

impl ApiReq for EditChannelPermissions {
    type Resp = ();

    /// Edits permission overrides for a channel target after validating the request and the bot's permissions.
    ///
    /// Validates the provided reason, ensures the current bot user exists and has the required `MANAGE_ROLES` permission on the channel, verifies that any requested allow/deny permissions are permitted for the bot, and then sends the edit request to the controller.
    ///
    /// # Returns
    ///
    /// `()` on success, or a `crate::Error` describing why the operation failed.
    ///
    /// # Examples
    ///
    /// ```
    /// use serenity::all::{GenericChannelId, TargetId, Permissions};
    /// use crate::api::channels::EditChannelPermissions;
    /// use crate::multioption::MultiOption;
    ///
    /// # async fn example(context: &crate::context::DiscordContext<impl crate::controller::DiscordProvider>) -> Result<(), crate::Error> {
    /// let req = EditChannelPermissions {
    ///     channel_id: GenericChannelId(123),
    ///     target_id: TargetId::User(456),
    ///     allow: MultiOption::Some(Permissions::SEND_MESSAGES),
    ///     deny: MultiOption::None,
    ///     kind: 0,
    ///     reason: "Adjusting permissions".into(),
    /// };
    ///
    /// req.execute(context).await?;
    /// # Ok(()) }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        let (_partial_guild, _bot_member, _guild_channel, perms) = this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_ROLES)
        .await?;

        if let Some(allow_permissions) = self.allow.as_inner_ref() {
            for perm in allow_permissions.iter() {
                if !perms.contains(perm) {
                    return Err(format!("Bot does not have permission to allow: {perm:?}").into());
                }
            }
        }

        if let Some(deny_permissions) = self.deny.as_inner_ref() {
            for perm in deny_permissions.iter() {
                if !perms.contains(perm) {
                    return Err(format!("Bot does not have permission to deny: {perm:?}").into());
                }
            }
        }

        this
            .controller()
            .edit_channel_permissions(
                self.channel_id,
                self.target_id,
                serde_json::json!({
                    "allow": self.allow,
                    "deny": self.deny,
                    "type": self.kind,
                }),
                Some(self.reason.as_str())
            )
            .await?;

        Ok(())
    }

    /// Convert this request into the API enum variant used by the request dispatcher.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // given an EditChannelPermissions request `req`
    /// // let req = EditChannelPermissions { /* fields */ };
    /// let api = req.to_apilist();
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditChannelPermissions(self)
    }
}