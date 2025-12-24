use serenity::all::{Permissions, RuleId};

use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct GetAutoModerationRuleOptions {
    pub rule_id: RuleId
}

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct GetAutoModerationRule {
    pub data: GetAutoModerationRuleOptions,
}

impl ApiReq for GetAutoModerationRule {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

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