use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BulkDeleteMessages {
    pub channel_id: serenity::all::GenericChannelId,
    pub messages: Vec<serenity::all::MessageId>,
    pub reason: String,
}

impl ApiReq for BulkDeleteMessages {
    type Resp = ();

    /// Executes a bulk deletion of messages in the specified channel.
    ///
    /// Verifies the current bot user exists and has the `MANAGE_MESSAGES` permission on the target channel,
    /// then asks the controller to delete the provided messages with an optional reason.
    ///
    /// # Errors
    /// Returns an error if the current user is not available, the permission check fails, or the controller operation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dapi::api::messages::BulkDeleteMessages;
    /// # async fn example<T: dapi::DiscordProvider>(ctx: &dapi::DiscordContext<T>) -> Result<(), dapi::Error> {
    /// let req = BulkDeleteMessages {
    ///     channel_id: /* GenericChannelId value */ unimplemented!(),
    ///     messages: vec![/* MessageId values */],
    ///     reason: String::from("moderation cleanup"),
    /// };
    ///
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
            .bulk_delete_messages(self.channel_id, serde_json::json!({"messages": self.messages}), Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    /// Convert the request into the `crate::apilist::API::BulkDeleteMessages` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// // Construct a request (fields shown for clarity; actual constructors may differ)
    /// let req = crate::api::messages::BulkDeleteMessages {
    ///     channel_id: /* GenericChannelId */ 0.into(),
    ///     messages: vec![/* MessageId */ 1.into()],
    ///     reason: String::from("cleanup"),
    /// };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::BulkDeleteMessages(_) => {}
    ///     _ => panic!("expected BulkDeleteMessages variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::BulkDeleteMessages(self)
    }
}