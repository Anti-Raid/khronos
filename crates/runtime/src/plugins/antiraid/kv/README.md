# @antiraid/kv

Utilities for key-value operations.

## Types

<div id="type.KvRecord" />

### KvRecord

KvRecord represents a key-value record with metadata.

```json
{
  "key": "",
  "value": null,
  "exists": false,
  "created_at": null,
  "last_updated_at": null
}
```

#### Fields

- `key` ([string](#type.string)): The key of the record.
- `value` ([any](#type.any)): The value of the record.
- `exists` ([bool](#type.bool)): Whether the record exists.
- `created_at` ([datetime](#type.datetime)): The time the record was created.
- `last_updated_at` ([datetime](#type.datetime)): The time the record was last updated.


<div id="type.KvExecutor" />

### KvExecutor

KvExecutor allows templates to get, store and find persistent data within a scope.

#### Methods

##### KvExecutor:find

Finds records in a scoped key-value database. ``%`` can be used as wildcards before/after the query. E.g. ``%{KEY}%`` will search for ``{KEY}`` anywhere in the string, ``%{KEY}`` will search for keys which end with ``{KEY}`` and ``_{KEY}`` will search for a single character before ``{KEY}``.

```lua
function KvExecutor:find(key: string): {KvRecord}
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**

###### Parameters

- `key` ([string](#type.string)): The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%

###### Returns

- `records` ([{KvRecord}](#type.KvRecord)): The records found.

##### KvExecutor:exists

Determines if a key exists in the scoped key-value database.

```lua
function KvExecutor:exists(key: string): bool
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**

###### Parameters

- `key` ([string](#type.string)): The key to check for existence.

###### Returns

- `exists` ([bool](#type.bool)): Whether the key exists.

##### KvExecutor:get

Returns the value of a key in the scoped key-value database.

```lua
function KvExecutor:get(key: string)
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `key` ([string](#type.string)): The key to get.


###### Returns

- `value` ([any](#type.any)): The value of the key.- `exists` ([bool](#type.bool)): Whether the key exists.
##### KvExecutor:getrecord

```lua
function KvExecutor:getrecord(key: string): KvRecord
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `key` ([string](#type.string)): The key to get.


###### Returns

- `record` ([KvRecord](#type.KvRecord)): The record of the key.
##### KvExecutor:set

```lua
function KvExecutor:set(key: string, value: any)
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `key` ([string](#type.string)): The key to set.
- `value` ([any](#type.any)): The value to set.

##### KvExecutor:delete

```lua
function KvExecutor:delete(key: string)
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `key` ([string](#type.string)): The key to delete.



## Methods

### new

```lua
function new(token: TemplateContext, scope: string?): KvExecutor
```

#### Parameters

- `token` ([TemplateContext](#type.TemplateContext)): The token of the template to use.
- `scope` ([string?](#type.string)): The scope of the executor. `this_guild` to use the originating guilds data, `owner_guild` to use the KV of the guild that owns the template on the shop. Defaults to `this_guild` if not specified.


#### Returns

- `executor` ([KvExecutor](#type.KvExecutor)): A key-value executor.


TO MOVE TO PRIMITIVES DOCS

- `guild_id` ([string](#type.string)): The guild ID the executor will perform key-value operations on.
- `origin_guild_id` ([string](#type.string)): The originating guild ID (the guild ID of the template itself).
- `allowed_caps` ([{string}](#type.{string})): The allowed capabilities in the current context.
- `has_cap` ([function](#type.function)): A function that returns `true` if the current context has the capability specified.
- `scope` ([string](#type.string)): The scope of the executor. Either ``this_guild`` for the originating guild, or ``owner_guild`` for the guild that owns the template (the template that owns the template on the shop if a shop template or the guild that owns the template otherwise).