use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CrosspostMessage {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for CrosspostMessage {
    type Resp = serde_json::Value;

    /// Crossposts a message in the specified channel and returns the resulting message payload.
    ///
    /// Verifies that the current bot user is available and that it has `SEND_MESSAGES` and
    /// `MANAGE_MESSAGES` on the target channel before delegating to the controller. Returns an error
    /// if the current user is missing, the permission check fails, or the controller call fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dapi::api::messages::CrosspostMessage;
    /// # use dapi::types::{GenericChannelId, MessageId};
    /// # async fn run(ctx: &dapi::DiscordContext<impl dapi::DiscordProvider>) -> Result<(), Box<dyn std::error::Error>> {
    /// let req = CrosspostMessage {
    ///     channel_id: GenericChannelId(123456789012345678),
    ///     message_id: MessageId(987654321098765432),
    /// };
    /// let payload = req.execute(ctx).await?; // `payload` is the JSON response for the crossposted message
    /// # Ok(()) }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::SEND_MESSAGES | Permissions::MANAGE_MESSAGES)
            .await?;

        let msg = this.controller()
            .crosspost_message(self.channel_id, self.message_id)
            .await?;

        Ok(msg)
    }

    /// Wraps this request in the `apilist::API::CrosspostMessage` enum variant.
    ///
    /// # Returns
    ///
    /// `crate::apilist::API::CrosspostMessage` containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// use dapi::api::messages::crosspost_message::CrosspostMessage;
    /// use dapi::apilist::API;
    /// use serenity::model::id::{ChannelId, MessageId};
    ///
    /// let req = CrosspostMessage {
    ///     channel_id: ChannelId(1).into(),
    ///     message_id: MessageId(2),
    /// };
    ///
    /// match req.to_apilist() {
    ///     API::CrosspostMessage(inner) => {
    ///         // `inner` is the original `CrosspostMessage`
    ///         let _ = inner;
    ///     }
    ///     _ => panic!("unexpected variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CrosspostMessage(self)
    }
}