use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::CreateMessage};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateMessageRequest {
    pub channel_id: serenity::all::GenericChannelId,
    pub data: CreateMessage,
}

impl ApiReq for CreateMessageRequest {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::SEND_MESSAGES)
            .await?;

        let files = if let Some(ref attachments) = self.data.attachments {
            attachments.take_files()?
        } else {
            Vec::new()
        };

        let msg = this.controller()
            .create_message(self.channel_id, files, &self.data)
            .await?;

        Ok(msg)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateMessage(self)
    }
}
