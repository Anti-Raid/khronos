<div id="primitives"></div>

# primitives

<div id="Types"></div>

## Types

<div id="u8"></div>

## u8

AntiRaid primitives



Last updated: June 22nd 2025

Core types

<details>
<summary>Raw Type</summary>

```luau
-- AntiRaid primitives
-- 
-- Last updated: June 22nd 2025
-- Core types
type u8 = number
```

</details>

[number](#number)

<div id="u16"></div>

## u16

<details>
<summary>Raw Type</summary>

```luau
type u16 = number
```

</details>

[number](#number)

<div id="u32"></div>

## u32

<details>
<summary>Raw Type</summary>

```luau
type u32 = number
```

</details>

[number](#number)

<div id="u64"></div>

## u64

<details>
<summary>Raw Type</summary>

```luau
type u64 = number
```

</details>

[number](#number)

<div id="i8"></div>

## i8

<details>
<summary>Raw Type</summary>

```luau
type i8 = number
```

</details>

[number](#number)

<div id="i16"></div>

## i16

<details>
<summary>Raw Type</summary>

```luau
type i16 = number
```

</details>

[number](#number)

<div id="i32"></div>

## i32

<details>
<summary>Raw Type</summary>

```luau
type i32 = number
```

</details>

[number](#number)

<div id="i64"></div>

## i64

<details>
<summary>Raw Type</summary>

```luau
type i64 = number
```

</details>

[number](#number)

<div id="f32"></div>

## f32

<details>
<summary>Raw Type</summary>

```luau
type f32 = number
```

</details>

[number](#number)

<div id="f64"></div>

## f64

<details>
<summary>Raw Type</summary>

```luau
type f64 = number
```

</details>

[number](#number)

<div id="bool"></div>

## bool

<details>
<summary>Raw Type</summary>

```luau
type bool = boolean
```

</details>

[boolean](#boolean)

<div id="char"></div>

## char

<details>
<summary>Raw Type</summary>

```luau
type char = string
```

</details>

[string](#string)

<div id="byte"></div>

## byte

<details>
<summary>Raw Type</summary>

```luau
type byte = number
```

</details>

[number](#number)

<div id="Event"></div>

## Event

An event that has been dispatched to the template.



In templates, this is ``arg``.

<details>
<summary>Raw Type</summary>

```luau
--- An event that has been dispatched to the template. 
--- 
--- In templates, this is ``arg``.
type Event = {
	--- The base name of the event.
	base_name: string,

	--- The name of the event.
	name: string,

	--- The data of the event.
	data: any,

	--- The author of the event, if any.
	author: string?
}
```

</details>

<div id="base_name"></div>

### base_name

The base name of the event.

[string](#string)

<div id="name"></div>

### name

The name of the event.

[string](#string)

<div id="data"></div>

### data

The data of the event.

[any](#any)

<div id="author"></div>

### author

The author of the event, if any.

*This field is optional and may not be specified*

[string](#string)?

<div id="ScriptData"></div>

## ScriptData

Information about a script

<details>
<summary>Raw Type</summary>

```luau
--- Information about a script
type ScriptData = {
	--- The guild ID the template is in.
	guild_id: string,

	--- The name of the template.
	name: string,

	--- The description of the template.
	description: string?,

	--- The name of the template as it appears on the template shop listing.
	shop_name: string?,

	--- The owner of the template on the template shop.
	shop_owner: string?,

	--- The events that this template listens to.
	events: {string},

	--- The channel to send errors to.
	error_channel: string?,

	--- The language of the template.
	lang: string,

	--- The allowed capabilities the template has access to.
	allowed_caps: {string},

	--- The user who created the template.
	created_by: string,

	--- The time the template was created.
	created_at: string,

	--- The user who last updated the template.
	updated_by: string,

	--- The time the template was last updated.
	updated_at: string
}
```

</details>

<div id="guild_id"></div>

### guild_id

The guild ID the template is in.

[string](#string)

<div id="name"></div>

### name

The name of the template.

[string](#string)

<div id="description"></div>

### description

The description of the template.

*This field is optional and may not be specified*

[string](#string)?

<div id="shop_name"></div>

### shop_name

The name of the template as it appears on the template shop listing.

*This field is optional and may not be specified*

[string](#string)?

<div id="shop_owner"></div>

### shop_owner

The owner of the template on the template shop.

*This field is optional and may not be specified*

[string](#string)?

<div id="events"></div>

### events

The events that this template listens to.

{[string](#string)}

<div id="error_channel"></div>

### error_channel

The channel to send errors to.

*This field is optional and may not be specified*

[string](#string)?

<div id="lang"></div>

### lang

The language of the template.

[string](#string)

<div id="allowed_caps"></div>

### allowed_caps

The allowed capabilities the template has access to.

{[string](#string)}

<div id="created_by"></div>

### created_by

The user who created the template.

[string](#string)

<div id="created_at"></div>

### created_at

The time the template was created.

[string](#string)

<div id="updated_by"></div>

### updated_by

The user who last updated the template.

[string](#string)

<div id="updated_at"></div>

### updated_at

The time the template was last updated.

[string](#string)

<div id="Limitations"></div>

## Limitations

<details>
<summary>Raw Type</summary>

```luau
type Limitations = {
	--- Capabilities that the template has access to.
	capabilities: {string}
}
```

</details>

<div id="capabilities"></div>

### capabilities

Capabilities that the template has access to.

{[string](#string)}

<div id="TemplateContext"></div>

## TemplateContext

TemplateContext is a struct that represents the context of a template.



Stores key data including the templates data, pragma and what capabilities it should have access to.



Passing a TemplateContext is often required when using AntiRaid plugins for getting the inner context

of a template.

<details>
<summary>Raw Type</summary>

```luau
--- TemplateContext is a struct that represents the context of a template. 
--- 
--- Stores key data including the templates data, pragma and what capabilities it should have access to. 
---
--- Passing a TemplateContext is often required when using AntiRaid plugins for getting the inner context
--- of a template.
type TemplateContext = {
	--- DataStores plugin
	DataStores: datastoresP.Plugin,

	--- Discord plugin
	Discord: discordP.Plugin,

	--- Image Captcha plugin
	ImageCaptcha: imgcaptchaP.Plugin,

	--- Key-Value plugin
	KV: kvP.Plugin,

	--- Lockdowns plugin
	Lockdowns: lockdownsP.Plugin,

	--- Object Storage plugin
	ObjectStorage: objectstorageP.Plugin,

	--- User Info plugin
	UserInfo: userinfoP.Plugin,

	--- The public data associated with the script.
	data: ScriptData,

	--- Returns the memory limit the VM has/the amount of memory the VM is allowed to use
	memory_limit: u64,

	--- The current guild ID the template is running on.
	guild_id: string,

	--- The owner guild ID of the template, if any. For shop templates, this will be the guild ID of the guild which owns the template on the shop
	--- For guild-owned templates, this will be the guild ID of the guild which owns the template [the same as guild_id]
	owner_guild_id: string,

	--- The name of the template itself
	template_name: string,

	--- The capabilities the template has access to.   
	allowed_caps: {string},

	--- Returns AntiRaid's discord user object [the current discord bot user driving the template].
	current_user: discord.UserObject,

	--- Returns if a template has a specific capability.
	has_cap: (self: TemplateContext, cap: string) -> boolean,

	--- Returns if a template has any of the specified capabilities.
	has_any_cap: (self: TemplateContext, caps: {string}) -> boolean,

	--- Returns a new TemplateContext with the specified limits which must be a strict subset
	--- of the current context's limits.
	withlimits: (self: TemplateContext, limits: Limitations) -> TemplateContext,

	--- The global/common store table shared across all templates within the same
	--- server
	store: {}
}
```

</details>

<div id="has_cap"></div>

### has_cap

Returns if a template has a specific capability.

<details>
<summary>Function Signature</summary>

```luau
--- Returns if a template has a specific capability.
has_cap: (self: TemplateContext, cap: string) -> boolean
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="cap"></div>

##### cap

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="has_any_cap"></div>

### has_any_cap

Returns if a template has any of the specified capabilities.

<details>
<summary>Function Signature</summary>

```luau
--- Returns if a template has any of the specified capabilities.
has_any_cap: (self: TemplateContext, caps: {string}) -> boolean
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="caps"></div>

##### caps

{[string](#string)}

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="withlimits"></div>

### withlimits

Returns a new TemplateContext with the specified limits which must be a strict subset

of the current context's limits.

<details>
<summary>Function Signature</summary>

```luau
--- Returns a new TemplateContext with the specified limits which must be a strict subset
--- of the current context's limits.
withlimits: (self: TemplateContext, limits: Limitations) -> TemplateContext
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="limits"></div>

##### limits

[Limitations](#Limitations)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[TemplateContext](#TemplateContext)<div id="DataStores"></div>

### DataStores

DataStores plugin

[datastoresP](./datastoresp.md).[Plugin](./datastoresp.md#Plugin)

<div id="Discord"></div>

### Discord

Discord plugin

[discordP](./discordp.md).[Plugin](./discordp.md#Plugin)

<div id="ImageCaptcha"></div>

### ImageCaptcha

Image Captcha plugin

[imgcaptchaP](./imgcaptchap.md).[Plugin](./imgcaptchap.md#Plugin)

<div id="KV"></div>

### KV

Key-Value plugin

[kvP](./kvp.md).[Plugin](./kvp.md#Plugin)

<div id="UnscopedKV"></div>

### UnscopedKV

Unscoped Key-Value plugin



This requires the `kv.meta:unscoped_allowed` capability to index into/use

[kvP](./kvp.md).[Plugin](./kvp.md#Plugin)

<div id="Lockdowns"></div>

### Lockdowns

Lockdowns plugin

[lockdownsP](./lockdownsp.md).[Plugin](./lockdownsp.md#Plugin)

<div id="ObjectStorage"></div>

### ObjectStorage

Object Storage plugin

[objectstorageP](./objectstoragep.md).[Plugin](./objectstoragep.md#Plugin)

<div id="UserInfo"></div>

### UserInfo

User Info plugin

[userinfoP](./userinfop.md).[Plugin](./userinfop.md#Plugin)

<div id="data"></div>

### data

The public data associated with the script.

[ScriptData](#ScriptData)

<div id="memory_limit"></div>

### memory_limit

Returns the memory limit the VM has/the amount of memory the VM is allowed to use

[u64](#u64)

<div id="guild_id"></div>

### guild_id

The current guild ID the template is running on.

[string](#string)

<div id="owner_guild_id"></div>

### owner_guild_id

The owner guild ID of the template, if any. For shop templates, this will be the guild ID of the guild which owns the template on the shop

For guild-owned templates, this will be the guild ID of the guild which owns the template [the same as guild_id]

[string](#string)

<div id="template_name"></div>

### template_name

The name of the template itself

[string](#string)

<div id="allowed_caps"></div>

### allowed_caps

The capabilities the template has access to.

{[string](#string)}

<div id="current_user"></div>

### current_user

Returns AntiRaid's discord user object [the current discord bot user driving the template].

[discord](./discord.md).[UserObject](./discord.md#UserObject)

<div id="store"></div>

### store

The global/common store table shared across all templates within the same

server

*This is an inline table type with the following fields*

