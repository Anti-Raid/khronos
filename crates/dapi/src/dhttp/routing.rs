use crate::{AnyId, ApplicationId, ChannelId, CommandId, EmojiId, GuildId, InteractionId, MessageId, RoleId, RuleId, StickerId, UserId, WebhookId};

/// A macro for defining routes. Takes as input a list of route definitions, and generates a definition for the `Route` enum and implements methods on it.
macro_rules! routes {
    ($lt:lifetime, {
        $(
            $name:ident $({ $($field_name:ident: $field_type:ty),* })?,
            $path:expr;
        )+
    }) => {
        #[derive(Clone, Copy, Debug)]
        pub enum Route<$lt> {
            $(
                $name $({ $($field_name: $field_type),* })?,
            )+
        }

        impl<$lt> Route<$lt> {
            #[must_use]
            pub fn path(self) -> String {
                match self {
                    $(
                        Self::$name $({ $($field_name),* })? => $path,
                    )+
                }
            }
        }
    };
}

// This macro takes as input a list of route definitions, represented in the following way:
// 1. The first line defines an enum variant representing an endpoint.
// 2. The second line provides the url for that endpoint.
// 3. The third line indicates what type of ratelimiting the endpoint employs.
routes! ('a, {
    Channel { channel_id: ChannelId },
    format!("/channels/{}", channel_id);

    ChannelInvites { channel_id: ChannelId },
    format!("/channels/{}/invites", channel_id);

    ChannelMessage { channel_id: ChannelId, message_id: MessageId },
    format!("/channels/{}/messages/{}", channel_id, message_id);

    ChannelMessageReaction { channel_id: ChannelId, message_id: MessageId, user_id: UserId, reaction: &'a str },
    format!("/channels/{}/messages/{}/reactions/{}/{}", channel_id, message_id, reaction, user_id);

    ChannelMessageReactionMe { channel_id: ChannelId, message_id: MessageId, reaction: &'a str },
    format!("/channels/{}/messages/{}/reactions/{}/@me", channel_id, message_id, reaction);

    ChannelMessageReactionEmoji { channel_id: ChannelId, message_id: MessageId, reaction: &'a str },
    format!("/channels/{}/messages/{}/reactions/{}", channel_id, message_id, reaction);

    ChannelMessageReactions { channel_id: ChannelId, message_id: MessageId },
    format!("/channels/{}/messages/{}/reactions", channel_id, message_id);

    ChannelMessages { channel_id: ChannelId },
    format!("/channels/{}/messages", channel_id);

    ChannelMessagesBulkDelete { channel_id: ChannelId },
    format!("/channels/{}/messages/bulk-delete", channel_id);

    ChannelFollowNews { channel_id: ChannelId },
    format!("/channels/{}/followers", channel_id);

    ChannelPermission { channel_id: ChannelId, target_id: AnyId },
    format!("/channels/{}/permissions/{}", channel_id, target_id);

    ChannelPin { channel_id: ChannelId, message_id: MessageId },
    format!("/channels/{}/pins/{}", channel_id, message_id);

    ChannelPins { channel_id: ChannelId },
    format!("/channels/{}/pins", channel_id);

    ChannelTyping { channel_id: ChannelId },
    format!("/channels/{}/typing", channel_id);

    ChannelWebhooks { channel_id: ChannelId },
    format!("/channels/{}/webhooks", channel_id);

    ChannelForumPosts { channel_id: ChannelId },
    format!("/channels/{}/threads", channel_id);

    ChannelVoiceStatus { channel_id: ChannelId },
    format!("/channels/{}/voice-status", channel_id);

    Guild { guild_id: GuildId },
    format!("/guilds/{}", guild_id);

    GuildAuditLogs { guild_id: GuildId },
    format!("/guilds/{}/audit-logs", guild_id);

    GuildAutomodRule { guild_id: GuildId, rule_id: RuleId },
    format!("/guilds/{}/auto-moderation/rules/{}", guild_id, rule_id);

    GuildAutomodRules { guild_id: GuildId },
    format!("/guilds/{}/auto-moderation/rules", guild_id);

    GuildBan { guild_id: GuildId, user_id: UserId },
    format!("/guilds/{}/bans/{}", guild_id, user_id);

    GuildBulkBan { guild_id: GuildId },
    format!("/guilds/{}/bulk-ban", guild_id);

    GuildBans { guild_id: GuildId },
    format!("/guilds/{}/bans", guild_id);

    GuildChannels { guild_id: GuildId },
    format!("/guilds/{}/channels", guild_id);

    GuildWidget { guild_id: GuildId },
    format!("/guilds/{}/widget", guild_id);

    GuildPreview { guild_id: GuildId },
    format!("/guilds/{}/preview", guild_id);

    GuildEmojis { guild_id: GuildId },
    format!("/guilds/{}/emojis", guild_id);

    GuildEmoji { guild_id: GuildId, emoji_id: EmojiId },
    format!("/guilds/{}/emojis/{}", guild_id, emoji_id);

    GuildInvites { guild_id: GuildId },
    format!("/guilds/{}/invites", guild_id);

    GuildMember { guild_id: GuildId, user_id: UserId },
    format!("/guilds/{}/members/{}", guild_id, user_id);

    GuildMemberRole { guild_id: GuildId, user_id: UserId, role_id: RoleId },
    format!("/guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);

    GuildMembers { guild_id: GuildId },
    format!("/guilds/{}/members", guild_id);

    GuildMembersSearch { guild_id: GuildId, query: &'a str, limit: u16 },
    format!("/guilds/{}/members/search?query={}&limit={}", guild_id, query, limit);

    GuildMemberMe { guild_id: GuildId },
    format!("/guilds/{}/members/@me", guild_id);

    GuildMfa { guild_id: GuildId },
    format!("/guilds/{}/mfa", guild_id);

    GuildPrune { guild_id: GuildId },
    format!("/guilds/{}/prune", guild_id);

    GuildRegions { guild_id: GuildId },
    format!("/guilds/{}/regions", guild_id);

    GuildRole { guild_id: GuildId, role_id: RoleId },
    format!("/guilds/{}/roles/{}", guild_id, role_id);

    GuildRoles { guild_id: GuildId },
    format!("/guilds/{}/roles", guild_id);

    GuildSticker { guild_id: GuildId, sticker_id: StickerId },
    format!("/guilds/{}/stickers/{}", guild_id, sticker_id);

    GuildStickers { guild_id: GuildId },
    format!("/guilds/{}/stickers", guild_id);

    GuildVanityUrl { guild_id: GuildId },
    format!("/guilds/{}/vanity-url", guild_id);

    GuildVoiceStates { guild_id: GuildId, user_id: UserId },
    format!("/guilds/{}/voice-states/{}", guild_id, user_id);

    GuildVoiceStateMe { guild_id: GuildId },
    format!("/guilds/{}/voice-states/@me", guild_id);

    GuildWebhooks { guild_id: GuildId },
    format!("/guilds/{}/webhooks", guild_id);

    GuildWelcomeScreen { guild_id: GuildId },
    format!("/guilds/{}/welcome-screen", guild_id);

    GuildThreadsActive { guild_id: GuildId },
    format!("/guilds/{}/threads/active", guild_id);

    Guilds,
    format!("/guilds");

    Invite { code: &'a str },
    format!("/invites/{}", code);

    OAuth2Token,
    format!("/oauth2/token");

    OAuth2TokenRevocation,
    format!("/oauth2/token/revoke");

    OAuth2ApplicationCurrent,
    format!("/oauth2/applications/@me");

    OAuth2AuthorizationCurrent,
    format!("/oauth2/@me");

    Sticker { sticker_id: StickerId },
    format!("/stickers/{}", sticker_id);

    User { user_id: UserId },
    format!("/users/{}", user_id);

    UserMe,
    format!("/users/@me");

    UserMeGuild { guild_id: GuildId },
    format!("/users/@me/guilds/{}", guild_id);

    UserMeGuildMember { guild_id: GuildId },
    format!("/users/@me/guilds/{}/member", guild_id);

    UserMeGuilds,
    format!("/users/@me/guilds");

    VoiceRegions,
    format!("/voice/regions");

    Webhook { webhook_id: WebhookId },
    format!("/webhooks/{}", webhook_id);

    WebhookWithToken { webhook_id: WebhookId, token: &'a str },
    format!("/webhooks/{}/{}", webhook_id, token);

    WebhookMessage { webhook_id: WebhookId, token: &'a str, message_id: MessageId },
    format!("/webhooks/{}/{}/messages/{}", webhook_id, token, message_id);

    WebhookOriginalInteractionResponse { application_id: ApplicationId, token: &'a str },
    format!("/webhooks/{}/{}/messages/@original", application_id, token);

    WebhookFollowupMessage { application_id: ApplicationId, token: &'a str, message_id: MessageId },
    format!("/webhooks/{}/{}/messages/{}", application_id, token, message_id);

    WebhookFollowupMessages { application_id: ApplicationId, token: &'a str },
    format!("/webhooks/{}/{}", application_id, token);

    InteractionResponse { interaction_id: InteractionId, token: &'a str },
    format!("/interactions/{}/{}/callback", interaction_id, token);

    Command { application_id: ApplicationId, command_id: CommandId },
    format!("/applications/{}/commands/{}", application_id, command_id);

    Commands { application_id: ApplicationId },
    format!("/applications/{}/commands", application_id);

    GuildCommand { application_id: ApplicationId, guild_id: GuildId, command_id: CommandId },
    format!("/applications/{}/guilds/{}/commands/{}", application_id, guild_id, command_id);

    GuildCommandPermissions { application_id: ApplicationId, guild_id: GuildId, command_id: CommandId },
    format!("/applications/{}/guilds/{}/commands/{}/permissions", application_id, guild_id, command_id);

    GuildCommands { application_id: ApplicationId, guild_id: GuildId },
    format!("/applications/{}/guilds/{}/commands", application_id, guild_id);

    GuildCommandsPermissions { application_id: ApplicationId, guild_id: GuildId },
    format!("/applications/{}/guilds/{}/commands/permissions", application_id, guild_id);

    Skus { application_id: ApplicationId },
    format!("/applications/{}/skus", application_id);

    Emoji { application_id: ApplicationId, emoji_id: EmojiId },
    format!("/applications/{}/emojis/{}", application_id, emoji_id);

    Emojis { application_id: ApplicationId },
    format!("/applications/{}/emojis", application_id);

    StageInstances,
    format!("/stage-instances");

    StageInstance { channel_id: ChannelId },
    format!("/stage-instances/{}", channel_id);
});
