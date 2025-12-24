use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::ReactionType};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetReactions {
    pub channel_id: serenity::all::GenericChannelId,
    pub message_id: serenity::all::MessageId,
    pub reaction: ReactionType,
    pub burst: Option<bool>,
    pub after: Option<serenity::all::UserId>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

impl ApiReq for GetReactions {
    type Resp = serde_json::Value;

    /// Execute the GetReactions request against the provided Discord context.
    ///
    /// This verifies the current bot user is present, checks channel permissions for the bot,
    /// delegates to the controller to fetch users who reacted to the specified message, and
    /// returns the controller's JSON response.
    ///
    /// # Returns
    ///
    /// The JSON value produced by the controller representing the reaction users.
    ///
    /// # Errors
    ///
    /// Returns a `crate::Error` if the current bot user is not available, if the permission
    /// check fails, or if the controller call returns an error.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use crate::api::reactions::GetReactions;
    /// use serenity::all::{GenericChannelId, MessageId, UserId};
    ///
    /// // Build a request (fields shown for illustration)
    /// let req = GetReactions {
    ///     channel_id: GenericChannelId(123),
    ///     message_id: MessageId(456),
    ///     reaction: /* ReactionType value */,
    ///     burst: None,
    ///     after: Some(UserId(789)),
    ///     limit: None,
    /// };
    ///
    /// // Execute against a DiscordContext (placeholder)
    /// // let resp = req.execute(&discord_context).await?;
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, serenity::all::Permissions::empty())
            .await?;

        let users = this.controller()
            .get_reactions(
                self.channel_id, 
                self.message_id,
                &self.reaction.into_serenity(),
                self.burst,
                self.after,
                self.limit,
            )
            .await?;

        Ok(users)
    }

    /// Convert this request into the API enum variant that represents it.
    ///
    /// # Returns
    ///
    /// An `API` enum value wrapping this `GetReactions` request as `API::GetReactions`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::api::reactions::get_reactions::GetReactions;
    /// # fn make_req() -> GetReactions { unsafe { std::mem::zeroed() } }
    /// let req = make_req();
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::GetReactions(_) => (),
    ///     _ => panic!("expected API::GetReactions"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetReactions(self)
    }
}