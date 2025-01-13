# @antiraid/typesext

Extra types used by Anti-Raid Lua templating subsystem to either add in common functionality such as streams or handle things like u64/i64 types performantly.

## Types

<div id="type.MultiOption" />

### MultiOption<T>

MultiOption allows distinguishing between `null` and empty fields. Use the value to show both existence and value (`Some(Some(value))`) an empty object to show existence (``Some(None)``) or null to show neither (`None`)



<div id="type.U64" />

### U64

U64 is a 64-bit unsigned integer type. Implements Add/Subtract/Multiply/Divide/Modulus/Power/Integer Division/Equality/Comparison (Lt/Le and its complements Gt/Ge) and ToString with a type name of U64



#### Methods

##### U64:to_ne_bytes

```lua
function U64:to_ne_bytes(): {u8}
```

Converts the U64 to a little-endian byte array.

###### Returns

- `bytes` ([{u8}](#type.u8)): The little-endian byte array.
##### U64:from_ne_bytes

```lua
function U64:from_ne_bytes(bytes: {u8}): U64
```

Converts a little-endian byte array to a U64.

###### Parameters

- `bytes` ([{u8}](#type.u8)): The little-endian byte array.


###### Returns

- `u64` ([U64](#type.U64)): The U64 value.
##### U64:to_le_bytes

```lua
function U64:to_le_bytes(): {u8}
```

Converts the U64 to a little-endian byte array.

###### Returns

- `bytes` ([{u8}](#type.u8)): The little-endian byte array.
##### U64:from_le_bytes

```lua
function U64:from_le_bytes(bytes: {u8}): U64
```

Converts a little-endian byte array to a U64.

###### Parameters

- `bytes` ([{u8}](#type.u8)): The little-endian byte array.


###### Returns

- `u64` ([U64](#type.U64)): The U64 value.
##### U64:to_be_bytes

```lua
function U64:to_be_bytes(): {u8}
```

Converts the U64 to a big-endian byte array.

###### Returns

- `bytes` ([{u8}](#type.u8)): The big-endian byte array.
##### U64:from_be_bytes

```lua
function U64:from_be_bytes(bytes: {u8}): U64
```

Converts a big-endian byte array to a U64.

###### Parameters

- `bytes` ([{u8}](#type.u8)): The big-endian byte array.


###### Returns

- `u64` ([U64](#type.U64)): The U64 value.
##### U64:to_i64

```lua
function U64:to_i64(): I64
```

Converts the U64 to an i64.

###### Returns

- `i64` ([I64](#type.I64)): The i64 value.


<div id="type.I64" />

### I64

I64 is a 64-bit signed integer type. Implements Add/Subtract/Multiply/Divide/Modulus/Power/Integer Division/Equality/Comparison (Lt/Le and its complements Gt/Ge) and ToString with a type name of I64



#### Methods

##### I64:to_ne_bytes

```lua
function I64:to_ne_bytes(): {u8}
```

Converts the I64 to a little-endian byte array.

###### Returns

- `bytes` ([{u8}](#type.u8)): The little-endian byte array.
##### I64:from_ne_bytes

```lua
function I64:from_ne_bytes(bytes: {u8}): I64
```

Converts a little-endian byte array to a I64.

###### Parameters

- `bytes` ([{u8}](#type.u8)): The little-endian byte array.


###### Returns

- `i64` ([I64](#type.I64)): The I64 value.
##### I64:to_le_bytes

```lua
function I64:to_le_bytes(): {u8}
```

Converts the I64 to a little-endian byte array.

###### Returns

- `bytes` ([{u8}](#type.u8)): The little-endian byte array.
##### I64:from_le_bytes

```lua
function I64:from_le_bytes(bytes: {u8}): I64
```

Converts a little-endian byte array to a I64.

###### Parameters

- `bytes` ([{u8}](#type.u8)): The little-endian byte array.


###### Returns

- `i64` ([I64](#type.I64)): The I64 value.
##### I64:to_be_bytes

```lua
function I64:to_be_bytes(): {u8}
```

Converts the I64 to a big-endian byte array.

###### Returns

- `bytes` ([{u8}](#type.u8)): The big-endian byte array.
##### I64:from_be_bytes

```lua
function I64:from_be_bytes(bytes: {u8}): I64
```

Converts a big-endian byte array to a I64.

###### Parameters

- `bytes` ([{u8}](#type.u8)): The big-endian byte array.


###### Returns

- `i64` ([I64](#type.I64)): The I64 value.
##### I64:to_u64

```lua
function I64:to_u64(): U64
```

Converts the I64 to a U64.

###### Returns

- `u64` ([U64](#type.U64)): The U64 value.


<div id="type.bitu64" />

### bitu64

[bit32](https://luau.org/library#bit32-library) but for U64 datatype. Note that bit64 is experimental and may not be properly documented at all times. When in doubt, reach for Luau's bit32 documentation and simply replace 31's with 63's



#### Methods

##### bitu64:band

```lua
function bitu64:band(values: {U64}): U64
```

Performs a bitwise AND operation on the given values.

###### Parameters

- `values` ([{U64}](#type.U64)): The values to perform the operation on.


###### Returns

- `result` ([U64](#type.U64)): The result of the operation.
##### bitu64:bnor

```lua
function bitu64:bnor(n: U64): U64
```

Performs a bitwise NOR operation on the given value.

###### Parameters

- `n` ([U64](#type.U64)): The value to perform the operation on.


###### Returns

- `result` ([U64](#type.U64)): The result of the operation.
##### bitu64:bor

```lua
function bitu64:bor(values: {U64}): U64
```

Performs a bitwise OR operation on the given values.

###### Parameters

- `values` ([{U64}](#type.U64)): The values to perform the operation on.


###### Returns

- `result` ([U64](#type.U64)): The result of the operation.
##### bitu64:bxor

```lua
function bitu64:bxor(values: {U64}): U64
```

Performs a bitwise XOR operation on the given values.

###### Parameters

- `values` ([{U64}](#type.U64)): The values to perform the operation on.


###### Returns

- `result` ([U64](#type.U64)): The result of the operation.
##### bitu64:btest

```lua
function bitu64:btest(values: {U64}): bool
```

Tests if the bitwise AND of the given values is not zero.

###### Parameters

- `values` ([{U64}](#type.U64)): The values to perform the operation on.


###### Returns

- `result` ([bool](#type.bool)): True if the bitwise AND of the values is not zero, false otherwise.
##### bitu64:extract

```lua
function bitu64:extract(n: U64, f: u64, w: u64): U64
```

Extracts a field from a value.

###### Parameters

- `n` ([U64](#type.U64)): The value to extract the field from.
- `f` ([u64](#type.u64)): The field to extract.
- `w` ([u64](#type.u64)): The width of the field to extract.


###### Returns

- `result` ([U64](#type.U64)): The extracted field.
##### bitu64:lrotate

```lua
function bitu64:lrotate(n: U64, i: i64): U64
```

Rotates a value left or right.

###### Parameters

- `n` ([U64](#type.U64)): The value to rotate.
- `i` ([i64](#type.i64)): The amount to rotate by.


###### Returns

- `result` ([U64](#type.U64)): The rotated value.
##### bitu64:lshift

```lua
function bitu64:lshift(n: U64, i: i64): U64
```

Shifts a value left or right.

###### Parameters

- `n` ([U64](#type.U64)): The value to shift.
- `i` ([i64](#type.i64)): The amount to shift by.


###### Returns

- `result` ([U64](#type.U64)): The shifted value.
##### bitu64:replace

```lua
function bitu64:replace(n: U64, v: U64, f: u64, w: u64): U64
```

Replaces a field in a value.

###### Parameters

- `n` ([U64](#type.U64)): The value to replace the field in.
- `v` ([U64](#type.U64)): The value to replace the field with.
- `f` ([u64](#type.u64)): The field to replace.
- `w` ([u64](#type.u64)): The width of the field to replace.


###### Returns

- `result` ([U64](#type.U64)): The value with the field replaced.
##### bitu64:rrotate

```lua
function bitu64:rrotate(n: U64, i: i64): U64
```

Rotates a value left or right.

###### Parameters

- `n` ([U64](#type.U64)): The value to rotate.
- `i` ([i64](#type.i64)): The amount to rotate by.


###### Returns

- `result` ([U64](#type.U64)): The rotated value.
##### bitu64:rshift

```lua
function bitu64:rshift(n: U64, i: i64): U64
```

Shifts a value left or right.

###### Parameters

- `n` ([U64](#type.U64)): The value to shift.
- `i` ([i64](#type.i64)): The amount to shift by.


###### Returns

- `result` ([U64](#type.U64)): The shifted value.


## Methods

### U64

```lua
function U64(value: u64): U64
```

Creates a new U64.

#### Parameters

- `value` ([u64](#type.u64)): The value of the U64.


#### Returns

- `u64` ([U64](#type.U64)): The U64 value.

### I64

```lua
function I64(value: i64): I64
```

Creates a new I64.

#### Parameters

- `value` ([i64](#type.i64)): The value of the I64.


#### Returns

- `i64` ([I64](#type.I64)): The I64 value.
