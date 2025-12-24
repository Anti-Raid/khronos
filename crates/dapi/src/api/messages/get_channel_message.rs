use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetChannelMessage {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for GetChannelMessage {
    type Resp = serde_json::Value;

    /// Fetches a single message from a channel after validating required channel permissions for the bot.
    ///
    /// Returns the retrieved message as JSON on success. Errors if the current bot user is not available,
    /// if the bot lacks the permission to connect to a voice channel when the target channel is voice,
    /// or if the underlying controller fails to retrieve the message.
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn example_usage(ctx: &crate::DiscordContext<impl crate::DiscordProvider>) -> Result<(), crate::Error> {
    /// use serenity::all::{GenericChannelId, MessageId};
    /// use crate::api::messages::get_channel_message::GetChannelMessage;
    ///
    /// let req = GetChannelMessage {
    ///     channel_id: GenericChannelId(123.into()),
    ///     message_id: MessageId(456.into()),
    /// };
    ///
    /// let msg_json = req.execute(ctx).await?;
    /// // `msg_json` is a serde_json::Value representing the message.
    /// # Ok(()) }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        // Perform required checks
        let (_, _, guild_channel, perms) = this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::VIEW_CHANNEL).await?;

        if guild_channel.base.kind == serenity::all::ChannelType::Voice 
        && !perms.connect() {
            return Err("Bot does not have permission to connect to the given voice channel".into());
        }

        let msg = this.controller()
            .get_channel_message(self.channel_id, self.message_id)
            .await?;

        Ok(msg)
    }

    /// Converts this request into the crate's API enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = GetChannelMessage {
    ///     channel_id: serenity::all::GenericChannelId(1),
    ///     message_id: serenity::all::MessageId(1),
    /// };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::GetChannelMessage(_) => (),
    ///     _ => panic!("unexpected variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetChannelMessage(self)
    }
}