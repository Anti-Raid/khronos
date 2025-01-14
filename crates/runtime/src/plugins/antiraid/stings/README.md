# @antiraid/stings

List, get, create, update and delete stings on Anti-Raid.

## Types

<div id="type.StingCreate" />

### StingCreate

A type representing a new sting to be created.

```json
{
  "src": "test",
  "stings": 10,
  "reason": "test",
  "void_reason": null,
  "guild_id": "128384",
  "creator": "system",
  "target": "user:1945824",
  "state": "active",
  "duration": {
    "secs": 60,
    "nanos": 0
  },
  "sting_data": {
    "a": "b"
  }
}
```

#### Fields

- `src` ([string?](#type.string)): The source of the sting.
- `stings` ([number](#type.number)): The number of stings.
- `reason` ([string?](#type.string)): The reason for the stings.
- `void_reason` ([string?](#type.string)): The reason the stings were voided.
- `guild_id` ([string](#type.string)): The guild ID the sting targets. **MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON**
- `creator` ([StingTarget](#type.StingTarget)): The creator of the sting.
- `target` ([StingTarget](#type.StingTarget)): The target of the sting.
- `state` ([string](#type.string)): The state of the sting. Must be one of 'active', 'voided' or 'handled'
- `duration` ([Duration?](#type.Duration)): When the sting expires as a duration.
- `sting_data` ([any?](#type.any)): The data/metadata present within the sting, if any.


<div id="type.Sting" />

### Sting

Represents a sting on AntiRaid

```json
{
  "id": "470a2958-3827-4e59-8b97-928a583a37a3",
  "src": "test",
  "stings": 10,
  "reason": "test",
  "void_reason": null,
  "guild_id": "128384",
  "creator": "system",
  "target": "user:1945824",
  "state": "active",
  "created_at": "2025-01-13T04:57:43.488668165Z",
  "duration": {
    "secs": 60,
    "nanos": 0
  },
  "sting_data": {
    "a": "b"
  },
  "handle_log": {
    "a": "b"
  }
}
```

#### Fields

- `id` ([string](#type.string)): The sting ID.
- `src` ([string?](#type.string)): The source of the sting.
- `stings` ([number](#type.number)): The number of stings.
- `reason` ([string?](#type.string)): The reason for the stings.
- `void_reason` ([string?](#type.string)): The reason the stings were voided.
- `guild_id` ([string](#type.string)): The guild ID the sting targets. **MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON**
- `creator` ([StingTarget](#type.StingTarget)): The creator of the sting.
- `target` ([StingTarget](#type.StingTarget)): The target of the sting.
- `state` ([StingState](#type.StingState)): The state of the sting.
- `duration` ([Duration?](#type.Duration)): When the sting expires as a duration.
- `sting_data` ([any?](#type.any)): The data/metadata present within the sting, if any.
- `handle_log` ([any](#type.any)): The handle log encountered while handling the sting.
- `created_at` ([string](#type.string)): When the sting was created at.


<div id="type.StingExecutor" />

### StingExecutor

An sting executor is used to execute actions related to stings from Lua templates



#### Methods

##### StingExecutor:list

```lua
function StingExecutor:list(page: number): {Sting}
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `page` ([number](#type.number)): The page number to fetch.


###### Returns

- `stings` ([{Sting}](#type.Sting)): The list of stings.
##### StingExecutor:get

```lua
function StingExecutor:get(id: string): Sting
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `id` ([string](#type.string)): The sting ID.


###### Returns

- `sting` ([Sting](#type.Sting)): The sting.
##### StingExecutor:create

```lua
function StingExecutor:create(data: StingCreate): string
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `data` ([StingCreate](#type.StingCreate)): The sting data.


###### Returns

- `id` ([string](#type.string)): The sting ID of the created sting.
##### StingExecutor:update

```lua
function StingExecutor:update(data: Sting)
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `data` ([Sting](#type.Sting)): The sting to update to. Note that if an invalid ID is used, this method may either do nothing or error out.

##### StingExecutor:delete

```lua
function StingExecutor:delete(id: string)
```

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**



###### Parameters

- `id` ([string](#type.string)): The sting ID.



## Enums

<div id="type.StingTarget" />

### StingTarget

The target of the sting.

There are two variants: ``system`` (A system target/no associated user) and ``user:{user_id}`` (A user target)
