use crate::{context::DiscordContext, controller::DiscordProvider};

pub mod context;
pub mod controller;
pub mod serenity_backports;
pub mod antiraid_check_permissions;
pub mod antiraid_check_channel_permissions;
pub mod antiraid_get_fused_member;
pub mod types;
pub mod api;
pub mod apilist;
pub mod multioption;
pub mod validator;
pub mod ensure_safe;

pub type Error = Box<dyn std::error::Error + Send + Sync>; // This is constant and should be copy pasted

#[allow(async_fn_in_trait)]
pub trait ApiReq {
    type Resp: 'static + serde::Serialize + for<'de> serde::Deserialize<'de> + Send;

    async fn execute<T: DiscordProvider>(self, this: &DiscordContext<T>) -> Result<Self::Resp, Error>;

    // Convert req to ApiList
    fn to_apilist(self) -> apilist::API;
}

#[inline(always)]
pub async fn exec_api<A: ApiReq, T: DiscordProvider>(
    this: &DiscordContext<T>,
    req: A,
) -> Result<A::Resp, crate::Error> {
    req.execute(this).await
}



/// Helper function to extract image format from a data URL
pub fn get_format_from_image_data<'a>(data: &'a str) -> Result<&'a str, crate::Error> {
    if !data.starts_with("data:image/") {
        return Err("Image must be a data URL".into());
    }

    let Some(format) = data.split(";").next() else {
        return Err("Image is not a valid data URL".into());
    };

    let Some(format) = format.split("/").nth(1) else {
        return Err("No format found in data URL".into());
    };

    Ok(format)
}

/// Constant of every event Discord currently has
pub const EVENT_LIST: [&str; 71] = [
  "APPLICATION_COMMAND_PERMISSIONS_UPDATE", // Application command permission was updated
  "AUTO_MODERATION_RULE_CREATE", // Auto Moderation rule was created
  "AUTO_MODERATION_RULE_UPDATE", // Auto Moderation rule was updated
  "AUTO_MODERATION_RULE_DELETE", // Auto Moderation rule was deleted
  "AUTO_MODERATION_ACTION_EXECUTION", // Auto Moderation rule was triggered and an action was executed (e.g. a message was blocked)
  "CHANNEL_CREATE", // New guild channel created
  "CHANNEL_UPDATE", // Channel was updated
  "CHANNEL_DELETE", // Channel was deleted
  "CHANNEL_PINS_UPDATE", // Message was pinned or unpinned
  "THREAD_CREATE", // Thread created, also sent when being added to a private thread
  "THREAD_UPDATE", // Thread was updated
  "THREAD_DELETE", // Thread was deleted
  "THREAD_LIST_SYNC", // Sent when gaining access to a channel, contains all active threads in that channel
  "THREAD_MEMBER_UPDATE", // Thread memberfor the current user was updated
  "THREAD_MEMBERS_UPDATE", // Some user(s) were added to or removed from a thread
  "ENTITLEMENT_CREATE", // Entitlement was created
  "ENTITLEMENT_UPDATE", // Entitlement was updated or renewed
  "ENTITLEMENT_DELETE", // Entitlement was deleted
  "GUILD_UPDATE", // Guild was updated
  "GUILD_AUDIT_LOG_ENTRY_CREATE", // A guild audit log entry was created
  "GUILD_BAN_ADD", // User was banned from a guild
  "GUILD_BAN_REMOVE", // User was unbanned from a guild
  "GUILD_EMOJIS_UPDATE", // Guild emojis were updated
  "GUILD_STICKERS_UPDATE", // Guild stickers were updated
  "GUILD_INTEGRATIONS_UPDATE", // Guild integration was updated
  "GUILD_MEMBER_ADD", // New user joined a guild
  "GUILD_MEMBER_REMOVE", // User was removed from a guild
  "GUILD_MEMBER_UPDATE", // Guild member was updated
  "GUILD_MEMBERS_CHUNK", // Response toRequest Guild Members
  "GUILD_ROLE_CREATE", // Guild role was created
  "GUILD_ROLE_UPDATE", // Guild role was updated
  "GUILD_ROLE_DELETE", // Guild role was deleted
  "GUILD_SCHEDULED_EVENT_CREATE", // Guild scheduled event was created
  "GUILD_SCHEDULED_EVENT_UPDATE", // Guild scheduled event was updated
  "GUILD_SCHEDULED_EVENT_DELETE", // Guild scheduled event was deleted
  "GUILD_SCHEDULED_EVENT_USER_ADD", // User subscribed to a guild scheduled event
  "GUILD_SCHEDULED_EVENT_USER_REMOVE", // User unsubscribed from a guild scheduled event
  "GUILD_SOUNDBOARD_SOUND_CREATE", // Guild soundboard sound was created
  "GUILD_SOUNDBOARD_SOUND_UPDATE", // Guild soundboard sound was updated
  "GUILD_SOUNDBOARD_SOUND_DELETE", // Guild soundboard sound was deleted
  "GUILD_SOUNDBOARD_SOUNDS_UPDATE", // Guild soundboard sounds were updated
  "SOUNDBOARD_SOUNDS", // Response toRequest Soundboard Sounds
  "INTEGRATION_CREATE", // Guild integration was created
  "INTEGRATION_UPDATE", // Guild integration was updated
  "INTEGRATION_DELETE", // Guild integration was deleted
  "INTERACTION_CREATE", // User used an interaction, such as anApplication Command
  "INVITE_CREATE", // Invite to a channel was created
  "INVITE_DELETE", // Invite to a channel was deleted
  "MESSAGE", // Message was created
  "MESSAGE_UPDATE", // Message was edited
  "MESSAGE_DELETE", // Message was deleted
  "MESSAGE_DELETE_BULK", // Multiple messages were deleted at once
  "MESSAGE_REACTION_ADD", // User reacted to a message
  "MESSAGE_REACTION_REMOVE", // User removed a reaction from a message
  "MESSAGE_REACTION_REMOVE_ALL", // All reactions were explicitly removed from a message
  "MESSAGE_REACTION_REMOVE_EMOJI", // All reactions for a given emoji were explicitly removed from a message
  "PRESENCE_UPDATE", // User was updated
  "STAGE_INSTANCE_CREATE", // Stage instance was created
  "STAGE_INSTANCE_UPDATE", // Stage instance was updated
  "STAGE_INSTANCE_DELETE", // Stage instance was deleted or closed
  "SUBSCRIPTION_CREATE", // Premium App Subscription was created
  "SUBSCRIPTION_UPDATE", // Premium App Subscription was updated
  "SUBSCRIPTION_DELETE", // Premium App Subscription was deleted
  "TYPING_START", // User started typing in a channel
  "USER_UPDATE", // Properties about the user changed
  "VOICE_CHANNEL_EFFECT_SEND", // Someone sent an effect in a voice channel the current user is connected to
  "VOICE_STATE_UPDATE", // Someone joined, left, or moved a voice channel
  "VOICE_SERVER_UPDATE", // Guild's voice server was updated
  "WEBHOOKS_UPDATE", // Guild channel webhook was created, update, or deleted
  "MESSAGE_POLL_VOTE_ADD", // User voted on a poll
  "MESSAGE_POLL_VOTE_REMOVE", // User removed a vote on a poll
];