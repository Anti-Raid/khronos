use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditMessage};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct EditMessageRequest {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub data: EditMessage,
}

impl ApiReq for EditMessageRequest {
    type Resp = serde_json::Value;

    /// Edits a message in the specified channel using the provided edit data and returns the edited message.
    ///
    /// The request data is validated before performing the edit. The operation requires the bot to have
    /// the `MANAGE_MESSAGES` permission in the target channel; otherwise an error is returned. If the
    /// edit includes attachments, they are sent with the edit. On success, the edited message is
    /// returned as JSON.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde_json::Value;
    /// # async fn example<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>) -> Result<(), crate::Error> {
    /// let req = crate::api::messages::edit_message::EditMessageRequest {
    ///     channel_id: /* GenericChannelId */ 0.into(),
    ///     message_id: /* MessageId */ 0.into(),
    ///     data: /* EditMessage */ Default::default(),
    /// };
    /// let edited: Value = req.execute(ctx).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        let msg = this.controller()
            .edit_message(self.channel_id, self.message_id, files, &self.data)
            .await?;

        Ok(msg)
    }

    /// Convert this request into the corresponding API enum variant for dispatch.
    ///
    /// # Examples
    ///
    /// ```
    /// // Build an `EditMessageRequest` and convert it into the API enum:
    /// // let req = EditMessageRequest { channel_id, message_id, data };
    /// // let api = req.to_apilist();
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditMessage(self)
    }
}