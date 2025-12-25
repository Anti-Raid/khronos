use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateInvite};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateChannelInvite {
    pub channel_id: serenity::all::GenericChannelId,
    pub data: CreateInvite,
    pub reason: String,
}

impl ApiReq for CreateChannelInvite {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::CREATE_INSTANT_INVITE)
        .await?;

        let invite = this
            .controller()
            .create_channel_invite(self.channel_id, &self.data, Some(self.reason.as_str()))
            .await?;

        Ok(invite)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateChannelInvite(self)
    }
}
