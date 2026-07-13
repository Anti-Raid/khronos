use crate::{AnyId, ApiReq, ChannelId, Permissions, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DeleteChannelPermission {
    pub channel_id: ChannelId,
    pub overwrite_id: AnyId,
    pub reason: String,
}

impl ApiReq for DeleteChannelPermission {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::MANAGE_ROLES)
        .await?;

        this
            .controller()
            .delete_channel_permission(self.channel_id, self.overwrite_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteChannelPermission(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
