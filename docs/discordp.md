<div id="discordp"></div>

# discordp

<div id="Types"></div>

## Types

<div id="GetAuditLogOptions"></div>

## GetAuditLogOptions

Options for getting audit logs in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting audit logs in Discord
type GetAuditLogOptions = {
	--- The action type to filter by
	action_type: discord.AuditLogEventType?,

	--- The user ID to filter by
	user_id: discord.Snowflake?,

	--- The audit log entry ID to filter
	before: discord.Snowflake?,

	--- The number of entries to return
	limit: number?
}
```

</details>

<div id="action_type"></div>

### action_type

The action type to filter by

*This field is optional and may not be specified*

[discord](./discord.md).[AuditLogEventType](./discord.md#AuditLogEventType)?

<div id="user_id"></div>

### user_id

The user ID to filter by

*This field is optional and may not be specified*

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)?

<div id="before"></div>

### before

The audit log entry ID to filter

*This field is optional and may not be specified*

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)?

<div id="limit"></div>

### limit

The number of entries to return

*This field is optional and may not be specified*

[number](#number)?

<div id="GetAutoModerationRuleOptions"></div>

## GetAutoModerationRuleOptions

Options for getting an auto moderation rule in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting an auto moderation rule in Discord
type GetAutoModerationRuleOptions = {
	--- The rule ID
	rule_id: discord.Snowflake
}
```

</details>

<div id="rule_id"></div>

### rule_id

The rule ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="CreateAutoModerationRuleOptions"></div>

## CreateAutoModerationRuleOptions

Options for creating an auto moderation rule in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating an auto moderation rule in Discord
type CreateAutoModerationRuleOptions = {
	--- The reason for creating the rule
	reason: string,

	--- The data to create the rule with
	data: discordRest.CreateAutoModerationRuleRequest
}
```

</details>

<div id="reason"></div>

### reason

The reason for creating the rule

[string](#string)

<div id="data"></div>

### data

The data to create the rule with

[discordRest](./discordrest.md).[CreateAutoModerationRuleRequest](./discordrest.md#CreateAutoModerationRuleRequest)

<div id="EditAutoModerationRuleOptions"></div>

## EditAutoModerationRuleOptions

Options for editing an auto moderation rule in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for editing an auto moderation rule in Discord
type EditAutoModerationRuleOptions = {
	--- The rule ID
	rule_id: discord.Snowflake,

	--- The reason for editing the rule
	reason: string,

	--- The data to edit the rule with
	data: discordRest.ModifyAutoModerationRuleRequest
}
```

</details>

<div id="rule_id"></div>

### rule_id

The rule ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for editing the rule

[string](#string)

<div id="data"></div>

### data

The data to edit the rule with

[discordRest](./discordrest.md).[ModifyAutoModerationRuleRequest](./discordrest.md#ModifyAutoModerationRuleRequest)

<div id="DeleteAutoModerationRuleOptions"></div>

## DeleteAutoModerationRuleOptions

Options for deleting an auto moderation rule in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting an auto moderation rule in Discord
type DeleteAutoModerationRuleOptions = {
	--- The rule ID
	rule_id: discord.Snowflake,

	--- The reason for deleting the rule
	reason: string
}
```

</details>

<div id="rule_id"></div>

### rule_id

The rule ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for deleting the rule

[string](#string)

<div id="EditChannelOptions"></div>

## EditChannelOptions

Options for editing a channel in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for editing a channel in Discord
type EditChannelOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The reason for the edit
	reason: string,

	--- The data to edit the channel with
	data: discordRest.ModifyChannelRequest
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for the edit

[string](#string)

<div id="data"></div>

### data

The data to edit the channel with

[discordRest](./discordrest.md).[ModifyChannelRequest](./discordrest.md#ModifyChannelRequest)

<div id="DeleteChannelOptions"></div>

## DeleteChannelOptions

Options for deleting a channel in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting a channel in Discord
type DeleteChannelOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The reason for the deletion
	reason: string
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for the deletion

[string](#string)

<div id="EditChannelPermissionsOptions"></div>

## EditChannelPermissionsOptions

Options for editting channel permissions

<details>
<summary>Raw Type</summary>

```luau
--- Options for editting channel permissions
type EditChannelPermissionsOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The target ID to edit permissions of
	target_id: discord.Snowflake,

	--- The allow permissions
	allow: typesext.MultiOption<string>,

	--- The deny permissions
	deny: typesext.MultiOption<string>,

	--- The type of the target
	kind: discord.OverwriteObjectType,

	--- The reason for the edit
	reason: string
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="target_id"></div>

### target_id

The target ID to edit permissions of

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="allow"></div>

### allow

The allow permissions

[typesext](./typesext.md).[MultiOption](./typesext.md#MultiOption)&lt;[string](#string)&gt;

<div id="deny"></div>

### deny

The deny permissions

[typesext](./typesext.md).[MultiOption](./typesext.md#MultiOption)&lt;[string](#string)&gt;

<div id="kind"></div>

### kind

The type of the target

[discord](./discord.md).[OverwriteObjectType](./discord.md#OverwriteObjectType)

<div id="reason"></div>

### reason

The reason for the edit

[string](#string)

<div id="CreateChannelOptions"></div>

## CreateChannelOptions

Options for editing a channel in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for editing a channel in Discord
type CreateChannelOptions = {
	--- The reason for the create
	reason: string,

	--- The data to edit the channel with
	data: discordRest.CreateGuildChannelRequest
}
```

</details>

<div id="reason"></div>

### reason

The reason for the create

[string](#string)

<div id="data"></div>

### data

The data to edit the channel with

[discordRest](./discordrest.md).[CreateGuildChannelRequest](./discordrest.md#CreateGuildChannelRequest)

<div id="AddGuildMemberRoleOptions"></div>

## AddGuildMemberRoleOptions

Options for adding a role to a member

<details>
<summary>Raw Type</summary>

```luau
--- Options for adding a role to a member
type AddGuildMemberRoleOptions = {
	--- The member ID
	user_id: discord.Snowflake,

	--- The role ID
	role_id: discord.Snowflake,

	--- The reason for adding the role
	reason: string
}
```

</details>

<div id="user_id"></div>

### user_id

The member ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="role_id"></div>

### role_id

The role ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for adding the role

[string](#string)

<div id="RemoveGuildMemberRoleOptions"></div>

## RemoveGuildMemberRoleOptions

Options for removing a role from a member

<details>
<summary>Raw Type</summary>

```luau
--- Options for removing a role from a member
type RemoveGuildMemberRoleOptions = {
	--- The member ID
	user_id: discord.Snowflake,

	--- The role ID
	role_id: discord.Snowflake,

	--- The reason for adding the role
	reason: string
}
```

</details>

<div id="user_id"></div>

### user_id

The member ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="role_id"></div>

### role_id

The role ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for adding the role

[string](#string)

<div id="RemoveGuildMemberOptions"></div>

## RemoveGuildMemberOptions

Options for removing a member from a guild

<details>
<summary>Raw Type</summary>

```luau
--- Options for removing a member from a guild
type RemoveGuildMemberOptions = {
	--- The member ID
	user_id: discord.Snowflake,

	--- The reason for removing the member
	reason: string
}
```

</details>

<div id="user_id"></div>

### user_id

The member ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for removing the member

[string](#string)

<div id="GetGuildBansOptions"></div>

## GetGuildBansOptions

Options for getting guild bans



Note: If both `before` and `after` are provided, `before` will take precedence.

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting guild bans
---
--- Note: If both `before` and `after` are provided, `before` will take precedence.
type GetGuildBansOptions = {
	--- The limit of bans to get (max 100)
	limit: number?,

	--- Before a certain user ID
	before: discord.Snowflake?,

	--- After a certain user ID
	after: discord.Snowflake?
}
```

</details>

<div id="limit"></div>

### limit

The limit of bans to get (max 100)

*This field is optional and may not be specified*

[number](#number)?

<div id="before"></div>

### before

Before a certain user ID

*This field is optional and may not be specified*

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)?

<div id="after"></div>

### after

After a certain user ID

*This field is optional and may not be specified*

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)?

<div id="CreateMessageOptions"></div>

## CreateMessageOptions

Options for sending a message to a channel in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for sending a message to a channel in Discord
type CreateMessageOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The data to send the message with
	data: discordRest.CreateMessageRequest
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="data"></div>

### data

The data to send the message with

[discordRest](./discordrest.md).[CreateMessageRequest](./discordrest.md#CreateMessageRequest)

<div id="ReactionType"></div>

## ReactionType

<details>
<summary>Raw Type</summary>

```luau
type ReactionType = {
	type: "Unicode",

	data: string
} | {
	type: "Custom",

	animated: boolean,

	id: discord.Snowflake,

	name: string?
}
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

```luau
"Unicode"
```

<div id="data"></div>

#### data

[string](#string)

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

```luau
"Custom"
```

<div id="animated"></div>

#### animated

[boolean](#boolean)

<div id="id"></div>

#### id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="name"></div>

#### name

*This field is optional and may not be specified*

[string](#string)?

</details>

<div id="CreateReactionOptions"></div>

## CreateReactionOptions

Options for creating a reaction in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating a reaction in Discord
type CreateReactionOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The message ID
	message_id: discord.Snowflake,

	--- The reaction to add
	reaction: ReactionType
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

### message_id

The message ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reaction"></div>

### reaction

The reaction to add

[ReactionType](#ReactionType)

<div id="DeleteOwnReactionOptions"></div>

## DeleteOwnReactionOptions

Options for deleting the reaction AntiRaid has made on Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting the reaction AntiRaid has made on Discord
type DeleteOwnReactionOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The message ID
	message_id: discord.Snowflake,

	--- The reaction to add
	reaction: ReactionType
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

### message_id

The message ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reaction"></div>

### reaction

The reaction to add

[ReactionType](#ReactionType)

<div id="DeleteUserReactionOptions"></div>

## DeleteUserReactionOptions

Options for deleting the reaction of a user on Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting the reaction of a user on Discord
type DeleteUserReactionOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The message ID
	message_id: discord.Snowflake,

	--- The user ID
	user_id: discord.Snowflake,

	--- The reaction to add
	reaction: ReactionType
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

### message_id

The message ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="user_id"></div>

### user_id

The user ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reaction"></div>

### reaction

The reaction to add

[ReactionType](#ReactionType)

<div id="ReactionTypeEnum"></div>

## ReactionTypeEnum

Normal = normal reaction, burst is a super reaction

<details>
<summary>Raw Type</summary>

```luau
--- Normal = normal reaction, burst is a super reaction
type ReactionTypeEnum = "Normal" | "Burst"
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Normal"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Burst"
```

</details>

<div id="GetReactionsOptions"></div>

## GetReactionsOptions

Options for getting reactions on a message

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting reactions on a message
type GetReactionsOptions = {
	channel_id: discord.Snowflake,

	message_id: discord.Snowflake,

	reaction: ReactionType,

	type: ReactionTypeEnum?,

	-- Normal or burst/super reaction
	after: discord.Snowflake?,

	-- After which ID to use
	limit: number?
}
```

</details>

<div id="channel_id"></div>

### channel_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

### message_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reaction"></div>

### reaction

[ReactionType](#ReactionType)

<div id="type"></div>

### type

*This field is optional and may not be specified*

[ReactionTypeEnum](#ReactionTypeEnum)?

<div id="after"></div>

### after

Normal or burst/super reaction

*This field is optional and may not be specified*

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)?

<div id="limit"></div>

### limit

After which ID to use

*This field is optional and may not be specified*

[number](#number)?

<div id="DeleteAllReactionsForEmojiOptions"></div>

## DeleteAllReactionsForEmojiOptions

Options for deleting all reactions on a message

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting all reactions on a message
type DeleteAllReactionsForEmojiOptions = {
	channel_id: discord.Snowflake,

	message_id: discord.Snowflake,

	reaction: ReactionType
}
```

</details>

<div id="channel_id"></div>

### channel_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

### message_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reaction"></div>

### reaction

[ReactionType](#ReactionType)

<div id="EditMessageOptions"></div>

## EditMessageOptions

<details>
<summary>Raw Type</summary>

```luau
type EditMessageOptions = {
	channel_id: discord.Snowflake,

	message_id: discord.Snowflake,

	data: discordRest.EditMessageRequest
}
```

</details>

<div id="channel_id"></div>

### channel_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

### message_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="data"></div>

### data

[discordRest](./discordrest.md).[EditMessageRequest](./discordrest.md#EditMessageRequest)

<div id="CreateCommandOptions"></div>

## CreateCommandOptions

Options for creating a command in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating a command in Discord
type CreateCommandOptions = {
	--- The data to create the command with
	data: discordRest.CreateGuildApplicationCommandRequest
}
```

</details>

<div id="data"></div>

### data

The data to create the command with

[discordRest](./discordrest.md).[CreateGuildApplicationCommandRequest](./discordrest.md#CreateGuildApplicationCommandRequest)

<div id="CreateCommandsOptions"></div>

## CreateCommandsOptions

Options for creating multiple command in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating multiple command in Discord
type CreateCommandsOptions = {
	--- The data to create the command with
	data: {discordRest.CreateGuildApplicationCommandRequest}
}
```

</details>

<div id="data"></div>

### data

The data to create the command with

{[discordRest](./discordrest.md).[CreateGuildApplicationCommandRequest](./discordrest.md#CreateGuildApplicationCommandRequest)}

<div id="CreateInteractionResponseOptions"></div>

## CreateInteractionResponseOptions

Options for creating an interaction response in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating an interaction response in Discord
type CreateInteractionResponseOptions = {
	--- The interaction ID
	interaction_id: discord.Snowflake,

	--- The interaction token
	interaction_token: string,

	--- The data to create the interaction response with
	data: discordRest.CreateInteractionRequest
}
```

</details>

<div id="interaction_id"></div>

### interaction_id

The interaction ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="interaction_token"></div>

### interaction_token

The interaction token

[string](#string)

<div id="data"></div>

### data

The data to create the interaction response with

[discordRest](./discordrest.md).[CreateInteractionRequest](./discordrest.md#CreateInteractionRequest)

<div id="EditInteractionResponseOptions"></div>

## EditInteractionResponseOptions

Options for editting an interaction response in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for editting an interaction response in Discord
type EditInteractionResponseOptions = {
	--- The interaction token
	interaction_token: string,

	--- The data to edit the interaction response with
	data: discordRest.EditWebhookMessageRequest
}
```

</details>

<div id="interaction_token"></div>

### interaction_token

The interaction token

[string](#string)

<div id="data"></div>

### data

The data to edit the interaction response with

[discordRest](./discordrest.md).[EditWebhookMessageRequest](./discordrest.md#EditWebhookMessageRequest)

<div id="GetFollowupMessageOptions"></div>

## GetFollowupMessageOptions

Options for getting a followup message in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting a followup message in Discord
type GetFollowupMessageOptions = {
	--- The interaction token
	interaction_token: string,

	--- The message ID
	message_id: discord.Snowflake
}
```

</details>

<div id="interaction_token"></div>

### interaction_token

The interaction token

[string](#string)

<div id="message_id"></div>

### message_id

The message ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="CreateFollowupMessageOptions"></div>

## CreateFollowupMessageOptions

Options for creating a followup message in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating a followup message in Discord
type CreateFollowupMessageOptions = {
	--- The interaction token
	interaction_token: string,

	--- The data to create the followup message with
	data: discordRest.CreateFollowupMessageRequest
}
```

</details>

<div id="interaction_token"></div>

### interaction_token

The interaction token

[string](#string)

<div id="data"></div>

### data

The data to create the followup message with

[discordRest](./discordrest.md).[CreateFollowupMessageRequest](./discordrest.md#CreateFollowupMessageRequest)

<div id="EditFollowupMessageOptions"></div>

## EditFollowupMessageOptions

Options for editting a followup message in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for editting a followup message in Discord
type EditFollowupMessageOptions = {
	--- The interaction token
	interaction_token: string,

	--- The message ID
	message_id: discord.Snowflake,

	--- The data to edit the followup message with
	data: discordRest.EditWebhookMessageRequest
}
```

</details>

<div id="interaction_token"></div>

### interaction_token

The interaction token

[string](#string)

<div id="message_id"></div>

### message_id

The message ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="data"></div>

### data

The data to edit the followup message with

[discordRest](./discordrest.md).[EditWebhookMessageRequest](./discordrest.md#EditWebhookMessageRequest)

<div id="DeleteFollowupMessageOptions"></div>

## DeleteFollowupMessageOptions

Options for deleting a followup message in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting a followup message in Discord
type DeleteFollowupMessageOptions = {
	--- The interaction token
	interaction_token: string,

	--- The message ID
	message_id: discord.Snowflake
}
```

</details>

<div id="interaction_token"></div>

### interaction_token

The interaction token

[string](#string)

<div id="message_id"></div>

### message_id

The message ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="MessagePagination"></div>

## MessagePagination

A message pagination object

<details>
<summary>Raw Type</summary>

```luau
--- A message pagination object
type MessagePagination = {
	type: "After" | "Around" | "Before",

	id: discord.Snowflake
}
```

</details>

<div id="type"></div>

### type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"After"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Around"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"Before"
```

</details>

<div id="id"></div>

### id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="GetMessagesOptions"></div>

## GetMessagesOptions

Options for getting messages in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting messages in Discord
type GetMessagesOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The target message
	target: MessagePagination?,

	--- The limit of messages to get
	limit: number?
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="target"></div>

### target

The target message

*This field is optional and may not be specified*

[MessagePagination](#MessagePagination)?

<div id="limit"></div>

### limit

The limit of messages to get

*This field is optional and may not be specified*

[number](#number)?

<div id="GetMessageOptions"></div>

## GetMessageOptions

Options for getting a message in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting a message in Discord
type GetMessageOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The message ID
	message_id: discord.Snowflake
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

### message_id

The message ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="CreateGuildBanOptions"></div>

## CreateGuildBanOptions

Options for creating a guild ban in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating a guild ban in Discord
type CreateGuildBanOptions = {
	--- The user ID to ban
	user_id: discord.Snowflake,

	--- The reason for the ban
	reason: string,

	--- The number of seconds to delete messages from
	delete_message_seconds: number?
}
```

</details>

<div id="user_id"></div>

### user_id

The user ID to ban

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for the ban

[string](#string)

<div id="delete_message_seconds"></div>

### delete_message_seconds

The number of seconds to delete messages from

*This field is optional and may not be specified*

[number](#number)?

<div id="RemoveGuildBanOptions"></div>

## RemoveGuildBanOptions

Options for removing a guild ban in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for removing a guild ban in Discord
type RemoveGuildBanOptions = {
	--- The user ID to unban
	user_id: discord.Snowflake,

	--- The reason for the unban
	reason: string
}
```

</details>

<div id="user_id"></div>

### user_id

The user ID to unban

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for the unban

[string](#string)

<div id="GetGuildMembersOptions"></div>

## GetGuildMembersOptions

Options for getting guild members

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting guild members
type GetGuildMembersOptions = {
	--- The limit of members to get
	limit: number?,

	--- The user ID to get members after
	after: discord.Snowflake?
}
```

</details>

<div id="limit"></div>

### limit

The limit of members to get

*This field is optional and may not be specified*

[number](#number)?

<div id="after"></div>

### after

The user ID to get members after

*This field is optional and may not be specified*

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)?

<div id="SearchGuildMembersOptions"></div>

## SearchGuildMembersOptions

Options for searching guild members

<details>
<summary>Raw Type</summary>

```luau
--- Options for searching guild members
type SearchGuildMembersOptions = {
	--- The query to search for
	query: string,

	--- The limit of members to get
	limit: number?
}
```

</details>

<div id="query"></div>

### query

The query to search for

[string](#string)

<div id="limit"></div>

### limit

The limit of members to get

*This field is optional and may not be specified*

[number](#number)?

<div id="ModifyGuildMemberOptions"></div>

## ModifyGuildMemberOptions

Options for modifying a guild member

<details>
<summary>Raw Type</summary>

```luau
--- Options for modifying a guild member
type ModifyGuildMemberOptions = {
	--- The user ID to modify
	user_id: discord.Snowflake,

	--- The reason for the modification
	reason: string,

	--- The data to modify the member with
	data: discordRest.ModifyGuildMemberRequest
}
```

</details>

<div id="user_id"></div>

### user_id

The user ID to modify

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for the modification

[string](#string)

<div id="data"></div>

### data

The data to modify the member with

[discordRest](./discordrest.md).[ModifyGuildMemberRequest](./discordrest.md#ModifyGuildMemberRequest)

<div id="ModifyGuildOptions"></div>

## ModifyGuildOptions

Options for modifying a guild

<details>
<summary>Raw Type</summary>

```luau
--- Options for modifying a guild
type ModifyGuildOptions = {
	data: discordRest.ModifyGuildRequest,

	reason: string
}
```

</details>

<div id="data"></div>

### data

[discordRest](./discordrest.md).[ModifyGuildRequest](./discordrest.md#ModifyGuildRequest)

<div id="reason"></div>

### reason

[string](#string)

<div id="AntiRaidCheckPermissionsOptions"></div>

## AntiRaidCheckPermissionsOptions

Options for checking if a user has the needed Discord permissions to perform an action

<details>
<summary>Raw Type</summary>

```luau
--- Options for checking if a user has the needed Discord permissions to perform an action
type AntiRaidCheckPermissionsOptions = {
	--- The user ID to check permissions for
	user_id: discord.Snowflake,

	--- The needed permissions
	needed_permissions: string
}
```

</details>

<div id="user_id"></div>

### user_id

The user ID to check permissions for

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="needed_permissions"></div>

### needed_permissions

The needed permissions

[string](#string)

<div id="AntiRaidCheckPermissionsAndHierarchyOptions"></div>

## AntiRaidCheckPermissionsAndHierarchyOptions

Options for checking if a user has the needed Discord permissions to perform an action

and is above the target user in terms of hierarchy

<details>
<summary>Raw Type</summary>

```luau
--- Options for checking if a user has the needed Discord permissions to perform an action 
--- and is above the target user in terms of hierarchy
type AntiRaidCheckPermissionsAndHierarchyOptions = {
	--- The user ID to check permissions for
	user_id: discord.Snowflake,

	--- The target ID to check permissions for
	target_id: discord.Snowflake,

	--- The needed permissions
	needed_permissions: string
}
```

</details>

<div id="user_id"></div>

### user_id

The user ID to check permissions for

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="target_id"></div>

### target_id

The target ID to check permissions for

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="needed_permissions"></div>

### needed_permissions

The needed permissions

[string](#string)

<div id="AntiRaidCheckPermissionsResponse"></div>

## AntiRaidCheckPermissionsResponse

Extra/additional success response for checking if a user has the needed Discord permissions to perform an action

<details>
<summary>Raw Type</summary>

```luau
--- Extra/additional success response for checking if a user has the needed Discord permissions to perform an action
type AntiRaidCheckPermissionsResponse = {
	--- The partial guild
	partial_guild: discord.Partial<discord.GuildObject>,

	--- The member
	member: discord.GuildMemberObject,

	--- The permissions
	permissions: string
}
```

</details>

<div id="partial_guild"></div>

### partial_guild

The partial guild

[discord](./discord.md).[Partial](./discord.md#Partial)&lt;[discord](./discord.md).[GuildObject](./discord.md#GuildObject)&gt;

<div id="member"></div>

### member

The member

[discord](./discord.md).[GuildMemberObject](./discord.md#GuildMemberObject)

<div id="permissions"></div>

### permissions

The permissions

[string](#string)

<div id="AntiRaidCheckChannelPermissionsOptions"></div>

## AntiRaidCheckChannelPermissionsOptions

<details>
<summary>Raw Type</summary>

```luau
type AntiRaidCheckChannelPermissionsOptions = {
	--- The user ID to check permissions for
	user_id: discord.Snowflake,

	--- The channel ID to check permissions for
	channel_id: discord.Snowflake,

	--- The needed permissions
	needed_permissions: string
}
```

</details>

<div id="user_id"></div>

### user_id

The user ID to check permissions for

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="channel_id"></div>

### channel_id

The channel ID to check permissions for

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="needed_permissions"></div>

### needed_permissions

The needed permissions

[string](#string)

<div id="AntiRaidCheckChannelPermissionsResponse"></div>

## AntiRaidCheckChannelPermissionsResponse

<details>
<summary>Raw Type</summary>

```luau
type AntiRaidCheckChannelPermissionsResponse = {
	--- The partial guild
	partial_guild: discord.Partial<discord.GuildObject>,

	--- The channel
	channel: discord.ChannelObject,

	--- The member
	member: discord.GuildMemberObject,

	--- The permissions
	permissions: string
}
```

</details>

<div id="partial_guild"></div>

### partial_guild

The partial guild

[discord](./discord.md).[Partial](./discord.md#Partial)&lt;[discord](./discord.md).[GuildObject](./discord.md#GuildObject)&gt;

<div id="channel"></div>

### channel

The channel

[discord](./discord.md).[ChannelObject](./discord.md#ChannelObject)

<div id="member"></div>

### member

The member

[discord](./discord.md).[GuildMemberObject](./discord.md#GuildMemberObject)

<div id="permissions"></div>

### permissions

The permissions

[string](#string)

<div id="AntiraidFusedMemberSingle"></div>

## AntiraidFusedMemberSingle

<details>
<summary>Raw Type</summary>

```luau
type AntiraidFusedMemberSingle = {
	--- The member
	member: discord.GuildMemberObject,

	--- The resolved permissions of the member in the guild
	resolved_perms: string
}
```

</details>

<div id="member"></div>

### member

The member

[discord](./discord.md).[GuildMemberObject](./discord.md#GuildMemberObject)

<div id="resolved_perms"></div>

### resolved_perms

The resolved permissions of the member in the guild

[string](#string)

<div id="AntiraidFusedMember"></div>

## AntiraidFusedMember

A fused member contains both a member, the guild and the resolved permissions of

the member in the guild. This is useful for operations that require both the member and the guild context, such as permission checks.

<details>
<summary>Raw Type</summary>

```luau
--- A fused member contains both a member, the guild and the resolved permissions of
--- the member in the guild. This is useful for operations that require both the member and the guild context, such as permission checks.
type AntiraidFusedMember = {
	--- The partial guild
	guild: discord.Partial<discord.GuildObject>,

	--- The member and resolved permissions
	members: {AntiraidFusedMemberSingle}
}
```

</details>

<div id="guild"></div>

### guild

The partial guild

[discord](./discord.md).[Partial](./discord.md#Partial)&lt;[discord](./discord.md).[GuildObject](./discord.md#GuildObject)&gt;

<div id="members"></div>

### members

The member and resolved permissions

{[AntiraidFusedMemberSingle](#AntiraidFusedMemberSingle)}

<div id="CreateGuildRoleOptions"></div>

## CreateGuildRoleOptions

Options for creating a guild role

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating a guild role
type CreateGuildRoleOptions = {
	--- The reason for the creation
	reason: string,

	--- The data to create the role with
	data: discordRest.CreateGuildRoleRequest
}
```

</details>

<div id="reason"></div>

### reason

The reason for the creation

[string](#string)

<div id="data"></div>

### data

The data to create the role with

[discordRest](./discordrest.md).[CreateGuildRoleRequest](./discordrest.md#CreateGuildRoleRequest)

<div id="ModifyRolePositionOptions"></div>

## ModifyRolePositionOptions

Options for modifying a guild role position

<details>
<summary>Raw Type</summary>

```luau
--- Options for modifying a guild role position
type ModifyRolePositionOptions = {
	--- The data to modify the role position with
	data: {discordRest.ModifyGuildRolePositionsRequest},

	--- The reason for the modification
	reason: string
}
```

</details>

<div id="data"></div>

### data

The data to modify the role position with

{[discordRest](./discordrest.md).[ModifyGuildRolePositionsRequest](./discordrest.md#ModifyGuildRolePositionsRequest)}

<div id="reason"></div>

### reason

The reason for the modification

[string](#string)

<div id="EditGuildRoleOptions"></div>

## EditGuildRoleOptions

Options for modifying a guild role

<details>
<summary>Raw Type</summary>

```luau
--- Options for modifying a guild role
type EditGuildRoleOptions = {
	--- The reason for the creation
	reason: string,

	--- The data to create the role with
	data: discordRest.ModifyGuildRoleRequest,

	--- Role ID
	role_id: discord.Snowflake
}
```

</details>

<div id="reason"></div>

### reason

The reason for the creation

[string](#string)

<div id="data"></div>

### data

The data to create the role with

[discordRest](./discordrest.md).[ModifyGuildRoleRequest](./discordrest.md#ModifyGuildRoleRequest)

<div id="role_id"></div>

### role_id

Role ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="DeleteGuildRoleOptions"></div>

## DeleteGuildRoleOptions

Options for deleting a guild role

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting a guild role
type DeleteGuildRoleOptions = {
	--- The reason for the creation
	reason: string,

	--- Role ID
	role_id: discord.Snowflake
}
```

</details>

<div id="reason"></div>

### reason

The reason for the creation

[string](#string)

<div id="role_id"></div>

### role_id

Role ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="CreateChannelInviteOptions"></div>

## CreateChannelInviteOptions

<details>
<summary>Raw Type</summary>

```luau
type CreateChannelInviteOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The data to create the invite with
	data: discordRest.CreateChannelInviteRequest,

	--- The reason for the creation
	reason: string
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="data"></div>

### data

The data to create the invite with

[discordRest](./discordrest.md).[CreateChannelInviteRequest](./discordrest.md#CreateChannelInviteRequest)

<div id="reason"></div>

### reason

The reason for the creation

[string](#string)

<div id="DeleteChannelPermissionOptions"></div>

## DeleteChannelPermissionOptions

<details>
<summary>Raw Type</summary>

```luau
type DeleteChannelPermissionOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The overwrite ID
	overwrite_id: discord.Snowflake,

	--- The reason for the deletion
	reason: string
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="overwrite_id"></div>

### overwrite_id

The overwrite ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

### reason

The reason for the deletion

[string](#string)

<div id="FollowAnnouncementChannelOptions"></div>

## FollowAnnouncementChannelOptions

<details>
<summary>Raw Type</summary>

```luau
type FollowAnnouncementChannelOptions = {
	--- The Channel ID
	channel_id: discord.Snowflake,

	--- Data
	data: discordRest.FollowAnnouncementChannelRequest,

	--- Reason
	reason: string
}
```

</details>

<div id="channel_id"></div>

### channel_id

The Channel ID

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="data"></div>

### data

Data

[discordRest](./discordrest.md).[FollowAnnouncementChannelRequest](./discordrest.md#FollowAnnouncementChannelRequest)

<div id="reason"></div>

### reason

Reason

[string](#string)

<div id="GetInviteOptions"></div>

## GetInviteOptions

[[
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetInviteOptions {
    pub code: String,
    pub with_counts: Option<bool>, // default to false
    pub with_expiration: Option<bool>, // default to false
    pub guild_scheduled_event_id: Option<serenity::all::ScheduledEventId>,    
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteInviteOptions {
    pub code: String,
    pub reason: String,
}
]]

<details>
<summary>Raw Type</summary>

```luau
--[[
#[derive(serde::Serialize, serde::Deserialize)]
pub struct GetInviteOptions {
    pub code: String,
    pub with_counts: Option<bool>, // default to false
    pub with_expiration: Option<bool>, // default to false
    pub guild_scheduled_event_id: Option<serenity::all::ScheduledEventId>,    
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DeleteInviteOptions {
    pub code: String,
    pub reason: String,
}
]]
type GetInviteOptions = {
	--- The invite code
	code: string,

	--- Whether to include counts
	with_counts: boolean?,

	--- Whether to include expiration
	with_expiration: boolean?,

	--- The guild scheduled event ID
	guild_scheduled_event_id: discord.Snowflake?
}
```

</details>

<div id="code"></div>

### code

The invite code

[string](#string)

<div id="with_counts"></div>

### with_counts

Whether to include counts

*This field is optional and may not be specified*

[boolean](#boolean)?

<div id="with_expiration"></div>

### with_expiration

Whether to include expiration

*This field is optional and may not be specified*

[boolean](#boolean)?

<div id="guild_scheduled_event_id"></div>

### guild_scheduled_event_id

The guild scheduled event ID

*This field is optional and may not be specified*

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)?

<div id="DeleteInviteOptions"></div>

## DeleteInviteOptions

<details>
<summary>Raw Type</summary>

```luau
type DeleteInviteOptions = {
	--- The invite code
	code: string,

	--- The reason for the deletion
	reason: string
}
```

</details>

<div id="code"></div>

### code

The invite code

[string](#string)

<div id="reason"></div>

### reason

The reason for the deletion

[string](#string)

<div id="DiscordExecutor"></div>

## DiscordExecutor

DiscordExecutor allows templates to access/use the Discord API in a sandboxed form.

<details>
<summary>Raw Type</summary>

```luau
--- DiscordExecutor allows templates to access/use the Discord API in a sandboxed form.
type DiscordExecutor = {
	-- Bulk operations
	--- When performing bulk operations, AntiRaid's standard GCRA based ratelimits might not work so well. 
	---
	--- For this, AntiRaid provides a ``antiraid_bulk_op`` which will return a discord ``Plugin`` that allows performing bulk operations. 
	---
	--- To do a bulk operation, your code must perform one operation at a time, and then call ``antiraid_bulk_op_wait`` to wait for the enforced wait period
	--- between each operation (otherwise, AntiRaid will reject the operation).
	---
	--- Note that the calls to ``antiraid_bulk_op`` and ``antiraid_bulk_op_wait`` will respect AntiRaid's standard GCRA 
	--- ratelimits (with the exception that global bucket will not be hit) to ensure user code cannot just keep creating 
	--- new bulk operations.
	---
	--- A bulk operaction executor will expire 10 seconds after the last call to ``antiraid_bulk_op`` or ``antiraid_bulk_op_wait`` returns.
	---
	--- Also note that a bulk operation executor can only be used by the thread that created it, and will error if used in another thread.
	antiraid_bulk_op: () -> DiscordExecutor,

	--- @yields
	---
	--- Waits for the bulk operation executor to finish the enforced wait period
	---
	--- Errors if the executor has expired or if the executor is used in a different thread than the one that created it
	--- or if the executor is not a bulk operation executor (the executor returned by ``antiraid_bulk_op``).
	antiraid_bulk_op_wait: (self: DiscordExecutor) -> nil,

	-- Antiraid helpers
	--- @yields
	---
	--- Checks an audit log reason for validity, errors out if invalid
	antiraid_check_reason: (self: DiscordExecutor, reason: string) -> nil,

	--- @yields
	---
	--- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the server
	antiraid_check_permissions: (self: DiscordExecutor, data: AntiRaidCheckPermissionsOptions) -> AntiRaidCheckPermissionsResponse,

	--- @yields
	---
	--- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the server and is above the target user with an ID of `data.target_id` in terms of hierarchy
	antiraid_check_permissions_and_hierarchy: (self: DiscordExecutor, data: AntiRaidCheckPermissionsAndHierarchyOptions) -> AntiRaidCheckPermissionsResponse,

	--- @yields
	---
	--- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the channel `data.channel_id`
	antiraid_check_channel_permissions: (self: DiscordExecutor, data: AntiRaidCheckChannelPermissionsOptions) -> AntiRaidCheckChannelPermissionsResponse,

	--- @yields
	---
	--- Fetches the member and resolved permissions of a user in the guild
	--- (also called a 'fused member' as it contains both the member and the resolved permissions)
	antiraid_get_fused_member: (self: DiscordExecutor, ids: {discord.Snowflake}) -> LazyAntiraidFusedMember,

	-- Discord API
	-- Audit Logs
	--- @yields
	---
	--- Gets the audit logs
	get_audit_logs: (self: DiscordExecutor, data: GetAuditLogOptions) -> LazyAuditLogObject,

	-- Auto Moderation
	--- @yields
	--- 
	--- Lists the auto moderation rules available
	list_auto_moderation_rules: (self: DiscordExecutor) -> LazyAutomoderationRuleObjectList,

	--- @yields
	---
	--- Gets an auto moderation rule by ID
	get_auto_moderation_rule: (self: DiscordExecutor, data: GetAutoModerationRuleOptions) -> LazyAutomoderationRuleObject,

	--- @yields
	---
	--- Creates an auto moderation rule
	create_auto_moderation_rule: (self: DiscordExecutor, data: CreateAutoModerationRuleOptions) -> LazyAutomoderationRuleObject,

	--- @yields
	---
	--- Edits an auto moderation rule
	edit_auto_moderation_rule: (self: DiscordExecutor, data: EditAutoModerationRuleOptions) -> LazyAutomoderationRuleObject,

	--- @yields
	---
	--- Deletes an auto moderation rule
	delete_auto_moderation_rule: (self: DiscordExecutor, data: DeleteAutoModerationRuleOptions) -> LazyAutomoderationRuleObject,

	-- Channel
	--- @yields
	---
	--- Gets a channel
	get_channel: (self: DiscordExecutor, channel_id: string) -> LazyChannelObject,

	--- @yields
	---
	--- Edits a channel
	edit_channel: (self: DiscordExecutor, data: EditChannelOptions) -> LazyChannelObject,

	--- @yields
	---
	--- Deletes a channel
	delete_channel: (self: DiscordExecutor, data: DeleteChannelOptions) -> LazyChannelObject,

	--- @yields
	---
	--- Edits channel permissions for a target
	edit_channel_permissions: (self: DiscordExecutor, data: EditChannelPermissionsOptions) -> nil,

	--- @yields
	---
	--- Gets all invites a channel has
	get_channel_invites: (self: DiscordExecutor) -> LazyInviteObjectList,

	--- @yields
	---
	--- Creates a channel invite
	create_channel_invite: (self: DiscordExecutor, data: CreateChannelInviteOptions) -> LazyInviteObject,

	--- @yields
	---
	--- Deletes a channel permission
	delete_channel_permission: (self: DiscordExecutor, data: DeleteChannelPermissionOptions) -> nil,

	--- @yields
	---
	--- Follows a announcement channel
	follow_announcement_channel: (self: DiscordExecutor, data: FollowAnnouncementChannelOptions) -> LazyChannelObject,

	-- Guild
	--- @yields
	---
	--- Gets the guild
	get_guild: (self: DiscordExecutor) -> LazyGuildObject,

	--- @yields
	---
	--- Gets the guilds preview
	get_guild_preview: (self: DiscordExecutor) -> LazyGuildPreviewObject,

	--- @yields
	---
	--- Edits the guild
	modify_guild: (self: DiscordExecutor, data: ModifyGuildOptions) -> LazyGuildObject,

	--- @yields
	---
	--- Gets the guild channels
	get_guild_channels: (self: DiscordExecutor) -> LazyChannelsObject,

	--- @yields
	---
	--- Creates a guild channel
	create_guild_channel: (self: DiscordExecutor, data: CreateChannelOptions) -> LazyChannelObject,

	--- @yields
	---
	--- Modify guild channel positions. Only channels to be modified are required to be passed in `data`.
	modify_guild_channel_positions: (self: DiscordExecutor, data: {discordRest.ModifyGuildChannelPositionsRequest}) -> nil,

	--- @yields
	---
	--- List active guild threads
	list_active_guild_threads: (self: DiscordExecutor) -> LazyActiveGuildThreadsResponse,

	--- @yields
	---
	--- Gets a guild member by ID
	get_guild_member: (self: DiscordExecutor, user_id: string) -> LazyGuildMemberObject,

	--- @yields
	---
	--- List all guild members
	list_guild_members: (self: DiscordExecutor, data: GetGuildMembersOptions) -> LazyGuildMembersObject,

	--- @yields
	---
	--- Search guild members
	search_guild_members: (self: DiscordExecutor, data: SearchGuildMembersOptions) -> LazyGuildMembersObject,

	--- @yields
	---
	--- Modify guild member (this includes timing out a member using `communication_disabled_until`)
	modify_guild_member: (self: DiscordExecutor, data: ModifyGuildMemberOptions) -> LazyGuildMemberObject,

	--- @yields
	---
	--- Adds a role to a member
	add_guild_member_role: (self: DiscordExecutor, data: AddGuildMemberRoleOptions) -> nil,

	--- @yields
	---
	--- Removes a role from a member
	remove_guild_member_role: (self: DiscordExecutor, data: RemoveGuildMemberRoleOptions) -> nil,

	--- @yields
	---
	--- Removes a member from a guild
	remove_guild_member: (self: DiscordExecutor, data: RemoveGuildMemberOptions) -> nil,

	--- @yields
	---
	--- Gets guild bans
	get_guild_bans: (self: DiscordExecutor, data: GetGuildBansOptions) -> LazyBanObjectList,

	--- @yields
	---
	--- Gets a guild ban for a user or nil if it does not exist
	get_guild_ban: (self: DiscordExecutor, user_id: discord.Snowflake) -> LazyBanOptionalObject,

	--- @yields
	---
	--- Creates a guild ban
	create_guild_ban: (self: DiscordExecutor, data: CreateGuildBanOptions) -> nil,

	--- @yields
	---
	--- Removes a guild ban
	remove_guild_ban: (self: DiscordExecutor, data: RemoveGuildBanOptions) -> nil,

	--- @yields
	---
	--- Returns the guild roles of a guild
	get_guild_roles: (self: DiscordExecutor) -> LazyRolesMapObject,

	--- @yields
	---
	--- Returns a guild role by ID
	get_guild_role: (self: DiscordExecutor, role_id: discord.Snowflake) -> LazyRoleObject,

	--- @yields
	---
	--- Creates a guild role
	create_guild_role: (self: DiscordExecutor, data: CreateGuildRoleOptions) -> LazyRoleObject,

	--- @yields
	---
	--- Modify guild role positions
	modify_guild_role_positions: (self: DiscordExecutor, data: ModifyRolePositionOptions) -> LazyRolesObject,

	--- @yields
	---
	--- Modifies a guild role
	modify_guild_role: (self: DiscordExecutor, data: EditGuildRoleOptions) -> LazyRoleObject,

	--- @yields
	---
	--- Deletes a guild role
	delete_guild_role: (self: DiscordExecutor, data: DeleteGuildRoleOptions) -> (),

	-- Invites
	--- @yields
	---
	--- Gets an invite by code
	get_invite: (self: DiscordExecutor, data: GetInviteOptions) -> LazyInviteObject,

	--- @yields
	---
	--- Deletes an invite by code
	delete_invite: (self: DiscordExecutor, data: DeleteInviteOptions) -> LazyInviteObject,

	-- Messages
	--- @yields
	---
	--- Gets messages from a channel
	get_channel_messages: (self: DiscordExecutor, data: GetMessagesOptions) -> LazyMessagesObject,

	--- @yields
	---
	--- Gets a message
	get_channel_message: (self: DiscordExecutor, data: GetMessageOptions) -> LazyMessageObject,

	--- @yields
	---
	--- Creates a message
	create_message: (self: DiscordExecutor, data: CreateMessageOptions) -> LazyMessageObject,

	--- @yields
	---
	--- Crossposts a message to an announcement channel
	crosspost_message: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake) -> LazyMessageObject,

	--- @yields
	---
	--- Creates a reaction to a message
	create_reaction: (self: DiscordExecutor, data: CreateReactionOptions) -> nil,

	--- @yields
	--- Deletes the reaction AntiRaid has made on a message
	delete_own_reaction: (self: DiscordExecutor, data: DeleteOwnReactionOptions) -> nil,

	--- @yields
	--- Deletes a reaction another user has made on a message (see ``delete_own_reaction`` for AntiRaid's
	--- reactions)
	delete_user_reaction: (self: DiscordExecutor, data: DeleteUserReactionOptions) -> nil,

	--- @yields
	---
	--- Gets all users who have reacted to awith the provided reaction based on provided criteria
	get_reactions: (self: DiscordExecutor, data: GetReactionsOptions) -> LazyUsersObject,

	--- @yields
	---
	--- Deletes all reactions on a message
	delete_all_reactions: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake) -> nil,

	--- @yields
	---
	--- Deletes all reactions for a specific emoji on a message
	delete_all_reactions_for_emoji: (self: DiscordExecutor, data: DeleteAllReactionsForEmojiOptions) -> nil,

	--- @yields
	---
	--- Edits a message
	edit_message: (self: DiscordExecutor, data: EditMessageOptions) -> LazyMessageObject,

	--- @yields
	---
	--- Deletes a message
	delete_message: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake, reason: string) -> nil,

	--- @yields
	---
	--- Bulk deletes messages in a channel
	bulk_delete_messages: (self: DiscordExecutor, channel_id: discord.Snowflake, message_ids: {discord.Snowflake}, reason: string) -> nil,

	-- Interactions
	--- @yields
	---
	--- Creates an interaction response
	create_interaction_response: (self: DiscordExecutor, data: CreateInteractionResponseOptions) -> nil,

	--- @yields
	---
	--- Gets the original interaction response
	get_original_interaction_response: (self: DiscordExecutor, interaction_token: string) -> LazyMessageObject,

	--- @yields
	---
	--- Edits the original interaction response
	edit_original_interaction_response: (self: DiscordExecutor, data: EditInteractionResponseOptions) -> LazyMessageObject,

	--- @yields
	---
	--- Deletes the original interaction response
	delete_original_interaction_response: (self: DiscordExecutor, interaction_token: string) -> nil,

	--- @yields
	---
	--- Gets a followup interaction response
	get_followup_message: (self: DiscordExecutor, data: GetFollowupMessageOptions) -> LazyMessageObject,

	--- @yields
	---
	--- Creates a followup interaction response
	create_followup_message: (self: DiscordExecutor, data: CreateFollowupMessageOptions) -> LazyMessageObject,

	--- @yields
	---
	--- Edits a followup interaction response
	edit_followup_message: (self: DiscordExecutor, data: EditFollowupMessageOptions) -> LazyMessageObject,

	--- @yields
	---
	--- Deletes a followup interaction response
	delete_followup_message: (self: DiscordExecutor, data: DeleteFollowupMessageOptions) -> nil,

	-- Uncategorized (for now)
	--- @yields
	---
	--- Gets all guild commands currently registered
	get_guild_commands: (self: DiscordExecutor) -> LazyApplicationCommandsObject,

	--- @yields
	---
	--- Creates a guild command
	create_guild_command: (self: DiscordExecutor, data: CreateCommandOptions) -> LazyApplicationCommandObject,

	--- @yields
	---
	--- Creates multiple guild commands
	create_guild_commands: (self: DiscordExecutor, data: CreateCommandsOptions) -> LazyApplicationCommandsObject
}
```

</details>

<div id="antiraid_bulk_op"></div>

### antiraid_bulk_op

Bulk operations

When performing bulk operations, AntiRaid's standard GCRA based ratelimits might not work so well.



For this, AntiRaid provides a ``antiraid_bulk_op`` which will return a discord ``Plugin`` that allows performing bulk operations.



To do a bulk operation, your code must perform one operation at a time, and then call ``antiraid_bulk_op_wait`` to wait for the enforced wait period

between each operation (otherwise, AntiRaid will reject the operation).



Note that the calls to ``antiraid_bulk_op`` and ``antiraid_bulk_op_wait`` will respect AntiRaid's standard GCRA

ratelimits (with the exception that global bucket will not be hit) to ensure user code cannot just keep creating

new bulk operations.



A bulk operaction executor will expire 10 seconds after the last call to ``antiraid_bulk_op`` or ``antiraid_bulk_op_wait`` returns.



Also note that a bulk operation executor can only be used by the thread that created it, and will error if used in another thread.

<details>
<summary>Function Signature</summary>

```luau
-- Bulk operations
--- When performing bulk operations, AntiRaid's standard GCRA based ratelimits might not work so well. 
---
--- For this, AntiRaid provides a ``antiraid_bulk_op`` which will return a discord ``Plugin`` that allows performing bulk operations. 
---
--- To do a bulk operation, your code must perform one operation at a time, and then call ``antiraid_bulk_op_wait`` to wait for the enforced wait period
--- between each operation (otherwise, AntiRaid will reject the operation).
---
--- Note that the calls to ``antiraid_bulk_op`` and ``antiraid_bulk_op_wait`` will respect AntiRaid's standard GCRA 
--- ratelimits (with the exception that global bucket will not be hit) to ensure user code cannot just keep creating 
--- new bulk operations.
---
--- A bulk operaction executor will expire 10 seconds after the last call to ``antiraid_bulk_op`` or ``antiraid_bulk_op_wait`` returns.
---
--- Also note that a bulk operation executor can only be used by the thread that created it, and will error if used in another thread.
antiraid_bulk_op: () -> DiscordExecutor
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[DiscordExecutor](#DiscordExecutor)<div id="antiraid_bulk_op_wait"></div>

### antiraid_bulk_op_wait

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Waits for the bulk operation executor to finish the enforced wait period



Errors if the executor has expired or if the executor is used in a different thread than the one that created it

or if the executor is not a bulk operation executor (the executor returned by ``antiraid_bulk_op``).

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Waits for the bulk operation executor to finish the enforced wait period
---
--- Errors if the executor has expired or if the executor is used in a different thread than the one that created it
--- or if the executor is not a bulk operation executor (the executor returned by ``antiraid_bulk_op``).
antiraid_bulk_op_wait: (self: DiscordExecutor) -> nil
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="antiraid_check_reason"></div>

### antiraid_check_reason

Antiraid helpers

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Checks an audit log reason for validity, errors out if invalid

<details>
<summary>Function Signature</summary>

```luau
-- Antiraid helpers
--- @yields
---
--- Checks an audit log reason for validity, errors out if invalid
antiraid_check_reason: (self: DiscordExecutor, reason: string) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="reason"></div>

##### reason

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="antiraid_check_permissions"></div>

### antiraid_check_permissions

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Checks if a specified user with an ID of `data.user_id` has the specified permissions in the server

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the server
antiraid_check_permissions: (self: DiscordExecutor, data: AntiRaidCheckPermissionsOptions) -> AntiRaidCheckPermissionsResponse
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[AntiRaidCheckPermissionsOptions](#AntiRaidCheckPermissionsOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[AntiRaidCheckPermissionsResponse](#AntiRaidCheckPermissionsResponse)<div id="antiraid_check_permissions_and_hierarchy"></div>

### antiraid_check_permissions_and_hierarchy

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Checks if a specified user with an ID of `data.user_id` has the specified permissions in the server and is above the target user with an ID of `data.target_id` in terms of hierarchy

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the server and is above the target user with an ID of `data.target_id` in terms of hierarchy
antiraid_check_permissions_and_hierarchy: (self: DiscordExecutor, data: AntiRaidCheckPermissionsAndHierarchyOptions) -> AntiRaidCheckPermissionsResponse
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[AntiRaidCheckPermissionsAndHierarchyOptions](#AntiRaidCheckPermissionsAndHierarchyOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[AntiRaidCheckPermissionsResponse](#AntiRaidCheckPermissionsResponse)<div id="antiraid_check_channel_permissions"></div>

### antiraid_check_channel_permissions

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Checks if a specified user with an ID of `data.user_id` has the specified permissions in the channel `data.channel_id`

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Checks if a specified user with an ID of `data.user_id` has the specified permissions in the channel `data.channel_id`
antiraid_check_channel_permissions: (self: DiscordExecutor, data: AntiRaidCheckChannelPermissionsOptions) -> AntiRaidCheckChannelPermissionsResponse
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[AntiRaidCheckChannelPermissionsOptions](#AntiRaidCheckChannelPermissionsOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[AntiRaidCheckChannelPermissionsResponse](#AntiRaidCheckChannelPermissionsResponse)<div id="antiraid_get_fused_member"></div>

### antiraid_get_fused_member

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Fetches the member and resolved permissions of a user in the guild

(also called a 'fused member' as it contains both the member and the resolved permissions)

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Fetches the member and resolved permissions of a user in the guild
--- (also called a 'fused member' as it contains both the member and the resolved permissions)
antiraid_get_fused_member: (self: DiscordExecutor, ids: {discord.Snowflake}) -> LazyAntiraidFusedMember
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="ids"></div>

##### ids

{[discord](./discord.md).[Snowflake](./discord.md#Snowflake)}

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyAntiraidFusedMember](#LazyAntiraidFusedMember)<div id="get_audit_logs"></div>

### get_audit_logs

Discord API

Audit Logs

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets the audit logs

<details>
<summary>Function Signature</summary>

```luau
-- Discord API
-- Audit Logs
--- @yields
---
--- Gets the audit logs
get_audit_logs: (self: DiscordExecutor, data: GetAuditLogOptions) -> LazyAuditLogObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetAuditLogOptions](#GetAuditLogOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyAuditLogObject](#LazyAuditLogObject)<div id="list_auto_moderation_rules"></div>

### list_auto_moderation_rules

Auto Moderation

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Lists the auto moderation rules available

<details>
<summary>Function Signature</summary>

```luau
-- Auto Moderation
--- @yields
--- 
--- Lists the auto moderation rules available
list_auto_moderation_rules: (self: DiscordExecutor) -> LazyAutomoderationRuleObjectList
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyAutomoderationRuleObjectList](#LazyAutomoderationRuleObjectList)<div id="get_auto_moderation_rule"></div>

### get_auto_moderation_rule

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets an auto moderation rule by ID

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets an auto moderation rule by ID
get_auto_moderation_rule: (self: DiscordExecutor, data: GetAutoModerationRuleOptions) -> LazyAutomoderationRuleObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetAutoModerationRuleOptions](#GetAutoModerationRuleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyAutomoderationRuleObject](#LazyAutomoderationRuleObject)<div id="create_auto_moderation_rule"></div>

### create_auto_moderation_rule

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates an auto moderation rule

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates an auto moderation rule
create_auto_moderation_rule: (self: DiscordExecutor, data: CreateAutoModerationRuleOptions) -> LazyAutomoderationRuleObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateAutoModerationRuleOptions](#CreateAutoModerationRuleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyAutomoderationRuleObject](#LazyAutomoderationRuleObject)<div id="edit_auto_moderation_rule"></div>

### edit_auto_moderation_rule

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Edits an auto moderation rule

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Edits an auto moderation rule
edit_auto_moderation_rule: (self: DiscordExecutor, data: EditAutoModerationRuleOptions) -> LazyAutomoderationRuleObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditAutoModerationRuleOptions](#EditAutoModerationRuleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyAutomoderationRuleObject](#LazyAutomoderationRuleObject)<div id="delete_auto_moderation_rule"></div>

### delete_auto_moderation_rule

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes an auto moderation rule

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes an auto moderation rule
delete_auto_moderation_rule: (self: DiscordExecutor, data: DeleteAutoModerationRuleOptions) -> LazyAutomoderationRuleObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteAutoModerationRuleOptions](#DeleteAutoModerationRuleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyAutomoderationRuleObject](#LazyAutomoderationRuleObject)<div id="get_channel"></div>

### get_channel

Channel

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a channel

<details>
<summary>Function Signature</summary>

```luau
-- Channel
--- @yields
---
--- Gets a channel
get_channel: (self: DiscordExecutor, channel_id: string) -> LazyChannelObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="channel_id"></div>

##### channel_id

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyChannelObject](#LazyChannelObject)<div id="edit_channel"></div>

### edit_channel

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Edits a channel

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Edits a channel
edit_channel: (self: DiscordExecutor, data: EditChannelOptions) -> LazyChannelObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditChannelOptions](#EditChannelOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyChannelObject](#LazyChannelObject)<div id="delete_channel"></div>

### delete_channel

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a channel

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a channel
delete_channel: (self: DiscordExecutor, data: DeleteChannelOptions) -> LazyChannelObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteChannelOptions](#DeleteChannelOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyChannelObject](#LazyChannelObject)<div id="edit_channel_permissions"></div>

### edit_channel_permissions

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Edits channel permissions for a target

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Edits channel permissions for a target
edit_channel_permissions: (self: DiscordExecutor, data: EditChannelPermissionsOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditChannelPermissionsOptions](#EditChannelPermissionsOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="get_channel_invites"></div>

### get_channel_invites

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets all invites a channel has

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets all invites a channel has
get_channel_invites: (self: DiscordExecutor) -> LazyInviteObjectList
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyInviteObjectList](#LazyInviteObjectList)<div id="create_channel_invite"></div>

### create_channel_invite

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a channel invite

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a channel invite
create_channel_invite: (self: DiscordExecutor, data: CreateChannelInviteOptions) -> LazyInviteObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateChannelInviteOptions](#CreateChannelInviteOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyInviteObject](#LazyInviteObject)<div id="delete_channel_permission"></div>

### delete_channel_permission

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a channel permission

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a channel permission
delete_channel_permission: (self: DiscordExecutor, data: DeleteChannelPermissionOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteChannelPermissionOptions](#DeleteChannelPermissionOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="follow_announcement_channel"></div>

### follow_announcement_channel

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Follows a announcement channel

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Follows a announcement channel
follow_announcement_channel: (self: DiscordExecutor, data: FollowAnnouncementChannelOptions) -> LazyChannelObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[FollowAnnouncementChannelOptions](#FollowAnnouncementChannelOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyChannelObject](#LazyChannelObject)<div id="get_guild"></div>

### get_guild

Guild

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets the guild

<details>
<summary>Function Signature</summary>

```luau
-- Guild
--- @yields
---
--- Gets the guild
get_guild: (self: DiscordExecutor) -> LazyGuildObject
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyGuildObject](#LazyGuildObject)<div id="get_guild_preview"></div>

### get_guild_preview

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets the guilds preview

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets the guilds preview
get_guild_preview: (self: DiscordExecutor) -> LazyGuildPreviewObject
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyGuildPreviewObject](#LazyGuildPreviewObject)<div id="modify_guild"></div>

### modify_guild

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Edits the guild

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Edits the guild
modify_guild: (self: DiscordExecutor, data: ModifyGuildOptions) -> LazyGuildObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[ModifyGuildOptions](#ModifyGuildOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyGuildObject](#LazyGuildObject)<div id="get_guild_channels"></div>

### get_guild_channels

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets the guild channels

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets the guild channels
get_guild_channels: (self: DiscordExecutor) -> LazyChannelsObject
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyChannelsObject](#LazyChannelsObject)<div id="create_guild_channel"></div>

### create_guild_channel

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a guild channel

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a guild channel
create_guild_channel: (self: DiscordExecutor, data: CreateChannelOptions) -> LazyChannelObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateChannelOptions](#CreateChannelOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyChannelObject](#LazyChannelObject)<div id="modify_guild_channel_positions"></div>

### modify_guild_channel_positions

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Modify guild channel positions. Only channels to be modified are required to be passed in `data`.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Modify guild channel positions. Only channels to be modified are required to be passed in `data`.
modify_guild_channel_positions: (self: DiscordExecutor, data: {discordRest.ModifyGuildChannelPositionsRequest}) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

{[discordRest](./discordrest.md).[ModifyGuildChannelPositionsRequest](./discordrest.md#ModifyGuildChannelPositionsRequest)}

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="list_active_guild_threads"></div>

### list_active_guild_threads

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



List active guild threads

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- List active guild threads
list_active_guild_threads: (self: DiscordExecutor) -> LazyActiveGuildThreadsResponse
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyActiveGuildThreadsResponse](#LazyActiveGuildThreadsResponse)<div id="get_guild_member"></div>

### get_guild_member

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a guild member by ID

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a guild member by ID
get_guild_member: (self: DiscordExecutor, user_id: string) -> LazyGuildMemberObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="user_id"></div>

##### user_id

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyGuildMemberObject](#LazyGuildMemberObject)<div id="list_guild_members"></div>

### list_guild_members

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



List all guild members

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- List all guild members
list_guild_members: (self: DiscordExecutor, data: GetGuildMembersOptions) -> LazyGuildMembersObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetGuildMembersOptions](#GetGuildMembersOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyGuildMembersObject](#LazyGuildMembersObject)<div id="search_guild_members"></div>

### search_guild_members

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Search guild members

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Search guild members
search_guild_members: (self: DiscordExecutor, data: SearchGuildMembersOptions) -> LazyGuildMembersObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[SearchGuildMembersOptions](#SearchGuildMembersOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyGuildMembersObject](#LazyGuildMembersObject)<div id="modify_guild_member"></div>

### modify_guild_member

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Modify guild member (this includes timing out a member using `communication_disabled_until`)

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Modify guild member (this includes timing out a member using `communication_disabled_until`)
modify_guild_member: (self: DiscordExecutor, data: ModifyGuildMemberOptions) -> LazyGuildMemberObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[ModifyGuildMemberOptions](#ModifyGuildMemberOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyGuildMemberObject](#LazyGuildMemberObject)<div id="add_guild_member_role"></div>

### add_guild_member_role

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Adds a role to a member

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Adds a role to a member
add_guild_member_role: (self: DiscordExecutor, data: AddGuildMemberRoleOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[AddGuildMemberRoleOptions](#AddGuildMemberRoleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="remove_guild_member_role"></div>

### remove_guild_member_role

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Removes a role from a member

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Removes a role from a member
remove_guild_member_role: (self: DiscordExecutor, data: RemoveGuildMemberRoleOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[RemoveGuildMemberRoleOptions](#RemoveGuildMemberRoleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="remove_guild_member"></div>

### remove_guild_member

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Removes a member from a guild

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Removes a member from a guild
remove_guild_member: (self: DiscordExecutor, data: RemoveGuildMemberOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[RemoveGuildMemberOptions](#RemoveGuildMemberOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="get_guild_bans"></div>

### get_guild_bans

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets guild bans

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets guild bans
get_guild_bans: (self: DiscordExecutor, data: GetGuildBansOptions) -> LazyBanObjectList
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetGuildBansOptions](#GetGuildBansOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyBanObjectList](#LazyBanObjectList)<div id="get_guild_ban"></div>

### get_guild_ban

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a guild ban for a user or nil if it does not exist

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a guild ban for a user or nil if it does not exist
get_guild_ban: (self: DiscordExecutor, user_id: discord.Snowflake) -> LazyBanOptionalObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="user_id"></div>

##### user_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyBanOptionalObject](#LazyBanOptionalObject)<div id="create_guild_ban"></div>

### create_guild_ban

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a guild ban

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a guild ban
create_guild_ban: (self: DiscordExecutor, data: CreateGuildBanOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateGuildBanOptions](#CreateGuildBanOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="remove_guild_ban"></div>

### remove_guild_ban

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Removes a guild ban

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Removes a guild ban
remove_guild_ban: (self: DiscordExecutor, data: RemoveGuildBanOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[RemoveGuildBanOptions](#RemoveGuildBanOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="get_guild_roles"></div>

### get_guild_roles

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Returns the guild roles of a guild

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Returns the guild roles of a guild
get_guild_roles: (self: DiscordExecutor) -> LazyRolesMapObject
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyRolesMapObject](#LazyRolesMapObject)<div id="get_guild_role"></div>

### get_guild_role

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Returns a guild role by ID

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Returns a guild role by ID
get_guild_role: (self: DiscordExecutor, role_id: discord.Snowflake) -> LazyRoleObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="role_id"></div>

##### role_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyRoleObject](#LazyRoleObject)<div id="create_guild_role"></div>

### create_guild_role

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a guild role

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a guild role
create_guild_role: (self: DiscordExecutor, data: CreateGuildRoleOptions) -> LazyRoleObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateGuildRoleOptions](#CreateGuildRoleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyRoleObject](#LazyRoleObject)<div id="modify_guild_role_positions"></div>

### modify_guild_role_positions

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Modify guild role positions

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Modify guild role positions
modify_guild_role_positions: (self: DiscordExecutor, data: ModifyRolePositionOptions) -> LazyRolesObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[ModifyRolePositionOptions](#ModifyRolePositionOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyRolesObject](#LazyRolesObject)<div id="modify_guild_role"></div>

### modify_guild_role

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Modifies a guild role

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Modifies a guild role
modify_guild_role: (self: DiscordExecutor, data: EditGuildRoleOptions) -> LazyRoleObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditGuildRoleOptions](#EditGuildRoleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyRoleObject](#LazyRoleObject)<div id="delete_guild_role"></div>

### delete_guild_role

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a guild role

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a guild role
delete_guild_role: (self: DiscordExecutor, data: DeleteGuildRoleOptions) -> ()
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteGuildRoleOptions](#DeleteGuildRoleOptions)

<div id="get_invite"></div>

### get_invite

Invites

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets an invite by code

<details>
<summary>Function Signature</summary>

```luau
-- Invites
--- @yields
---
--- Gets an invite by code
get_invite: (self: DiscordExecutor, data: GetInviteOptions) -> LazyInviteObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetInviteOptions](#GetInviteOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyInviteObject](#LazyInviteObject)<div id="delete_invite"></div>

### delete_invite

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes an invite by code

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes an invite by code
delete_invite: (self: DiscordExecutor, data: DeleteInviteOptions) -> LazyInviteObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteInviteOptions](#DeleteInviteOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyInviteObject](#LazyInviteObject)<div id="get_channel_messages"></div>

### get_channel_messages

Messages

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets messages from a channel

<details>
<summary>Function Signature</summary>

```luau
-- Messages
--- @yields
---
--- Gets messages from a channel
get_channel_messages: (self: DiscordExecutor, data: GetMessagesOptions) -> LazyMessagesObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetMessagesOptions](#GetMessagesOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessagesObject](#LazyMessagesObject)<div id="get_channel_message"></div>

### get_channel_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a message

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a message
get_channel_message: (self: DiscordExecutor, data: GetMessageOptions) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetMessageOptions](#GetMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="create_message"></div>

### create_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a message

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a message
create_message: (self: DiscordExecutor, data: CreateMessageOptions) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateMessageOptions](#CreateMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="crosspost_message"></div>

### crosspost_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Crossposts a message to an announcement channel

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Crossposts a message to an announcement channel
crosspost_message: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="channel_id"></div>

##### channel_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

##### message_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="create_reaction"></div>

### create_reaction

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a reaction to a message

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a reaction to a message
create_reaction: (self: DiscordExecutor, data: CreateReactionOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateReactionOptions](#CreateReactionOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="delete_own_reaction"></div>

### delete_own_reaction

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>

Deletes the reaction AntiRaid has made on a message

<details>
<summary>Function Signature</summary>

```luau
--- @yields
--- Deletes the reaction AntiRaid has made on a message
delete_own_reaction: (self: DiscordExecutor, data: DeleteOwnReactionOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteOwnReactionOptions](#DeleteOwnReactionOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="delete_user_reaction"></div>

### delete_user_reaction

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>

Deletes a reaction another user has made on a message (see ``delete_own_reaction`` for AntiRaid's

reactions)

<details>
<summary>Function Signature</summary>

```luau
--- @yields
--- Deletes a reaction another user has made on a message (see ``delete_own_reaction`` for AntiRaid's
--- reactions)
delete_user_reaction: (self: DiscordExecutor, data: DeleteUserReactionOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteUserReactionOptions](#DeleteUserReactionOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="get_reactions"></div>

### get_reactions

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets all users who have reacted to awith the provided reaction based on provided criteria

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets all users who have reacted to awith the provided reaction based on provided criteria
get_reactions: (self: DiscordExecutor, data: GetReactionsOptions) -> LazyUsersObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetReactionsOptions](#GetReactionsOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyUsersObject](#LazyUsersObject)<div id="delete_all_reactions"></div>

### delete_all_reactions

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes all reactions on a message

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes all reactions on a message
delete_all_reactions: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="channel_id"></div>

##### channel_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

##### message_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="delete_all_reactions_for_emoji"></div>

### delete_all_reactions_for_emoji

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes all reactions for a specific emoji on a message

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes all reactions for a specific emoji on a message
delete_all_reactions_for_emoji: (self: DiscordExecutor, data: DeleteAllReactionsForEmojiOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteAllReactionsForEmojiOptions](#DeleteAllReactionsForEmojiOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="edit_message"></div>

### edit_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Edits a message

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Edits a message
edit_message: (self: DiscordExecutor, data: EditMessageOptions) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditMessageOptions](#EditMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="delete_message"></div>

### delete_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a message

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a message
delete_message: (self: DiscordExecutor, channel_id: discord.Snowflake, message_id: discord.Snowflake, reason: string) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="channel_id"></div>

##### channel_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_id"></div>

##### message_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="reason"></div>

##### reason

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="bulk_delete_messages"></div>

### bulk_delete_messages

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Bulk deletes messages in a channel

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Bulk deletes messages in a channel
bulk_delete_messages: (self: DiscordExecutor, channel_id: discord.Snowflake, message_ids: {discord.Snowflake}, reason: string) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="channel_id"></div>

##### channel_id

[discord](./discord.md).[Snowflake](./discord.md#Snowflake)

<div id="message_ids"></div>

##### message_ids

{[discord](./discord.md).[Snowflake](./discord.md#Snowflake)}

<div id="reason"></div>

##### reason

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="create_interaction_response"></div>

### create_interaction_response

Interactions

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates an interaction response

<details>
<summary>Function Signature</summary>

```luau
-- Interactions
--- @yields
---
--- Creates an interaction response
create_interaction_response: (self: DiscordExecutor, data: CreateInteractionResponseOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateInteractionResponseOptions](#CreateInteractionResponseOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="get_original_interaction_response"></div>

### get_original_interaction_response

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets the original interaction response

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets the original interaction response
get_original_interaction_response: (self: DiscordExecutor, interaction_token: string) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="interaction_token"></div>

##### interaction_token

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="edit_original_interaction_response"></div>

### edit_original_interaction_response

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Edits the original interaction response

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Edits the original interaction response
edit_original_interaction_response: (self: DiscordExecutor, data: EditInteractionResponseOptions) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditInteractionResponseOptions](#EditInteractionResponseOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="delete_original_interaction_response"></div>

### delete_original_interaction_response

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes the original interaction response

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes the original interaction response
delete_original_interaction_response: (self: DiscordExecutor, interaction_token: string) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="interaction_token"></div>

##### interaction_token

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="get_followup_message"></div>

### get_followup_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a followup interaction response

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a followup interaction response
get_followup_message: (self: DiscordExecutor, data: GetFollowupMessageOptions) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetFollowupMessageOptions](#GetFollowupMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="create_followup_message"></div>

### create_followup_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a followup interaction response

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a followup interaction response
create_followup_message: (self: DiscordExecutor, data: CreateFollowupMessageOptions) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateFollowupMessageOptions](#CreateFollowupMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="edit_followup_message"></div>

### edit_followup_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Edits a followup interaction response

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Edits a followup interaction response
edit_followup_message: (self: DiscordExecutor, data: EditFollowupMessageOptions) -> LazyMessageObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditFollowupMessageOptions](#EditFollowupMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyMessageObject](#LazyMessageObject)<div id="delete_followup_message"></div>

### delete_followup_message

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a followup interaction response

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a followup interaction response
delete_followup_message: (self: DiscordExecutor, data: DeleteFollowupMessageOptions) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteFollowupMessageOptions](#DeleteFollowupMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="get_guild_commands"></div>

### get_guild_commands

Uncategorized (for now)

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets all guild commands currently registered

<details>
<summary>Function Signature</summary>

```luau
-- Uncategorized (for now)
--- @yields
---
--- Gets all guild commands currently registered
get_guild_commands: (self: DiscordExecutor) -> LazyApplicationCommandsObject
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyApplicationCommandsObject](#LazyApplicationCommandsObject)<div id="create_guild_command"></div>

### create_guild_command

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a guild command

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a guild command
create_guild_command: (self: DiscordExecutor, data: CreateCommandOptions) -> LazyApplicationCommandObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateCommandOptions](#CreateCommandOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyApplicationCommandObject](#LazyApplicationCommandObject)<div id="create_guild_commands"></div>

### create_guild_commands

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates multiple guild commands

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates multiple guild commands
create_guild_commands: (self: DiscordExecutor, data: CreateCommandsOptions) -> LazyApplicationCommandsObject
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateCommandsOptions](#CreateCommandsOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LazyApplicationCommandsObject](#LazyApplicationCommandsObject)<div id="Plugin"></div>

## Plugin

<details>
<summary>Raw Type</summary>

```luau
type Plugin = DiscordExecutor
```

</details>

[DiscordExecutor](#DiscordExecutor)

