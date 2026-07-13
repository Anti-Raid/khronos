use crate::{ApiReq, Permissions, context::DiscordContext, controller::DiscordProvider, types::channels::ModifyChannelPosition};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildChannelPositions {
    pub data: Vec<ModifyChannelPosition>,
}

impl ApiReq for ModifyGuildChannelPositions {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::MANAGE_CHANNELS)
            .await?;

        this.controller()
            .modify_guild_channel_positions(&self.data)
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyGuildChannelPositions(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
