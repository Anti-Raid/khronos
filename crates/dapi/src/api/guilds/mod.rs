use crate::{ApiReq, context::DiscordContext, controller::DiscordProvider, get_format_from_image_data, types::EditGuild};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ModifyGuild {
    pub data: EditGuild,
    pub reason: String,
}

impl ApiReq for ModifyGuild {
    type Resp = serde_json::Value;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, crate::Error> {
        this.check_reason(&self.reason)?;

        let Some(bot_user) = this.controller().current_user() else {
            return Err("Internal error: Current user not found".into());
        };

        if let Some(ref name) = self.data.name {
            if name.len() < 2 || name.len() > 100 {
                return Err(
                    "Name must be between 2 and 100 characters".into(),
                );
            }
        }

        if let Some(ref description) = self.data.description {
            if description.len() > 300 {
                return Err(
                    "Description must be less than 300 characters".into(),
                );
            }
        }

        if let Some(icon) = self.data.icon.as_inner_ref() {
            let format = get_format_from_image_data(icon)?;

            if format != "png" && format != "jpeg" && format != "gif" {
                return Err(
                    "Icon must be a PNG, JPEG, or GIF format".into()
                );
            }
        }

        let splash_format = {
            if let Some(splash) = self.data.splash.as_inner_ref() {
                let format = get_format_from_image_data(splash)?;

                if format != "png" && format != "jpeg" {
                    return Err(
                        "Splash must be a PNG or JPEG format".into()
                    );
                }

                Some(format)
            } else {
                None
            }
        };

        let discovery_splash_format = {
            if let Some(discovery_splash) = self.data.discovery_splash.as_inner_ref() {
                let format = get_format_from_image_data(discovery_splash)?;

                if format != "png" && format != "jpeg" {
                    return Err(
                        "Discovery splash must be a PNG or JPEG format".into()
                    );
                }

                Some(format)
            } else {
                None
            }
        };

        let banner_format = {
            if let Some(banner) = self.data.banner.as_inner_ref() {
                let format = get_format_from_image_data(banner)?;

                if format != "png" && format != "jpeg" && format != "gif" {
                    return Err(
                        "Banner must be a PNG, JPEG, or GIF format".into()
                    );
                }

                Some(format)
            } else {
                None
            }
        };

        // TODO: Check afk_channel_id, system_channel_id, rules_channel_id, public_updates_channel_id, safety_alerts_channel_id too

        let (guild, _member, perms) = this.check_permissions(
            bot_user.id,
            serenity::all::Permissions::MANAGE_GUILD,
        )
        .await?;

        let mut guild_has_community = false;
        let mut guild_has_invite_splash = false;
        let mut guild_has_discoverable = false;
        let mut guild_has_banner = false;
        let mut guild_has_animated_banner = false;

        for feature in guild.features.iter() {
            match feature.as_str() {
                "COMMUNITY" => guild_has_community = true,
                "INVITE_SPLASH" => guild_has_invite_splash = true,
                "DISCOVERABLE" => guild_has_discoverable = true,
                "BANNER" => guild_has_banner = true,
                "ANIMATED_BANNER" => guild_has_animated_banner = true,
                _ => {}
            }
        }

        if let Some(ref features) = self.data.features {
            if (
                (features.contains(&"COMMUNITY".into()) && !guild_has_community) || 
                (!features.contains(&"COMMUNITY".into()) && guild_has_community)
            ) && !perms.contains(serenity::all::Permissions::ADMINISTRATOR) {
                return Err("Enabling/disabling the community feature requires the bot to have the Administrator permission".into());
            }
        }

        if !guild_has_invite_splash && splash_format.is_some() {
            return Err("Guild does not have the Invite Splash feature and as such cannot have an invite splash".into());
        }

        if !guild_has_discoverable && discovery_splash_format.is_some() {
            return Err("Guild does not have the Discoverable feature and as such cannot have a discovery splash".into());
        }

        if banner_format.is_some() {
            if !guild_has_banner {
                return Err("Guild does not have the Banner feature and as such cannot have a banner".into());
            }

            if !guild_has_animated_banner && banner_format == Some("gif") {
                return Err("Guild does not have the Animated Banner feature and as such cannot have an (animated) GIF banner".into());
            }
        }

        let new_guild = this
            .controller()
            .modify_guild(
                self.data,
                Some(self.reason.as_str()),
            )
            .await?;

        Ok(new_guild)
    }
}