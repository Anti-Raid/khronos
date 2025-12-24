use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetGuildWebhooks;

impl ApiReq for GetGuildWebhooks {
    type Resp = serde_json::Value;

    /// Executes the request to fetch guild webhooks after verifying the bot's MANAGE_WEBHOOKS permission.
    ///
    /// # Returns
    ///
    /// `serde_json::Value` containing the guild webhooks data as returned by the controller.
    ///
    /// # Errors
    ///
    /// Returns an error if the current bot user is not available, if the bot lacks `MANAGE_WEBHOOKS` permission,
    /// or if the controller fails to retrieve the webhooks.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::{DiscordContext, GetGuildWebhooks, DiscordProvider};
    /// # async fn example<T: DiscordProvider>(ctx: &DiscordContext<T>) -> Result<(), crate::Error> {
    /// let req = GetGuildWebhooks;
    /// let webhooks = req.execute(ctx).await?;
    /// println!("{}", webhooks);
    /// # Ok(())
    /// # }
    /// ```
    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(
            bot_user.id,   
            Permissions::MANAGE_WEBHOOKS,
        )
        .await?;

        let webhooks = this.controller()
            .get_guild_webhooks()
            .await?;

        Ok(webhooks)
    }

    /// Convert this request into the API enum variant used for the public API list.
    ///
    /// Returns the `crate::apilist::API::GetGuildWebhooks` variant wrapping this request.
    ///
    /// # Examples
    ///
    /// ```
    /// let req = GetGuildWebhooks {};
    /// let api = req.to_apilist();
    /// assert!(matches!(api, crate::apilist::API::GetGuildWebhooks(_)));
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetGuildWebhooks(self)
    }
}