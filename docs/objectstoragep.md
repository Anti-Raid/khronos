<div id="objectstoragep"></div>

# objectstoragep

<div id="Types"></div>

## Types

<div id="Bucket"></div>

## Bucket

An object storage bucket



Object storage is mainly useful over kv when dealing with blobs of data

(otherwise, kv is easier to use as it has support for serializing most AntiRaid types

etc). As an example, datetimes should be stored using object storage for automatic

conversion to/from the storage bit for you while object storage can be used for storing

compressed assets, images etc.



Object storage also works over HTTP calls and may be slower than key-value which

may use a custom binary protocol over unix socket

<details>
<summary>Raw Type</summary>

```luau
--- An object storage bucket
---
--- Object storage is mainly useful over kv when dealing with blobs of data
--- (otherwise, kv is easier to use as it has support for serializing most AntiRaid types
--- etc). As an example, datetimes should be stored using object storage for automatic
--- conversion to/from the storage bit for you while object storage can be used for storing
--- compressed assets, images etc.
---
--- Object storage also works over HTTP calls and may be slower than key-value which 
--- may use a custom binary protocol over unix socket
type Bucket = {
	--- The buckets name
	bucket_name: string,

	--- @yields
	---
	--- List all files in the bucket
	list_files: (self: Bucket, prefix: string?) -> {ObjectMetadata},

	--- @yields
	---
	--- Returns if a file exists
	file_exists: (self: Bucket, path: string) -> boolean,

	--- @yields
	---
	--- Downloads a file. The resulting file must fit into the VM's memory limit
	download_file: (self: Bucket, path: string, opts: DownloadFileOpts?) -> blob.Blob,

	--- @yields
	---
	--- Creates a presigned url for referring to the file
	get_file_url: (self: Bucket, path: string, expiry: datetime.TimeDelta) -> string,

	--- @yields
	---
	--- Upload a file to a bucket
	upload_file: (self: Bucket, path: string, data: blob.BlobTaker) -> nil,

	--- @yields
	---
	--- Deletes a file from the bucket
	delete_file: (self: Bucket, path: string) -> nil
}
```

</details>

<div id="list_files"></div>

### list_files

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



List all files in the bucket

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- List all files in the bucket
list_files: (self: Bucket, prefix: string?) -> {ObjectMetadata}
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="prefix"></div>

##### prefix

*This field is optional and may not be specified*

[string](#string)?

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

{[ObjectMetadata](#ObjectMetadata)}<div id="file_exists"></div>

### file_exists

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Returns if a file exists

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Returns if a file exists
file_exists: (self: Bucket, path: string) -> boolean
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="path"></div>

##### path

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="download_file"></div>

### download_file

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Downloads a file. The resulting file must fit into the VM's memory limit

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Downloads a file. The resulting file must fit into the VM's memory limit
download_file: (self: Bucket, path: string, opts: DownloadFileOpts?) -> blob.Blob
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="path"></div>

##### path

[string](#string)

<div id="opts"></div>

##### opts

*This field is optional and may not be specified*

[DownloadFileOpts](#DownloadFileOpts)?

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[blob](./blob.md).[Blob](./blob.md#Blob)<div id="get_file_url"></div>

### get_file_url

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Creates a presigned url for referring to the file

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Creates a presigned url for referring to the file
get_file_url: (self: Bucket, path: string, expiry: datetime.TimeDelta) -> string
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="path"></div>

##### path

[string](#string)

<div id="expiry"></div>

##### expiry

[datetime](./datetime.md).[TimeDelta](./datetime.md#TimeDelta)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="upload_file"></div>

### upload_file

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Upload a file to a bucket

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Upload a file to a bucket
upload_file: (self: Bucket, path: string, data: blob.BlobTaker) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="path"></div>

##### path

[string](#string)

<div id="data"></div>

##### data

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="delete_file"></div>

### delete_file

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Deletes a file from the bucket

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Deletes a file from the bucket
delete_file: (self: Bucket, path: string) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="path"></div>

##### path

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="bucket_name"></div>

### bucket_name

The buckets name

[string](#string)

<div id="ObjectMetadata"></div>

## ObjectMetadata

Metadata about an object.

<details>
<summary>Raw Type</summary>

```luau
--- Metadata about an object.
type ObjectMetadata = {
	key: string,

	last_modified: datetime.DateTime?,

	size: number,

	etag: string?
}
```

</details>

<div id="key"></div>

### key

[string](#string)

<div id="last_modified"></div>

### last_modified

*This field is optional and may not be specified*

[datetime](./datetime.md).[DateTime](./datetime.md#DateTime)?

<div id="size"></div>

### size

[number](#number)

<div id="etag"></div>

### etag

*This field is optional and may not be specified*

[string](#string)?

<div id="ObjectStorageReadRange"></div>

## ObjectStorageReadRange

<details>
<summary>Raw Type</summary>

```luau
type ObjectStorageReadRange = {
	read_start: number,

	read_end: number
}
```

</details>

<div id="read_start"></div>

### read_start

[number](#number)

<div id="read_end"></div>

### read_end

[number](#number)

<div id="DownloadFileOpts"></div>

## DownloadFileOpts

<details>
<summary>Raw Type</summary>

```luau
type DownloadFileOpts = {
	range: ObjectStorageReadRange?
}
```

</details>

<div id="range"></div>

### range

*This field is optional and may not be specified*

[ObjectStorageReadRange](#ObjectStorageReadRange)?

<div id="Plugin"></div>

## Plugin

<details>
<summary>Raw Type</summary>

```luau
type Plugin = Bucket
```

</details>

[Bucket](#Bucket)

