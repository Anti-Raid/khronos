use crate::{ApiReq, Permissions, ChannelId, MessageId, context::DiscordContext, controller::{DiscordProvider, SuperUserMessageTransform, SuperUserMessageTransformFlags}, types::EditMessage};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EditMessageRequest {
    pub channel_id: ChannelId,
    pub message_id: MessageId,
    pub data: EditMessage,
}

impl ApiReq for EditMessageRequest {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(mut self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        {
            // Apply superuser transformation to the message before sending, if applicable
            let transform = this
            .controller().
            superuser_transform_message_before_send(SuperUserMessageTransform {
                embeds: self.data.embeds.unwrap_or_default(),
                content: self.data.content
            }, SuperUserMessageTransformFlags::IS_EDIT)?;
            self.data.embeds = Some(transform.embeds);
            self.data.content = transform.content;
        }

        let bot_user = this.current_user();

        this.check_channel_permissions(bot_user.id, self.channel_id, Permissions::MANAGE_MESSAGES)
            .await?;

        let msg = this.controller()
            .edit_message(self.channel_id, self.message_id, &self.data)
            .await?;

        Ok(msg)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::EditMessage(self)
    }
}
