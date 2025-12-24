use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::channels::ModifyChannelPosition};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildChannelPositions {
    pub data: Vec<ModifyChannelPosition>,
}

impl ApiReq for ModifyGuildChannelPositions {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(bot_user.id, Permissions::MANAGE_CHANNELS)
            .await?;

        this.controller()
            .modify_guild_channel_positions(self.data.iter())
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyGuildChannelPositions(self)
    }
}
