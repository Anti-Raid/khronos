use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteMessage {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reason: String,
}

impl ApiReq for DeleteMessage {
    type Resp = ();

    /// Deletes a message in the specified channel using the bot account.
    ///
    /// Performs a permission check for `MANAGE_MESSAGES` for the bot user before calling the controller
    /// to delete the message. Returns `Err` if the current user is not available, the permission check
    /// fails, or the controller fails to delete the message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serenity::all::{GenericChannelId, MessageId};
    /// # use crate::api::messages::delete_message::DeleteMessage;
    /// # async fn example(ctx: &crate::DiscordContext<impl crate::DiscordProvider>) -> Result<(), crate::Error> {
    /// let req = DeleteMessage {
    ///     channel_id: GenericChannelId(123),
    ///     message_id: MessageId(456),
    ///     reason: "cleanup".into(),
    /// };
    /// req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .delete_message(self.channel_id, self.message_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    /// Converts this `DeleteMessage` request into the crate's `apilist::API::DeleteMessage` variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = DeleteMessage {
    ///     channel_id: (1u64).into(),
    ///     message_id: (2u64).into(),
    ///     reason: "cleanup".into(),
    /// };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::DeleteMessage(_) => {}
    ///     _ => panic!("expected DeleteMessage variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteMessage(self)
    }
}