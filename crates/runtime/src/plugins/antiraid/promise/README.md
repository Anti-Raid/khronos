# @antiraid/promise

Lua Promises, yield for a promise to execute the async action returning its result.

## Types

<div id="type.LuaPromise" />

### LuaPromise<T>

LuaPromise<T> provides a promise that must be yielded to actually execute and get the result of the async action.

## Methods

### yield

```lua
function yield(promise: LuaPromise<T>): T
```

Yields the promise to execute the async action and return its result. Note that this is the only function other than `stream.next` that yields.

#### Parameters

- `promise` ([LuaPromise<T>](#type.LuaPromise<T>)): The promise to yield.


#### Returns

- `T` ([T](#type.T)): The result of executing the promise.

---

## Promise Execution Cycle

When you create a promise, it does not do anything (in essence, it acts like a *future*). You must yield the promise to actually execute the async action and get the result. This is because the Lua VM is single-threaded and cannot execute things concurrently so your code must yield to allow the Promises' internal code to run and return the result back, which *resumes* your code.

```lua
local promise = someAsyncAction() -- A LuaPromise<T> is returned
local result = promise.yield(promise) -- Now, result is a ``T``!
```

While usually not very useful, the created promise can also be re-used multiple times, as it is not consumed by yielding it.

```lua
local promise = someAsyncAction() -- A LuaPromise<T> is returned
local result1 = promise.yield(promise) -- Now, result1 is a ``T``!
local result2 = promise.yield(promise) -- Now, result2 is a ``T``!
```
