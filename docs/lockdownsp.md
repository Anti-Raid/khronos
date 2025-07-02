<div id="lockdownsp"></div>

# lockdownsp

<div id="Types"></div>

## Types

<div id="CreateLockdownMode"></div>

## CreateLockdownMode

Structure for creating new lockdown modes

<details>
<summary>Raw Type</summary>

```luau
--- Structure for creating new lockdown modes
type CreateLockdownMode = {
	--- The syntax of the lockdown mode
	syntax: string,

	--- Converts the string form of the lockdown mode to a LockdownMode
	to_lockdown_mode: (self: CreateLockdownMode, string_form: string) -> LockdownMode?
}
```

</details>

<div id="to_lockdown_mode"></div>

### to_lockdown_mode

Converts the string form of the lockdown mode to a LockdownMode

<details>
<summary>Function Signature</summary>

```luau
--- Converts the string form of the lockdown mode to a LockdownMode
to_lockdown_mode: (self: CreateLockdownMode, string_form: string) -> LockdownMode?
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="string_form"></div>

##### string_form

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LockdownMode](#LockdownMode)?<div id="syntax"></div>

### syntax

The syntax of the lockdown mode

[string](#string)

<div id="LockdownMode"></div>

## LockdownMode

LockdownMode represents a lockdown mode

<details>
<summary>Raw Type</summary>

```luau
--- LockdownMode represents a lockdown mode
type LockdownMode = {
	--- The syntax of the lockdown mode
	syntax: string,

	--- The creator of the lockdown mode
	creator: CreateLockdownMode,

	--- The string form of the lockdown mode
	string_form: string,

	--- The specificity of the lockdown mode
	specificity: number
}
```

</details>

<div id="syntax"></div>

### syntax

The syntax of the lockdown mode

[string](#string)

<div id="creator"></div>

### creator

The creator of the lockdown mode

[CreateLockdownMode](#CreateLockdownMode)

<div id="string_form"></div>

### string_form

The string form of the lockdown mode

[string](#string)

<div id="specificity"></div>

### specificity

The specificity of the lockdown mode

[number](#number)

<div id="Lockdown"></div>

## Lockdown

Lockdown represents a currently applied lockdown

<details>
<summary>Raw Type</summary>

```luau
--- Lockdown represents a currently applied lockdown
type Lockdown = {
	--- The ID of the lockdown.
	id: string,

	--- The reason for the lockdown.
	reason: string,

	--- The type of the lockdown in its string form
	type: LockdownMode,

	--- The data internally stored for the lockdown.
	data: any,

	--- The timestamp when the lockdown was created.
	created_at: datetime.DateTime
}
```

</details>

<div id="id"></div>

### id

The ID of the lockdown.

[string](#string)

<div id="reason"></div>

### reason

The reason for the lockdown.

[string](#string)

<div id="type"></div>

### type

The type of the lockdown in its string form

[LockdownMode](#LockdownMode)

<div id="data"></div>

### data

The data internally stored for the lockdown.

[any](#any)

<div id="created_at"></div>

### created_at

The timestamp when the lockdown was created.

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)

<div id="GuildLockdownSettings"></div>

## GuildLockdownSettings

<details>
<summary>Raw Type</summary>

```luau
type GuildLockdownSettings = {
	member_roles: {string},

	require_correct_layout: boolean
}
```

</details>

<div id="member_roles"></div>

### member_roles

{[string](#string)}

<div id="require_correct_layout"></div>

### require_correct_layout

[boolean](#boolean)

<div id="LockdownSet"></div>

## LockdownSet

<details>
<summary>Raw Type</summary>

```luau
type LockdownSet = {
	--- Lockdowns currently applied to the guild
	lockdowns: {Lockdown},

	--- The settings for the guild
	settings: GuildLockdownSettings,

	--- Sorts the lockdowns by specificity in descending order. Mutates the lockdowns array of the LockdownSet
	---
	--- This acquires a exclusive write lock on the LockdownSet
	sort: (self: LockdownSet) -> (),

	--- @yields
	---
	--- Applies a lockdown. 
	---
	--- This acquires a exclusive write lock on the LockdownSet
	apply: (self: LockdownSet, lockdown_type: LockdownMode, reason: string) -> LockdownAddStatus,

	--- @yields
	---
	--- Removes a lockdown by ID
	---
	--- This acquires a exclusive write lock on the LockdownSet
	remove: (self: LockdownSet, id: string) -> LockdownRemoveStatus
}
```

</details>

<div id="sort"></div>

### sort

Sorts the lockdowns by specificity in descending order. Mutates the lockdowns array of the LockdownSet



This acquires a exclusive write lock on the LockdownSet

<details>
<summary>Function Signature</summary>

```luau
--- Sorts the lockdowns by specificity in descending order. Mutates the lockdowns array of the LockdownSet
---
--- This acquires a exclusive write lock on the LockdownSet
sort: (self: LockdownSet) -> ()
```

</details>

<div id="apply"></div>

### apply

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Applies a lockdown.



This acquires a exclusive write lock on the LockdownSet

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Applies a lockdown. 
---
--- This acquires a exclusive write lock on the LockdownSet
apply: (self: LockdownSet, lockdown_type: LockdownMode, reason: string) -> LockdownAddStatus
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="lockdown_type"></div>

##### lockdown_type

[LockdownMode](#LockdownMode)

<div id="reason"></div>

##### reason

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LockdownAddStatus](#LockdownAddStatus)<div id="remove"></div>

### remove

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Removes a lockdown by ID



This acquires a exclusive write lock on the LockdownSet

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Removes a lockdown by ID
---
--- This acquires a exclusive write lock on the LockdownSet
remove: (self: LockdownSet, id: string) -> LockdownRemoveStatus
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LockdownRemoveStatus](#LockdownRemoveStatus)<div id="lockdowns"></div>

### lockdowns

Lockdowns currently applied to the guild

{[Lockdown](#Lockdown)}

<div id="settings"></div>

### settings

The settings for the guild

[GuildLockdownSettings](#GuildLockdownSettings)

<div id="LockdownAddStatus"></div>

## LockdownAddStatus

LockdownAddStatus represents the result of adding a lockdown

<details>
<summary>Raw Type</summary>

```luau
--- LockdownAddStatus represents the result of adding a lockdown
type LockdownAddStatus = {
	--- Whether or not the lockdown was added
	ok: boolean?,

	--- The error type
	type: "Ok" | "Error" | "LockdownTestFailed",

	--- If ok is true, the ID of the created lockdown
	id: string?,

	--- If type is LockdownTestFailed, the test result (which can be used to try fixing the issue automagically)
	test_result: LockdownTestResult?,

	--- The error as a string
	error: string?,

	-- Metatable
	--- Converts the LockdownAddStatus to a error string
	---
	--- The format is unspecified and may change in the future
	__tostring: (self: LockdownAddStatus) -> string
}
```

</details>

<div id="ok"></div>

### ok

Whether or not the lockdown was added

*This field is optional and may not be specified*

[boolean](#boolean)?

<div id="type"></div>

### type

The error type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Ok"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Error"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"LockdownTestFailed"
```

</details>

<div id="id"></div>

### id

If ok is true, the ID of the created lockdown

*This field is optional and may not be specified*

[string](#string)?

<div id="test_result"></div>

### test_result

If type is LockdownTestFailed, the test result (which can be used to try fixing the issue automagically)

*This field is optional and may not be specified*

[LockdownTestResult](#LockdownTestResult)?

<div id="error"></div>

### error

The error as a string

*This field is optional and may not be specified*

[string](#string)?

<div id="MetatableFields"></div>

### Metatable Fields

<div id="__tostring"></div>

#### __tostring

Converts the LockdownAddStatus to a error string



The format is unspecified and may change in the future

<details>
<summary>Function Signature</summary>

```luau
--- Converts the LockdownAddStatus to a error string
---
--- The format is unspecified and may change in the future
__tostring: (self: LockdownAddStatus) -> string
```

</details>

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="LockdownRemoveStatus"></div>

## LockdownRemoveStatus

LockdownRemoveStatus represents the result of removing a lockdown

<details>
<summary>Raw Type</summary>

```luau
--- LockdownRemoveStatus represents the result of removing a lockdown
type LockdownRemoveStatus = {
	--- Whether or not the lockdown was removed
	ok: boolean?,

	--- The error type
	type: "Ok" | "Error" | "LockdownTestFailed",

	--- If type is LockdownTestFailed, the test result (which can be used to try fixing the issue automagically)
	test_result: LockdownTestResult?,

	--- The error as a string
	error: string?,

	-- Metatable
	--- Converts the LockdownRemoveStatus to a error string
	---
	--- The format is unspecified and may change in the future
	__tostring: (self: LockdownAddStatus) -> string
}
```

</details>

<div id="ok"></div>

### ok

Whether or not the lockdown was removed

*This field is optional and may not be specified*

[boolean](#boolean)?

<div id="type"></div>

### type

The error type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Ok"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Error"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"LockdownTestFailed"
```

</details>

<div id="test_result"></div>

### test_result

If type is LockdownTestFailed, the test result (which can be used to try fixing the issue automagically)

*This field is optional and may not be specified*

[LockdownTestResult](#LockdownTestResult)?

<div id="error"></div>

### error

The error as a string

*This field is optional and may not be specified*

[string](#string)?

<div id="MetatableFields"></div>

### Metatable Fields

<div id="__tostring"></div>

#### __tostring

Converts the LockdownRemoveStatus to a error string



The format is unspecified and may change in the future

<details>
<summary>Function Signature</summary>

```luau
--- Converts the LockdownRemoveStatus to a error string
---
--- The format is unspecified and may change in the future
__tostring: (self: LockdownAddStatus) -> string
```

</details>

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="LockdownTestResult"></div>

## LockdownTestResult

The result of testing a servers state prior to applying a lockdown

<details>
<summary>Raw Type</summary>

```luau
--- The result of testing a servers state prior to applying a lockdown
type LockdownTestResult = {
	can_apply_perfectly: boolean,

	--- The role changes needed. Type is a tuple of (Add | Remove, Permissions)
	role_changes_needed: {
		[string]: any
	},

	--- Other changes needed
	other_changes_needed: {string},

	--- Display the changeset, resolved with role names
	display_changeset: (self: LockdownTestResult, lockdown_set: LockdownSet) -> string,

	--- @yields
	---
	--- Tries to autofix the issues
	try_auto_fix: (self: LockdownTestResult, lockdown_set: LockdownSet) -> nil,

	-- Metatable
	--- Converts the LockdownTestResult to a error string
	__tostring: (self: LockdownTestResult) -> string
}
```

</details>

<div id="can_apply_perfectly"></div>

### can_apply_perfectly

[boolean](#boolean)

<div id="role_changes_needed"></div>

### role_changes_needed

The role changes needed. Type is a tuple of (Add | Remove, Permissions)

*This is an inline table type with the following fields*

<div id="[string]"></div>

##### [string]

[any](#any)

<div id="other_changes_needed"></div>

### other_changes_needed

Other changes needed

{[string](#string)}

<div id="display_changeset"></div>

### display_changeset

Display the changeset, resolved with role names

<details>
<summary>Function Signature</summary>

```luau
--- Display the changeset, resolved with role names
display_changeset: (self: LockdownTestResult, lockdown_set: LockdownSet) -> string
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="lockdown_set"></div>

##### lockdown_set

[LockdownSet](#LockdownSet)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="try_auto_fix"></div>

### try_auto_fix

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Tries to autofix the issues

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Tries to autofix the issues
try_auto_fix: (self: LockdownTestResult, lockdown_set: LockdownSet) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="lockdown_set"></div>

##### lockdown_set

[LockdownSet](#LockdownSet)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="MetatableFields"></div>

### Metatable Fields

<div id="__tostring"></div>

#### __tostring

Converts the LockdownTestResult to a error string

<details>
<summary>Function Signature</summary>

```luau
--- Converts the LockdownTestResult to a error string
__tostring: (self: LockdownTestResult) -> string
```

</details>

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="LockdownExecutor"></div>

## LockdownExecutor

LockdownExecutor allows templates to list, create and delete AntiRaid lockdowns

@class LockdownExecutor

<details>
<summary>Raw Type</summary>

```luau
--- LockdownExecutor allows templates to list, create and delete AntiRaid lockdowns
---@class LockdownExecutor
type LockdownExecutor = {
	--- @yields
	--- Fetches the current lockdown set of the guild
	---
	--- A lockdown set is the main entrypoint for viewing and applying lockdowns
	---
	--- Needed capability: ``lockdowns:fetch_lockdown_set``
	fetch_lockdown_set: (self: LockdownExecutor) -> LockdownSet
}
```

</details>

<div id="fetch_lockdown_set"></div>

### fetch_lockdown_set

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>

Fetches the current lockdown set of the guild



A lockdown set is the main entrypoint for viewing and applying lockdowns



Needed capability: ``lockdowns:fetch_lockdown_set``

<details>
<summary>Function Signature</summary>

```luau
--- @yields
--- Fetches the current lockdown set of the guild
---
--- A lockdown set is the main entrypoint for viewing and applying lockdowns
---
--- Needed capability: ``lockdowns:fetch_lockdown_set``
fetch_lockdown_set: (self: LockdownExecutor) -> LockdownSet
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LockdownSet](#LockdownSet)<div id="Plugin"></div>

## Plugin

<details>
<summary>Raw Type</summary>

```luau
type Plugin = {
	Client: LockdownExecutor,

	CreateQuickServerLockdown: CreateLockdownMode,

	CreateTraditionalServerLockdown: CreateLockdownMode,

	CreateSingleChannelLockdown: (channel_id: string) -> CreateLockdownMode,

	CreateRoleLockdown: (role_id: string) -> CreateLockdownMode,

	QuickServerLockdown: () -> LockdownMode,

	TraditionalServerLockdown: () -> LockdownMode,

	SingleChannelLockdown: (channel_id: string) -> LockdownMode,

	RoleLockdown: (role_id: string) -> LockdownMode
}
```

</details>

<div id="CreateSingleChannelLockdown"></div>

### CreateSingleChannelLockdown

<details>
<summary>Function Signature</summary>

```luau
CreateSingleChannelLockdown: (channel_id: string) -> CreateLockdownMode
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

[CreateLockdownMode](#CreateLockdownMode)<div id="CreateRoleLockdown"></div>

### CreateRoleLockdown

<details>
<summary>Function Signature</summary>

```luau
CreateRoleLockdown: (role_id: string) -> CreateLockdownMode
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="role_id"></div>

##### role_id

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[CreateLockdownMode](#CreateLockdownMode)<div id="QuickServerLockdown"></div>

### QuickServerLockdown

<details>
<summary>Function Signature</summary>

```luau
QuickServerLockdown: () -> LockdownMode
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LockdownMode](#LockdownMode)<div id="TraditionalServerLockdown"></div>

### TraditionalServerLockdown

<details>
<summary>Function Signature</summary>

```luau
TraditionalServerLockdown: () -> LockdownMode
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LockdownMode](#LockdownMode)<div id="SingleChannelLockdown"></div>

### SingleChannelLockdown

<details>
<summary>Function Signature</summary>

```luau
SingleChannelLockdown: (channel_id: string) -> LockdownMode
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

[LockdownMode](#LockdownMode)<div id="RoleLockdown"></div>

### RoleLockdown

<details>
<summary>Function Signature</summary>

```luau
RoleLockdown: (role_id: string) -> LockdownMode
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="role_id"></div>

##### role_id

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[LockdownMode](#LockdownMode)<div id="Client"></div>

### Client

[LockdownExecutor](#LockdownExecutor)

<div id="CreateQuickServerLockdown"></div>

### CreateQuickServerLockdown

[CreateLockdownMode](#CreateLockdownMode)

<div id="CreateTraditionalServerLockdown"></div>

### CreateTraditionalServerLockdown

[CreateLockdownMode](#CreateLockdownMode)

