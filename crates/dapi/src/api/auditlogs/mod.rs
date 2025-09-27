use crate::{context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetAuditLogOptions {
    pub action_type: Option<u16>,
    pub user_id: Option<serenity::all::UserId>,
    pub before: Option<serenity::all::AuditLogEntryId>,
    pub limit: Option<serenity::nonmax::NonMaxU8>,
}

pub async fn get_audit_logs<T: DiscordProvider>(this: &DiscordContext<T>, data: GetAuditLogOptions) -> Result<serde_json::Value, crate::Error> {
    let Some(bot_user) = this.controller().current_user() else {
        return Err("Internal error: Current user not found".into());
    };

    this.check_permissions(bot_user.id, serenity::all::Permissions::VIEW_AUDIT_LOG)
        .await?;

    let logs = this
        .controller()
        .get_audit_logs(
            data.action_type,
            data.user_id,
            data.before,
            data.limit,
        )
        .await?;

    Ok(logs)
}