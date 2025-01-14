# @antiraid/userinfo

This plugin allows for templates to interact with user's core information on AntiRaid (permissions etc)

## Types

<div id="type.UserInfo" />

### UserInfo

A user info object

```json
{
  "discord_permissions": "2111062325329919",
  "kittycat_staff_permissions": {
    "user_positions": [],
    "perm_overrides": [
      {
        "namespace": "global",
        "perm": "*",
        "negator": false
      }
    ]
  },
  "kittycat_resolved_permissions": [
    {
      "namespace": "moderation",
      "perm": "kick",
      "negator": false
    },
    {
      "namespace": "moderation",
      "perm": "ban",
      "negator": false
    }
  ],
  "guild_owner_id": "1234567890",
  "guild_roles": [],
  "member_roles": [
    "1234567890"
  ]
}
```

#### Fields

- `discord_permissions` ([string](#type.string)): The discord permissions of the user
- `kittycat_staff_permissions` ([StaffPermissions](#type.StaffPermissions)): The staff permissions of the user
- `kittycat_resolved_permissions` ([{Permission}](#type.Permission)): The resolved permissions of the user
- `guild_owner_id` ([string](#type.string)): The guild owner id
- `guild_roles` ([{[string]: Serenity.Role}](#type.[string]: Serenity.Role)): The roles of the guild
- `member_roles` ([{string}](#type.string)): The roles of the member


<div id="type.UserInfoExecutor" />

### UserInfoExecutor

UserInfoExecutor allows templates to access/use user infos not otherwise sent via events.



#### Methods

##### UserInfoExecutor:get

```lua
function UserInfoExecutor:get(user: string): 
```

Gets the user info of a user.

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `user` ([string](#type.string)): The user id to get the info of.


###### Returns

- `UserInfo` ([](#type.)): The user info of the user.


## Methods

### new

```lua
function new(token: TemplateContext): UserInfoExecutor
```

#### Parameters

- `token` ([TemplateContext](#type.TemplateContext)): The token of the template to use.


#### Returns

- `executor` ([UserInfoExecutor](#type.UserInfoExecutor)): A userinfo executor.