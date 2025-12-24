use crate::{ApiReq, antiraid_check_channel_permissions::AntiRaidCheckChannelPermissions, antiraid_check_permissions::{AntiRaidCheckPermissions, AntiRaidCheckPermissionsAndHierarchy}, antiraid_get_fused_member::AntiRaidGetFusedMember, api::{auditlogs::GetAuditLog, automoderation::{get_auto_moderation_rule::GetAutoModerationRule, list_auto_moderation_rules::ListAutoModerationRules}, channels::edit_channel::EditChannel, guilds::modify_guild::ModifyGuild}, context::DiscordContext, controller::DiscordProvider};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum API {
    // Antiraid specific
    AntiRaidCheckChannelPermissions(AntiRaidCheckChannelPermissions),
    AntiRaidCheckPermissions(AntiRaidCheckPermissions),
    AntiRaidCheckPermissionsAndHierarchy(AntiRaidCheckPermissionsAndHierarchy),
    AntiRaidGetFusedMember(AntiRaidGetFusedMember),

    // Audit logs
    GetAuditLog(GetAuditLog),

    // Auto Moderation
    GetAutoModerationRule(GetAutoModerationRule),
    ListAutoModerationRules(ListAutoModerationRules),

    // Channels
    EditChannel(EditChannel),

    // Guilds
    ModifyGuild(ModifyGuild)
}

macro_rules! jsonify {
    ($e:expr) => {
        serde_json::to_value($e)?
    };
}

macro_rules! op {
    ($this:expr, $req:expr) => {
        {
            let resp = $req.execute($this).await?;
            Ok(jsonify!(resp))
        }
    };
}

impl ApiReq for API {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        match self {
            API::AntiRaidCheckChannelPermissions(req) => op!(this, req),
            API::AntiRaidCheckPermissions(req) => op!(this, req),
            API::AntiRaidCheckPermissionsAndHierarchy(req) => op!(this, req),
            API::AntiRaidGetFusedMember(req) => op!(this, req),
            API::GetAuditLog(req) => op!(this, req),
            API::GetAutoModerationRule(req) => op!(this, req),
            API::ListAutoModerationRules(req) => op!(this, req),
            API::EditChannel(req) => op!(this, req),
            API::ModifyGuild(req) => op!(this, req),
        }
    }

    fn to_apilist(self) -> crate::apilist::API {
        self
    }
}