<div id="@antiraid/datamgmt"></div>

# @antiraid/datamgmt

<div id="Types"></div>

## Types

<div id="TarArchive"></div>

## TarArchive

<details>
<summary>Raw Type</summary>

```luau
type TarArchive = {
	--- Takes out an entry from the tar archive by file name returning nil if not found
	takefile: (self: TarArchive, name: string) -> blob.Blob?,

	--- Adds an entry to the tar archive with the given file name and contents
	addfile: (self: TarArchive, name: string, contents: blob.BlobTaker) -> nil,

	--- Returns the names of all entries in the tar archive
	entries: (self: TarArchive) -> {string},

	--- Converts the tar archive to a Blob
	---
	--- This will destroy the tar archive (hence making it unusable for future Luau operations) 
	--- and return a Blob containing the tar archive data
	toblob: (self: TarArchive) -> blob.Blob
}
```

</details>

<div id="takefile"></div>

### takefile

Takes out an entry from the tar archive by file name returning nil if not found

<details>
<summary>Function Signature</summary>

```luau
--- Takes out an entry from the tar archive by file name returning nil if not found
takefile: (self: TarArchive, name: string) -> blob.Blob?
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="name"></div>

##### name

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[blob](./blob.md).[Blob](./blob.md#Blob)?<div id="addfile"></div>

### addfile

Adds an entry to the tar archive with the given file name and contents

<details>
<summary>Function Signature</summary>

```luau
--- Adds an entry to the tar archive with the given file name and contents
addfile: (self: TarArchive, name: string, contents: blob.BlobTaker) -> nil
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="name"></div>

##### name

[string](#string)

<div id="contents"></div>

##### contents

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[nil](#nil)<div id="entries"></div>

### entries

Returns the names of all entries in the tar archive

<details>
<summary>Function Signature</summary>

```luau
--- Returns the names of all entries in the tar archive
entries: (self: TarArchive) -> {string}
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

{[string](#string)}<div id="toblob"></div>

### toblob

Converts the tar archive to a Blob



This will destroy the tar archive (hence making it unusable for future Luau operations)

and return a Blob containing the tar archive data

<details>
<summary>Function Signature</summary>

```luau
--- Converts the tar archive to a Blob
---
--- This will destroy the tar archive (hence making it unusable for future Luau operations) 
--- and return a Blob containing the tar archive data
toblob: (self: TarArchive) -> blob.Blob
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[blob](./blob.md).[Blob](./blob.md#Blob)<div id="Functions"></div>

# Functions

<div id="newblob"></div>

## newblob

Creates a new Blob from the given data

<details>
<summary>Function Signature</summary>

```luau
--- Creates a new Blob from the given data
function newblob(data: blob.BlobTaker) -> blob.Blob end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="data"></div>

### data

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[blob](./blob.md).[Blob](./blob.md#Blob)<div id="TarArchive"></div>

## TarArchive

Creates a new TarArchive with an optional initial data Blob to load the initial TarArchive's contents from

<details>
<summary>Function Signature</summary>

```luau
--- Creates a new TarArchive with an optional initial data Blob to load the initial TarArchive's contents from
function TarArchive(buf: blob.BlobTaker?) -> TarArchive end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="buf"></div>

### buf

*This field is optional and may not be specified*

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)?

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TarArchive](#TarArchive)<div id="aes256encrypt"></div>

## aes256encrypt

Encrypts the Blob using AES256 encryption (Argon2id for key derivation)

Format: ``<salt><nonce><ciphertext>``

<details>
<summary>Function Signature</summary>

```luau
--- Encrypts the Blob using AES256 encryption (Argon2id for key derivation)
--- Format: ``<salt><nonce><ciphertext>``
function aes256encrypt(data: blob.BlobTaker, key: string) -> blob.Blob end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="data"></div>

### data

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)

<div id="key"></div>

### key

[string](#string)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[blob](./blob.md).[Blob](./blob.md#Blob)<div id="aes256decrypt"></div>

## aes256decrypt

Decrypts the Blob using AES256 decryption (Argon2id for key derivation)

Format: ``<salt><nonce><ciphertext>``

<details>
<summary>Function Signature</summary>

```luau
--- Decrypts the Blob using AES256 decryption (Argon2id for key derivation)
--- Format: ``<salt><nonce><ciphertext>``
function aes256decrypt(data: blob.BlobTaker, key: string) -> blob.Blob end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="data"></div>

### data

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)

<div id="key"></div>

### key

[string](#string)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[blob](./blob.md).[Blob](./blob.md#Blob)<div id="aes256decryptcustom"></div>

## aes256decryptcustom

Decrypts the Blob using AES256 decryption (Argon2id for key derivation)

Format: ``<salt><nonce><ciphertext>``

<details>
<summary>Function Signature</summary>

```luau
--- Decrypts the Blob using AES256 decryption (Argon2id for key derivation)
--- Format: ``<salt><nonce><ciphertext>``
function aes256decryptcustom(salt: blob.BlobTaker, nonce: blob.BlobTaker, ciphertext: blob.BlobTaker, key: string) -> blob.Blob end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="salt"></div>

### salt

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)

<div id="nonce"></div>

### nonce

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)

<div id="ciphertext"></div>

### ciphertext

[blob](./blob.md).[BlobTaker](./blob.md#BlobTaker)

<div id="key"></div>

### key

[string](#string)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[blob](./blob.md).[Blob](./blob.md#Blob)