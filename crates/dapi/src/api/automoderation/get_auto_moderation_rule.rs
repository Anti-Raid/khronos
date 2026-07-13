use crate::{ApiReq, Permissions, RuleId, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, Default, serde::Deserialize)]
pub struct GetAutoModerationRuleOptions {
    pub rule_id: RuleId
}

#[derive(Debug, serde::Serialize, Default, serde::Deserialize)]
pub struct GetAutoModerationRule {
    pub data: GetAutoModerationRuleOptions,
}

impl ApiReq for GetAutoModerationRule {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::MANAGE_GUILD)
        .await?;

        let rule = this
            .controller()
            .get_auto_moderation_rule(self.data.rule_id)
            .await?;

        Ok(rule)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetAutoModerationRule(self)
    }
}