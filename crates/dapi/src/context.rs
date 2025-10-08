use serenity::all::{Mentionable, UserId};

use crate::{controller::DiscordProvider, serenity_backports::member_permissions};

// Base types
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiraidFusedMemberSingle {
    pub member: serenity::all::Member,
    pub resolved_perms: serenity::all::Permissions,
}

/// A fused member contains both a member, the guild and the resolved permissions of
/// the member in the guild. This is useful for operations that require both the member and the guild context, such as permission checks.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct AntiraidFusedMember {
    pub guild: serenity::all::PartialGuild,
    pub members: Vec<AntiraidFusedMemberSingle>,
}


/// A context for Discord operations, tied to a specific guild and HTTP client.
#[derive(Clone)]
pub struct DiscordContext<T: DiscordProvider> {
    discord_provider: T,
}

impl<T: DiscordProvider> DiscordContext<T> {
    pub fn new(discord_provider: T) -> Self {
        Self { discord_provider }
    }

    pub fn guild_id(&self) -> serenity::all::GuildId {
        self.discord_provider.guild_id()
    }

    pub fn check_reason(&self, reason: &str) -> Result<(), crate::Error> {
        if reason.len() > 512 {
            return Err("Reason is too long".into());
        } else if reason.is_empty() {
            return Err("Reason is empty".into());
        }

        Ok(())
    }

    pub fn controller(&self) -> &T {
        &self.discord_provider
    }

    pub fn serenity_http(&self) -> &serenity::http::Http {
        self.discord_provider.serenity_http()
    }

    pub fn current_user(&self) -> Option<serenity::all::CurrentUser> {
        self.discord_provider.current_user()
    }

    pub async fn check_permissions(
        &self,
        user_id: serenity::all::UserId,
        needed_permissions: serenity::all::Permissions,
    ) -> Result<(
        serenity::all::PartialGuild,
        serenity::all::Member,
        serenity::all::Permissions,
    ), crate::Error> {
        // Get the guild
        let guild_json = self
            .discord_provider
            .get_guild()
            .await?;

        let guild: serenity::all::PartialGuild = serde_json::from_value(guild_json)?;

        let member_json = self
            .discord_provider
            .get_guild_member(user_id)
            .await?;

        if member_json.is_null() {
            return Err(format!(
                "User not found in guild: {}",
                user_id.mention()
            ).into());
        }

        let member: serenity::all::Member = serde_json::from_value(member_json)?;

        let member_perms = member_permissions(&guild, &member);

        if !member_perms.contains(needed_permissions) {
            return Err(format!(
                "User does not have the required permissions: {needed_permissions:?}: user_id={}",
                user_id,
            ).into());
        }

        Ok((guild, member, member_perms))
    }

    pub async fn check_permissions_and_hierarchy(
        &self,
        user_id: serenity::all::UserId,
        target_id: serenity::all::UserId,
        needed_permissions: serenity::all::Permissions,
    ) -> Result<(
        serenity::all::PartialGuild,
        serenity::all::Member,
        serenity::all::Permissions,
    ), crate::Error> {
        let guild_json = self
            .discord_provider
            .get_guild()
            .await?;

        let guild: serenity::all::PartialGuild = serde_json::from_value(guild_json)?;

        let member_json = self
            .discord_provider
            .get_guild_member(user_id)
            .await?;

        if member_json.is_null() {
            return Err(format!(
                "User not found in guild: {}",
                user_id.mention()
            ).into());
        }

        let member: serenity::all::Member = serde_json::from_value(member_json)?;

        let member_perms = member_permissions(&guild, &member);
        if !member_perms.contains(needed_permissions) {
            return Err(format!(
                "User does not have the required permissions: {needed_permissions:?}: user_id={}",
                user_id,
            ).into());
        }

        let target_member_json = self
            .discord_provider
            .get_guild_member(target_id)
            .await?;

        if target_member_json.is_null() {
            return Err(format!(
                "User not found in guild: {}",
                target_id.mention()
            ).into());
        }

        let target_member: serenity::all::Member = serde_json::from_value(target_member_json)?;

        let higher_id = guild
            .greater_member_hierarchy(&member, &target_member)
            .ok_or_else(|| {
                format!(
                    "User does not have a higher role than the target user: user_id={user_id}, target_id={target_id}",
                )
            })?;

        if higher_id != member.user.id {
            return Err(format!(
                "User does not have a higher role than the target user: user_id={user_id}, target_id={target_id}",
            ).into());
        }

        Ok((guild, target_member, member_perms))
    }

    /// Returns the channel permissions
    /// 
    /// The returned GuildChannel will either be the GuildChannel or the parent GuildChannel of a thread (if the channel id is one for a thread)
    pub async fn check_channel_permissions(
        &self,
        user_id: serenity::all::UserId,
        channel_id: serenity::all::GenericChannelId,
        needed_permissions: serenity::all::Permissions,
    ) -> Result<(
        serenity::all::PartialGuild,
        serenity::all::Member,
        serenity::all::GuildChannel,
        serenity::all::Permissions,
    ), crate::Error> {
        let mut id = channel_id;
        loop {
            // This call should do access control checks (channel in guild) etc.
            let channel_val = self
                .discord_provider
                .get_channel(id)
                .await?;

            let channel: serenity::all::Channel = serde_json::from_value(channel_val)?;

            let member_json = self
                .discord_provider
                .get_guild_member(user_id)
                .await?;

            if member_json.is_null() {
                return Err(format!(
                    "User not found in guild: {}",
                    user_id.mention()
                ).into());
            }

            let member: serenity::all::Member = serde_json::from_value(member_json)?;

            let guild_json = self
                .discord_provider
                .get_guild()
                .await?;

            let guild: serenity::all::PartialGuild = serde_json::from_value(guild_json)?;

            match channel {
                serenity::all::Channel::Private(_) => {
                    return Err("Private channels are not supported by check_channel_permissions".into());
                },
                serenity::all::Channel::Guild(guild_channel) => {
                    let perms = guild.user_permissions_in(&guild_channel, &member);

                    if !perms.contains(needed_permissions) {
                        return Err(format!(
                            "User does not have the required permissions: {needed_permissions:?}: {user_id}",
                        ).into());
                    }

                    return Ok((guild, member, guild_channel, perms))
                }
                serenity::all::Channel::GuildThread(gt) => {
                    // Threads are always under a parent channel, so we need to get the parent channel
                    id = gt.parent_id.widen();
                    continue; // Loop again with the parent channel id
                },
                _ => {
                    return Err("Unsupported channel type in check_channel_permissions".into());
                }
            }
        }
    }

    pub async fn get_fused_member(&self, user_ids: Vec<UserId>) -> Result<AntiraidFusedMember, crate::Error> {
        // Fetch the partial guild *once*
        let partial_guild_json = self.discord_provider
            .get_guild()
            .await?;

        let partial_guild: serenity::all::PartialGuild = serde_json::from_value(partial_guild_json)?;

        let mut member_and_resolved_perms = Vec::with_capacity(user_ids.len());

        for id in user_ids {
            let member_json = self.discord_provider
            .get_guild_member(id)
            .await?;

            if member_json.is_null() {
                return Err(format!(
                    "User not found in guild: {}",
                    id.mention()
                ).into());
            }

            let member: serenity::all::Member = serde_json::from_value(member_json)?;

            let resolved_perms = member_permissions(&partial_guild, &member);

            member_and_resolved_perms.push(AntiraidFusedMemberSingle {
                member,
                resolved_perms,
            });
        }
        
        Ok(AntiraidFusedMember {
            guild: partial_guild,
            members: member_and_resolved_perms,
        })
    }
}