use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateAutoModRule};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateAutoModerationRule {
    pub reason: String,
    pub data: CreateAutoModRule,
}

impl ApiReq for CreateAutoModerationRule {
    type Resp = serde_json::Value;

    /// Creates an auto-moderation rule after validating the request and permissions.
    ///
    /// Validates the provided `reason` and the embedded rule `data`, ensures the current bot user exists
    /// and has the `MANAGE_GUILD` permission, then calls the controller to create and return the rule.
    ///
    /// # Returns
    /// The created auto-moderation rule as a `serde_json::Value`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // In an async context with a valid `DiscordContext`
    /// let req = CreateAutoModerationRule {
    ///     reason: "Block spam messages".into(),
    ///     data: /* CreateAutoModRule value */,
    /// };
    /// let created = req.execute(&ctx).await?;
    /// assert!(created.is_object());
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(bot_user.id, Permissions::MANAGE_GUILD)
            .await?;

        self.data.validate()?;

        let rule = this
            .controller()
            .create_auto_moderation_rule(&self.data, Some(self.reason.as_str()))
            .await?;

        Ok(rule)
    }

    /// Converts this request into the corresponding `crate::apilist::API` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = CreateAutoModerationRule { reason: "spam".into(), data: Default::default() };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::CreateAutoModerationRule(_) => {},
    ///     _ => panic!("expected CreateAutoModerationRule variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateAutoModerationRule(self)
    }
}