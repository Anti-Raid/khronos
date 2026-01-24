use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::{CreateEmbedFooter, CreateMessage}, validator::MESSAGE_CONTENT_LIMIT};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateMessageRequest {
    pub channel_id: serenity::all::GenericChannelId,
    pub data: CreateMessage,
}

const DISCLAIMER: &str = "Content provided by users is the sole responsibility of the author. AntiRaid does not monitor, verify, or endorse any user-generated messages.";

impl ApiReq for CreateMessageRequest {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(mut self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        self.data.validate()?;

        {
            // Insert disclaimer into all embeds about being user-generated content
            let mut disclaimer_handled = false;
            for embed in self.data.embeds.iter_mut() {
                if let Some(footer) = &mut embed.footer {
                    footer.text = format!("{}\n{}", footer.text, DISCLAIMER);
                } else {
                    embed.footer = Some(CreateEmbedFooter {
                        text: DISCLAIMER.to_string(),
                        icon_url: None,
                    });
                }
                disclaimer_handled = true;
            }

            // Insert disclaimer into content
            if !disclaimer_handled {
                if let Some(content) = &mut self.data.content {
                    let required_space = DISCLAIMER.len() + 4; // +4 for \n\n**
                    if content.len() > required_space {
                        content.truncate(MESSAGE_CONTENT_LIMIT - required_space);
                    }
                    content.push_str(format!("\n\n*{}*", DISCLAIMER).as_str());
                } else {
                    // Edge case: No embed AND no content (e.g. file upload)? 
                    // Create content with just the disclaimer.
                    self.data.content = Some(format!("*{}*", DISCLAIMER));
                }
            } 
        }

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
