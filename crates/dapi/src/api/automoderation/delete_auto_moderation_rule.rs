use crate::{ApiReq, Permissions, RuleId, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteAutoModerationRule {
    pub rule_id: RuleId,
    pub reason: String,
}

impl ApiReq for DeleteAutoModerationRule {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::MANAGE_GUILD)
            .await?;

        this
            .controller()
            .delete_auto_moderation_rule(self.rule_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteAutoModerationRule(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
