# @antiraid/lazy

This plugin allows for templates to interact with and create 'lazy' data as well as providing documentation for the type. Note that events are *not* 'lazy' data's and have their own semantics.

## Types

<div id="type.Lazy<T>" />

### Lazy<T>

A lazy data type that is only serialized to Lua upon first access. This can be much more efficient than serializing the data every time it is accessed. Note that events are *not* 'lazy' data's and have their own semantics.

#### Fields

- `data` ([T](#type.T)): The inner data. This is cached upon first access
- `lazy` ([boolean](#type.boolean)): Always returns true. Allows the user to check if the data is a lazy or not


## Methods

### new

```lua
function new(data: TemplateContext): Lazy<any>
```

Creates a new Lazy type from data. This can be useful as a deep-copy implementation [``lazy.new(value).data`` is guaranteed to do a deepcopy of data as long as ``value`` is serializable]

#### Parameters

- `data` ([TemplateContext](#type.TemplateContext)): The data to wrap in a lazy

#### Returns

- `lazy` ([Lazy<any>](#type.Lazy)): A lazy value