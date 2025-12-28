use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteChannelPermission {
    pub channel_id: serenity::all::GenericChannelId,
    pub overwrite_id: serenity::all::TargetId,
    pub reason: String,
}

impl ApiReq for DeleteChannelPermission {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

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
