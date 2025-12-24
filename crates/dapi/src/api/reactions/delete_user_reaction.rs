use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteUserReaction {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
    pub user_id: serenity::all::UserId,
}

impl ApiReq for DeleteUserReaction {
    type Resp = ();

    /// Executes the DeleteUserReaction request: verifies the bot has MANAGE_MESSAGES on the channel and removes the specified user's reaction from the message.
    ///
    /// On success the requested reaction is removed; on failure an error is returned.
    ///
    /// # Errors
    ///
    /// Returns an error if the current bot user is not available, if the bot lacks `MANAGE_MESSAGES` for the channel, or if the underlying provider/controller call fails.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Construct and execute the request (example, provider and context wiring omitted)
    /// let req = DeleteUserReaction {
    ///     channel_id: /* channel id */,
    ///     message_id: /* message id */,
    ///     reaction: /* ReactionType */,
    ///     user_id: /* user id */,
    /// };
    /// req.execute(&discord_context).await?;
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        this.controller()
            .delete_user_reaction(self.channel_id, self.message_id, self.user_id, &self.reaction.into_serenity())
            .await?;

        Ok(())
    }

    /// Converts this `DeleteUserReaction` request into its API enum representation.
    ///
    /// # Returns
    ///
    /// The `crate::apilist::API::DeleteUserReaction` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```
    /// use dapi::api::reactions::delete_user_reaction::DeleteUserReaction;
    /// use serenity::all::{GenericChannelId, MessageId, UserId};
    /// use dapi::model::ReactionType;
    ///
    /// let req = DeleteUserReaction {
    ///     channel_id: GenericChannelId(1),
    ///     message_id: MessageId(2),
    ///     reaction: ReactionType::Unicode { name: "👍".into() },
    ///     user_id: UserId(3),
    /// };
    ///
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::DeleteUserReaction(r) => {
    ///         let _ = r; // `r` is the original request moved into the enum
    ///     }
    ///     _ => panic!("unexpected variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteUserReaction(self)
    }
}