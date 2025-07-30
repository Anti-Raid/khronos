<div id="httpclientp"></div>

# httpclientp

<div id="Types"></div>

## Types

<div id="Url"></div>

## Url

<details>
<summary>Raw Type</summary>

```luau
type Url = {
	host: string,

	-- The host of the URL
	port: number,

	-- The port of the URL
	scheme: string,

	-- The scheme of the URL (e.g., "http", "https")
	path: string,

	-- The path of the URL
	query: string?,

	-- The query string of the URL
	fragment: string?
}
```

</details>

<div id="host"></div>

### host

[string](#string)

<div id="port"></div>

### port

The host of the URL

[number](#number)

<div id="scheme"></div>

### scheme

The port of the URL

[string](#string)

<div id="path"></div>

### path

The scheme of the URL (e.g., "http", "https")

[string](#string)

<div id="query"></div>

### query

The path of the URL

*This field is optional and may not be specified*

[string](#string)?

<div id="fragment"></div>

### fragment

The query string of the URL

*This field is optional and may not be specified*

[string](#string)?

<div id="Headers"></div>

## Headers

<details>
<summary>Raw Type</summary>

```luau
type Headers = {
	get: (self: Headers, key: string) -> string?,

	-- Get a header by key
	set: (self: Headers, key: string, value: string) -> (),

	-- Set a header
	remove: (self: Headers, key: string) -> (),

	-- Remove a header by key
	headers_bytes: (self: Headers) -> {
		[string]: {number}
	},

	-- Get all headers as a table of strings to bytes
	headers_str: (self: Headers) -> {
		[string]: string
	}
}
```

</details>

<div id="get"></div>

### get

<details>
<summary>Function Signature</summary>

```luau
get: (self: Headers, key: string) -> string?
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[string](#string)?<div id="set"></div>

### set

Get a header by key

<details>
<summary>Function Signature</summary>

```luau
-- Get a header by key
set: (self: Headers, key: string, value: string) -> ()
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

[string](#string)

<div id="value"></div>

##### value

[string](#string)

<div id="remove"></div>

### remove

Set a header

<details>
<summary>Function Signature</summary>

```luau
-- Set a header
remove: (self: Headers, key: string) -> ()
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

[string](#string)

<div id="headers_bytes"></div>

### headers_bytes

Remove a header by key

<details>
<summary>Function Signature</summary>

```luau
-- Remove a header by key
headers_bytes: (self: Headers) -> {
		[string]: {number}
	}
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

{[string]: {[number](#number)}}<div id="headers_str"></div>

### headers_str

Get all headers as a table of strings to bytes

<details>
<summary>Function Signature</summary>

```luau
-- Get all headers as a table of strings to bytes
headers_str: (self: Headers) -> {
		[string]: string
	}
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

{[string]: [string](#string)}<div id="Request"></div>

## Request

<details>
<summary>Raw Type</summary>

```luau
type Request = {
	--- The HTTP method (e.g., "GET", "POST")
	method: string,

	--- The URL of the request
	---
	--- The returned object will be a copy of the URL so mutating this object without explicit assignment
	--- will not affect the original URL.
	url: Url,

	--- The headers of the request
	--- The returned object will be a copy of the headers so mutating this object without explicit assignment
	--- will not affect the original headers. 
	headers: Headers,

	--- The body of the request, can be a string, table, or buffer. When set, it will be serialized to bytes.
	---
	--- When reading, the body will be a buffer
	body_bytes: any,

	--- The timeout for the request, can be a number (in seconds) or a `timedelta` object
	---
	--- Max value: 5 seconds
	timeout: dt.TimeDelta,

	--- The HTTP version of the request, defaults to "HTTP/1.1"
	version: "HTTP/0.9" | "HTTP/1.0" | "HTTP/1.1" | "HTTP/2.0" | "HTTP/3.0",

	--- @yields
	---
	--- Sends the request and returns a response object
	send: (self: Request) -> Response
}
```

</details>

<div id="send"></div>

### send

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Sends the request and returns a response object

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Sends the request and returns a response object
send: (self: Request) -> Response
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Response](#Response)<div id="method"></div>

### method

The HTTP method (e.g., "GET", "POST")

[string](#string)

<div id="url"></div>

### url

The URL of the request



The returned object will be a copy of the URL so mutating this object without explicit assignment

will not affect the original URL.

[Url](#Url)

<div id="headers"></div>

### headers

The headers of the request



The returned object will be a copy of the headers so mutating this object without explicit assignment

will not affect the original headers.

[Headers](#Headers)

<div id="body_bytes"></div>

### body_bytes

The body of the request, can be a string, table, or buffer. When set, it will be serialized to bytes.



When reading, the body will be a buffer

[any](#any)

<div id="timeout"></div>

### timeout

The timeout for the request, can be a number (in seconds) or a `timedelta` object



Max value: 5 seconds

[dt](./dt.md).[TimeDelta](./dt.md#TimeDelta)

<div id="version"></div>

### version

The HTTP version of the request, defaults to "HTTP/1.1"

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"HTTP/0.9"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"HTTP/1.0"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"HTTP/1.1"
```

</details>

<details>
<summary>Variant 4</summary>

```luau
"HTTP/2.0"
```

</details>

<details>
<summary>Variant 5</summary>

```luau
"HTTP/3.0"
```

</details>

<div id="Response"></div>

## Response

<details>
<summary>Raw Type</summary>

```luau
type Response = {
	--- URL of the response
	url: Url,

	--- The status code of the response
	status: number,

	--- The content length of the response
	content_length: number?,

	--- The headers of the response
	headers: Headers,

	--- @yields
	---
	--- Reads the response as text (but does not have to be UTF-8 encoded) 
	---
	--- Note that this will destroy the Response object and cause all calls to it to fail after this.
	text: (self: Headers) -> string,

	--- @yields
	---
	--- Reads the response as JSON, will return a Lua table
	---
	--- Note that this will destroy the Response object and cause all calls to it to fail after this.
	json: (self: Headers) -> any,

	--- @yields
	---
	--- Reads the response as bytes, will return a blob
	---
	--- Note that this will destroy the Response object and cause all calls to it to fail after this.
	blob: (self: Headers) -> blob.Blob
}
```

</details>

<div id="text"></div>

### text

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Reads the response as text (but does not have to be UTF-8 encoded)



Note that this will destroy the Response object and cause all calls to it to fail after this.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Reads the response as text (but does not have to be UTF-8 encoded) 
---
--- Note that this will destroy the Response object and cause all calls to it to fail after this.
text: (self: Headers) -> string
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="json"></div>

### json

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Reads the response as JSON, will return a Lua table



Note that this will destroy the Response object and cause all calls to it to fail after this.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Reads the response as JSON, will return a Lua table
---
--- Note that this will destroy the Response object and cause all calls to it to fail after this.
json: (self: Headers) -> any
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[any](#any)<div id="blob"></div>

### blob

<div class="warning">
This function yields the thread its executing in. This may cause issues in some contexts such as within metamethods (as Luau does not support yieldable metamethods).
</div>



Reads the response as bytes, will return a blob



Note that this will destroy the Response object and cause all calls to it to fail after this.

<details>
<summary>Function Signature</summary>

```luau
--- @yields
---
--- Reads the response as bytes, will return a blob
---
--- Note that this will destroy the Response object and cause all calls to it to fail after this.
blob: (self: Headers) -> blob.Blob
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[blob](./blob.md).[Blob](./blob.md#Blob)<div id="url"></div>

### url

URL of the response

[Url](#Url)

<div id="status"></div>

### status

The status code of the response

[number](#number)

<div id="content_length"></div>

### content_length

The content length of the response

*This field is optional and may not be specified*

[number](#number)?

<div id="headers"></div>

### headers

The headers of the response

[Headers](#Headers)

<div id="Plugin"></div>

## Plugin

<details>
<summary>Raw Type</summary>

```luau
type Plugin = {
	--- Creates a new request
	new_request: (method: string, url: string) -> Request,

	--- Creates a new empty headers object
	new_headers: () -> Headers,

	--- Parses a URL string into a Url object
	new_url: (url: string) -> Url
}
```

</details>

<div id="new_request"></div>

### new_request

Creates a new request

<details>
<summary>Function Signature</summary>

```luau
--- Creates a new request
new_request: (method: string, url: string) -> Request
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="method"></div>

##### method

[string](#string)

<div id="url"></div>

##### url

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Request](#Request)<div id="new_headers"></div>

### new_headers

Creates a new empty headers object

<details>
<summary>Function Signature</summary>

```luau
--- Creates a new empty headers object
new_headers: () -> Headers
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Headers](#Headers)<div id="new_url"></div>

### new_url

Parses a URL string into a Url object

<details>
<summary>Function Signature</summary>

```luau
--- Parses a URL string into a Url object
new_url: (url: string) -> Url
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="url"></div>

##### url

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Url](#Url)