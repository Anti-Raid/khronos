use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateMessage};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateMessageRequest {
    pub channel_id: serenity::all::GenericChannelId,
    pub data: CreateMessage,
}

impl ApiReq for CreateMessageRequest {
    type Resp = serde_json::Value;

    /// Creates a message in the specified channel after validating input, checking permissions, and handling attachments.
    ///
    /// Validates `data`, ensures the current bot user is available, verifies the bot has `SEND_MESSAGES` permission in the target channel, extracts any attachment files, and delegates message creation to the controller.
    ///
    /// # Returns
    /// `serde_json::Value` representing the created message.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use serenity::all::GenericChannelId;
    /// # use dapi::api::messages::CreateMessageRequest;
    /// # use dapi::CreateMessage;
    /// # async fn example(ctx: &dapi::DiscordContext<impl dapi::DiscordProvider>) -> Result<(), Box<dyn std::error::Error>> {
    /// let data = CreateMessage { /* fields */ };
    /// let req = CreateMessageRequest { channel_id: GenericChannelId(123), data };
    /// let created = req.execute(ctx).await?;
    /// println!("{}", created);
    /// # Ok(()) }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::SEND_MESSAGES)
            .await?;

        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        let msg = this.controller()
            .create_message(self.channel_id, files, &self.data)
            .await?;

        Ok(msg)
    }

    /// Convert this request into the `API::CreateMessage` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// // assuming `req` is a `CreateMessageRequest`
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::CreateMessage(_) => (),
    ///     _ => panic!("expected CreateMessage variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateMessage(self)
    }
}