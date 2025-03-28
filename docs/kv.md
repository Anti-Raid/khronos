<div id="@antiraid/kv"></div>

# @antiraid/kv

<div id="Types"></div>

## Types

<div id="KvRecord"></div>

## KvRecord

KvRecord represents a key-value record with metadata.

@class KvRecord

<details>
<summary>Raw Type</summary>

```luau
--- KvRecord represents a key-value record with metadata.
---@class KvRecord
---@field key string The key of the record.
---@field value any The value of the record.
---@field exists boolean Whether the record exists.
---@field created_at string The timestamp when the record was created.
---@field last_updated_at string The timestamp when the record was last updated.
type KvRecord = {
	key: string,

	value: any,

	exists: boolean,

	created_at: string,

	last_updated_at: string
}
```

</details>

<div id="key"></div>

### key

[string](#string)

<div id="value"></div>

### value

[any](#any)

<div id="exists"></div>

### exists

[boolean](#boolean)

<div id="created_at"></div>

### created_at

[string](#string)

<div id="last_updated_at"></div>

### last_updated_at

[string](#string)

<div id="KvExecutor"></div>

## KvExecutor

KvExecutor allows templates to get, store and find persistent data within a server.

@class KvExecutor

<details>
<summary>Raw Type</summary>

```luau
--- KvExecutor allows templates to get, store and find persistent data within a server.
---@class KvExecutor
type KvExecutor = {
	--- The guild ID the executor will perform key-value operations on.
	guild_id: string,

	--- The originating guild ID (the guild ID of the template itself).
	origin_guild_id: string,

	--- The scope of the executor.
	scope: ExecutorScope.ExecutorScope,

	--- Finds records in the key-value store.
	--- @param query string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%
	--- @return {KvRecord} The records.
	find: (self: KvExecutor, query: string) -> Promise.LuaPromise<{KvRecord}>,

	--- Gets a value from the key-value store.
	--- @param key string The key of the record.
	--- @return any The value of the record.
	get: (self: KvExecutor, key: string) -> Promise.LuaPromise<any>,

	--- Gets a record from the key-value store.
	--- @param key string The key of the record.
	--- @return KvRecord The record.
	getrecord: (self: KvExecutor, key: string) -> Promise.LuaPromise<KvRecord>,

	--- Sets a record in the key-value store.
	--- @param key string The key of the record.
	--- @param value any The value of the record.
	--- @return KvRecord The record.
	set: (self: KvExecutor, key: string, value: any) -> Promise.LuaPromise<KvRecord>,

	--- Deletes a record from the key-value store.
	--- @param key string The key of the record.
	delete: (self: KvExecutor, key: string) -> Promise.LuaPromise<nil>
}
```

</details>

<div id="find"></div>

### find

Finds records in the key-value store.

<details>
<summary>Function Signature</summary>

```luau
--- Finds records in the key-value store.
--- @param query string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%
--- @return {KvRecord} The records.
find: (self: KvExecutor, query: string) -> Promise.LuaPromise<{KvRecord}>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="query"></div>

##### query

string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="{KvRecord}"></div>

##### {KvRecord}

The records.

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;{[KvRecord](#KvRecord)}&gt;<div id="get"></div>

### get

Gets a value from the key-value store.

<details>
<summary>Function Signature</summary>

```luau
--- Gets a value from the key-value store.
--- @param key string The key of the record.
--- @return any The value of the record.
get: (self: KvExecutor, key: string) -> Promise.LuaPromise<any>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="any"></div>

##### any

The value of the record.

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[any](#any)&gt;<div id="getrecord"></div>

### getrecord

Gets a record from the key-value store.

<details>
<summary>Function Signature</summary>

```luau
--- Gets a record from the key-value store.
--- @param key string The key of the record.
--- @return KvRecord The record.
getrecord: (self: KvExecutor, key: string) -> Promise.LuaPromise<KvRecord>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="KvRecord"></div>

##### KvRecord

The record.

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[KvRecord](#KvRecord)&gt;<div id="set"></div>

### set

Sets a record in the key-value store.

<details>
<summary>Function Signature</summary>

```luau
--- Sets a record in the key-value store.
--- @param key string The key of the record.
--- @param value any The value of the record.
--- @return KvRecord The record.
set: (self: KvExecutor, key: string, value: any) -> Promise.LuaPromise<KvRecord>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="value"></div>

##### value

any The value of the record.

[any](#any)

<div id="Returns"></div>

#### Returns

<div id="KvRecord"></div>

##### KvRecord

The record.

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[KvRecord](#KvRecord)&gt;<div id="delete"></div>

### delete

Deletes a record from the key-value store.

<details>
<summary>Function Signature</summary>

```luau
--- Deletes a record from the key-value store.
--- @param key string The key of the record.
delete: (self: KvExecutor, key: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[nil](#nil)&gt;<div id="guild_id"></div>

### guild_id

The guild ID the executor will perform key-value operations on.

[string](#string)

<div id="origin_guild_id"></div>

### origin_guild_id

The originating guild ID (the guild ID of the template itself).

[string](#string)

<div id="scope"></div>

### scope

The scope of the executor.

[ExecutorScope](./executorscope.md).[ExecutorScope](./executorscope.md#ExecutorScope)

<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

<details>
<summary>Function Signature</summary>

```luau
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> KvExecutor end
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

[KvExecutor](#KvExecutor)