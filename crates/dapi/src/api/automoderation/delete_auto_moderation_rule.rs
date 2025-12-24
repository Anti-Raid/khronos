use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteAutoModerationRule {
    pub rule_id: serenity::all::RuleId,
    pub reason: String,
}

impl ApiReq for DeleteAutoModerationRule {
    type Resp = ();

    /// Deletes an auto moderation rule after validating the provided reason and verifying permissions.
    ///
    /// Validates `reason` with the context, ensures the current bot user is available and has the `MANAGE_GUILD` permission, then instructs the controller to delete the rule identified by `self.rule_id`.
    ///
    /// # Errors
    ///
    /// Returns an error if reason validation fails, the current user is not found, the bot lacks `MANAGE_GUILD` permission, or the controller fails to delete the rule.
    ///
    /// # Examples
    ///
    /// ```
    /// // In an async context:
    /// // let req = DeleteAutoModerationRule { rule_id, reason: "no longer needed".into() };
    /// // req.execute(&ctx).await?;
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(bot_user.id, Permissions::MANAGE_GUILD)
            .await?;

        this
            .controller()
            .delete_auto_moderation_rule(self.rule_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    /// Convert this request into its API list representation.
    ///
    /// This consumes the request and wraps it in the `crate::apilist::API::DeleteAutoModerationRule` variant,
    /// which is used by the API routing layer.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let req = DeleteAutoModerationRule {
    ///     rule_id: /* a RuleId value */,
    ///     reason: "violation".to_string(),
    /// };
    /// let api = req.to_apilist();
    /// // `api` is now `crate::apilist::API::DeleteAutoModerationRule(...)`
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteAutoModerationRule(self)
    }
}