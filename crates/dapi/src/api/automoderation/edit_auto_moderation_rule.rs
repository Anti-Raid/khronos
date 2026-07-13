use crate::{ApiReq, Permissions, RuleId, context::DiscordContext, controller::DiscordProvider, types::EditAutoModRule};

#[derive(Debug, serde::Serialize, Default, serde::Deserialize)]
pub struct EditAutoModerationRule {
    pub rule_id: RuleId,
    pub reason: String,
    pub data: EditAutoModRule,
}

impl ApiReq for EditAutoModerationRule {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::MANAGE_GUILD)
            .await?;

        self.data.validate()?;

        let rule = this
            .controller()
            .edit_auto_moderation_rule(self.rule_id, &self.data, Some(self.reason.as_str()))
            .await?;

        Ok(rule)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditAutoModerationRule(self)
    }
}
