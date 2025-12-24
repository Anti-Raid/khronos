use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteOwnReaction {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
}

impl ApiReq for DeleteOwnReaction {
    type Resp = ();

    /// Executes the DeleteOwnReaction request, removing the bot's own reaction from a message.
    ///
    /// This will verify the current bot user and ensure the bot has the required channel permissions
    /// before attempting to remove the reaction. Errors are returned if the current user is unavailable,
    /// the permission check fails, or the controller call fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dapi::api::reactions::DeleteOwnReaction;
    /// # use dapi::types::{GenericChannelId, MessageId, ReactionType};
    /// let req = DeleteOwnReaction {
    ///     channel_id: GenericChannelId(1),
    ///     message_id: MessageId(1),
    ///     reaction: ReactionType::Unicode("👍".into()),
    /// };
    /// // executor.run(async { req.execute(&context).await.unwrap(); });
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::READ_MESSAGE_HISTORY | Permissions::ADD_REACTIONS)
            .await?;

        this.controller()
            .delete_own_reaction(self.channel_id, self.message_id, &self.reaction.into_serenity())
            .await?;

        Ok(())
    }

    /// Converts this request into the corresponding `crate::apilist::API` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::reactions::delete_own_reaction::DeleteOwnReaction;
    ///
    /// let req = DeleteOwnReaction {
    ///     channel_id: /* GenericChannelId value */,
    ///     message_id: /* MessageId value */,
    ///     reaction: /* ReactionType value */,
    /// };
    ///
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::DeleteOwnReaction(_) => {},
    ///     _ => panic!("expected DeleteOwnReaction variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteOwnReaction(self)
    }
}