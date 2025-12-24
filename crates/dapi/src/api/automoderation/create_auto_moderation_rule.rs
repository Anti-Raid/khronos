use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateAutoModRule};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct CreateAutoModerationRule {
    pub reason: String,
    pub data: CreateAutoModRule,
}

impl ApiReq for CreateAutoModerationRule {
    type Resp = serde_json::Value;

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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateAutoModerationRule(self)
    }
}
