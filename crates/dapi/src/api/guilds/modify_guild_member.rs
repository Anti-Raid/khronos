use serenity::all::Permissions;
use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, types::EditMember, serenity_backports::highest_role};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuildMember {
    pub user_id: serenity::all::UserId,
    pub reason: String,
    pub data: EditMember,
}

impl ApiReq for ModifyGuildMember {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(mut self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        if bot_user.id == self.user_id {
            return Err("Cannot modify self".into());
        }

        let mut needed_perms = Permissions::empty();

        if let Some(ref nick) = self.data.nick {
            if nick.is_empty() {
                return Err("Nickname cannot be empty string if provided".into());
            }

            if nick.len() > 32 { // MAX_NICKNAME_LENGTH
                return Err("Nickname must be less than 32 characters".into());
            }

            needed_perms |= Permissions::MANAGE_NICKNAMES;
        }

        if let Some(ref roles) = self.data.roles {
            if roles.is_empty() {
                return Err("Roles cannot be empty if provided".into());
            }

            needed_perms |= Permissions::MANAGE_ROLES;
        }

        if let Some(mute) = self.data.mute {
            if mute {
                needed_perms |= Permissions::MUTE_MEMBERS;
            }
        }

        if let Some(deaf) = self.data.deaf {
            if deaf {
                needed_perms |= Permissions::DEAFEN_MEMBERS;
            }
        }

        if self.data.channel_id.is_some() {
            needed_perms |= Permissions::MOVE_MEMBERS;
        }

        if let Some(communication_disabled_until) = *self.data.communication_disabled_until {
            if let Some(crdu) = communication_disabled_until {
                if crdu > (chrono::Utc::now() + chrono::Duration::days(28) + chrono::Duration::seconds(10)) {
                    return Err("Communication disabled until must be less than 28 days in the future".into());
                }    
            }

            needed_perms |= Permissions::MODERATE_MEMBERS;
        }

        let (guild, member, perms) = this.check_permissions(
            bot_user.id,
            needed_perms,
        )
        .await?;

        if let Some(ref mut flags) = self.data.flags {
            if !(perms.contains(Permissions::MANAGE_GUILD) || perms.contains(Permissions::MANAGE_ROLES) || perms.contains(Permissions::MODERATE_MEMBERS | Permissions::KICK_MEMBERS | Permissions::BAN_MEMBERS)) {
                return Err("Modifying member flags requires either MANAGE_GUILD, MANAGE_ROLES, or (MODERATE_MEMBERS and KICK_MEMBERS and BAN_MEMBERS)".into());
            }

            let mut p_flags = serenity::all::GuildMemberFlags::empty();
            if flags.contains(serenity::all::GuildMemberFlags::BYPASSES_VERIFICATION) {
                p_flags |= serenity::all::GuildMemberFlags::BYPASSES_VERIFICATION;
            }
            
            *flags = p_flags;
        }

        // Check roles
        let bot_highest_role = highest_role(&guild, &member)
            .ok_or_else(|| "Bot does not have a role")?;

        if let Some(ref roles) = self.data.roles {
            for role in roles.iter() {
                let Some(role) = guild.roles.get(role) else {
                    return Err("Role not found in guild".into());
                };

                if *role >= bot_highest_role {
                    return Err(format!("Bot does not have permission to add the requested role to the member specified ({}, ``{}``)", role.id, role.name.replace("`", "\\`")).into());
                }
            }
        }

        let member = this.controller()
            .modify_guild_member(
                self.user_id,
                self.data,
                Some(self.reason.as_str()),
            )
            .await?;

        Ok(member)
    }

    fn to_apilist(self) -> crate::apilist::API {
        crate::apilist::API::ModifyGuildMember(self)
    }
}
