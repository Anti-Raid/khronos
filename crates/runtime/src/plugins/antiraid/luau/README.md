# @antiraid/luau

Make and evaluate custom luau code chunks.

## Types

<div id="type.Chunk" />

### Chunk

A luau code chunk.

#### Fields

- `environment` (table): The environment the chunk can access. Requires ``luau:eval.set_environment`` to modify
- `optimization_level` ([number](#type.number)): The Luau compiler optimization level to use. Can be either ``0``, ``1`` or ``2``. Requires ``luau:eval.set_optimization_level`` to modify
- `code` ([string](#type.string)): The luau code to execute. Can be set at chunk creation time with just ``luau:eval`` and modified after creation with ``luau:eval.modify_set_code``
- ``chunk_name`` ([string](#type.string)): The name of the chunk. Requires  ``luau:eval.set_chunk_name`` to modify

#### Methods

##### call

```lua
function call(...): ...
```

Calls the chunk synchronously with the provided arguments. Requires ``luau:eval.call`` to use.

###### Parameters

- `...` ([...](#type.any)): The arguments to pass to the chunk.


###### Returns

- `result` ([...](#type.any)): The result of evaluating the chunk.

##### call_async

```lua
function call_async(...): Promise<...>
```

Calls the chunk asynchronously with the provided arguments. Requires ``luau:eval.call_async`` to use.

**Note that this method returns a promise that must be yielded using [`promise.yield`](#type.promise.yield) to actually execute and return results.**

###### Parameters

- `...` ([...](#type.any)): The arguments to pass to the chunk.


###### Returns

- `result` ([...](#type.any)): The result of evaluating the chunk.

## Methods

### load

```lua
function load(code: string): Chunk
```

Loads a luau code chunk. Note that setting other properties on the chunk must be done after loading and requires their respective capabilities. Loading a chunk does *not* parse or validate the code, it is only stored for later use.

#### Parameters

- `code` ([string](#type.string)): The luau code to load.

#### Returns

- `chunk` ([Chunk](#type.Chunk)): The loaded chunk.