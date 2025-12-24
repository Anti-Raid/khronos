use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditAutoModRule};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct EditAutoModerationRule {
    pub rule_id: serenity::all::RuleId,
    pub reason: String,
    pub data: EditAutoModRule,
}

impl ApiReq for EditAutoModerationRule {
    type Resp = serde_json::Value;

    /// Edits an auto-moderation rule using the payload and returns the updated rule.
    ///
    /// Validates the provided reason and rule data, ensures the current bot user exists and has
    /// `MANAGE_GUILD` permission, applies the edit via the controller, and returns the resulting rule as JSON.
    ///
    /// # Returns
    ///
    /// `serde_json::Value` representing the updated auto-moderation rule.
    ///
    /// # Examples
    ///
    /// ```
    /// use serenity::all::RuleId;
    /// use dapi::api::automoderation::edit_auto_moderation_rule::EditAutoModerationRule;
    ///
    /// let req = EditAutoModerationRule {
    ///     rule_id: RuleId::from(1),
    ///     reason: String::from("Update rule threshold"),
    ///     data: Default::default(),
    /// };
    ///
    /// assert_eq!(req.reason, "Update rule threshold");
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
            .edit_auto_moderation_rule(self.rule_id, &self.data, Some(self.reason.as_str()))
            .await?;

        Ok(rule)
    }

    /// Create an API list entry representing this edit auto-moderation rule request.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::api::automoderation::edit_auto_moderation_rule::EditAutoModerationRule;
    /// use crate::apilist::API;
    ///
    /// let req = EditAutoModerationRule {
    ///     rule_id: serenity::all::RuleId(0),
    ///     reason: String::new(),
    ///     data: Default::default(),
    /// };
    ///
    /// let api = req.to_apilist();
    /// matches!(api, API::EditAutoModerationRule(_));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditAutoModerationRule(self)
    }
}