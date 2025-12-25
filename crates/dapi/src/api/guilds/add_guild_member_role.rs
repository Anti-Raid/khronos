// use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, serenity_backports::{member_permissions, highest_role}};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AddGuildMemberRole {
    pub user_id: serenity::all::UserId,
    pub role_id: serenity::all::RoleId,
    pub reason: String,
}

impl ApiReq for AddGuildMemberRole {
    type Resp = ();

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        let bot_member_json = this.controller().get_guild_member(bot_user.id).await?;

        if bot_member_json.is_null() {
            return Err("Bot user not found in guild".into());
        }

        let bot_member = serde_json::from_value::<serenity::all::Member>(bot_member_json)?;

        let guild_json = this.controller().get_guild().await?;
        let guild = serde_json::from_value::<serenity::all::PartialGuild>(guild_json)?;

        let target_member_json = this.controller().get_guild_member(self.user_id).await?;
        if target_member_json.is_null() {
            return Err("Target user not found in guild".into());
        }

        let resolved = member_permissions(&guild, &bot_member);

        if !resolved.manage_roles() {
            return Err("Bot does not have permission to manage roles".into());
        }

        let Some(bot_highest_role) = highest_role(&guild, &bot_member) else {
            return Err("Bot does not have a role".into());
        };

        let Some(role_to_add) = guild.roles.get(&self.role_id) else {
            return Err("Role to add to member not found in guild".into());
        };

        if *role_to_add >= bot_highest_role {
            return Err(format!("Bot does not have permission to add the requested role ({}, ``{}``) to the member", role_to_add.id, role_to_add.name.replace("`", "\\`")).into());
        }

        this.controller()
            .add_guild_member_role(self.user_id, self.role_id, Some(self.reason.as_str()))
            .await?;

        Ok(())
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::AddGuildMemberRole(self)
    }
}
