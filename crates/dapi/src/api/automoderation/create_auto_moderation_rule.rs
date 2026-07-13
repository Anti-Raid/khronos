use crate::{ApiReq, Permissions, context::DiscordContext, controller::DiscordProvider, types::CreateAutoModRule};

#[derive(Debug, serde::Serialize, Default, serde::Deserialize)]
pub struct CreateAutoModerationRule {
    pub reason: String,
    pub data: CreateAutoModRule,
}

impl ApiReq for CreateAutoModerationRule {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::MANAGE_GUILD)
            .await?;

        self.data.validate()?;

        let rule = this
            .controller()
            .create_auto_moderation_rule(&self.data, Some(self.reason.as_str()))
            .await?;

        Ok(rule)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateAutoModerationRule(self)
    }
}
