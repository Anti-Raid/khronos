use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildBans {
    pub limit: Option<serenity::nonmax::NonMaxU16>,
    pub before: Option<serenity::all::UserId>,
    pub after: Option<serenity::all::UserId>,
}

impl ApiReq for GetGuildBans {
    type Resp = serde_json::Value;

    /// Fetches guild bans as JSON, applying optional pagination and an optional item limit after ensuring the bot has ban permissions.
    ///
    /// The request will be paginated using `before` or `after` if provided (`before` takes precedence). If `limit` is provided it must be less than or equal to 1000.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - the current bot user is not available,
    /// - the bot lacks the `BAN_MEMBERS` permission,
    /// - `limit` is greater than 1000,
    /// - or the underlying controller call fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serenity::all::UserId;
    /// # use serenity::nonmax::NonMaxU16;
    /// # use crates::dapi::api::guilds::GetGuildBans;
    /// // Construct a request for the first 100 bans
    /// let req = GetGuildBans { limit: Some(NonMaxU16::new(100).unwrap()), before: None, after: None };
    /// // `ctx` would be a `&DiscordContext<_>` available in the real runtime
    /// // let bans = req.execute(&ctx).await.unwrap();
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(bot_user.id, Permissions::BAN_MEMBERS)
        .await?;

        let mut target = None;
        if let Some(before) = self.before {
            target = Some(serenity::all::UserPagination::Before(before));
        } else if let Some(after) = self.after {
            target = Some(serenity::all::UserPagination::After(after));
        }

        if let Some(limit) = self.limit {
            if limit.get() > 1000 {
                return Err("Limit must be less than 1000".into());
            }
        }

        let bans = this.controller()
            .get_guild_bans(target, self.limit)
            .await?;

        Ok(bans)
    }

    /// Convert this request into the corresponding `crate::apilist::API` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = crate::api::guilds::get_guild_bans::GetGuildBans {
    ///     limit: None,
    ///     before: None,
    ///     after: None,
    /// };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::GetGuildBans(_) => {}
    ///     _ => panic!("expected GetGuildBans variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildBans(self)
    }
}