# @antiraid/discord

This plugin allows for templates to interact with the Discord API. Types are as defined by Discord if not explicitly documented

## Types

<div id="Discord.CreateMessageAttachment" />

### CreateMessageAttachment

The standard type for creating Discord message attachments using AntiRaid

#### CreateMessageAttachment (New)

To create a new attachment

```json
[
  {
    "content": [0, 1, 2, 3],
    "filename": "test.txt",
    "description": "Test file"
  }
]
```

- `filename` ([string](#string)): The filename of the attachment
- `description` ([string?](#string)): The description (if any) of the attachment
- `content` ([{byte}](#byte)): The content of the attachment

#### CreateMessageAttachment (Existing)

To keep an existing attachment

```json
{
  "id": "123456789012345678",
}
```

- `id` ([string](#string)): The ID of the attachment



<div id="DiscordExecutor" />

## DiscordExecutor

DiscordExecutor allows templates to access/use the Discord API in a sandboxed form.

### DiscordExecutor:get_audit_logs

```lua
function DiscordExecutor:get_audit_logs(data: GetAuditLogOptions): 
```

Gets the audit logs

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Parameters

- `data` ([GetAuditLogOptions](#GetAuditLogOptions)): Options for getting audit logs.
<
##### GetAuditLogOptions

Options for getting audit logs in Discord

```json
{
  "action_type": 1,
  "user_id": "0",
  "before": "0",
  "limit": 0
}
```

- `action_type` ([u8?](#u8)): The action type to filter by. Must be a valid action type (as per discord documentation)
- `user_id` ([string?](#string)): The user ID to filter by
- `before` ([string?](#string)): The entry ID to filter by
- `limit` ([number?](#number)): The maximum number of entries to return. Must be between 1 and 100

#### Returns

- `value` [Discord.AuditLogs](#Discord.AuditLogs): The audit log entry

### DiscordExecutor:get_channel

```lua
function DiscordExecutor:get_channel(data: GetChannelOptions): 
```

Gets a channel

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Parameters

- `data` ([GetChannelOptions](#GetChannelOptions)): Options for getting a channel.

##### GetChannelOptions

Options for getting a channel in Discord

```json
{
  "channel_id": "0"
}
```

- `channel_id` ([string](#string)): The channel ID to get

#### Returns

- `Lazy<Discord.GuildChannel>` ([Discord.GuildChannel](#Discord.GuildChannel)): The guild channel

### DiscordExecutor:list_auto_moderation_rules

```lua
function DiscordExecutor:list_auto_moderation_rules(): 
```

Lists the auto moderation rules

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Returns

- `Lazy<Discord.AutoModerationRule>` ([Discord.AutoModerationRule](#Discord.AutoModerationRule)): The auto moderation rules

### DiscordExecutor:get_auto_moderation_rule

```lua
function DiscordExecutor:get_auto_moderation_rule(data: GetAutoModerationRuleOptions): 
```

Gets an auto moderation rule given a known rule ID

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Parameters

- `data` ([GetAutoModerationRuleOptions](#GetAutoModerationRuleOptions)): Options for getting an auto moderation rule.

##### GetAutoModerationRuleOptions

Options for getting an auto moderation rule in Discord

```json
{
  "rule_id": "123456789012345678"
}
```

- `rule_id` ([string](#string)): The rule ID to get

#### Returns

- `Lazy<Discord.AutoModerationRule>` ([Discord.AutoModerationRule](#Discord.AutoModerationRule)): The auto moderation rule

### DiscordExecutor:create_auto_moderation_rule

```lua
function DiscordExecutor:create_auto_moderation_rule(data: CreateAutoModerationRuleOptions): 
```

Creates an auto moderation rule with the given options

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Parameters

- `data` ([CreateAutoModerationRuleOptions](#CreateAutoModerationRuleOptions)): Options for creating an auto moderation rule.

##### CreateAutoModerationRuleOptions

```json
{
  "reason": "My reason",
  "data": { ... }
}
```

- `reason` ([string?](#string)): The reason for creating the rule
- `data` ([CreateAutoModRule](#CreateAutoModRule)): The data for the rule. See below for more information

##### CreateAutoModRule

The inner data containing the actual auto moderation rule to create.

```json
{
  "name": "My rule",
  "trigger_type": 1,
  "event_type": 1,
  "actions": [
    {
      "type": 1,
      "metadata": { "custom_message": "Please keep financial discussions limited to the #finance channel" }
    },
    {
      "type": 2,
      "metadata": { "channel_id": "123456789123456789" }
    },
    {
      "type": 3,
      "metadata": { "duration_seconds": 60 }
    }
  ],
  "trigger_metadata": {
    "keyword_filter": ["cat*", "*dog", "*ana*", "i like c++"],
    "regex_patterns": ["(b|c)at", "^(?:[0-9]{1,3}\\.){3}[0-9]{1,3}$"]
  },
  "enabled": true,
  "exempt_roles": ["323456789123456789", "423456789123456789"],
  "exempt_channels": ["523456789123456789"]
}
```

- `name` ([string?](#string)): The name of the rule
- `event_type` ([number](#number)): The event type of the rule. See [Discord's documentation on event types](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-event-types) for more information
- `trigger_type`, `trigger_metadata`, `keyword_filter`, `regex_patterns`, `presets`, `allow_list`, `mention_total_limit`, `mention_raid_protection_enabled` (non-exhaustive list) ([Discord.AutoModTrigger?](#Discord.AutoModTrigger)): The trigger of the rule. See [Discord's documentation on triggers](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-types) for more information
- `actions` ([Discord.AutoModAction?](#Discord.AutoModAction)): The actions of the rule. See [Discord's documentation on actions](https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-action-object) for more information
- `enabled` ([bool?](#bool)): Whether the rule is enabled
- `exempt_roles` ([{Discord.RoleId}?](#Discord.RoleId)): The roles to exempt from the rule
- `exempt_channels` ([{Discord.ChannelId}?](#Discord.ChannelId)): The channels to exempt from the rule

#### Returns

- `Lazy<Discord.AutoModerationRule>` ([Discord.AutoModerationRule](#Discord.AutoModerationRule)): The auto moderation rule

### DiscordExecutor:edit_channel

```lua
function DiscordExecutor:edit_channel(data: EditChannelOptions): 
```

Edits a channel

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Parameters

- `data` ([EditChannelOptions](#EditChannelOptions)): Options for editing a channel.

##### EditChannelOptions

The options for editing a channel in Discord. See [Discord's documentation](https://discord.com/developers/docs/resources/channel#modify-channel) for more information

```json
{
  "channel_id": "0",
  "reason": "",
  "data": { ... }
}
```

- `channel_id` ([string](#string)): The channel ID to edit
- `reason` ([string?](#string)): The reason for editing the channel
- `data` ([EditChannel](#EditChannel)): The data for the channel. See below for more information

##### EditChannel

The inner data containing the channel edit data. See [Discord's documentation](https://discord.com/developers/docs/resources/channel#modify-channel) for more information

```json
{
  "name": "my-channel",
  "type": 0,
  "position": 7,
  "topic": "My channel topic",
  "nsfw": true,
  "rate_limit_per_user": 5,
  "bitrate": 64000,
  "permission_overwrites": [],
  "parent_id": "0",
  "rtc_region": "us-west",
  "video_quality_mode": 1,
  "default_auto_archive_duration": 1440,
  "flags": 18,
}

```json
{
  "name": "my-channel",
  "type": 0,
  "position": 7,
  "topic": "My channel topic",
  "nsfw": true,
  "rate_limit_per_user": 5,
  "user_limit": 10,
  "parent_id": "0",
  "rtc_region": "us-west",
  "video_quality_mode": 1,
  "default_auto_archive_duration": 1440,
  "flags": 18,
  "default_reaction_emoji": {
    "emoji_id": "0",
    "emoji_name": null
  },
  "status": "online",
  "archived": false,
  "auto_archive_duration": 1440,
  "locked": false,
  "invitable": true
}
```

- `type` ([number?](#number)): The type of the channel
- `position` ([number?](#number)): The position of the channel
- `topic` ([string?](#string)): The topic of the channel
- `nsfw` ([bool?](#bool)): Whether the channel is NSFW
- `rate_limit_per_user` ([number?](#number)): The rate limit per user/Slow mode of the channel
- `bitrate` ([number?](#number)): The bitrate of the channel
- `permission_overwrites` ([{Discord.PermissionOverwrite}?](#Discord.PermissionOverwrite)): The permission overwrites of the channel
- `parent_id` ([string??](#string)): The parent ID of the channel
- `rtc_region` ([string??](#string)): The RTC region of the channel
- `video_quality_mode` ([number?](#number)): The video quality mode of the channel
- `default_auto_archive_duration` ([number?](#number)): The default auto archive duration of the channel
- `flags` ([string?](#string)): The flags of the channel
- `available_tags` ([{Discord.ForumTag}?](#Discord.ForumTag)): The available tags of the channel
- `default_reaction_emoji` ([Discord.ForumEmoji??](#Discord.ForumEmoji)): The default reaction emoji of the channel
- `default_thread_rate_limit_per_user` ([number?](#number)): The default thread rate limit per user
- `default_sort_order` ([number?](#number)): The default sort order of the channel
- `default_forum_layout` ([number?](#number)): The default forum layout of the channel
- `archived` ([bool?](#bool)): Whether the thread is archived (thread only)
- `auto_archive_duration` ([number?](#number)): The auto archive duration of the thread (thread only)
- `locked` ([bool?](#bool)): Whether the thread is locked (thread only)
- `invitable` ([bool?](#bool)): Whether the thread is invitable (thread only)
- `applied_tags` ([{Discord.ForumTag}?](#Discord.ForumTag)): The applied tags of the thread (thread only)

#### Returns

- `Lazy<Discord.GuildChannel>` ([Discord.GuildChannel](#GuildChannel)): The created guild channel

### DiscordExecutor:delete_channel

```lua
function DiscordExecutor:delete_channel(data: DeleteChannelOptions): 
```

Deletes a channel

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Parameters

- `data` ([DeleteChannelOptions](#DeleteChannelOptions)): Options for deleting a channel.


<div id="type.DeleteChannelOptions" />

##### DeleteChannelOptions

Options for deleting a channel in Discord

```json
{
  "channel_id": "0",
  "reason": ""
}
```

- `channel_id` ([string](#string)): The channel ID to delete
- `reason` ([string](#string)): The reason for deleting the channel

#### Returns

- `Lazy<Discord.GuildChannel>` ([](#)): The guild channel

### DiscordExecutor:create_message

```lua
function DiscordExecutor:create_message(data: CreateMessageOptions): 
```

Creates a message

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Parameters

- `data` ([CreateMessageOptions](#CreateMessageOptions)): Options for creating a message.

##### CreateMessageOptions

Options for creating a message in Discord

```json
{
  "channel_id": "0",
  "data": { ... },
}
```

- `channel_id` ([string](#string)): The channel ID to send the message in
- `data` ([CreateMessage](#CreateMessage)): The data of the message to send

##### CreateMessage

The inner data for sending a message

```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Nonce>,
    #[serde(default)]
    pub tts: bool,
    #[serde(default)]
    pub embeds: Vec<CreateEmbed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<CreateAllowedMentions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<ActionRow>>,
    #[serde(default)]
    pub sticker_ids: Vec<StickerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(default)]
    pub enforce_nonce: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<CreatePoll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<CreateMessageAttachment>,
```

```json
{
  "content": "Hello world",
  "nonce": "123456789012345678", // Optional
  "tts": false,
  "embeds": [
    {
      "title": "My embed",
      "description": "My description",
      "url": "https://example.com",
      "fields": [
        {
          "name": "Field 1",
          "value": "Value 1",
          "inline": true
        },
        {
          "name": "Field 2",
          "value": "Value 2",
          "inline": false
        }
      ]
    }
  ],
  "allowed_mentions": {
    "parse": ["roles"], // Can also be "users" or "everyone"
    "users": ["123456"],
    "replied_user": true,
  },
  "components": [
    {
      "type": 1, // Action row
      "components": [
        {
          "type": 2,
          "label": "My button",
          "style": 1,
          "custom_id": "my_button"
        }
      ]
    }
  ],
  "poll": {
    "question": {
      "text": "My question",
    },
    "answers": [
      {
        "text": "Option 1",
        "emoji": {
          "id": null,
          "name": "👍"
        }
      },
      {
        "text": "Option 2",
        "emoji": {
          "id": null,
          "name": "👎"
        }
      }
    ],
    "duration": 24, // Number of hours the poll can last for
    "allow_multiselect": true, // Whether the poll can have multiple options selected
    "layout_type": 1 // DEFAULT layout type
  },
  "attachments": [
    {
      "id": "123456789012345678", // Existing attachment
    },
    {
      "filename": "test.txt", // New attachment
      "content": [0, 1, 2, 3],
      "description": "Test file"
    }
  ],
}
```

- `content` ([string?](#string)): The content of the message
- `nonce` ([string?](#string)): The nonce of the message. Only needed if guaranteed delivery is important and generally discouraged in AntiRaid
- `tts` ([bool?](#bool)): Whether the message is TTS
- `embeds` ([{Discord.Embed}?](#Discord.Embed)): The embeds of the message
- `allowed_mentions` ([Discord.AllowedMentions?](#Discord.AllowedMentions)): The allowed mentions of the message
- `message_reference` ([Discord.MessageReference?](#Discord.MessageReference)): The message reference of the message
- `components` ([{Discord.Component}?](#Discord.Component)): The components of the message
- `sticker_ids` ([{Discord.StickerId}?](#Discord.StickerId)): The sticker IDs of the message
- `flags` ([Discord.MessageFlags?](#Discord.MessageFlags)): The flags of the message
- `enforce_nonce` ([bool?](#bool)): Whether to enforce the nonce. Generally discouraged in AntiRaid
- `poll` ([Discord.CreatePoll?](#Discord.CreatePoll)): The poll of the message
- `attachments` ([{CreateMessageAttachment}?](#CreateMessageAttachment)): The attachments of the message.

#### Returns

- `Lazy<Message>` ([Discord.Message](#Discord.Message)): The message

### DiscordExecutor:create_interaction_response

```lua
function DiscordExecutor:create_interaction_response(data: CreateInteractionResponse): 
```

Creates an interaction response

**Note that this method returns a promise that must be yielded using [`promise.yield`](#promise.yield) to actually execute and return results.**

#### Parameters

- `data` ([CreateInteractionResponse](#CreateInteractionResponse)): Options for creating a message.

##### CreateInteractionResponse

Options for creating an interaction response in Discord

```json
{
  "interaction_id": "0",
  "interaction_token": "0",
  "data": { ... },
  "files": [ ... ]
}
```

- `interaction_id` ([string](#string)): The interaction ID to respond to
- `interaction_token` ([string](#string)): The interaction token to respond to
- `data` ([Discord.InteractionResponse](#Discord.InteractionResponse)): The interaction response body
- `attachments` ([{CreateMessageAttachment}?](#CreateMessageAttachment)): The attachments to send with the response

#### Returns

- `Lazy<Discord.Message>` ([Discord.Message](#Discord.Message)): The message

## Other Methods

### new

```lua
function new(token: TemplateContext): DiscordExecutor
```

#### Parameters

- `token` ([TemplateContext](#TemplateContext)): The token of the template to use.


#### Returns

- `executor` ([DiscordExecutor](#DiscordExecutor)): A discord executor.