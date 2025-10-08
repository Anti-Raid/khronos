use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetAuditLogOptions {
    pub action_type: Option<u16>,
    pub user_id: Option<serenity::all::UserId>,
    pub before: Option<serenity::all::AuditLogEntryId>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetAuditLog {
    pub data: GetAuditLogOptions
}

impl ApiReq for GetAuditLog {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let Some(bot_user) = this.controller().current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        this.check_permissions(bot_user.id, serenity::all::Permissions::VIEW_AUDIT_LOG)
            .await?;

        let logs = this
            .controller()
            .get_audit_logs(
                self.data.action_type,
                self.data.user_id,
                self.data.before,
                self.data.limit,
            )
            .await?;

        Ok(logs)
    }
}
