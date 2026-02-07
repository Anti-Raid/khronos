use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteInvite {
    pub code: String,
    pub reason: String,
}

impl ApiReq for DeleteInvite {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };    

        // Call get_invite to find the channel id
        let invite_json = this.controller()
            .get_invite(&self.code, false, false, None)
            .await?;

        let invite = serde_json::from_value::<serenity::all::Invite>(invite_json)?;

        if let Some(guild) = invite.guild {
            if guild.id != this.controller().guild_context()? {
                return Err("Invite does not belong to the current guild".into());
            }
        }

        let (_partial_guild, _bot_member, _channel, perms) = this.check_channel_permissions(bot_user.id, invite.channel.id.widen(), Permissions::empty())
            .await?;

        let has_perms = perms.manage_guild() || perms.manage_channels();

        if !has_perms {
            return Err("Bot does not have permission to manage channels (either Manage Server globally or Manage Channels on the channel level)".into());
        }

        let invite = this.controller()
        .delete_invite(&self.code, Some(self.reason.as_str()))
        .await?;

        Ok(invite)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::DeleteInvite(self)
    }
}
