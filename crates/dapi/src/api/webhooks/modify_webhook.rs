use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditWebhook, get_format_from_image_data};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyWebhook {
    pub webhook_id: serenity::all::WebhookId,
    pub data: EditWebhook,
    pub reason: String,
}

impl ApiReq for ModifyWebhook {
    type Resp = serde_json::Value;

    /// Modifies the specified webhook after validating the reason, avatar format, ownership, and required permissions.
    ///
    /// Performs these checks in order: validates `reason`; if an avatar is provided, ensures its format is PNG, JPEG, or GIF; verifies the current bot user exists; if `channel_id` is present, checks channel-specific permissions; checks the `MANAGE_WEBHOOKS` permission; verifies the webhook belongs to the current guild; then applies the modification and returns the updated webhook payload.
    ///
    /// # Returns
    ///
    /// The modified webhook payload as JSON (`serde_json::Value`).
    ///
    /// # Examples
    ///
    /// ```
    /// # async fn run_example() -> Result<(), crate::Error> {
    /// let modify = ModifyWebhook {
    ///     webhook_id: /* ... */,
    ///     data: /* EditWebhook payload ... */,
    ///     reason: "Updating webhook".into(),
    /// };
    /// let updated: serde_json::Value = modify.execute(&context).await?;
    /// # Ok(()) }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        if let Some(ref avatar) = self.data.avatar.as_inner_ref() {
            let format = get_format_from_image_data(avatar)?;

            if format != "png" && format != "jpeg" && format != "gif" {
                return Err("Icon must be a PNG, JPEG, or GIF format".into());
            }
        }

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        if let Some(channel_id) = self.data.channel_id {
            this.check_channel_permissions(
                bot_user.id,   
                channel_id.widen(),
                Permissions::empty(),
            )
            .await?;
        }

        this.check_permissions(
            bot_user.id,   
            Permissions::MANAGE_WEBHOOKS,
        )
        .await?;

        let webhook = this.controller()
            .get_webhook(self.webhook_id)
            .await?;

        let Some(serde_json::Value::String(guild_id)) = webhook.get("guild_id") else {
            return Err("Webhook does not belong to a guild".into());
        };

        if guild_id != &this.guild_id().to_string() {
            return Err("Webhook does not belong to a guild".into());
        }
        
        let webhook = this.controller()
            .modify_webhook(
                self.webhook_id,
                self.data,
                Some(self.reason.as_str())
            )
            .await?;

        Ok(webhook)
    }

    /// Convert this `ModifyWebhook` request into its corresponding `API` enum variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::apilist::API;
    /// use std::mem::MaybeUninit;
    ///
    /// // Construct a `ModifyWebhook` without initializing its fields purely for example purposes.
    /// // This pattern is only used here to avoid depending on concrete field constructors.
    /// let req: crate::api::webhooks::modify_webhook::ModifyWebhook = unsafe {
    ///     MaybeUninit::zeroed().assume_init()
    /// };
    ///
    /// let api = req.to_apilist();
    /// assert!(matches!(api, API::ModifyWebhook(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyWebhook(self)
    }
}