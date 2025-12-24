use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteAllReactionsForEmoji {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
}

impl ApiReq for DeleteAllReactionsForEmoji {
    type Resp = ();

    /// Delete all reactions that match the given emoji from a message in the specified channel.
    ///
    /// This verifies the bot's presence and that it has the `MANAGE_MESSAGES` permission on the channel,
    /// then requests the controller to remove every reaction instance of the provided emoji from the message.
    /// Returns an error if the current bot user is missing, the permission check fails, or the controller call fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # async fn example<T: crate::DiscordProvider>(ctx: &crate::DiscordContext<T>) -> Result<(), crate::Error> {
    /// use crate::api::reactions::DeleteAllReactionsForEmoji;
    /// use serenity::all::{GenericChannelId, MessageId};
    /// use crate::ReactionType;
    ///
    /// let req = DeleteAllReactionsForEmoji {
    ///     channel_id: GenericChannelId(123),
    ///     message_id: MessageId(456),
    ///     reaction: ReactionType::Unicode("👍".into()),
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
            .delete_all_reactions_for_emoji(self.channel_id, self.message_id, &self.reaction.into_serenity())
            .await?;

        Ok(())
    }

    /// Convert this request into its corresponding API enum variant.
    ///
    /// # Returns
    ///
    /// An `crate::apilist::API::DeleteAllReactionsForEmoji` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = DeleteAllReactionsForEmoji { channel_id, message_id, reaction };
    /// let api = req.to_apilist();
    /// matches!(api, crate::apilist::API::DeleteAllReactionsForEmoji(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteAllReactionsForEmoji(self)
    }
}