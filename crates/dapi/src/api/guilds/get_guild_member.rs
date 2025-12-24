use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildMember {
    pub user_id: serenity::all::UserId,
}

impl ApiReq for GetGuildMember {
    type Resp = serde_json::Value;

    /// Fetches guild member information for the request's `user_id` from the Discord controller.
    ///
    /// # Returns
    ///
    /// The JSON value containing the guild member data on success; returns an error if the controller fails to retrieve the member.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serenity::all::UserId;
    /// # use crate::api::guilds::get_guild_member::GetGuildMember;
    /// # async fn run_example() -> Result<(), Box<dyn std::error::Error>> {
    /// let req = GetGuildMember { user_id: UserId(123) };
    /// // `ctx` must be a valid `DiscordContext` in real usage.
    /// let ctx = /* obtain DiscordContext */ todo!();
    /// let resp = req.execute(&ctx).await?;
    /// println!("{}", resp);
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let data = this.controller()
            .get_guild_member(self.user_id)
            .await?;

        Ok(data)
    }

    /// Convert this request into its corresponding `crate::apilist::API` variant.
    ///
    /// Consumes the request and wraps it in `API::GetGuildMember`.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::guilds::get_guild_member::GetGuildMember;
    /// use serenity::all::UserId;
    ///
    /// let req = GetGuildMember { user_id: UserId::from(1) };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::GetGuildMember(_) => {}
    ///     _ => panic!("expected GetGuildMember variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildMember(self)
    }
}