use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateReaction {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
}

impl ApiReq for CreateReaction {
    type Resp = ();

    /// Execute the create-reaction request against the configured Discord provider.
    ///
    /// Validates the current bot user exists, ensures the bot has READ_MESSAGE_HISTORY and
    /// ADD_REACTIONS permissions for the target channel, and asks the provider controller
    /// to add the specified reaction to the message.
    ///
    /// # Returns
    ///
    /// `()` on success, `Err(crate::Error)` if the current user is missing, permission checks fail,
    /// or the provider/controller call fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use dapi::api::reactions::CreateReaction;
    /// # async fn example(this: &dapi::DiscordContext<impl dapi::DiscordProvider>) -> Result<(), dapi::Error> {
    /// let req = CreateReaction {
    ///     channel_id: /* ... */ unimplemented!(),
    ///     message_id: /* ... */ unimplemented!(),
    ///     reaction: /* ... */ unimplemented!(),
    /// };
    /// req.execute(this).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::READ_MESSAGE_HISTORY | Permissions::ADD_REACTIONS)
            .await?;

        this.controller()
            .create_reaction(self.channel_id, self.message_id, &self.reaction.into_serenity())
            .await?;

        Ok(())
    }

    /// Wraps this `CreateReaction` request into the API enum variant.
    ///
    /// # Returns
    ///
    /// The `crate::apilist::API::CreateReaction` variant containing this request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let req = CreateReaction { channel_id: todo!(), message_id: todo!(), reaction: todo!() };
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::CreateReaction(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateReaction(self)
    }
}