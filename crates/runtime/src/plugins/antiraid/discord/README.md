# @antiraid/discord

This plugin allows for templates to interact with the Discord API. Types are as defined by Discord if not explicitly documented

## Types

<div id="type.GetAuditLogOptions" />

### GetAuditLogOptions

Options for getting audit logs in Discord

```json
{
  "action_type": 1,
  "user_id": "0",
  "before": "0",
  "limit": 0
}
```

#### Fields

- `action_type` ([Serenity.AuditLogs.Action?](#type.Serenity.AuditLogs.Action)): The action type to filter by
- `user_id` ([string?](#type.string)): The user ID to filter by
- `before` ([string?](#type.string)): The entry ID to filter by
- `limit` ([number?](#type.number)): The limit of entries to return


<div id="type.GetChannelOptions" />

### GetChannelOptions

Options for getting a channel in Discord

```json
{
  "channel_id": "0"
}
```

#### Fields

- `channel_id` ([string](#type.string)): The channel ID to get


<div id="type.EditChannel" />

### EditChannel

The data for editing a channel in Discord

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

#### Fields

- `type` ([number?](#type.number)): The type of the channel
- `position` ([number?](#type.number)): The position of the channel
- `topic` ([string?](#type.string)): The topic of the channel
- `nsfw` ([bool?](#type.bool)): Whether the channel is NSFW
- `rate_limit_per_user` ([number?](#type.number)): The rate limit per user/Slow mode of the channel
- `bitrate` ([number?](#type.number)): The bitrate of the channel
- `permission_overwrites` ([{Serenity.PermissionOverwrite}?](#type.Serenity.PermissionOverwrite)): The permission overwrites of the channel
- `parent_id` ([string??](#type.string)): The parent ID of the channel
- `rtc_region` ([string??](#type.string)): The RTC region of the channel
- `video_quality_mode` ([number?](#type.number)): The video quality mode of the channel
- `default_auto_archive_duration` ([number?](#type.number)): The default auto archive duration of the channel
- `flags` ([string?](#type.string)): The flags of the channel
- `available_tags` ([{Serenity.ForumTag}?](#type.Serenity.ForumTag)): The available tags of the channel
- `default_reaction_emoji` ([Serenity.ForumEmoji??](#type.Serenity.ForumEmoji)): The default reaction emoji of the channel
- `default_thread_rate_limit_per_user` ([number?](#type.number)): The default thread rate limit per user
- `default_sort_order` ([number?](#type.number)): The default sort order of the channel
- `default_forum_layout` ([number?](#type.number)): The default forum layout of the channel
- `archived` ([bool?](#type.bool)): Whether the thread is archived (thread only)
- `auto_archive_duration` ([number?](#type.number)): The auto archive duration of the thread (thread only)
- `locked` ([bool?](#type.bool)): Whether the thread is locked (thread only)
- `invitable` ([bool?](#type.bool)): Whether the thread is invitable (thread only)
- `applied_tags` ([{Serenity.ForumTag}?](#type.Serenity.ForumTag)): The applied tags of the thread (thread only)


<div id="type.EditChannelOptions" />

### EditChannelOptions

Options for editing a channel in Discord

```json
{
  "channel_id": "0",
  "reason": "",
  "data": {
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
}
```

#### Fields

- `channel_id` ([string](#type.string)): The channel ID to edit
- `reason` ([string](#type.string)): The reason for editing the channel
- `data` ([EditChannel](#type.EditChannel)): The new channels' data


<div id="type.DeleteChannelOptions" />

### DeleteChannelOptions

Options for deleting a channel in Discord

```json
{
  "channel_id": "0",
  "reason": ""
}
```

#### Fields

- `channel_id` ([string](#type.string)): The channel ID to delete
- `reason` ([string](#type.string)): The reason for deleting the channel


<div id="type.CreateMessageAttachment" />

### CreateMessageAttachment

An attachment in a message

```json
[
  {
    "id": 0,
    "filename": "test.txt",
    "description": "Test file"
  }
]
```

#### Fields

- `filename` ([string](#type.string)): The filename of the attachment
- `description` ([string?](#type.string)): The description (if any) of the attachment
- `content` ([{byte}](#type.byte)): The content of the attachment


<div id="type.CreateMessageOptions" />

### CreateMessageOptions

Options for sending a message in a channel in Discord

```json
{
  "channel_id": "0",
  "data": {
    "tts": false,
    "embeds": [],
    "sticker_ids": [],
    "enforce_nonce": false
  }
}
```

#### Fields

- `channel_id` ([string](#type.string)): The channel ID to send the message in
- `data` ([Serenity.CreateMessage](#type.Serenity.CreateMessage)): The data of the message to send


<div id="type.CreateInteractionResponse" />

### CreateInteractionResponse

Options for creating an interaction response in Discord



#### Fields

- `interaction_id` ([string](#type.string)): The interaction ID to respond to
- `interaction_token` ([string](#type.string)): The interaction token to respond to
- `data` ([Serenity.InteractionResponse](#type.Serenity.InteractionResponse)): The interaction response body
- `files` ([{Serenity.CreateMessageAttachment}?](#type.Serenity.CreateMessageAttachment)): The files to send with the response


<div id="type.DiscordExecutor" />

### DiscordExecutor

DiscordExecutor allows templates to access/use the Discord API in a sandboxed form.



#### Methods

##### DiscordExecutor:get_audit_logs

```lua
function DiscordExecutor:get_audit_logs(data: GetAuditLogOptions): 
```

Gets the audit logs

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `data` ([GetAuditLogOptions](#type.GetAuditLogOptions)): Options for getting audit logs.


###### Returns

- `Lazy<Serenity.AuditLogs>` ([](#type.)): The audit log entry
##### DiscordExecutor:get_channel

```lua
function DiscordExecutor:get_channel(data: GetChannelOptions): 
```

Gets a channel

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `data` ([GetChannelOptions](#type.GetChannelOptions)): Options for getting a channel.


###### Returns

- `Lazy<Serenity.GuildChannel>` ([](#type.)): The guild channel
##### DiscordExecutor:edit_channel

```lua
function DiscordExecutor:edit_channel(data: EditChannelOptions): 
```

Edits a channel

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `data` ([EditChannelOptions](#type.EditChannelOptions)): Options for editing a channel.


###### Returns

- `Lazy<Serenity.GuildChannel>` ([](#type.)): The guild channel
##### DiscordExecutor:delete_channel

```lua
function DiscordExecutor:delete_channel(data: DeleteChannelOptions): 
```

Deletes a channel

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `data` ([DeleteChannelOptions](#type.DeleteChannelOptions)): Options for deleting a channel.


###### Returns

- `Lazy<Serenity.GuildChannel>` ([](#type.)): The guild channel
##### DiscordExecutor:create_message

```lua
function DiscordExecutor:create_message(data: SendMessageChannelAction): 
```

Creates a message

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `data` ([SendMessageChannelAction](#type.SendMessageChannelAction)): Options for creating a message.


###### Returns

- `Lazy<Message>` ([](#type.)): The message
##### DiscordExecutor:create_interaction_response

```lua
function DiscordExecutor:create_interaction_response(data: CreateInteractionResponse): 
```

Creates an interaction response

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `data` ([CreateInteractionResponse](#type.CreateInteractionResponse)): Options for creating a message.


###### Returns

- `Lazy<Message>` ([](#type.)): The message


## Methods

### new

```lua
function new(token: TemplateContext): DiscordExecutor
```

#### Parameters

- `token` ([TemplateContext](#type.TemplateContext)): The token of the template to use.


#### Returns

- `executor` ([DiscordExecutor](#type.DiscordExecutor)): A discord executor.