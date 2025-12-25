use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, Default, serde::Deserialize)]
pub struct DeleteAutoModerationRule {
    pub rule_id: serenity::all::RuleId,
    pub reason: String,
}

impl ApiReq for DeleteAutoModerationRule {
    type Resp = ();

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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteAutoModerationRule(self)
    }
}
