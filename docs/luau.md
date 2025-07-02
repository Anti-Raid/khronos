<div id="@antiraid/luau"></div>

# @antiraid/luau

<div id="Types"></div>

## Types

<div id="Chunk"></div>

## Chunk

<details>
<summary>Raw Type</summary>

```luau
type Chunk = {
	--- Sets the environment of the chunk (_G).
	environment: {
		[any]: any
	}?,

	--- Sets the optimization level of the chunk.
	optimization_level: number?,

	--- Text code to be evaluated. Bytecode evaluation is not allowed due to
	--- security reasons.
	code: string,

	---  The name of the chunk, used for debugging purposes.
	chunk_name: string?,

	--- Takes in args and returns the returned values from the ``code`` being evaluated. This will run the code in main thread / coroutine.running() == nil
	call: (self: Chunk, args: any) -> any,

	--- @yields
	---
	--- Takes in args and returns the returned values from the ``code`` being evaluated.
	---
	--- This runs the code asynchronously within a coroutine, allowing it to call
	--- yielding functions
	call_async: (self: Chunk, args: any) -> any
}
```

</details>

<div id="call"></div>

### call

Takes in args and returns the returned values from the ``code`` being evaluated. This will run the code in main thread / coroutine.running() == nil

<details>
<summary>Function Signature</summary>

```luau
--- Takes in args and returns the returned values from the ``code`` being evaluated. This will run the code in main thread / coroutine.running() == nil
call: (self: Chunk, args: any) -> any
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="args"></div>

##### args

[any](#any)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[any](#any)<div id="call_async"></div>

### call_async

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Takes in args and returns the returned values from the ``code`` being evaluated.



This runs the code asynchronously within a coroutine, allowing it to call

yielding functions

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Takes in args and returns the returned values from the ``code`` being evaluated.
---
--- This runs the code asynchronously within a coroutine, allowing it to call
--- yielding functions
call_async: (self: Chunk, args: any) -> any
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="args"></div>

##### args

[any](#any)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[any](#any)<div id="environment"></div>

### environment

Sets the environment of the chunk (_G).

*This field is optional and may not be specified*

{[any]: [any](#any)}?

<div id="optimization_level"></div>

### optimization_level

Sets the optimization level of the chunk.

*This field is optional and may not be specified*

[number](#number)?

<div id="code"></div>

### code

Text code to be evaluated. Bytecode evaluation is not allowed due to

security reasons.

[string](#string)

<div id="chunk_name"></div>

### chunk_name

The name of the chunk, used for debugging purposes.

*This field is optional and may not be specified*

[string](#string)?

<div id="Functions"></div>

# Functions

<div id="load"></div>

## load

Loads a Luau chunk.

<details>
<summary>Function Signature</summary>

```luau
--- Loads a Luau chunk.
function load(code: string) -> Chunk end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="code"></div>

### code

[string](#string)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[Chunk](#Chunk)<div id="format"></div>

## format

Formats a set of values to a string

<details>
<summary>Function Signature</summary>

```luau
--- Formats a set of values to a string
function format(...: any) -> string end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="..."></div>

### ...

[any](#any)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[string](#string)