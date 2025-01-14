# Primitives

<div id="type.u8" />

## u8

```lua
type u8 = number
```

An unsigned 8-bit integer. **Note: u8 arrays (`{u8}`) are often used to represent an array of bytes in AntiRaid**

### Constraints

- **range**: The range of values this number can take on (accepted values: 0-255)

---

<div id="type.u16" />

## u16

```lua
type u16 = number
```

An unsigned 16-bit integer.

### Constraints

- **range**: The range of values this number can take on (accepted values: 0-65535)

---

<div id="type.u32" />

## u32

```lua
type u32 = number
```

An unsigned 32-bit integer.

### Constraints

- **range**: The range of values this number can take on (accepted values: 0-4294967295)

---

<div id="type.u64" />

## u64

```lua
type u64 = number
```

An unsigned 64-bit integer. **Note that most, if not all, cases of `i64` in the actual API are either `string` or the `I64` custom type from typesext**

### Constraints

- **range**: The range of values this number can take on (accepted values: 0-18446744073709551615)

---

<div id="type.i8" />

## i8

```lua
type i8 = number
```

A signed 8-bit integer.

### Constraints

- **range**: The range of values this number can take on (accepted values: -128-127)

---

<div id="type.i16" />

## i16

```lua
type i16 = number
```

A signed 16-bit integer.

### Constraints

- **range**: The range of values this number can take on (accepted values: -32768-32767)

---

<div id="type.i32" />

## i32

```lua
type i32 = number
```

A signed 32-bit integer.

### Constraints

- **range**: The range of values this number can take on (accepted values: -2147483648-2147483647)

---

<div id="type.i64" />

## i64

```lua
type i64 = number
```

A signed 64-bit integer. **Note that most, if not all, cases of `i64` in the actual API are either `string` or the `I64` custom type from typesext**

### Constraints

- **range**: The range of values this number can take on (accepted values: -9223372036854775808-9223372036854775807)

---

<div id="type.f32" />

## f32

```lua
type f32 = number
```

A 32-bit floating point number.

### Constraints

- **range**: The range of values this number can take on (accepted values: IEEE 754 single-precision floating point)

---

<div id="type.f64" />

## f64

```lua
type f64 = number
```

A 64-bit floating point number.

### Constraints

- **range**: The range of values this number can take on (accepted values: IEEE 754 double-precision floating point)

---

<div id="type.byte" />

## byte

```lua
type byte = number
```

An unsigned 8-bit integer that semantically stores a byte of information

### Constraints

- **range**: The range of values this number can take on (accepted values: 0-255)

---

<div id="type.bool" />

## bool

```lua
type bool = boolean
```

A boolean value.

---

<div id="type.char" />

## char

```lua
type char = string
```

A single Unicode character.

### Constraints

- **length**: The length of the string (accepted values: 1)

---

<div id="type.string" />

## string

```lua
type string = string
```

A UTF-8 encoded string.

### Constraints

- **encoding**: Accepted character encoding (accepted values: UTF-8 *only*)

---

<div id="type.function" />

## function

```lua
type function = function
```

A Lua function.

---

# Types

<div id="type.Event" />

## Event

An event that has been dispatched to the template. This is what `args` is in the template.



### Fields

- `base_name` ([string](#type.string)): The base name of the event.
- `name` ([string](#type.string)): The name of the event.
- `data` ([unknown](#type.unknown)): The data of the event.
- `can_respond` ([boolean](#type.boolean)): Whether the event can be responded to.
- `response` ([unknown](#type.unknown)): The current response of the event. This can be overwritten by the template by just setting it to a new value.
- `author` ([string?](#type.string)): The author of the event, if any. If there is no known author, this field will either be `nil` or `null`.


<div id="type.Template" />

## Template

`Template` is a struct that represents the data associated with a template. Fields are still being documented and subject to change.

```json
{
  "guild_id": "0",
  "name": "",
  "description": null,
  "shop_name": null,
  "shop_owner": null,
  "events": [],
  "error_channel": null,
  "content": "",
  "lang": "luau",
  "allowed_caps": [],
  "created_by": "",
  "created_at": "1970-01-01T00:00:00Z",
  "updated_by": "",
  "updated_at": "1970-01-01T00:00:00Z"
}
```

### Fields

- `language` ([string](#type.string)): The language of the template.
- `allowed_caps` ([{string}](#type.string)): The allowed capabilities provided to the template.


<div id="type.TemplateContext" />

## TemplateContext

`TemplateContext` is a struct that represents the context of a template. Stores data including the templates data, pragma and what capabilities it should have access to. Passing a TemplateContext is often required when using AntiRaid plugins for security purposes.



### Fields

- `template_data` ([TemplateData](#type.TemplateData)): The data associated with the template.
- `guild_id` ([string](#type.string)): The current guild ID the template is running on.
- `current_user` ([Serenity.User](#type.Serenity.User)): Returns AntiRaid's discord user object [the current discord bot user driving the template].
