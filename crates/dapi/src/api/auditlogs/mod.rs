use crate::{AnyId, ApiReq, Permissions, UserId, context::DiscordContext, controller::DiscordProvider};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetAuditLogOptions {
    pub action_type: Option<u16>,
    pub user_id: Option<UserId>,
    pub before: Option<AnyId>,
    pub limit: Option<u8>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct GetAuditLog {
    #[serde(flatten)]
    pub data: GetAuditLogOptions
}

impl ApiReq for GetAuditLog {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        let bot_user = this.current_user();

        this.check_permissions(bot_user.id, Permissions::VIEW_AUDIT_LOG)
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

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::GetAuditLog(self)
    }
}
