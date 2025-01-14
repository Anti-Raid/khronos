# @antiraid/lockdowns

This plugin allows for templates to interact with AntiRaid lockdowns

## Types

<div id="type.Lockdown" />

### Lockdown

A created lockdown

```json
{
  "id": "805c0dd1-a625-4875-81e4-8edc6a14f659",
  "reason": "Testing",
  "type": "qsl",
  "data": {},
  "created_at": "2025-01-13T04:57:43.488340240Z"
}
```

#### Fields

- `id` ([string](#type.string)): The id of the lockdown
- `reason` ([string](#type.string)): The reason for the lockdown
- `type` ([string](#type.string)): The type of lockdown in string form
- `data` ([any](#type.any)): The data associated with the lockdown
- `created_at` ([string](#type.string)): The time the lockdown was created


<div id="type.LockdownExecutor" />

### LockdownExecutor

An executor for listing, creating and removing lockdowns



#### Methods

##### LockdownExecutor:list

```lua
function LockdownExecutor:list(): {Lockdown}
```

Lists all active lockdowns

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Returns

- `lockdowns` ([{Lockdown}](#type.Lockdown)): A list of all currently active lockdowns
##### LockdownExecutor:qsl

```lua
function LockdownExecutor:qsl(reason: string)
```

Starts a quick server lockdown

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `reason` ([string](#type.string)): The reason for the lockdown

##### LockdownExecutor:tsl

```lua
function LockdownExecutor:tsl(reason: string)
```

Starts a traditional server lockdown

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `reason` ([string](#type.string)): The reason for the lockdown

##### LockdownExecutor:scl

```lua
function LockdownExecutor:scl(channel: string, reason: string)
```

Starts a lockdown on a single channel

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `channel` ([string](#type.string)): The channel to lock down
- `reason` ([string](#type.string)): The reason for the lockdown

##### LockdownExecutor:role

```lua
function LockdownExecutor:role(role: string, reason: string)
```

Starts a lockdown on a role

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `role` ([string](#type.string)): The role to lock down
- `reason` ([string](#type.string)): The reason for the lockdown

##### LockdownExecutor:remove

```lua
function LockdownExecutor:remove(id: string)
```

Removes a lockdown

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `id` ([string](#type.string)): The id of the lockdown to remove



## Methods

### new

```lua
function new(token: TemplateContext): LockdownExecutor
```

#### Parameters

- `token` ([TemplateContext](#type.TemplateContext)): The token of the template to use


#### Returns

- `executor` ([LockdownExecutor](#type.LockdownExecutor)): A lockdown executor