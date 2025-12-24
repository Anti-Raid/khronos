use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteAllReactions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
}

impl ApiReq for DeleteAllReactions {
    type Resp = ();

    /// Deletes all reactions from a message after verifying the bot has the `MANAGE_MESSAGES` permission.
    ///
    /// The request ensures the current bot user is available, checks that the bot has `MANAGE_MESSAGES` on the target channel, and then asks the controller to remove every reaction from the specified message. Returns an error if the current user is missing, the permission check fails, or the controller operation fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dapi::api::reactions::DeleteAllReactions;
    ///
    /// // Construct the request and call `execute` inside an async context with a DiscordContext `ctx`.
    /// let req = DeleteAllReactions { channel_id: /* GenericChannelId */, message_id: /* MessageId */ };
    /// // req.execute(&ctx).await?;
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .delete_all_reactions(self.channel_id, self.message_id)
            .await?;

        Ok(())
    }

    /// Convert this request into the global API enum as the `DeleteAllReactions` variant.
    ///
    /// # Returns
    ///
    /// An `apilist::API` value containing this request wrapped as `API::DeleteAllReactions`.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = DeleteAllReactions { channel_id, message_id };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::DeleteAllReactions(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteAllReactions(self)
    }
}