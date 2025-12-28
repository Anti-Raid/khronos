use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateGuildBan {
    pub user_id: serenity::all::UserId,
    pub reason: String,
    pub delete_message_seconds: Option<u32>,
}

impl ApiReq for CreateGuildBan {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let delete_message_seconds = {
            if let Some(seconds) = self.delete_message_seconds {
                if seconds > 604800 {
                    return Err("Delete message seconds must be between 0 and 604800".into());
                }

                seconds
            } else {
                0
            }
        };

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions_and_hierarchy(
            bot_user.id,
            self.user_id,
            Permissions::BAN_MEMBERS,
        )
        .await?;

        this.controller()
            .create_guild_ban(
                self.user_id,
                delete_message_seconds,
                Some(self.reason.as_str()),
            )
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildBan(self)
    }

    fn is_primitive_response() -> bool {
        true
    }
}
