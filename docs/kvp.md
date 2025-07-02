<div id="kvp"></div>

# kvp

<div id="Types"></div>

## Types

<div id="SetResult"></div>

## SetResult

Result from a set operation

<details>
<summary>Raw Type</summary>

```luau
--- Result from a set operation
type SetResult = {
	--- Whether or not the key previously existed.
	exists: boolean,

	--- The ID of the record that was set.
	id: string
}
```

</details>

<div id="exists"></div>

### exists

Whether or not the key previously existed.

[boolean](#boolean)

<div id="id"></div>

### id

The ID of the record that was set.

[string](#string)

<div id="KvRecord"></div>

## KvRecord

KvRecord represents a key-value record with metadata.

@class KvRecord

<details>
<summary>Raw Type</summary>

```luau
--- KvRecord represents a key-value record with metadata.
---@class KvRecord
type KvRecord = {
	--- The ID of the record
	id: string,

	--- The key of the record.
	key: string,

	--- The value of the record. This can be any type, depending on what was stored.
	value: khronosvalue.KhronosValue,

	--- Indicates whether the record exists in the key-value store.
	exists: true,

	--- The scopes the key has
	scopes: {string},

	--- The timestamp when the record was created, in ISO 8601 format (e.g., "2023-10-01T12:00:00Z").
	created_at: datetime.DateTime,

	--- The timestamp when the record was last updated, in ISO 8601 format (e.g., "2023-10-01T12:00:00Z").
	last_updated_at: datetime.DateTime,

	--- When the record will expire, if any
	expires_at: datetime.DateTime?
} | {
	--- The key of the record.
	key: string,

	--- Indicates whether the record exists in the key-value store.
	exists: false
}
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

*This is an inline table type with the following fields*

<div id="id"></div>

#### id

The ID of the record

[string](#string)

<div id="key"></div>

#### key

The key of the record.

[string](#string)

<div id="value"></div>

#### value

The value of the record. This can be any type, depending on what was stored.

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)

<div id="exists"></div>

#### exists

Indicates whether the record exists in the key-value store.

[true](#true)

<div id="scopes"></div>

#### scopes

The scopes the key has

{[string](#string)}

<div id="created_at"></div>

#### created_at

The timestamp when the record was created, in ISO 8601 format (e.g., "2023-10-01T12:00:00Z").

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)

<div id="last_updated_at"></div>

#### last_updated_at

The timestamp when the record was last updated, in ISO 8601 format (e.g., "2023-10-01T12:00:00Z").

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)

<div id="expires_at"></div>

#### expires_at

When the record will expire, if any

*This field is optional and may not be specified*

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)?

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

<div id="key"></div>

#### key

The key of the record.

[string](#string)

<div id="exists"></div>

#### exists

Indicates whether the record exists in the key-value store.

[false](#false)

</details>

<div id="KvRecordList"></div>

## KvRecordList

A list of KvRecords

<details>
<summary>Raw Type</summary>

```luau
--- A list of KvRecords
type KvRecordList = {KvRecord}
```

</details>

{[KvRecord](#KvRecord)}

<div id="KvExecutor"></div>

## KvExecutor

KvExecutor allows templates to get, store and find persistent data within a server.

@class KvExecutor

<details>
<summary>Raw Type</summary>

```luau
--- KvExecutor allows templates to get, store and find persistent data within a server.
---@class KvExecutor
type KvExecutor = {
	--- The guild ID the executor will perform key-value operations on.
	guild_id: string,

	--- The originating guild ID (the guild ID of the template itself).
	origin_guild_id: string,

	--- Whether or not unscoped operations are allowed.
	unscoped_allowed: boolean,

	--- @yields
	---
	--- Returns a list of all key-value scopes/
	--- This is only guaranteed to return scopes that actually have at least one key inside it.
	---
	--- Needs the `kv.meta:list_scopes` capability to use
	list_scopes: (self: KvExecutor) -> {string},

	--- @yields
	---
	--- Finds matching records in this key-value scope.
	--- @param query string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%
	--- @return {KvRecord} The records.
	find: (self: KvExecutor, query: string, scopes: {string}) -> KvRecordList,

	--- @yields
	---
	--- Gets a value from this key-value scope.
	--- @param key string The key of the record.
	--- @return any The value of the record.
	get: (self: KvExecutor, key: string, scopes: {string}) -> khronosvalue.KhronosValue,

	--- @yields
	---
	--- Gets a value by ID
	--- @param id string The ID of the record.
	--- @return any The value of the record.
	getbyid: (self: KvExecutor, id: string) -> khronosvalue.KhronosValue,

	--- @yields
	---
	--- Gets a record from this key-value scope.
	--- @param key string The key of the record.
	--- @return KvRecord The record.
	getrecord: (self: KvExecutor, key: string, scopes: {string}) -> KvRecord,

	--- @yields
	---
	--- Gets a record by ID
	--- @param id string The ID of the record.
	--- @return KvRecord The record.
	getrecordbyid: (self: KvExecutor, id: string) -> KvRecord,

	--- @yields
	---
	--- Returns all keys present in this key-value scope.
	---
	--- This needs the ``kv.meta[SCOPE]:keys`` capability for all scopes provided.
	keys: (self: KvExecutor, scopes: {string}) -> KvRecordList,

	--- @yields
	---
	--- Sets a record in the key-value store.
	--- @param key string The key of the record.
	--- @param value any The value of the record.
	--- @param scopes {string} The scopes to set the record in. If not provided, the record will be set in the unscoped scope.
	--- @param expires_at datetime.DateTime? The expiration time of the record, if any
	--- @return The result of the set operation, containing whether the key previously existed and the ID of the record.
	set: (self: KvExecutor, key: string, value: khronosvalue.KhronosValue, scopes: {string}, expires_at: datetime.DateTime?) -> SetResult,

	--- @yields
	--- Sets a record in the key-value store by ID.
	--- @param id string The ID of the record.
	--- @param value any The value of the record.
	--- @param expires_at datetime.DateTime? The expiration time of the record, if any.
	setbyid: (self: KvExecutor, id: string, value: khronosvalue.KhronosValue, expires_at: datetime.DateTime?) -> nil,

	--- @yields
	---
	--- Sets the expiry time of a record.
	--- @param key string The key of the record.
	--- @param expires_at datetime.DateTime The new expiration time of the record.
	setexpiry: (self: KvExecutor, key: string, scopes: {string}, expires_at: datetime.DateTime?) -> nil,

	--- @yields
	---
	--- Sets the expiry time of a record by ID.
	--- @param id string The ID of the record.
	--- @param expires_at datetime.DateTime The new expiration time of the record.
	setexpirybyid: (self: KvExecutor, id: string, expires_at: datetime.DateTime?) -> nil,

	--- @yields
	---
	--- Deletes a record from this key-value scope
	--- @param key string The key of the record.
	delete: (self: KvExecutor, key: string, scopes: {string}) -> nil,

	--- @yields
	---
	--- Deletes a record by ID.
	--- @param id string The ID of the record.
	deletebyid: (self: KvExecutor, id: string) -> nil
}
```

</details>

<div id="list_scopes"></div>

### list_scopes

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Returns a list of all key-value scopes/

This is only guaranteed to return scopes that actually have at least one key inside it.



Needs the `kv.meta:list_scopes` capability to use

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Returns a list of all key-value scopes/
--- This is only guaranteed to return scopes that actually have at least one key inside it.
---
--- Needs the `kv.meta:list_scopes` capability to use
list_scopes: (self: KvExecutor) -> {string}
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

{[string](#string)}<div id="find"></div>

### find

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Finds matching records in this key-value scope.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Finds matching records in this key-value scope.
--- @param query string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%
--- @return {KvRecord} The records.
find: (self: KvExecutor, query: string, scopes: {string}) -> KvRecordList
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="query"></div>

##### query

string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%

[string](#string)

<div id="scopes"></div>

##### scopes

{[string](#string)}

<div id="Returns"></div>

#### Returns

<div id="{KvRecord}"></div>

##### {KvRecord}

The records.

[KvRecordList](#KvRecordList)<div id="get"></div>

### get

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a value from this key-value scope.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a value from this key-value scope.
--- @param key string The key of the record.
--- @return any The value of the record.
get: (self: KvExecutor, key: string, scopes: {string}) -> khronosvalue.KhronosValue
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="scopes"></div>

##### scopes

{[string](#string)}

<div id="Returns"></div>

#### Returns

<div id="any"></div>

##### any

The value of the record.

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)<div id="getbyid"></div>

### getbyid

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a value by ID

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a value by ID
--- @param id string The ID of the record.
--- @return any The value of the record.
getbyid: (self: KvExecutor, id: string) -> khronosvalue.KhronosValue
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="any"></div>

##### any

The value of the record.

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)<div id="getrecord"></div>

### getrecord

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a record from this key-value scope.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a record from this key-value scope.
--- @param key string The key of the record.
--- @return KvRecord The record.
getrecord: (self: KvExecutor, key: string, scopes: {string}) -> KvRecord
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="scopes"></div>

##### scopes

{[string](#string)}

<div id="Returns"></div>

#### Returns

<div id="KvRecord"></div>

##### KvRecord

The record.

[KvRecord](#KvRecord)<div id="getrecordbyid"></div>

### getrecordbyid

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Gets a record by ID

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Gets a record by ID
--- @param id string The ID of the record.
--- @return KvRecord The record.
getrecordbyid: (self: KvExecutor, id: string) -> KvRecord
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="KvRecord"></div>

##### KvRecord

The record.

[KvRecord](#KvRecord)<div id="keys"></div>

### keys

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Returns all keys present in this key-value scope.



This needs the ``kv.meta[SCOPE]:keys`` capability for all scopes provided.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Returns all keys present in this key-value scope.
---
--- This needs the ``kv.meta[SCOPE]:keys`` capability for all scopes provided.
keys: (self: KvExecutor, scopes: {string}) -> KvRecordList
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="scopes"></div>

##### scopes

{[string](#string)}

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[KvRecordList](#KvRecordList)<div id="set"></div>

### set

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Sets a record in the key-value store.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Sets a record in the key-value store.
--- @param key string The key of the record.
--- @param value any The value of the record.
--- @param scopes {string} The scopes to set the record in. If not provided, the record will be set in the unscoped scope.
--- @param expires_at datetime.DateTime? The expiration time of the record, if any
--- @return The result of the set operation, containing whether the key previously existed and the ID of the record.
set: (self: KvExecutor, key: string, value: khronosvalue.KhronosValue, scopes: {string}, expires_at: datetime.DateTime?) -> SetResult
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="value"></div>

##### value

any The value of the record.

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)

<div id="scopes"></div>

##### scopes

{string} The scopes to set the record in. If not provided, the record will be set in the unscoped scope.

{[string](#string)}

<div id="expires_at"></div>

##### expires_at

datetime.DateTime? The expiration time of the record, if any

*This field is optional and may not be specified*

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)?

<div id="Returns"></div>

#### Returns

<div id="The"></div>

##### The

result of the set operation, containing whether the key previously existed and the ID of the record.

[SetResult](#SetResult)<div id="setbyid"></div>

### setbyid

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>

Sets a record in the key-value store by ID.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
--- Sets a record in the key-value store by ID.
--- @param id string The ID of the record.
--- @param value any The value of the record.
--- @param expires_at datetime.DateTime? The expiration time of the record, if any.
setbyid: (self: KvExecutor, id: string, value: khronosvalue.KhronosValue, expires_at: datetime.DateTime?) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the record.

[string](#string)

<div id="value"></div>

##### value

any The value of the record.

[khronosvalue](./khronosvalue.md).[KhronosValue](./khronosvalue.md#KhronosValue)

<div id="expires_at"></div>

##### expires_at

datetime.DateTime? The expiration time of the record, if any.

*This field is optional and may not be specified*

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)?

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="setexpiry"></div>

### setexpiry

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Sets the expiry time of a record.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Sets the expiry time of a record.
--- @param key string The key of the record.
--- @param expires_at datetime.DateTime The new expiration time of the record.
setexpiry: (self: KvExecutor, key: string, scopes: {string}, expires_at: datetime.DateTime?) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="scopes"></div>

##### scopes

{[string](#string)}

<div id="expires_at"></div>

##### expires_at

datetime.DateTime The new expiration time of the record.

*This field is optional and may not be specified*

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)?

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="setexpirybyid"></div>

### setexpirybyid

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Sets the expiry time of a record by ID.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Sets the expiry time of a record by ID.
--- @param id string The ID of the record.
--- @param expires_at datetime.DateTime The new expiration time of the record.
setexpirybyid: (self: KvExecutor, id: string, expires_at: datetime.DateTime?) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the record.

[string](#string)

<div id="expires_at"></div>

##### expires_at

datetime.DateTime The new expiration time of the record.

*This field is optional and may not be specified*

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)?

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="delete"></div>

### delete

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a record from this key-value scope

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a record from this key-value scope
--- @param key string The key of the record.
delete: (self: KvExecutor, key: string, scopes: {string}) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="scopes"></div>

##### scopes

{[string](#string)}

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="deletebyid"></div>

### deletebyid

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a record by ID.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a record by ID.
--- @param id string The ID of the record.
deletebyid: (self: KvExecutor, id: string) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="guild_id"></div>

### guild_id

The guild ID the executor will perform key-value operations on.

[string](#string)

<div id="origin_guild_id"></div>

### origin_guild_id

The originating guild ID (the guild ID of the template itself).

[string](#string)

<div id="unscoped_allowed"></div>

### unscoped_allowed

Whether or not unscoped operations are allowed.

[boolean](#boolean)

<div id="Plugin"></div>

## Plugin

<details>
<summary>Raw Type</summary>

```luau
type Plugin = KvExecutor
```

</details>

[KvExecutor](#KvExecutor)

