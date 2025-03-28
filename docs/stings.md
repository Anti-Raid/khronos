<div id="@antiraid/stings"></div>

# @antiraid/stings

<div id="Types"></div>

## Types

<div id="StingCreate"></div>

## StingCreate

A type representing a new sting to be created.

<details>
<summary>Raw Type</summary>

```luau
--- A type representing a new sting to be created.
type StingCreate = {
	--- The source of the sting.
	src: string?,

	--- The number of stings.
	stings: number,

	--- The reason for the stings.
	reason: string?,

	--- The reason the stings were voided.
	void_reason: string?,

	--- The guild ID the sting targets. MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON
	guild_id: string,

	--- The creator of the sting. Must be either 'system' or 'user:{user_id}'
	creator: string,

	--- The target of the sting. Must be either 'system' or 'user:{user_id}'
	target: string,

	--- The state of the sting.
	state: "active" | "voided" | "handled",

	--- When the sting expires as a duration. Format: {duration}{unit} e.g. 1d, 1h, 1m, 1s
	duration: string?,

	--- The data/metadata present within the sting, if any.
	sting_data: any?
}
```

</details>

<div id="src"></div>

### src

The source of the sting.

*This field is optional and may not be specified*

[string](#string)?

<div id="stings"></div>

### stings

The number of stings.

[number](#number)

<div id="reason"></div>

### reason

The reason for the stings.

*This field is optional and may not be specified*

[string](#string)?

<div id="void_reason"></div>

### void_reason

The reason the stings were voided.

*This field is optional and may not be specified*

[string](#string)?

<div id="guild_id"></div>

### guild_id

The guild ID the sting targets. MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON

[string](#string)

<div id="creator"></div>

### creator

The creator of the sting. Must be either 'system' or 'user:{user_id}'

[string](#string)

<div id="target"></div>

### target

The target of the sting. Must be either 'system' or 'user:{user_id}'

[string](#string)

<div id="state"></div>

### state

The state of the sting.

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"active"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"voided"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"handled"
```

</details>

<div id="duration"></div>

### duration

When the sting expires as a duration. Format: {duration}{unit} e.g. 1d, 1h, 1m, 1s

*This field is optional and may not be specified*

[string](#string)?

<div id="sting_data"></div>

### sting_data

The data/metadata present within the sting, if any.

*This field is optional and may not be specified*

[any](#any)?

<div id="Sting"></div>

## Sting

A type representing a sting.

<details>
<summary>Raw Type</summary>

```luau
--- A type representing a sting.
type Sting = {
	--- The ID of the sting.
	id: string,

	--- The source of the sting.
	src: string?,

	--- The number of stings.
	stings: number,

	--- The reason for the stings.
	reason: string?,

	--- The reason the stings were voided.
	void_reason: string?,

	--- The guild ID the sting targets. MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON
	guild_id: string,

	--- The creator of the sting. Must be either 'system' or 'user:{user_id}'
	creator: string,

	--- The target of the sting. Must be either 'system' or 'user:{user_id}'
	target: string,

	--- The state of the sting.
	state: "active" | "voided" | "handled",

	--- When the sting expires as a duration. Format: {duration}{unit} e.g. 1d, 1h, 1m, 1s
	duration: string?,

	--- The data/metadata present within the sting, if any.
	sting_data: any?,

	--- The log of the sting as it was being handled internally by AntiRaid's internal systens
	handle_log: any?
}
```

</details>

<div id="id"></div>

### id

The ID of the sting.

[string](#string)

<div id="src"></div>

### src

The source of the sting.

*This field is optional and may not be specified*

[string](#string)?

<div id="stings"></div>

### stings

The number of stings.

[number](#number)

<div id="reason"></div>

### reason

The reason for the stings.

*This field is optional and may not be specified*

[string](#string)?

<div id="void_reason"></div>

### void_reason

The reason the stings were voided.

*This field is optional and may not be specified*

[string](#string)?

<div id="guild_id"></div>

### guild_id

The guild ID the sting targets. MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON

[string](#string)

<div id="creator"></div>

### creator

The creator of the sting. Must be either 'system' or 'user:{user_id}'

[string](#string)

<div id="target"></div>

### target

The target of the sting. Must be either 'system' or 'user:{user_id}'

[string](#string)

<div id="state"></div>

### state

The state of the sting.

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"active"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"voided"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"handled"
```

</details>

<div id="duration"></div>

### duration

When the sting expires as a duration. Format: {duration}{unit} e.g. 1d, 1h, 1m, 1s

*This field is optional and may not be specified*

[string](#string)?

<div id="sting_data"></div>

### sting_data

The data/metadata present within the sting, if any.

*This field is optional and may not be specified*

[any](#any)?

<div id="handle_log"></div>

### handle_log

The log of the sting as it was being handled internally by AntiRaid's internal systens

*This field is optional and may not be specified*

[any](#any)?

<div id="StingExecutor"></div>

## StingExecutor

<details>
<summary>Raw Type</summary>

```luau
type StingExecutor = {
	--- Lists a page of stings. The number of stings per page is undefined at this time
	--- @param page number The page to list
	--- @return Promise.LuaPromise<{Sting}> The list of stings
	list: (self: StingExecutor, page: number) -> Promise.LuaPromise<{Sting}>,

	--- Gets a sting by its ID
	--- @param id string The ID of the sting
	--- @return Promise.LuaPromise<Sting> The sting
	get: (self: StingExecutor, id: string) -> Promise.LuaPromise<Sting>,

	--- Creates a new sting
	--- @param data StingCreate The data to create the sting with
	--- @return Promise.LuaPromise<string> The ID of the created sting
	create: (self: StingExecutor, data: StingCreate) -> Promise.LuaPromise<string>,

	--- Updates a sting 
	--- @param data Sting The data to update the sting with. Note that the ID of the sting must exist in DB and the previous sting
	--- with said ID will be replaced with ``data``.
	--- @return Promise.LuaPromise<nil>
	update: (self: StingExecutor, data: Sting) -> Promise.LuaPromise<nil>,

	--- Deletes a sting by its ID
	--- @param id string The ID of the sting
	--- @return Promise.LuaPromise<nil>
	delete: (self: StingExecutor, id: string) -> Promise.LuaPromise<nil>
}
```

</details>

<div id="list"></div>

### list

Lists a page of stings. The number of stings per page is undefined at this time

<details>
<summary>Function Signature</summary>

```luau
--- Lists a page of stings. The number of stings per page is undefined at this time
--- @param page number The page to list
--- @return Promise.LuaPromise<{Sting}> The list of stings
list: (self: StingExecutor, page: number) -> Promise.LuaPromise<{Sting}>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="page"></div>

##### page

number The page to list

[number](#number)

<div id="Returns"></div>

#### Returns

<div id="Promise.LuaPromise<{Sting}>"></div>

##### Promise.LuaPromise<{Sting}>

The list of stings

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;{[Sting](#Sting)}&gt;<div id="get"></div>

### get

Gets a sting by its ID

<details>
<summary>Function Signature</summary>

```luau
--- Gets a sting by its ID
--- @param id string The ID of the sting
--- @return Promise.LuaPromise<Sting> The sting
get: (self: StingExecutor, id: string) -> Promise.LuaPromise<Sting>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the sting

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="Promise.LuaPromise<Sting>"></div>

##### Promise.LuaPromise<Sting>

The sting

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[Sting](#Sting)&gt;<div id="create"></div>

### create

Creates a new sting

<details>
<summary>Function Signature</summary>

```luau
--- Creates a new sting
--- @param data StingCreate The data to create the sting with
--- @return Promise.LuaPromise<string> The ID of the created sting
create: (self: StingExecutor, data: StingCreate) -> Promise.LuaPromise<string>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

StingCreate The data to create the sting with

[StingCreate](#StingCreate)

<div id="Returns"></div>

#### Returns

<div id="Promise.LuaPromise<string>"></div>

##### Promise.LuaPromise<string>

The ID of the created sting

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[string](#string)&gt;<div id="update"></div>

### update

Updates a sting

with said ID will be replaced with ``data``.

<details>
<summary>Function Signature</summary>

```luau
--- Updates a sting 
--- @param data Sting The data to update the sting with. Note that the ID of the sting must exist in DB and the previous sting
--- with said ID will be replaced with ``data``.
--- @return Promise.LuaPromise<nil>
update: (self: StingExecutor, data: Sting) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

Sting The data to update the sting with. Note that the ID of the sting must exist in DB and the previous sting

[Sting](#Sting)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](./promise.md).[LuaPromise](./promise.md#LuaPromise)&lt;[nil](#nil)&gt;<div id="delete"></div>

### delete

Deletes a sting by its ID

<details>
<summary>Function Signature</summary>

```luau
--- Deletes a sting by its ID
--- @param id string The ID of the sting
--- @return Promise.LuaPromise<nil>
delete: (self: StingExecutor, id: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the sting

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
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> StingExecutor end
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

[StingExecutor](#StingExecutor)