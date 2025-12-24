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

    /// Executes the guild ban: validates the reason and delete-message window, checks bot permissions and hierarchy, and creates the ban for the specified user.
    ///
    /// Performs these observable steps:
    /// - Validates the provided ban reason.
    /// - Normalizes `delete_message_seconds` to 0 if not provided and enforces it is between 0 and 604800.
    /// - Ensures the current bot user is available and has `BAN_MEMBERS` permission and proper hierarchy relative to the target user.
    /// - Creates the guild ban with the given reason and message-deletion window.
    ///
    /// # Returns
    ///
    /// `()` on success.
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

    /// Converts this `CreateGuildBan` request into the crate's API enum.
    ///
    /// This wraps the value in `crate::apilist::API::CreateGuildBan` for transmission or routing.
    ///
    /// # Examples
    ///
    /// ```
    /// use serenity::all::UserId;
    /// let req = crate::api::guilds::CreateGuildBan {
    ///     user_id: UserId(123),
    ///     reason: "spam".into(),
    ///     delete_message_seconds: Some(3600),
    /// };
    /// let api = req.to_apilist();
    /// match api {
    ///     crate::apilist::API::CreateGuildBan(inner) => assert_eq!(inner.reason, "spam"),
    ///     _ => panic!("unexpected variant"),
    /// }
    /// ```
    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::CreateGuildBan(self)
    }
}