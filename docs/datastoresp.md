<div id="datastoresp"></div>

# datastoresp

<div id="Types"></div>

## Types

<div id="DataStore"></div>

## DataStore

A Base DataStore object.

<details>
<summary>Raw Type</summary>

```luau
--- A Base DataStore object.
type DataStore = {
	--- The name of the DataStore
	name: string,

	--- Whether or not a specific operation needs capabilities (either ``datastore:{name}`` or ``datastore:{name}:{operation}``)
	needs_caps: (op: string) -> boolean,

	--- The methods exposed by the DataStore
	methods: () -> {string}
}
```

</details>

<div id="needs_caps"></div>

### needs_caps

Whether or not a specific operation needs capabilities (either ``datastore:{name}`` or ``datastore:{name}:{operation}``)

<details>
<summary>Function Signature</summary>

```luau
--- Whether or not a specific operation needs capabilities (either ``datastore:{name}`` or ``datastore:{name}:{operation}``)
needs_caps: (op: string) -> boolean
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="op"></div>

##### op

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="methods"></div>

### methods

The methods exposed by the DataStore

<details>
<summary>Function Signature</summary>

```luau
--- The methods exposed by the DataStore
methods: () -> {string}
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

{[string](#string)}<div id="name"></div>

### name

The name of the DataStore

[string](#string)

<div id="CopyDataStore"></div>

## CopyDataStore

Datastore to copy a KhronosValue to another KhronosValue

<details>
<summary>Raw Type</summary>

```luau
--- Datastore to copy a KhronosValue to another KhronosValue
type CopyDataStore = DataStore & {
	copy: (...: ...khronosvalue.KhronosValue) -> khronosvalue.KhronosValue
}
```

</details>

Intersection with variants:

<details>
<summary>Variant 1</summary>

[DataStore](#DataStore)

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

<div id="copy"></div>

#### copy

<details>
<summary>Function Signature</summary>

```luau
copy: (...: ...khronosvalue.KhronosValue) -> khronosvalue.KhronosValue
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="..."></div>

##### ...

...

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)</details>

<div id="StatsStore"></div>

## StatsStore

Datastore providing basic statistics

<details>
<summary>Raw Type</summary>

```luau
--- Datastore providing basic statistics
type StatsStore = DataStore & {
	--- @yields
	---
	--- Returns the statistics of the bot.
	stats: () -> {
		total_cached_guilds: number,

		total_guilds: number,

		total_users: number,

		last_started_at: datetime.DateTime
	}
}
```

</details>

Intersection with variants:

<details>
<summary>Variant 1</summary>

[DataStore](#DataStore)

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

<div id="stats"></div>

#### stats

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Returns the statistics of the bot.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Returns the statistics of the bot.
stats: () -> {
		total_cached_guilds: number,

		total_guilds: number,

		total_users: number,

		last_started_at: datetime.DateTime
	}
```

</details>

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

{total_cached_guilds: [number](#number), total_guilds: [number](#number), total_users: [number](#number), last_started_at: [datetime](./datetime.md).[DateTime](./datetime.md#DateTime)}</details>

<div id="LinksStore"></div>

## LinksStore

<details>
<summary>Raw Type</summary>

```luau
type LinksStore = DataStore & {
	links: () -> {
		support_server: string,

		api_url: string,

		frontend_url: string,

		docs_url: string
	}
}
```

</details>

Intersection with variants:

<details>
<summary>Variant 1</summary>

[DataStore](#DataStore)

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

<div id="links"></div>

#### links

<details>
<summary>Function Signature</summary>

```luau
links: () -> {
		support_server: string,

		api_url: string,

		frontend_url: string,

		docs_url: string
	}
```

</details>

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

{support_server: [string](#string), api_url: [string](#string), frontend_url: [string](#string), docs_url: [string](#string)}</details>

<div id="Spawn"></div>

## Spawn

Jobserver

<details>
<summary>Raw Type</summary>

```luau
-- Jobserver
type Spawn = {
	--- The name of the job
	name: string,

	--- The data to be used in the job
	data: khronosvalue.KhronosValue,

	--- Whether or not to create the job if it does not exist
	create: boolean,

	--- Whether or not to execute the job immediately
	execute: boolean,

	--- If create is false, this is required
	id: string?
}
```

</details>

<div id="name"></div>

### name

The name of the job

[string](#string)

<div id="data"></div>

### data

The data to be used in the job

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)

<div id="create"></div>

### create

Whether or not to create the job if it does not exist

[boolean](#boolean)

<div id="execute"></div>

### execute

Whether or not to execute the job immediately

[boolean](#boolean)

<div id="id"></div>

### id

If create is false, this is required

*This field is optional and may not be specified*

[string](#string)?

<div id="Statuses"></div>

## Statuses

Jobsrver Status

<details>
<summary>Raw Type</summary>

```luau
--- Jobsrver Status
type Statuses = {
	level: string,

	msg: string,

	--- Timestamp in seconds
	ts: number,

	--- ptional list of fields to ignore
	bot_display_ignore: {string}?,

	-- Extra information as a key-value map 
	extra_info: {
		[string]: khronosvalue.KhronosValue
	}
}
```

</details>

<div id="level"></div>

### level

[string](#string)

<div id="msg"></div>

### msg

[string](#string)

<div id="ts"></div>

### ts

Timestamp in seconds

[number](#number)

<div id="bot_display_ignore"></div>

### bot_display_ignore

ptional list of fields to ignore

*This field is optional and may not be specified*

{[string](#string)}?

<div id="extra_info"></div>

### extra_info

Extra information as a key-value map

*This is an inline table type with the following fields*

<div id="[string]"></div>

##### [string]

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)

<div id="Output"></div>

## Output

<details>
<summary>Raw Type</summary>

```luau
type Output = {
	filename: string,

	--- Temporary flag for migrations
	perguild: boolean?
}
```

</details>

<div id="filename"></div>

### filename

[string](#string)

<div id="perguild"></div>

### perguild

Temporary flag for migrations

*This field is optional and may not be specified*

[boolean](#boolean)?

<div id="Job"></div>

## Job

Jobserver Job

<details>
<summary>Raw Type</summary>

```luau
--- Jobserver Job
type Job = {
	id: string,

	name: string,

	output: Output?,

	fields: {
		[string]: any
	},

	statuses: {Statuses},

	guild_id: string,

	expiry: datetime.TimeDelta?,

	state: string,

	resumable: boolean,

	created_at: datetime.DateTime,

	job_path: string,

	job_file_path: string?
}
```

</details>

<div id="id"></div>

### id

[string](#string)

<div id="name"></div>

### name

[string](#string)

<div id="output"></div>

### output

*This field is optional and may not be specified*

[Output](#Output)?

<div id="fields"></div>

### fields

*This is an inline table type with the following fields*

<div id="[string]"></div>

##### [string]

[any](#any)

<div id="statuses"></div>

### statuses

{[Statuses](#Statuses)}

<div id="guild_id"></div>

### guild_id

[string](#string)

<div id="expiry"></div>

### expiry

*This field is optional and may not be specified*

[datetime](./datetime.md).[TimeDelta](./datetime.md#TimeDelta)?

<div id="state"></div>

### state

[string](#string)

<div id="resumable"></div>

### resumable

[boolean](#boolean)

<div id="created_at"></div>

### created_at

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)

<div id="job_path"></div>

### job_path

[string](#string)

<div id="job_file_path"></div>

### job_file_path

*This field is optional and may not be specified*

[string](#string)?

<div id="JobServerStore"></div>

## JobServerStore

<details>
<summary>Raw Type</summary>

```luau
type JobServerStore = DataStore & {
	--- @yields
	---
	--- Spawns a new job on the jobserver returning the job ID.
	spawn: (spawn: Spawn) -> string,

	--- @yields
	---
	--- Lists all jobs created for this server
	---
	--- If needs_statuses is set to true, then statuses are sent, otherwise
	---- the statuses will be an empty table to reduce memory consumption
	list: (needs_statuses: boolean?) -> {Job},

	--- @yields
	---
	--- Lists all jobs created for this server with the given task name
	---
	--- If needs_statuses is set to true, then statuses are sent, otherwise
	---- the statuses will be an empty table to reduce memory consumption
	list_named: (name: string, needs_statuses: boolean?) -> {Job},

	--- @yields
	---
	--- Gets a job from jobserver given its job ID
	---
	--- If needs_statuses is set to true, then statuses are sent, otherwise
	---- the statuses will be an empty table to reduce memory consumption
	get: (id: string, needs_statuses: boolean?) -> Job,

	--- @yields
	---
	--- Deletes a job given its job ID
	delete: (id: string) -> nil
}
```

</details>

Intersection with variants:

<details>
<summary>Variant 1</summary>

[DataStore](#DataStore)

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

<div id="spawn"></div>

#### spawn

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Spawns a new job on the jobserver returning the job ID.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Spawns a new job on the jobserver returning the job ID.
spawn: (spawn: Spawn) -> string
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="spawn"></div>

##### spawn

[Spawn](#Spawn)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="list"></div>

#### list

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Lists all jobs created for this server



If needs_statuses is set to true, then statuses are sent, otherwise

- the statuses will be an empty table to reduce memory consumption

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Lists all jobs created for this server
---
--- If needs_statuses is set to true, then statuses are sent, otherwise
---- the statuses will be an empty table to reduce memory consumption
list: (needs_statuses: boolean?) -> {Job}
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="needs_statuses"></div>

##### needs_statuses

*This field is optional and may not be specified*

[boolean](#boolean)?

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

{[Job](#Job)}<div id="list_named"></div>

#### list_named

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Lists all jobs created for this server with the given task name



If needs_statuses is set to true, then statuses are sent, otherwise

- the statuses will be an empty table to reduce memory consumption

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Lists all jobs created for this server with the given task name
---
--- If needs_statuses is set to true, then statuses are sent, otherwise
---- the statuses will be an empty table to reduce memory consumption
list_named: (name: string, needs_statuses: boolean?) -> {Job}
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="name"></div>

##### name

[string](#string)

<div id="needs_statuses"></div>

##### needs_statuses

*This field is optional and may not be specified*

[boolean](#boolean)?

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

{[Job](#Job)}<div id="get"></div>

#### get

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a job from jobserver given its job ID



If needs_statuses is set to true, then statuses are sent, otherwise

- the statuses will be an empty table to reduce memory consumption

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a job from jobserver given its job ID
---
--- If needs_statuses is set to true, then statuses are sent, otherwise
---- the statuses will be an empty table to reduce memory consumption
get: (id: string, needs_statuses: boolean?) -> Job
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="id"></div>

##### id

[string](#string)

<div id="needs_statuses"></div>

##### needs_statuses

*This field is optional and may not be specified*

[boolean](#boolean)?

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[Job](#Job)<div id="delete"></div>

#### delete

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a job given its job ID

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a job given its job ID
delete: (id: string) -> nil
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="id"></div>

##### id

[string](#string)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)</details>

<div id="DataStoreExecutor"></div>

## DataStoreExecutor

A datastore executor holds DataStore's

<details>
<summary>Raw Type</summary>

```luau
--- A datastore executor holds DataStore's 
type DataStoreExecutor = {
	StatsStore: StatsStore?,

	-- AntiRaid bot only (CLI not supported)
	CopyDataStore: CopyDataStore?,

	-- Should be present
	LinksStore: LinksStore?,

	-- AntiRaid bot only (CLI not supported)
	JobServerStore: JobServerStore?
}
```

</details>

<div id="StatsStore"></div>

### StatsStore

*This field is optional and may not be specified*

[StatsStore](#StatsStore)?

<div id="CopyDataStore"></div>

### CopyDataStore

AntiRaid bot only (CLI not supported)

*This field is optional and may not be specified*

[CopyDataStore](#CopyDataStore)?

<div id="LinksStore"></div>

### LinksStore

Should be present

*This field is optional and may not be specified*

[LinksStore](#LinksStore)?

<div id="JobServerStore"></div>

### JobServerStore

AntiRaid bot only (CLI not supported)

*This field is optional and may not be specified*

[JobServerStore](#JobServerStore)?

<div id="Plugin"></div>

## Plugin

<details>
<summary>Raw Type</summary>

```luau
type Plugin = DataStoreExecutor
```

</details>

[DataStoreExecutor](#DataStoreExecutor)

