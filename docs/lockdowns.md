<div id="@antiraid/lockdowns"></div>

# @antiraid/lockdowns

<div id="Types"></div>

## Types

<div id="Lockdown"></div>

## Lockdown

Lockdown represents a currently applied lockdown

@class Lockdown

<details>
<summary>Raw Type</summary>

```luau
--- Lockdown represents a currently applied lockdown
---@class Lockdown
---@field id string The ID of the lockdown.
---@field reason string The reason for the lockdown.
---@field type string The type of the lockdown in its string form
---@field data string The data internally stored for the lockdown.
---@field created_at string The timestamp when the lockdown was created.
type Lockdown = {
	id: string,

	reason: string,

	type: string,

	data: any,

	created_at: string
}
```

</details>

<div id="id"></div>

### id

[string](#string)

<div id="reason"></div>

### reason

[string](#string)

<div id="type"></div>

### type

[string](#string)

<div id="data"></div>

### data

[any](#any)

<div id="created_at"></div>

### created_at

[string](#string)

<div id="LockdownExecutor"></div>

## LockdownExecutor

LockdownExecutor allows templates to list, create and delete AntiRaid lockdowns

@class LockdownExecutor

<details>
<summary>Raw Type</summary>

```luau
--- LockdownExecutor allows templates to list, create and delete AntiRaid lockdowns
---@class LockdownExecutor
type LockdownExecutor = {
	--- Lists all active lockdowns
	--- @return Promise.LuaPromise<{Lockdown}> The active lockdowns
	list: (self: LockdownExecutor) -> Promise.LuaPromise<{Lockdown}>,

	--- Starts a quick server lockdown
	--- @param reason string The reason for the lockdown
	qsl: (self: LockdownExecutor, reason: string) -> Promise.LuaPromise<nil>,

	--- Starts a traditional server lockdown.
	--- 
	--- This is *much* slower than a QSL but also does not require
	--- any special server setup.
	--- @param reason string The reason for the lockdown
	tsl: (self: LockdownExecutor, reason: string) -> Promise.LuaPromise<nil>,

	--- Starts a lockdown on a single channel
	--- @param channel_id string The ID of the channel to lock down
	--- @param reason string The reason for the lockdown
	scl: (self: LockdownExecutor, channel_id: string, reason: string) -> Promise.LuaPromise<nil>,

	--- Starts a lockdown on a role
	--- @param role_id string The ID of the role to lock down
	--- @param reason string The reason for the lockdown
	role: (self: LockdownExecutor, role_id: string, reason: string) -> Promise.LuaPromise<nil>,

	--- Removes a lockdown (ends it)
	--- @param id string The ID of the lockdown to remove
	remove: (self: LockdownExecutor, id: string) -> Promise.LuaPromise<nil>
}
```

</details>

<div id="list"></div>

### list

Lists all active lockdowns

<details>
<summary>Function Signature</summary>

```luau
--- Lists all active lockdowns
--- @return Promise.LuaPromise<{Lockdown}> The active lockdowns
list: (self: LockdownExecutor) -> Promise.LuaPromise<{Lockdown}>
```

</details>

<div id="Returns"></div>

#### Returns

<div id="Promise.LuaPromise<{Lockdown}>"></div>

##### Promise.LuaPromise<{Lockdown}>

The active lockdowns

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;{[Lockdown](#Lockdown)}&gt;<div id="qsl"></div>

### qsl

Starts a quick server lockdown

<details>
<summary>Function Signature</summary>

```luau
--- Starts a quick server lockdown
--- @param reason string The reason for the lockdown
qsl: (self: LockdownExecutor, reason: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="reason"></div>

##### reason

string The reason for the lockdown

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[nil](#nil)&gt;<div id="tsl"></div>

### tsl

Starts a traditional server lockdown.



This is *much* slower than a QSL but also does not require

any special server setup.

<details>
<summary>Function Signature</summary>

```luau
--- Starts a traditional server lockdown.
--- 
--- This is *much* slower than a QSL but also does not require
--- any special server setup.
--- @param reason string The reason for the lockdown
tsl: (self: LockdownExecutor, reason: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="reason"></div>

##### reason

string The reason for the lockdown

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[nil](#nil)&gt;<div id="scl"></div>

### scl

Starts a lockdown on a single channel

<details>
<summary>Function Signature</summary>

```luau
--- Starts a lockdown on a single channel
--- @param channel_id string The ID of the channel to lock down
--- @param reason string The reason for the lockdown
scl: (self: LockdownExecutor, channel_id: string, reason: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="channel_id"></div>

##### channel_id

string The ID of the channel to lock down

[string](#string)

<div id="reason"></div>

##### reason

string The reason for the lockdown

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[nil](#nil)&gt;<div id="role"></div>

### role

Starts a lockdown on a role

<details>
<summary>Function Signature</summary>

```luau
--- Starts a lockdown on a role
--- @param role_id string The ID of the role to lock down
--- @param reason string The reason for the lockdown
role: (self: LockdownExecutor, role_id: string, reason: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="role_id"></div>

##### role_id

string The ID of the role to lock down

[string](#string)

<div id="reason"></div>

##### reason

string The reason for the lockdown

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[nil](#nil)&gt;<div id="remove"></div>

### remove

Removes a lockdown (ends it)

<details>
<summary>Function Signature</summary>

```luau
--- Removes a lockdown (ends it)
--- @param id string The ID of the lockdown to remove
remove: (self: LockdownExecutor, id: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the lockdown to remove

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[nil](#nil)&gt;<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

<details>
<summary>Function Signature</summary>

```luau
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> LockdownExecutor end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="token"></div>

### token

[Primitives](./primitives.md).[TemplateContext](./primitives.md#TemplateContext)

<div id="scope"></div>

### scope

*This field is optional and may not be specified*

[ExecutorScope](./executorscope.md).[ExecutorScope](./executorscope.md#ExecutorScope)?

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[LockdownExecutor](#LockdownExecutor)