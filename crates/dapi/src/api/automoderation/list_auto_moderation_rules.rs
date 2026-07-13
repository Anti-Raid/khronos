use crate::{ApiReq, Permissions, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, Default, serde::Deserialize)]
pub struct ListAutoModerationRules {
}

impl ApiReq for ListAutoModerationRules {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::MANAGE_GUILD)
        .await?;

        let rules = this
            .controller()
            .list_auto_moderation_rules()
            .await?;

        Ok(rules)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ListAutoModerationRules(self)
    }
}