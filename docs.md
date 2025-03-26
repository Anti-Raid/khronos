<div id="@antiraid/datetime.luau"></div>

# @antiraid/datetime.luau

<div id="Types"></div>

## Types

<div id="TimeDelta"></div>

## TimeDelta

<details>
<summary>Raw Type</summary>

```luau
type TimeDelta = {
	--- @field nanos: The number of nanoseconds in the time delta.
	nanos: number,

	--- @field micros: The number of microseconds in the time delta.
	micros: number,

	--- @field millis: The number of milliseconds in the time delta.
	millis: number,

	--- @field seconds: The number of seconds in the time delta.
	seconds: number,

	--- @field minutes: The number of minutes in the time delta.
	minutes: number,

	--- @field hours: The number of hours in the time delta.
	hours: number,

	--- @field days: The number of days in the time delta.
	days: number,

	--- @field weeks: The number of weeks in the time delta.
	weeks: number,

	--- @function () -> string
	--- Returns an 'offset' string representation of the time delta.
	--- E.g. "+05:30" for 5 hours and 30 minutes.
	offset_string: (self: TimeDelta) -> string,

	-- Metatable
	__add: (TimeDelta, TimeDelta) -> TimeDelta,
	__sub: (TimeDelta, TimeDelta) -> TimeDelta,
	__le: (TimeDelta, TimeDelta) -> boolean,
	__lt: (TimeDelta, TimeDelta) -> boolean,
	__tostring: (TimeDelta) -> string
}
```

</details>

<div id="nanos"></div>

### nanos

[number](#number)

<div id="micros"></div>

### micros

[number](#number)

<div id="millis"></div>

### millis

[number](#number)

<div id="seconds"></div>

### seconds

[number](#number)

<div id="minutes"></div>

### minutes

[number](#number)

<div id="hours"></div>

### hours

[number](#number)

<div id="days"></div>

### days

[number](#number)

<div id="weeks"></div>

### weeks

[number](#number)

<div id="offset_string"></div>

### offset_string

@function () -> string

Returns an 'offset' string representation of the time delta.

E.g. "+05:30" for 5 hours and 30 minutes.

<details>
<summary>Function Signature</summary>

```luau
--- @function () -> string
--- Returns an 'offset' string representation of the time delta.
--- E.g. "+05:30" for 5 hours and 30 minutes.
offset_string: (self: TimeDelta) -> string
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="MetatableFields"></div>

### Metatable Fields

<div id="__add"></div>

#### __add

<details>
<summary>Function Signature</summary>

```luau
__add: (TimeDelta, TimeDelta) -> TimeDelta
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[TimeDelta](#TimeDelta)

<div id="arg2"></div>

##### arg2

[TimeDelta](#TimeDelta)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[TimeDelta](#TimeDelta)<div id="__sub"></div>

#### __sub

<details>
<summary>Function Signature</summary>

```luau
__sub: (TimeDelta, TimeDelta) -> TimeDelta
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[TimeDelta](#TimeDelta)

<div id="arg2"></div>

##### arg2

[TimeDelta](#TimeDelta)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[TimeDelta](#TimeDelta)<div id="__le"></div>

#### __le

<details>
<summary>Function Signature</summary>

```luau
__le: (TimeDelta, TimeDelta) -> boolean
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[TimeDelta](#TimeDelta)

<div id="arg2"></div>

##### arg2

[TimeDelta](#TimeDelta)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="__lt"></div>

#### __lt

<details>
<summary>Function Signature</summary>

```luau
__lt: (TimeDelta, TimeDelta) -> boolean
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[TimeDelta](#TimeDelta)

<div id="arg2"></div>

##### arg2

[TimeDelta](#TimeDelta)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="__tostring"></div>

#### __tostring

<details>
<summary>Function Signature</summary>

```luau
__tostring: (TimeDelta) -> string
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[TimeDelta](#TimeDelta)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="TimeZone"></div>

## TimeZone

@class TimeZone

@within TimeZone

A timezone object.

<details>
<summary>Raw Type</summary>

```luau
--- @class TimeZone
--- @within TimeZone
--- A timezone object.
type TimeZone = {
	--- @function (year: number, month: number, day: number, hour: number, minute: number, second: number, all: boolean?) -> DateTime
	--- Translates a timestamp in UTC time to a datetime in the said specific timezone. If `all` is set to true, then multiple times
	--- may be returned in the case of ambiguous times, otherwise the first time is returned.
	utcToTz: (self: TimeZone, year: number, month: number, day: number, hours: number, minutes: number, secs: number) -> DateTime,

	--- @function (year: number, month: number, day: number, hour: number, minute: number, second: number, all: boolean?) -> DateTime
	--- Translates a timestamp in the specified timezone to a datetime in UTC. If `all` is set to true, then multiple times
	--- may be returned in the case of ambiguous times, otherwise the first time is returned.
	tzToUtc: (self: TimeZone, year: number, month: number, day: number, hours: number, minutes: number, secs: number) -> DateTime,

	--- @function (hour: number, minute: number, second: number) -> DateTime
	--- Translates a time of the current day in UTC time to a datetime in the said specific timezone
	timeUtcToTz: (self: TimeZone, hours: number, minutes: number, secs: number) -> DateTime,

	--- @function (hour: number, minute: number, second: number) -> DateTime
	--- Translates a time of the current day in the said specific timezone to a datetime in UTC
	timeTzToUtc: (self: TimeZone, hours: number, minutes: number, secs: number) -> DateTime,

	--- @function () -> DateTime
	--- Translates the current timestamp to a datetime in the said specific timezone
	now: (self: TimeZone) -> DateTime,

	-- Metatable
	__tostring: (TimeZone) -> string,
	__eq: (TimeZone, TimeZone) -> boolean
}
```

</details>

<div id="utcToTz"></div>

### utcToTz

@function (year: number, month: number, day: number, hour: number, minute: number, second: number, all: boolean?) -> DateTime

Translates a timestamp in UTC time to a datetime in the said specific timezone. If `all` is set to true, then multiple times

may be returned in the case of ambiguous times, otherwise the first time is returned.

<details>
<summary>Function Signature</summary>

```luau
--- @function (year: number, month: number, day: number, hour: number, minute: number, second: number, all: boolean?) -> DateTime
--- Translates a timestamp in UTC time to a datetime in the said specific timezone. If `all` is set to true, then multiple times
--- may be returned in the case of ambiguous times, otherwise the first time is returned.
utcToTz: (self: TimeZone, year: number, month: number, day: number, hours: number, minutes: number, secs: number) -> DateTime
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="year"></div>

##### year

[number](#number)

<div id="month"></div>

##### month

[number](#number)

<div id="day"></div>

##### day

[number](#number)

<div id="hours"></div>

##### hours

[number](#number)

<div id="minutes"></div>

##### minutes

[number](#number)

<div id="secs"></div>

##### secs

[number](#number)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[DateTime](#DateTime)<div id="tzToUtc"></div>

### tzToUtc

@function (year: number, month: number, day: number, hour: number, minute: number, second: number, all: boolean?) -> DateTime

Translates a timestamp in the specified timezone to a datetime in UTC. If `all` is set to true, then multiple times

may be returned in the case of ambiguous times, otherwise the first time is returned.

<details>
<summary>Function Signature</summary>

```luau
--- @function (year: number, month: number, day: number, hour: number, minute: number, second: number, all: boolean?) -> DateTime
--- Translates a timestamp in the specified timezone to a datetime in UTC. If `all` is set to true, then multiple times
--- may be returned in the case of ambiguous times, otherwise the first time is returned.
tzToUtc: (self: TimeZone, year: number, month: number, day: number, hours: number, minutes: number, secs: number) -> DateTime
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="year"></div>

##### year

[number](#number)

<div id="month"></div>

##### month

[number](#number)

<div id="day"></div>

##### day

[number](#number)

<div id="hours"></div>

##### hours

[number](#number)

<div id="minutes"></div>

##### minutes

[number](#number)

<div id="secs"></div>

##### secs

[number](#number)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[DateTime](#DateTime)<div id="timeUtcToTz"></div>

### timeUtcToTz

@function (hour: number, minute: number, second: number) -> DateTime

Translates a time of the current day in UTC time to a datetime in the said specific timezone

<details>
<summary>Function Signature</summary>

```luau
--- @function (hour: number, minute: number, second: number) -> DateTime
--- Translates a time of the current day in UTC time to a datetime in the said specific timezone
timeUtcToTz: (self: TimeZone, hours: number, minutes: number, secs: number) -> DateTime
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="hours"></div>

##### hours

[number](#number)

<div id="minutes"></div>

##### minutes

[number](#number)

<div id="secs"></div>

##### secs

[number](#number)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[DateTime](#DateTime)<div id="timeTzToUtc"></div>

### timeTzToUtc

@function (hour: number, minute: number, second: number) -> DateTime

Translates a time of the current day in the said specific timezone to a datetime in UTC

<details>
<summary>Function Signature</summary>

```luau
--- @function (hour: number, minute: number, second: number) -> DateTime
--- Translates a time of the current day in the said specific timezone to a datetime in UTC
timeTzToUtc: (self: TimeZone, hours: number, minutes: number, secs: number) -> DateTime
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="hours"></div>

##### hours

[number](#number)

<div id="minutes"></div>

##### minutes

[number](#number)

<div id="secs"></div>

##### secs

[number](#number)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[DateTime](#DateTime)<div id="now"></div>

### now

@function () -> DateTime

Translates the current timestamp to a datetime in the said specific timezone

<details>
<summary>Function Signature</summary>

```luau
--- @function () -> DateTime
--- Translates the current timestamp to a datetime in the said specific timezone
now: (self: TimeZone) -> DateTime
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[DateTime](#DateTime)<div id="MetatableFields"></div>

### Metatable Fields

<div id="__tostring"></div>

#### __tostring

<details>
<summary>Function Signature</summary>

```luau
__tostring: (TimeZone) -> string
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[TimeZone](#TimeZone)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="__eq"></div>

#### __eq

<details>
<summary>Function Signature</summary>

```luau
__eq: (TimeZone, TimeZone) -> boolean
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[TimeZone](#TimeZone)

<div id="arg2"></div>

##### arg2

[TimeZone](#TimeZone)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="DateTime"></div>

## DateTime

@class DateTime

@within DateTime

A datetime object.



Supports addition/subtraction/equality with TimeDelta objects as well as comparisons with other DateTime objects.

<details>
<summary>Raw Type</summary>

```luau
--- @class DateTime
--- @within DateTime
--- A datetime object. 
---
--- Supports addition/subtraction/equality with TimeDelta objects as well as comparisons with other DateTime objects.
type DateTime = {
	--- @field year
	--- The year of the datetime.
	year: number,

	--- @field month
	--- The month of the datetime.
	month: number,

	--- @field day
	--- The day of the datetime.
	day: number,

	--- @field hour
	--- The hour of the datetime.
	hour: number,

	--- @field minute
	--- The minute of the datetime.
	minute: number,

	--- @field second
	--- The second of the datetime.
	second: number,

	--- @field timestamp_seconds
	--- The timestamp in seconds of the datetime from the Unix epoch.
	timestamp_seconds: number,

	--- @field timestamp_millis
	--- The timestamp in milliseconds of the datetime from the Unix epoch.
	timestamp_millis: number,

	--- @field timestamp_micros
	--- The timestamp in microseconds of the datetime from the Unix epoch.
	timestamp_micros: number,

	--- @field timestamp_nanos
	--- The timestamp in nanoseconds of the datetime from the Unix epoch.
	timestamp_nanos: number,

	--- @field timezone: TimeZone
	--- The timezone of the datetime.
	timezone: TimeZone,

	--- @field base_offset: TimeDelta
	--- The base (non-DST) offset of the datetime.
	base_offset: TimeDelta,

	--- @field dst_offset: TimeDelta
	--- The additional DST offset of the datetime.
	dst_offset: TimeDelta,

	--- @function (TimeZone) -> DateTime
	--- Converts the datetime to the specified timezone.
	with_timezone: (self: TimeZone, TimeZone) -> DateTime,

	--- @function (string) -> string
	--- Formats the datetime using the specified format string.
	format: (self: TimeZone, string) -> string,

	--- @function (DateTime) -> TimeDelta
	--- Calculates the duration between the current datetime and another datetime.
	duration_since: (self: TimeZone, DateTime) -> TimeDelta,

	-- Metatable
	__add: (DateTime, TimeDelta) -> DateTime,
	__sub: (DateTime, TimeDelta) -> DateTime,
	__le: (DateTime, DateTime) -> boolean,
	__lt: (DateTime, DateTime) -> boolean,
	__eq: (DateTime, DateTime) -> boolean,
	__tostring: (DateTime) -> string
}
```

</details>

<div id="year"></div>

### year

The year of the datetime.

[number](#number)

<div id="month"></div>

### month

The month of the datetime.

[number](#number)

<div id="day"></div>

### day

The day of the datetime.

[number](#number)

<div id="hour"></div>

### hour

The hour of the datetime.

[number](#number)

<div id="minute"></div>

### minute

The minute of the datetime.

[number](#number)

<div id="second"></div>

### second

The second of the datetime.

[number](#number)

<div id="timestamp_seconds"></div>

### timestamp_seconds

The timestamp in seconds of the datetime from the Unix epoch.

[number](#number)

<div id="timestamp_millis"></div>

### timestamp_millis

The timestamp in milliseconds of the datetime from the Unix epoch.

[number](#number)

<div id="timestamp_micros"></div>

### timestamp_micros

The timestamp in microseconds of the datetime from the Unix epoch.

[number](#number)

<div id="timestamp_nanos"></div>

### timestamp_nanos

The timestamp in nanoseconds of the datetime from the Unix epoch.

[number](#number)

<div id="timezone"></div>

### timezone

The timezone of the datetime.

[TimeZone](#TimeZone)

<div id="base_offset"></div>

### base_offset

The base (non-DST) offset of the datetime.

[TimeDelta](#TimeDelta)

<div id="dst_offset"></div>

### dst_offset

The additional DST offset of the datetime.

[TimeDelta](#TimeDelta)

<div id="with_timezone"></div>

### with_timezone

@function (TimeZone) -> DateTime

Converts the datetime to the specified timezone.

<details>
<summary>Function Signature</summary>

```luau
--- @function (TimeZone) -> DateTime
--- Converts the datetime to the specified timezone.
with_timezone: (self: TimeZone, TimeZone) -> DateTime
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="arg2"></div>

##### arg2

[TimeZone](#TimeZone)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[DateTime](#DateTime)<div id="format"></div>

### format

@function (string) -> string

Formats the datetime using the specified format string.

<details>
<summary>Function Signature</summary>

```luau
--- @function (string) -> string
--- Formats the datetime using the specified format string.
format: (self: TimeZone, string) -> string
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="arg2"></div>

##### arg2

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="duration_since"></div>

### duration_since

@function (DateTime) -> TimeDelta

Calculates the duration between the current datetime and another datetime.

<details>
<summary>Function Signature</summary>

```luau
--- @function (DateTime) -> TimeDelta
--- Calculates the duration between the current datetime and another datetime.
duration_since: (self: TimeZone, DateTime) -> TimeDelta
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="arg2"></div>

##### arg2

[DateTime](#DateTime)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[TimeDelta](#TimeDelta)<div id="MetatableFields"></div>

### Metatable Fields

<div id="__add"></div>

#### __add

<details>
<summary>Function Signature</summary>

```luau
__add: (DateTime, TimeDelta) -> DateTime
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[DateTime](#DateTime)

<div id="arg2"></div>

##### arg2

[TimeDelta](#TimeDelta)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[DateTime](#DateTime)<div id="__sub"></div>

#### __sub

<details>
<summary>Function Signature</summary>

```luau
__sub: (DateTime, TimeDelta) -> DateTime
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[DateTime](#DateTime)

<div id="arg2"></div>

##### arg2

[TimeDelta](#TimeDelta)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[DateTime](#DateTime)<div id="__le"></div>

#### __le

<details>
<summary>Function Signature</summary>

```luau
__le: (DateTime, DateTime) -> boolean
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[DateTime](#DateTime)

<div id="arg2"></div>

##### arg2

[DateTime](#DateTime)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="__lt"></div>

#### __lt

<details>
<summary>Function Signature</summary>

```luau
__lt: (DateTime, DateTime) -> boolean
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[DateTime](#DateTime)

<div id="arg2"></div>

##### arg2

[DateTime](#DateTime)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="__eq"></div>

#### __eq

<details>
<summary>Function Signature</summary>

```luau
__eq: (DateTime, DateTime) -> boolean
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[DateTime](#DateTime)

<div id="arg2"></div>

##### arg2

[DateTime](#DateTime)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[boolean](#boolean)<div id="__tostring"></div>

#### __tostring

<details>
<summary>Function Signature</summary>

```luau
__tostring: (DateTime) -> string
```

</details>

<div id="Arguments"></div>

##### Arguments

<div id="arg1"></div>

##### arg1

[DateTime](#DateTime)

<div id="Returns"></div>

##### Returns

<div id="ret1"></div>

##### ret1

[string](#string)<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

@function (timezone: string) -> TimeZone

@return TimeZone (The timezone object.)

Returns a new Timezone object if the timezone is recognized/supported.

<details>
<summary>Function Signature</summary>

```luau
--- @function (timezone: string) -> TimeZone
--- @param timezone: string (The timezone to get the offset for.)
--- @return TimeZone (The timezone object.)
--- Returns a new Timezone object if the timezone is recognized/supported.
function new(timezone: string) -> TimeZone end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="timezone"></div>

### timezone

[string](#string)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeZone](#TimeZone)<div id="timedelta_weeks"></div>

## timedelta_weeks

@function (weeks: number) -> TimeDelta

@return TimeDelta

Creates a new TimeDelta object with the specified number of weeks.

<details>
<summary>Function Signature</summary>

```luau
--- @function (weeks: number) -> TimeDelta
--- @param weeks: number (The number of weeks to create the TimeDelta object with.)
--- @return TimeDelta
--- Creates a new TimeDelta object with the specified number of weeks.
function timedelta_weeks(weeks: number) -> TimeDelta end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="weeks"></div>

### weeks

[number](#number)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeDelta](#TimeDelta)<div id="timedelta_days"></div>

## timedelta_days

@function (days: number) -> TimeDelta

@return TimeDelta

Creates a new TimeDelta object with the specified number of days.

<details>
<summary>Function Signature</summary>

```luau
--- @function (days: number) -> TimeDelta
--- @param days: number (The number of days to create the TimeDelta object with.)
--- @return TimeDelta
--- Creates a new TimeDelta object with the specified number of days.
function timedelta_days(days: number) -> TimeDelta end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="days"></div>

### days

[number](#number)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeDelta](#TimeDelta)<div id="timedelta_hours"></div>

## timedelta_hours

@function (hours: number) -> TimeDelta

@return TimeDelta

Creates a new TimeDelta object with the specified number of hours.

<details>
<summary>Function Signature</summary>

```luau
--- @function (hours: number) -> TimeDelta
--- @param hours: number (The number of hours to create the TimeDelta object with.)
--- @return TimeDelta
--- Creates a new TimeDelta object with the specified number of hours.
function timedelta_hours(hours: number) -> TimeDelta end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="hours"></div>

### hours

[number](#number)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeDelta](#TimeDelta)<div id="timedelta_minutes"></div>

## timedelta_minutes

@function (minutes: number) -> TimeDelta

@return TimeDelta

Creates a new TimeDelta object with the specified number of minutes.

<details>
<summary>Function Signature</summary>

```luau
--- @function (minutes: number) -> TimeDelta
--- @param minutes: number (The number of minutes to create the TimeDelta object with.)
--- @return TimeDelta
--- Creates a new TimeDelta object with the specified number of minutes.
function timedelta_minutes(minutes: number) -> TimeDelta end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="minutes"></div>

### minutes

[number](#number)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeDelta](#TimeDelta)<div id="timedelta_seconds"></div>

## timedelta_seconds

@function (seconds: number) -> TimeDelta

@return TimeDelta

Creates a new TimeDelta object with the specified number of seconds.

<details>
<summary>Function Signature</summary>

```luau
--- @function (seconds: number) -> TimeDelta
--- @param seconds: number (The number of seconds to create the TimeDelta object with.)
--- @return TimeDelta
--- Creates a new TimeDelta object with the specified number of seconds.
function timedelta_seconds(seconds: number) -> TimeDelta end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="seconds"></div>

### seconds

[number](#number)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeDelta](#TimeDelta)<div id="timedelta_millis"></div>

## timedelta_millis

@function (millis: number) -> TimeDelta

@return TimeDelta

Creates a new TimeDelta object with the specified number of milliseconds.

<details>
<summary>Function Signature</summary>

```luau
--- @function (millis: number) -> TimeDelta
--- @param millis: number (The number of milliseconds to create the TimeDelta object with.)
--- @return TimeDelta
--- Creates a new TimeDelta object with the specified number of milliseconds.
function timedelta_millis(millis: number) -> TimeDelta end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="millis"></div>

### millis

[number](#number)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeDelta](#TimeDelta)<div id="timedelta_micros"></div>

## timedelta_micros

@function (micros: number) -> TimeDelta

@return TimeDelta

Creates a new TimeDelta object with the specified number of microseconds.

<details>
<summary>Function Signature</summary>

```luau
--- @function (micros: number) -> TimeDelta
--- @param micros: number (The number of microseconds to create the TimeDelta object with.)
--- @return TimeDelta
--- Creates a new TimeDelta object with the specified number of microseconds.
function timedelta_micros(micros: number) -> TimeDelta end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="micros"></div>

### micros

[number](#number)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeDelta](#TimeDelta)<div id="timedelta_nanos"></div>

## timedelta_nanos

@function (nanos: number) -> TimeDelta

@return TimeDelta

Creates a new TimeDelta object with the specified number of nanoseconds.

<details>
<summary>Function Signature</summary>

```luau
--- @function (nanos: number) -> TimeDelta
--- @param nanos: number (The number of nanoseconds to create the TimeDelta object with.)
--- @return TimeDelta
--- Creates a new TimeDelta object with the specified number of nanoseconds.
function timedelta_nanos(nanos: number) -> TimeDelta end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="nanos"></div>

### nanos

[number](#number)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[TimeDelta](#TimeDelta)

---

<div id="@antiraid/discord.luau"></div>

# @antiraid/discord.luau

<div id="Types"></div>

## Types

<div id="GetAuditLogOptions"></div>

## GetAuditLogOptions

Options for getting audit logs in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting audit logs in Discord
type GetAuditLogOptions = {
	--- The action type to filter by
	action_type: discord.AuditLogEventType?,

	--- The user ID to filter by
	user_id: discord.Snowflake?,

	--- The audit log entry ID to filter
	before: discord.Snowflake?,

	--- The number of entries to return
	limit: number?
}
```

</details>

<div id="action_type"></div>

### action_type

The action type to filter by

*This field is optional and may not be specified*

[discord](#module.discord).[AuditLogEventType](#AuditLogEventType)?

<div id="user_id"></div>

### user_id

The user ID to filter by

*This field is optional and may not be specified*

[discord](#module.discord).[Snowflake](#Snowflake)?

<div id="before"></div>

### before

The audit log entry ID to filter

*This field is optional and may not be specified*

[discord](#module.discord).[Snowflake](#Snowflake)?

<div id="limit"></div>

### limit

The number of entries to return

*This field is optional and may not be specified*

[number](#number)?

<div id="GetAutoModerationRuleOptions"></div>

## GetAutoModerationRuleOptions

Options for getting an auto moderation rule in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting an auto moderation rule in Discord
type GetAutoModerationRuleOptions = {
	--- The rule ID
	rule_id: discord.Snowflake
}
```

</details>

<div id="rule_id"></div>

### rule_id

The rule ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="CreateAutoModerationRuleOptions"></div>

## CreateAutoModerationRuleOptions

Options for creating an auto moderation rule in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating an auto moderation rule in Discord
type CreateAutoModerationRuleOptions = {
	--- The reason for creating the rule
	reason: string,

	--- The data to create the rule with
	data: discordRest.CreateAutoModerationRuleRequest
}
```

</details>

<div id="reason"></div>

### reason

The reason for creating the rule

[string](#string)

<div id="data"></div>

### data

The data to create the rule with

[discordRest](#module.discordRest).[CreateAutoModerationRuleRequest](#CreateAutoModerationRuleRequest)

<div id="EditAutoModerationRuleOptions"></div>

## EditAutoModerationRuleOptions

Options for editing an auto moderation rule in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for editing an auto moderation rule in Discord
type EditAutoModerationRuleOptions = {
	--- The rule ID
	rule_id: discord.Snowflake,

	--- The reason for editing the rule
	reason: string,

	--- The data to edit the rule with
	data: discordRest.ModifyAutoModerationRuleRequest
}
```

</details>

<div id="rule_id"></div>

### rule_id

The rule ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="reason"></div>

### reason

The reason for editing the rule

[string](#string)

<div id="data"></div>

### data

The data to edit the rule with

[discordRest](#module.discordRest).[ModifyAutoModerationRuleRequest](#ModifyAutoModerationRuleRequest)

<div id="DeleteAutoModerationRuleOptions"></div>

## DeleteAutoModerationRuleOptions

Options for deleting an auto moderation rule in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting an auto moderation rule in Discord
type DeleteAutoModerationRuleOptions = {
	--- The rule ID
	rule_id: discord.Snowflake,

	--- The reason for deleting the rule
	reason: string
}
```

</details>

<div id="rule_id"></div>

### rule_id

The rule ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="reason"></div>

### reason

The reason for deleting the rule

[string](#string)

<div id="GetChannelOptions"></div>

## GetChannelOptions

Options for getting a channel in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting a channel in Discord
type GetChannelOptions = {
	--- The channel ID
	channel_id: discord.Snowflake
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="EditChannelOptions"></div>

## EditChannelOptions

Options for editing a channel in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for editing a channel in Discord
type EditChannelOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The reason for the edit
	reason: string,

	--- The data to edit the channel with
	data: discordRest.ModifyChannelRequest
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="reason"></div>

### reason

The reason for the edit

[string](#string)

<div id="data"></div>

### data

The data to edit the channel with

[discordRest](#module.discordRest).[ModifyChannelRequest](#ModifyChannelRequest)

<div id="DeleteChannelOptions"></div>

## DeleteChannelOptions

Options for deleting a channel in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for deleting a channel in Discord
type DeleteChannelOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The reason for the deletion
	reason: string
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="reason"></div>

### reason

The reason for the deletion

[string](#string)

<div id="EditChannelPermissionsOptions"></div>

## EditChannelPermissionsOptions

Options for editting channel permissions

<details>
<summary>Raw Type</summary>

```luau
--- Options for editting channel permissions
type EditChannelPermissionsOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The target ID to edit permissions of
	target_id: discord.Snowflake,

	--- The allow permissions
	allow: typesext.MultiOption<string>,

	--- The deny permissions
	deny: typesext.MultiOption<string>,

	--- The type of the target
	kind: discord.OverwriteObjectType,

	--- The reason for the edit
	reason: string
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="target_id"></div>

### target_id

The target ID to edit permissions of

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="allow"></div>

### allow

The allow permissions

[typesext](#module.typesext).[MultiOption](#MultiOption)&lt;[string](#string)&gt;

<div id="deny"></div>

### deny

The deny permissions

[typesext](#module.typesext).[MultiOption](#MultiOption)&lt;[string](#string)&gt;

<div id="kind"></div>

### kind

The type of the target

[discord](#module.discord).[OverwriteObjectType](#OverwriteObjectType)

<div id="reason"></div>

### reason

The reason for the edit

[string](#string)

<div id="AddGuildMemberRoleOptions"></div>

## AddGuildMemberRoleOptions

Options for adding a role to a member

<details>
<summary>Raw Type</summary>

```luau
--- Options for adding a role to a member
type AddGuildMemberRoleOptions = {
	--- The member ID
	user_id: discord.Snowflake,

	--- The role ID
	role_id: discord.Snowflake,

	--- The reason for adding the role
	reason: string
}
```

</details>

<div id="user_id"></div>

### user_id

The member ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="role_id"></div>

### role_id

The role ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="reason"></div>

### reason

The reason for adding the role

[string](#string)

<div id="RemoveGuildMemberRoleOptions"></div>

## RemoveGuildMemberRoleOptions

Options for removing a role from a member

<details>
<summary>Raw Type</summary>

```luau
--- Options for removing a role from a member
type RemoveGuildMemberRoleOptions = {
	--- The member ID
	user_id: discord.Snowflake,

	--- The role ID
	role_id: discord.Snowflake,

	--- The reason for adding the role
	reason: string
}
```

</details>

<div id="user_id"></div>

### user_id

The member ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="role_id"></div>

### role_id

The role ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="reason"></div>

### reason

The reason for adding the role

[string](#string)

<div id="RemoveGuildMemberOptions"></div>

## RemoveGuildMemberOptions

Options for removing a member from a guild

<details>
<summary>Raw Type</summary>

```luau
--- Options for removing a member from a guild
type RemoveGuildMemberOptions = {
	--- The member ID
	user_id: discord.Snowflake,

	--- The reason for removing the member
	reason: string
}
```

</details>

<div id="user_id"></div>

### user_id

The member ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="reason"></div>

### reason

The reason for removing the member

[string](#string)

<div id="GetGuildBansOptions"></div>

## GetGuildBansOptions

Options for getting guild bans



Note: If both `before` and `after` are provided, `before` will take precedence.

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting guild bans
---
--- Note: If both `before` and `after` are provided, `before` will take precedence.
type GetGuildBansOptions = {
	--- The limit of bans to get (max 100)
	limit: number?,

	--- Before a certain user ID
	before: discord.Snowflake?,

	--- After a certain user ID
	after: discord.Snowflake?
}
```

</details>

<div id="limit"></div>

### limit

The limit of bans to get (max 100)

*This field is optional and may not be specified*

[number](#number)?

<div id="before"></div>

### before

Before a certain user ID

*This field is optional and may not be specified*

[discord](#module.discord).[Snowflake](#Snowflake)?

<div id="after"></div>

### after

After a certain user ID

*This field is optional and may not be specified*

[discord](#module.discord).[Snowflake](#Snowflake)?

<div id="CreateMessageOptions"></div>

## CreateMessageOptions

Options for sending a message to a channel in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for sending a message to a channel in Discord
type CreateMessageOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The data to send the message with
	data: discordRest.CreateMessageRequest
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="data"></div>

### data

The data to send the message with

[discordRest](#module.discordRest).[CreateMessageRequest](#CreateMessageRequest)

<div id="CreateCommandOptions"></div>

## CreateCommandOptions

Options for creating a command in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating a command in Discord
type CreateCommandOptions = {
	--- The data to create the command with
	data: discordRest.CreateGuildApplicationCommandRequest
}
```

</details>

<div id="data"></div>

### data

The data to create the command with

[discordRest](#module.discordRest).[CreateGuildApplicationCommandRequest](#CreateGuildApplicationCommandRequest)

<div id="CreateInteractionResponseOptions"></div>

## CreateInteractionResponseOptions

Options for creating an interaction response in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating an interaction response in Discord
type CreateInteractionResponseOptions = {
	--- The interaction ID
	interaction_id: discord.Snowflake,

	--- The interaction token
	interaction_token: string,

	--- The data to create the interaction response with
	data: discordRest.CreateInteractionRequest
}
```

</details>

<div id="interaction_id"></div>

### interaction_id

The interaction ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="interaction_token"></div>

### interaction_token

The interaction token

[string](#string)

<div id="data"></div>

### data

The data to create the interaction response with

[discordRest](#module.discordRest).[CreateInteractionRequest](#CreateInteractionRequest)

<div id="CreateFollowupMessageOptions"></div>

## CreateFollowupMessageOptions

Options for creating a followup message in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for creating a followup message in Discord
type CreateFollowupMessageOptions = {
	--- The interaction token
	interaction_token: string,

	--- The data to create the followup message with
	data: discordRest.CreateFollowupMessageRequest
}
```

</details>

<div id="interaction_token"></div>

### interaction_token

The interaction token

[string](#string)

<div id="data"></div>

### data

The data to create the followup message with

[discordRest](#module.discordRest).[CreateFollowupMessageRequest](#CreateFollowupMessageRequest)

<div id="MessagePagination"></div>

## MessagePagination

A message pagination object

<details>
<summary>Raw Type</summary>

```luau
--- A message pagination object
type MessagePagination = {
	type: "After" | "Around" | "Before",

	id: discord.Snowflake
}
```

</details>

<div id="type"></div>

### type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"After"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Around"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"Before"
```

</details>

<div id="id"></div>

### id

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="GetMessagesOptions"></div>

## GetMessagesOptions

Options for getting messages in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting messages in Discord
type GetMessagesOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The target message
	target: MessagePagination?,

	--- The limit of messages to get
	limit: number?
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="target"></div>

### target

The target message

*This field is optional and may not be specified*

[MessagePagination](#MessagePagination)?

<div id="limit"></div>

### limit

The limit of messages to get

*This field is optional and may not be specified*

[number](#number)?

<div id="GetMessageOptions"></div>

## GetMessageOptions

Options for getting a message in Discord

<details>
<summary>Raw Type</summary>

```luau
--- Options for getting a message in Discord
type GetMessageOptions = {
	--- The channel ID
	channel_id: discord.Snowflake,

	--- The message ID
	message_id: discord.Snowflake
}
```

</details>

<div id="channel_id"></div>

### channel_id

The channel ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="message_id"></div>

### message_id

The message ID

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="DiscordExecutor"></div>

## DiscordExecutor

DiscordExecutor allows templates to access/use the Discord API in a sandboxed form.

<details>
<summary>Raw Type</summary>

```luau
--- DiscordExecutor allows templates to access/use the Discord API in a sandboxed form.
type DiscordExecutor = {
	--- Gets the audit logs
	get_audit_logs: (self: DiscordExecutor, data: GetAuditLogOptions) -> promise.LuaPromise<LazyAuditLogObject>,

	--- Lists the auto moderation rules available
	list_auto_moderation_rules: (self: DiscordExecutor) -> promise.LuaPromise<LazyAutomoderationRuleObjectList>,

	--- Gets an auto moderation rule by ID
	get_auto_moderation_rule: (self: DiscordExecutor, data: GetAutoModerationRuleOptions) -> promise.LuaPromise<LazyAutomoderationRuleObject>,

	--- Creates an auto moderation rule
	create_auto_moderation_rule: (self: DiscordExecutor, data: CreateAutoModerationRuleOptions) -> promise.LuaPromise<LazyAutomoderationRuleObject>,

	--- Edits an auto moderation rule
	edit_auto_moderation_rule: (self: DiscordExecutor, data: EditAutoModerationRuleOptions) -> promise.LuaPromise<LazyAutomoderationRuleObject>,

	--- Deletes an auto moderation rule
	delete_auto_moderation_rule: (self: DiscordExecutor, data: DeleteAutoModerationRuleOptions) -> promise.LuaPromise<LazyAutomoderationRuleObject>,

	--- Gets a channel
	get_channel: (self: DiscordExecutor, data: GetChannelOptions) -> promise.LuaPromise<LazyChannelObject>,

	--- Edits a channel
	edit_channel: (self: DiscordExecutor, data: EditChannelOptions) -> promise.LuaPromise<LazyChannelObject>,

	--- Deletes a channel
	delete_channel: (self: DiscordExecutor, data: DeleteChannelOptions) -> promise.LuaPromise<LazyChannelObject>,

	--- Edits channel permissions for a target
	edit_channel_permissions: (self: DiscordExecutor, data: EditChannelPermissionsOptions) -> promise.LuaPromise<nil>,

	--- Adds a role to a member
	add_guild_member_role: (self: DiscordExecutor, data: AddGuildMemberRoleOptions) -> promise.LuaPromise<nil>,

	--- Removes a role from a member
	remove_guild_member_role: (self: DiscordExecutor, data: RemoveGuildMemberRoleOptions) -> promise.LuaPromise<nil>,

	-- Removes a member from a guild
	remove_guild_member: (self: DiscordExecutor, data: RemoveGuildMemberOptions) -> promise.LuaPromise<nil>,

	--- Gets guild bans
	get_guild_bans: (self: DiscordExecutor, data: GetGuildBansOptions) -> promise.LuaPromise<LazyBanObjectList>,

	--- Returns the guild roles of a guild
	get_guild_roles: (self: DiscordExecutor, guild_id: discord.Snowflake) -> promise.LuaPromise<LazyRolesMap>,

	--- Gets messages from a channel
	get_messages: (self: DiscordExecutor, data: GetMessagesOptions) -> promise.LuaPromise<LazyMessagesObject>,

	--- Gets a message
	get_message: (self: DiscordExecutor, data: GetMessageOptions) -> promise.LuaPromise<LazyMessageObject>,

	--- Creates a message
	create_message: (self: DiscordExecutor, data: CreateMessageOptions) -> promise.LuaPromise<LazyMessageObject>,

	--- Creates an interaction response
	create_interaction_response: (self: DiscordExecutor, data: CreateInteractionResponseOptions) -> promise.LuaPromise<nil>,

	--- Creates a followup interaction response
	create_followup_message: (self: DiscordExecutor, data: CreateFollowupMessageOptions) -> promise.LuaPromise<LazyMessageObject>,

	--- Gets the original interaction response
	get_original_interaction_response: (self: DiscordExecutor, interaction_token: string) -> promise.LuaPromise<LazyMessageObject>,

	--- Gets all guild commands currently registered
	get_guild_commands: (self: DiscordExecutor) -> promise.LuaPromise<LazyApplicationCommandObject>,

	--- Creates a guild command
	create_guild_command: (self: DiscordExecutor, data: CreateCommandOptions) -> promise.LuaPromise<LazyApplicationCommandObject>
}
```

</details>

<div id="get_audit_logs"></div>

### get_audit_logs

Gets the audit logs

<details>
<summary>Function Signature</summary>

```luau
--- Gets the audit logs
get_audit_logs: (self: DiscordExecutor, data: GetAuditLogOptions) -> promise.LuaPromise<LazyAuditLogObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetAuditLogOptions](#GetAuditLogOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyAuditLogObject](#LazyAuditLogObject)&gt;<div id="list_auto_moderation_rules"></div>

### list_auto_moderation_rules

Lists the auto moderation rules available

<details>
<summary>Function Signature</summary>

```luau
--- Lists the auto moderation rules available
list_auto_moderation_rules: (self: DiscordExecutor) -> promise.LuaPromise<LazyAutomoderationRuleObjectList>
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyAutomoderationRuleObjectList](#LazyAutomoderationRuleObjectList)&gt;<div id="get_auto_moderation_rule"></div>

### get_auto_moderation_rule

Gets an auto moderation rule by ID

<details>
<summary>Function Signature</summary>

```luau
--- Gets an auto moderation rule by ID
get_auto_moderation_rule: (self: DiscordExecutor, data: GetAutoModerationRuleOptions) -> promise.LuaPromise<LazyAutomoderationRuleObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetAutoModerationRuleOptions](#GetAutoModerationRuleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyAutomoderationRuleObject](#LazyAutomoderationRuleObject)&gt;<div id="create_auto_moderation_rule"></div>

### create_auto_moderation_rule

Creates an auto moderation rule

<details>
<summary>Function Signature</summary>

```luau
--- Creates an auto moderation rule
create_auto_moderation_rule: (self: DiscordExecutor, data: CreateAutoModerationRuleOptions) -> promise.LuaPromise<LazyAutomoderationRuleObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateAutoModerationRuleOptions](#CreateAutoModerationRuleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyAutomoderationRuleObject](#LazyAutomoderationRuleObject)&gt;<div id="edit_auto_moderation_rule"></div>

### edit_auto_moderation_rule

Edits an auto moderation rule

<details>
<summary>Function Signature</summary>

```luau
--- Edits an auto moderation rule
edit_auto_moderation_rule: (self: DiscordExecutor, data: EditAutoModerationRuleOptions) -> promise.LuaPromise<LazyAutomoderationRuleObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditAutoModerationRuleOptions](#EditAutoModerationRuleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyAutomoderationRuleObject](#LazyAutomoderationRuleObject)&gt;<div id="delete_auto_moderation_rule"></div>

### delete_auto_moderation_rule

Deletes an auto moderation rule

<details>
<summary>Function Signature</summary>

```luau
--- Deletes an auto moderation rule
delete_auto_moderation_rule: (self: DiscordExecutor, data: DeleteAutoModerationRuleOptions) -> promise.LuaPromise<LazyAutomoderationRuleObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteAutoModerationRuleOptions](#DeleteAutoModerationRuleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyAutomoderationRuleObject](#LazyAutomoderationRuleObject)&gt;<div id="get_channel"></div>

### get_channel

Gets a channel

<details>
<summary>Function Signature</summary>

```luau
--- Gets a channel
get_channel: (self: DiscordExecutor, data: GetChannelOptions) -> promise.LuaPromise<LazyChannelObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetChannelOptions](#GetChannelOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyChannelObject](#LazyChannelObject)&gt;<div id="edit_channel"></div>

### edit_channel

Edits a channel

<details>
<summary>Function Signature</summary>

```luau
--- Edits a channel
edit_channel: (self: DiscordExecutor, data: EditChannelOptions) -> promise.LuaPromise<LazyChannelObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditChannelOptions](#EditChannelOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyChannelObject](#LazyChannelObject)&gt;<div id="delete_channel"></div>

### delete_channel

Deletes a channel

<details>
<summary>Function Signature</summary>

```luau
--- Deletes a channel
delete_channel: (self: DiscordExecutor, data: DeleteChannelOptions) -> promise.LuaPromise<LazyChannelObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[DeleteChannelOptions](#DeleteChannelOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyChannelObject](#LazyChannelObject)&gt;<div id="edit_channel_permissions"></div>

### edit_channel_permissions

Edits channel permissions for a target

<details>
<summary>Function Signature</summary>

```luau
--- Edits channel permissions for a target
edit_channel_permissions: (self: DiscordExecutor, data: EditChannelPermissionsOptions) -> promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[EditChannelPermissionsOptions](#EditChannelPermissionsOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="add_guild_member_role"></div>

### add_guild_member_role

Adds a role to a member

<details>
<summary>Function Signature</summary>

```luau
--- Adds a role to a member
add_guild_member_role: (self: DiscordExecutor, data: AddGuildMemberRoleOptions) -> promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[AddGuildMemberRoleOptions](#AddGuildMemberRoleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="remove_guild_member_role"></div>

### remove_guild_member_role

Removes a role from a member

<details>
<summary>Function Signature</summary>

```luau
--- Removes a role from a member
remove_guild_member_role: (self: DiscordExecutor, data: RemoveGuildMemberRoleOptions) -> promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[RemoveGuildMemberRoleOptions](#RemoveGuildMemberRoleOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="remove_guild_member"></div>

### remove_guild_member

Removes a member from a guild

<details>
<summary>Function Signature</summary>

```luau
-- Removes a member from a guild
remove_guild_member: (self: DiscordExecutor, data: RemoveGuildMemberOptions) -> promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[RemoveGuildMemberOptions](#RemoveGuildMemberOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="get_guild_bans"></div>

### get_guild_bans

Gets guild bans

<details>
<summary>Function Signature</summary>

```luau
--- Gets guild bans
get_guild_bans: (self: DiscordExecutor, data: GetGuildBansOptions) -> promise.LuaPromise<LazyBanObjectList>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetGuildBansOptions](#GetGuildBansOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyBanObjectList](#LazyBanObjectList)&gt;<div id="get_guild_roles"></div>

### get_guild_roles

Returns the guild roles of a guild

<details>
<summary>Function Signature</summary>

```luau
--- Returns the guild roles of a guild
get_guild_roles: (self: DiscordExecutor, guild_id: discord.Snowflake) -> promise.LuaPromise<LazyRolesMap>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="guild_id"></div>

##### guild_id

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyRolesMap](#LazyRolesMap)&gt;<div id="get_messages"></div>

### get_messages

Gets messages from a channel

<details>
<summary>Function Signature</summary>

```luau
--- Gets messages from a channel
get_messages: (self: DiscordExecutor, data: GetMessagesOptions) -> promise.LuaPromise<LazyMessagesObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetMessagesOptions](#GetMessagesOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyMessagesObject](#LazyMessagesObject)&gt;<div id="get_message"></div>

### get_message

Gets a message

<details>
<summary>Function Signature</summary>

```luau
--- Gets a message
get_message: (self: DiscordExecutor, data: GetMessageOptions) -> promise.LuaPromise<LazyMessageObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[GetMessageOptions](#GetMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyMessageObject](#LazyMessageObject)&gt;<div id="create_message"></div>

### create_message

Creates a message

<details>
<summary>Function Signature</summary>

```luau
--- Creates a message
create_message: (self: DiscordExecutor, data: CreateMessageOptions) -> promise.LuaPromise<LazyMessageObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateMessageOptions](#CreateMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyMessageObject](#LazyMessageObject)&gt;<div id="create_interaction_response"></div>

### create_interaction_response

Creates an interaction response

<details>
<summary>Function Signature</summary>

```luau
--- Creates an interaction response
create_interaction_response: (self: DiscordExecutor, data: CreateInteractionResponseOptions) -> promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateInteractionResponseOptions](#CreateInteractionResponseOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="create_followup_message"></div>

### create_followup_message

Creates a followup interaction response

<details>
<summary>Function Signature</summary>

```luau
--- Creates a followup interaction response
create_followup_message: (self: DiscordExecutor, data: CreateFollowupMessageOptions) -> promise.LuaPromise<LazyMessageObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateFollowupMessageOptions](#CreateFollowupMessageOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyMessageObject](#LazyMessageObject)&gt;<div id="get_original_interaction_response"></div>

### get_original_interaction_response

Gets the original interaction response

<details>
<summary>Function Signature</summary>

```luau
--- Gets the original interaction response
get_original_interaction_response: (self: DiscordExecutor, interaction_token: string) -> promise.LuaPromise<LazyMessageObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="interaction_token"></div>

##### interaction_token

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyMessageObject](#LazyMessageObject)&gt;<div id="get_guild_commands"></div>

### get_guild_commands

Gets all guild commands currently registered

<details>
<summary>Function Signature</summary>

```luau
--- Gets all guild commands currently registered
get_guild_commands: (self: DiscordExecutor) -> promise.LuaPromise<LazyApplicationCommandObject>
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyApplicationCommandObject](#LazyApplicationCommandObject)&gt;<div id="create_guild_command"></div>

### create_guild_command

Creates a guild command

<details>
<summary>Function Signature</summary>

```luau
--- Creates a guild command
create_guild_command: (self: DiscordExecutor, data: CreateCommandOptions) -> promise.LuaPromise<LazyApplicationCommandObject>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

[CreateCommandOptions](#CreateCommandOptions)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;[LazyApplicationCommandObject](#LazyApplicationCommandObject)&gt;<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

<details>
<summary>Function Signature</summary>

```luau
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> DiscordExecutor end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="token"></div>

### token

[Primitives](#module.Primitives).[TemplateContext](#TemplateContext)

<div id="scope"></div>

### scope

*This field is optional and may not be specified*

[ExecutorScope](#module.ExecutorScope).[ExecutorScope](#ExecutorScope)?

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[DiscordExecutor](#DiscordExecutor)

---

<div id="@antiraid/img_captcha.luau"></div>

# @antiraid/img_captcha.luau

<div id="Types"></div>

## Types

<div id="Geometry"></div>

## Geometry

Represents the area which contains text in a CAPTCHA.

<details>
<summary>Raw Type</summary>

```luau
--- Represents the area which contains text in a CAPTCHA.
type Geometry = {
	--- The minimum x coordinate of the area which contains text (inclusive).
	left: Primitives.u32,

	--- The maximum x coordinate of the area which contains text (inclusive).
	right: Primitives.u32,

	--- The minimum y coordinate of the area which contains text (inclusive).
	top: Primitives.u32,

	--- The maximum y coordinate of the area which contains text (inclusive).
	bottom: Primitives.u32
}
```

</details>

<div id="left"></div>

### left

The minimum x coordinate of the area which contains text (inclusive).

[Primitives](#module.Primitives).[u32](#u32)

<div id="right"></div>

### right

The maximum x coordinate of the area which contains text (inclusive).

[Primitives](#module.Primitives).[u32](#u32)

<div id="top"></div>

### top

The minimum y coordinate of the area which contains text (inclusive).

[Primitives](#module.Primitives).[u32](#u32)

<div id="bottom"></div>

### bottom

The maximum y coordinate of the area which contains text (inclusive).

[Primitives](#module.Primitives).[u32](#u32)

<div id="SerdeColor"></div>

## SerdeColor

Represents a color in RGB format.

<details>
<summary>Raw Type</summary>

```luau
--- Represents a color in RGB format.
type SerdeColor = {
	--- The red component of the color.
	r: Primitives.u8,

	--- The green component of the color.
	g: Primitives.u8,

	--- The blue component of the color.
	b: Primitives.u8
}
```

</details>

<div id="r"></div>

### r

The red component of the color.

[Primitives](#module.Primitives).[u8](#u8)

<div id="g"></div>

### g

The green component of the color.

[Primitives](#module.Primitives).[u8](#u8)

<div id="b"></div>

### b

The blue component of the color.

[Primitives](#module.Primitives).[u8](#u8)

<div id="ColorInvertFilter"></div>

## ColorInvertFilter

Filter that inverts the colors of an image.

<details>
<summary>Raw Type</summary>

```luau
--- Filter that inverts the colors of an image.
type ColorInvertFilter = {
	filter: "ColorInvert"
}
```

</details>

<div id="filter"></div>

### filter

```luau
"ColorInvert"
```

<div id="CowFilter"></div>

## CowFilter

Filter that generates a CAPTCHA with a specified number of cows (circles/other curvical noise)

<details>
<summary>Raw Type</summary>

```luau
--- Filter that generates a CAPTCHA with a specified number of cows (circles/other curvical noise)
type CowFilter = {
	filter: "Cow",

	--- The minimum radius of the cows
	min_radius: Primitives.u32,

	--- The maximum radius of the cows
	max_radius: Primitives.u32,

	--- The number of cows to generate
	n: Primitives.u32,

	--- Whether to allow duplicate cows
	allow_duplicates: boolean,

	--- The geometry of the area which contains text
	geometry: Geometry?
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Cow"
```

<div id="min_radius"></div>

### min_radius

The minimum radius of the cows

[Primitives](#module.Primitives).[u32](#u32)

<div id="max_radius"></div>

### max_radius

The maximum radius of the cows

[Primitives](#module.Primitives).[u32](#u32)

<div id="n"></div>

### n

The number of cows to generate

[Primitives](#module.Primitives).[u32](#u32)

<div id="allow_duplicates"></div>

### allow_duplicates

Whether to allow duplicate cows

[boolean](#boolean)

<div id="geometry"></div>

### geometry

The geometry of the area which contains text

*This field is optional and may not be specified*

[Geometry](#Geometry)?

<div id="DotFilter"></div>

## DotFilter

Filter that creates a specified number of dots

<details>
<summary>Raw Type</summary>

```luau
--- Filter that creates a specified number of dots
type DotFilter = {
	filter: "Dot",

	--- The number of dots to generate
	n: Primitives.u32,

	--- The minimum radius of the dots
	min_radius: Primitives.u32,

	--- The maximum radius of the dots
	max_radius: Primitives.u32
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Dot"
```

<div id="n"></div>

### n

The number of dots to generate

[Primitives](#module.Primitives).[u32](#u32)

<div id="min_radius"></div>

### min_radius

The minimum radius of the dots

[Primitives](#module.Primitives).[u32](#u32)

<div id="max_radius"></div>

### max_radius

The maximum radius of the dots

[Primitives](#module.Primitives).[u32](#u32)

<div id="GridFilter"></div>

## GridFilter

Filter that creates a grid (horizontal/vertical lines with a specified gap in X and Y direction)



(think graph paper)

<details>
<summary>Raw Type</summary>

```luau
--- Filter that creates a grid (horizontal/vertical lines with a specified gap in X and Y direction)
---
--- (think graph paper)
type GridFilter = {
	filter: "Grid",

	--- The Y gap between the vertical lines
	y_gap: Primitives.u32,

	--- The X gap between the horizontal lines
	x_gap: Primitives.u32
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Grid"
```

<div id="y_gap"></div>

### y_gap

The Y gap between the vertical lines

[Primitives](#module.Primitives).[u32](#u32)

<div id="x_gap"></div>

### x_gap

The X gap between the horizontal lines

[Primitives](#module.Primitives).[u32](#u32)

<div id="LineFilter"></div>

## LineFilter

Draw lines/rectangles on the screen

<details>
<summary>Raw Type</summary>

```luau
--- Draw lines/rectangles on the screen
type LineFilter = {
	filter: "Line",

	--- Point 1, must be an array of two numbers
	p1: {Primitives.f32},

	--- Point 2, must be an array of two numbers
	p2: {Primitives.f32},

	--- The thickness of the line
	thickness: Primitives.f32,

	--- The color of the line
	color: SerdeColor
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Line"
```

<div id="p1"></div>

### p1

Point 1, must be an array of two numbers

{[Primitives](#module.Primitives).[f32](#f32)}

<div id="p2"></div>

### p2

Point 2, must be an array of two numbers

{[Primitives](#module.Primitives).[f32](#f32)}

<div id="thickness"></div>

### thickness

The thickness of the line

[Primitives](#module.Primitives).[f32](#f32)

<div id="color"></div>

### color

The color of the line

[SerdeColor](#SerdeColor)

<div id="NoiseFilter"></div>

## NoiseFilter

Adds some random noise at a specified probability

<details>
<summary>Raw Type</summary>

```luau
--- Adds some random noise at a specified probability
type NoiseFilter = {
	filter: "Noise",

	--- The probability of adding noise
	prob: Primitives.f32
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Noise"
```

<div id="prob"></div>

### prob

The probability of adding noise

[Primitives](#module.Primitives).[f32](#f32)

<div id="RandomLineFilter"></div>

## RandomLineFilter

Creates a random line somewhere

<details>
<summary>Raw Type</summary>

```luau
--- Creates a random line somewhere
type RandomLineFilter = {
	filter: "RandomLine"
}
```

</details>

<div id="filter"></div>

### filter

```luau
"RandomLine"
```

<div id="WaveFilter"></div>

## WaveFilter

Creates a wave in a given direction (horizontal/vertical)

<details>
<summary>Raw Type</summary>

```luau
--- Creates a wave in a given direction (horizontal/vertical)
type WaveFilter = {
	filter: "Wave",

	--- The frequency of the wave
	f: Primitives.f32,

	--- The amplitude of the wave
	amp: Primitives.f32,

	--- The direction of the wave
	d: "horizontal" | "vertical"
}
```

</details>

<div id="filter"></div>

### filter

```luau
"Wave"
```

<div id="f"></div>

### f

The frequency of the wave

[Primitives](#module.Primitives).[f32](#f32)

<div id="amp"></div>

### amp

The amplitude of the wave

[Primitives](#module.Primitives).[f32](#f32)

<div id="d"></div>

### d

The direction of the wave

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"horizontal"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"vertical"
```

</details>

<div id="Filter"></div>

## Filter

Represents a filter that can be applied to an image.

<details>
<summary>Raw Type</summary>

```luau
--- Represents a filter that can be applied to an image.
type Filter = ColorInvertFilter | CowFilter | DotFilter | GridFilter | LineFilter | NoiseFilter | RandomLineFilter | WaveFilter
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

[ColorInvertFilter](#ColorInvertFilter)

</details>

<details>
<summary>Variant 2</summary>

[CowFilter](#CowFilter)

</details>

<details>
<summary>Variant 3</summary>

[DotFilter](#DotFilter)

</details>

<details>
<summary>Variant 4</summary>

[GridFilter](#GridFilter)

</details>

<details>
<summary>Variant 5</summary>

[LineFilter](#LineFilter)

</details>

<details>
<summary>Variant 6</summary>

[NoiseFilter](#NoiseFilter)

</details>

<details>
<summary>Variant 7</summary>

[RandomLineFilter](#RandomLineFilter)

</details>

<details>
<summary>Variant 8</summary>

[WaveFilter](#WaveFilter)

</details>

<div id="CaptchaConfig"></div>

## CaptchaConfig

Captcha configuration

<details>
<summary>Raw Type</summary>

```luau
--- Captcha configuration
type CaptchaConfig = {
	--- The number of characters the CAPTCHA should have.
	char_count: Primitives.u8,

	--- The list of filters
	filters: {Filter},

	--- The size of the viewbox to render the CAPTCHA in.
	--- (first element is width, second element is height)
	viewbox_size: {Primitives.u32},

	--- At what index of CAPTCHA generation should a viewbox be created at.
	set_viewbox_at_idx: Primitives.u8
}
```

</details>

<div id="char_count"></div>

### char_count

The number of characters the CAPTCHA should have.

[Primitives](#module.Primitives).[u8](#u8)

<div id="filters"></div>

### filters

The list of filters

{[Filter](#Filter)}

<div id="viewbox_size"></div>

### viewbox_size

The size of the viewbox to render the CAPTCHA in.

(first element is width, second element is height)

{[Primitives](#module.Primitives).[u32](#u32)}

<div id="set_viewbox_at_idx"></div>

### set_viewbox_at_idx

At what index of CAPTCHA generation should a viewbox be created at.

[Primitives](#module.Primitives).[u8](#u8)

<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

Creates a new CAPTCHA with the given configuration.

<details>
<summary>Function Signature</summary>

```luau
--- Creates a new CAPTCHA with the given configuration.
function new(config: CaptchaConfig) -> promise.LuaPromise<{Primitives.u8}> end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="config"></div>

### config

[CaptchaConfig](#CaptchaConfig)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[promise](#module.promise).[LuaPromise](#LuaPromise)&lt;{[Primitives](#module.Primitives).[u8](#u8)}&gt;

---

<div id="@antiraid/interop.luau"></div>

# @antiraid/interop.luau

<div id="Types"></div>

## Types

<div id="null"></div>

## null

<details>
<summary>Raw Type</summary>

```luau
type null = {
	___placeholder: number
}
```

</details>

<div id="___placeholder"></div>

### ___placeholder

[number](#number)



---

<div id="@antiraid/kv.luau"></div>

# @antiraid/kv.luau

<div id="Types"></div>

## Types

<div id="KvRecord"></div>

## KvRecord

KvRecord represents a key-value record with metadata.

@class KvRecord

<details>
<summary>Raw Type</summary>

```luau
--- KvRecord represents a key-value record with metadata.
---@class KvRecord
---@field key string The key of the record.
---@field value any The value of the record.
---@field exists boolean Whether the record exists.
---@field created_at string The timestamp when the record was created.
---@field last_updated_at string The timestamp when the record was last updated.
type KvRecord = {
	key: string,

	value: any,

	exists: boolean,

	created_at: string,

	last_updated_at: string
}
```

</details>

<div id="key"></div>

### key

[string](#string)

<div id="value"></div>

### value

[any](#any)

<div id="exists"></div>

### exists

[boolean](#boolean)

<div id="created_at"></div>

### created_at

[string](#string)

<div id="last_updated_at"></div>

### last_updated_at

[string](#string)

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

	--- The scope of the executor.
	scope: ExecutorScope.ExecutorScope,

	--- Finds records in the key-value store.
	--- @param query string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%
	--- @return {KvRecord} The records.
	find: (self: KvExecutor, query: string) -> Promise.LuaPromise<{KvRecord}>,

	--- Gets a value from the key-value store.
	--- @param key string The key of the record.
	--- @return any The value of the record.
	get: (self: KvExecutor, key: string) -> Promise.LuaPromise<any>,

	--- Gets a record from the key-value store.
	--- @param key string The key of the record.
	--- @return KvRecord The record.
	getrecord: (self: KvExecutor, key: string) -> Promise.LuaPromise<KvRecord>,

	--- Sets a record in the key-value store.
	--- @param key string The key of the record.
	--- @param value any The value of the record.
	--- @return KvRecord The record.
	set: (self: KvExecutor, key: string, value: any) -> Promise.LuaPromise<KvRecord>,

	--- Deletes a record from the key-value store.
	--- @param key string The key of the record.
	delete: (self: KvExecutor, key: string) -> Promise.LuaPromise<nil>
}
```

</details>

<div id="find"></div>

### find

Finds records in the key-value store.

@return {KvRecord} The records.

<details>
<summary>Function Signature</summary>

```luau
--- Finds records in the key-value store.
--- @param query string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%
--- @return {KvRecord} The records.
find: (self: KvExecutor, query: string) -> Promise.LuaPromise<{KvRecord}>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="query"></div>

##### query

string The key to search for. % matches zero or more characters; _ matches a single character. To search anywhere in a string, surround {KEY} with %, e.g. %{KEY}%

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;{[KvRecord](#KvRecord)}&gt;<div id="get"></div>

### get

Gets a value from the key-value store.

@return any The value of the record.

<details>
<summary>Function Signature</summary>

```luau
--- Gets a value from the key-value store.
--- @param key string The key of the record.
--- @return any The value of the record.
get: (self: KvExecutor, key: string) -> Promise.LuaPromise<any>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[any](#any)&gt;<div id="getrecord"></div>

### getrecord

Gets a record from the key-value store.

@return KvRecord The record.

<details>
<summary>Function Signature</summary>

```luau
--- Gets a record from the key-value store.
--- @param key string The key of the record.
--- @return KvRecord The record.
getrecord: (self: KvExecutor, key: string) -> Promise.LuaPromise<KvRecord>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[KvRecord](#KvRecord)&gt;<div id="set"></div>

### set

Sets a record in the key-value store.

@return KvRecord The record.

<details>
<summary>Function Signature</summary>

```luau
--- Sets a record in the key-value store.
--- @param key string The key of the record.
--- @param value any The value of the record.
--- @return KvRecord The record.
set: (self: KvExecutor, key: string, value: any) -> Promise.LuaPromise<KvRecord>
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

[any](#any)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[KvRecord](#KvRecord)&gt;<div id="delete"></div>

### delete

Deletes a record from the key-value store.

<details>
<summary>Function Signature</summary>

```luau
--- Deletes a record from the key-value store.
--- @param key string The key of the record.
delete: (self: KvExecutor, key: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="key"></div>

##### key

string The key of the record.

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="guild_id"></div>

### guild_id

The guild ID the executor will perform key-value operations on.

[string](#string)

<div id="origin_guild_id"></div>

### origin_guild_id

The originating guild ID (the guild ID of the template itself).

[string](#string)

<div id="scope"></div>

### scope

The scope of the executor.

[ExecutorScope](#module.ExecutorScope).[ExecutorScope](#ExecutorScope)

<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

<details>
<summary>Function Signature</summary>

```luau
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> KvExecutor end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="token"></div>

### token

[Primitives](#module.Primitives).[TemplateContext](#TemplateContext)

<div id="scope"></div>

### scope

*This field is optional and may not be specified*

[ExecutorScope](#module.ExecutorScope).[ExecutorScope](#ExecutorScope)?

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[KvExecutor](#KvExecutor)

---

<div id="@antiraid/lazy.luau"></div>

# @antiraid/lazy.luau

<div id="Types"></div>

## Types

<div id="Lazy"></div>

## Lazy

A lazy value is a value that is only serialized when accessed

<details>
<summary>Raw Type</summary>

```luau
--- A lazy value is a value that is only serialized when accessed
type Lazy<T> = {
	--- The inner value. Only serialized when accessed and then cached.
	data: T,

	--- Always returns true
	lazy: boolean
}
```

</details>

<div id="data"></div>

### data

The inner value. Only serialized when accessed and then cached.

[T](#T)

<div id="lazy"></div>

### lazy

Always returns true

[boolean](#boolean)

<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

<details>
<summary>Function Signature</summary>

```luau
function new<T>(value: T) -> Lazy<T> end
```

</details>

<div id="Generics"></div>

## Generics

<div id="T"></div>

### T

This generic is unconstrained and can be any type

<div id="Arguments"></div>

## Arguments

<div id="value"></div>

### value

[T](#T)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[Lazy](#Lazy)&lt;[T](#T)&gt;

---

<div id="@antiraid/lockdowns.luau"></div>

# @antiraid/lockdowns.luau

<div id="Types"></div>

## Types

<div id="Lockdown"></div>

## Lockdown

Lockdown represents a currently applied lockdown

@class Lockdown

<details>
<summary>Raw Type</summary>

```luau
--- Lockdown represents a currently applied lockdown
---@class Lockdown
---@field id string The ID of the lockdown.
---@field reason string The reason for the lockdown.
---@field type string The type of the lockdown in its string form
---@field data string The data internally stored for the lockdown.
---@field created_at string The timestamp when the lockdown was created.
type Lockdown = {
	id: string,

	reason: string,

	type: string,

	data: any,

	created_at: string
}
```

</details>

<div id="id"></div>

### id

[string](#string)

<div id="reason"></div>

### reason

[string](#string)

<div id="type"></div>

### type

[string](#string)

<div id="data"></div>

### data

[any](#any)

<div id="created_at"></div>

### created_at

[string](#string)

<div id="LockdownExecutor"></div>

## LockdownExecutor

LockdownExecutor allows templates to list, create and delete AntiRaid lockdowns

@class LockdownExecutor

<details>
<summary>Raw Type</summary>

```luau
--- LockdownExecutor allows templates to list, create and delete AntiRaid lockdowns
---@class LockdownExecutor
type LockdownExecutor = {
	--- Lists all active lockdowns
	--- @return Promise.LuaPromise<{Lockdown}> The active lockdowns
	list: (self: LockdownExecutor) -> Promise.LuaPromise<{Lockdown}>,

	--- Starts a quick server lockdown
	--- @param reason string The reason for the lockdown
	qsl: (self: LockdownExecutor, reason: string) -> Promise.LuaPromise<nil>,

	--- Starts a traditional server lockdown.
	--- 
	--- This is *much* slower than a QSL but also does not require
	--- any special server setup.
	--- @param reason string The reason for the lockdown
	tsl: (self: LockdownExecutor, reason: string) -> Promise.LuaPromise<nil>,

	--- Starts a lockdown on a single channel
	--- @param channel_id string The ID of the channel to lock down
	--- @param reason string The reason for the lockdown
	scl: (self: LockdownExecutor, channel_id: string, reason: string) -> Promise.LuaPromise<nil>,

	--- Starts a lockdown on a role
	--- @param role_id string The ID of the role to lock down
	--- @param reason string The reason for the lockdown
	role: (self: LockdownExecutor, role_id: string, reason: string) -> Promise.LuaPromise<nil>,

	--- Removes a lockdown (ends it)
	--- @param id string The ID of the lockdown to remove
	remove: (self: LockdownExecutor, id: string) -> Promise.LuaPromise<nil>
}
```

</details>

<div id="list"></div>

### list

Lists all active lockdowns

@return Promise.LuaPromise<{Lockdown}> The active lockdowns

<details>
<summary>Function Signature</summary>

```luau
--- Lists all active lockdowns
--- @return Promise.LuaPromise<{Lockdown}> The active lockdowns
list: (self: LockdownExecutor) -> Promise.LuaPromise<{Lockdown}>
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;{[Lockdown](#Lockdown)}&gt;<div id="qsl"></div>

### qsl

Starts a quick server lockdown

<details>
<summary>Function Signature</summary>

```luau
--- Starts a quick server lockdown
--- @param reason string The reason for the lockdown
qsl: (self: LockdownExecutor, reason: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="reason"></div>

##### reason

string The reason for the lockdown

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="tsl"></div>

### tsl

Starts a traditional server lockdown.



This is *much* slower than a QSL but also does not require

any special server setup.

<details>
<summary>Function Signature</summary>

```luau
--- Starts a traditional server lockdown.
--- 
--- This is *much* slower than a QSL but also does not require
--- any special server setup.
--- @param reason string The reason for the lockdown
tsl: (self: LockdownExecutor, reason: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="reason"></div>

##### reason

string The reason for the lockdown

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="scl"></div>

### scl

Starts a lockdown on a single channel

<details>
<summary>Function Signature</summary>

```luau
--- Starts a lockdown on a single channel
--- @param channel_id string The ID of the channel to lock down
--- @param reason string The reason for the lockdown
scl: (self: LockdownExecutor, channel_id: string, reason: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="channel_id"></div>

##### channel_id

string The ID of the channel to lock down

[string](#string)

<div id="reason"></div>

##### reason

string The reason for the lockdown

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="role"></div>

### role

Starts a lockdown on a role

<details>
<summary>Function Signature</summary>

```luau
--- Starts a lockdown on a role
--- @param role_id string The ID of the role to lock down
--- @param reason string The reason for the lockdown
role: (self: LockdownExecutor, role_id: string, reason: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="role_id"></div>

##### role_id

string The ID of the role to lock down

[string](#string)

<div id="reason"></div>

##### reason

string The reason for the lockdown

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="remove"></div>

### remove

Removes a lockdown (ends it)

<details>
<summary>Function Signature</summary>

```luau
--- Removes a lockdown (ends it)
--- @param id string The ID of the lockdown to remove
remove: (self: LockdownExecutor, id: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the lockdown to remove

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

<details>
<summary>Function Signature</summary>

```luau
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> LockdownExecutor end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="token"></div>

### token

[Primitives](#module.Primitives).[TemplateContext](#TemplateContext)

<div id="scope"></div>

### scope

*This field is optional and may not be specified*

[ExecutorScope](#module.ExecutorScope).[ExecutorScope](#ExecutorScope)?

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[LockdownExecutor](#LockdownExecutor)

---

<div id="@antiraid/luau.luau"></div>

# @antiraid/luau.luau

<div id="Types"></div>

## Types

<div id="Chunk"></div>

## Chunk

<details>
<summary>Raw Type</summary>

```luau
type Chunk = {
	--- Requires the ``luau:eval.set_environment`` capability to modify
	environment: {
		[any]: any
	}?,

	--- Requires the ``luau:eval.set_optimization_level`` capability to modify
	optimization_level: number?,

	--- Requires the ``luau:eval.modify_set_code`` capability to modify
	code: string,

	--- Requires the ``luau:eval.set_chunk_name`` capability to modify
	chunk_name: string?,

	--- Requires the ``luau:eval.call`` capability to use. Takes in args and returns the 
	--- returned values from the ``code`` being evaluated.
	call: (self: Chunk, args: any) -> any,

	--- Requires the ``luau:eval.call_async`` capability to use. Takes in args and returns the
	--- returned values from the ``code`` being evaluated.
	---
	--- This runs the code asynchronously
	call_async: (self: Chunk, args: any) -> Promise.LuaPromise<any>
}
```

</details>

<div id="call"></div>

### call

Requires the ``luau:eval.call`` capability to use. Takes in args and returns the

returned values from the ``code`` being evaluated.

<details>
<summary>Function Signature</summary>

```luau
--- Requires the ``luau:eval.call`` capability to use. Takes in args and returns the 
--- returned values from the ``code`` being evaluated.
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

Requires the ``luau:eval.call_async`` capability to use. Takes in args and returns the

returned values from the ``code`` being evaluated.



This runs the code asynchronously

<details>
<summary>Function Signature</summary>

```luau
--- Requires the ``luau:eval.call_async`` capability to use. Takes in args and returns the
--- returned values from the ``code`` being evaluated.
---
--- This runs the code asynchronously
call_async: (self: Chunk, args: any) -> Promise.LuaPromise<any>
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

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[any](#any)&gt;<div id="environment"></div>

### environment

Requires the ``luau:eval.set_environment`` capability to modify

*This field is optional and may not be specified*

{[any]: [any](#any)}?

<div id="optimization_level"></div>

### optimization_level

Requires the ``luau:eval.set_optimization_level`` capability to modify

*This field is optional and may not be specified*

[number](#number)?

<div id="code"></div>

### code

Requires the ``luau:eval.modify_set_code`` capability to modify

[string](#string)

<div id="chunk_name"></div>

### chunk_name

Requires the ``luau:eval.set_chunk_name`` capability to modify

*This field is optional and may not be specified*

[string](#string)?

<div id="Functions"></div>

# Functions

<div id="load"></div>

## load

Requires the ``luau:eval`` capability to use. Be careful as this allows

for arbitrary code execution.

<details>
<summary>Function Signature</summary>

```luau
--- Requires the ``luau:eval`` capability to use. Be careful as this allows
--- for arbitrary code execution.
function load(token: Primitives.TemplateContext, code: string) -> Chunk end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="token"></div>

### token

[Primitives](#module.Primitives).[TemplateContext](#TemplateContext)

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

---

<div id="@antiraid/pages.luau"></div>

# @antiraid/pages.luau

<div id="Types"></div>

## Types

<div id="SettingOperations"></div>

## SettingOperations

Supported setting operations for a setting

<details>
<summary>Raw Type</summary>

```luau
--- Supported setting operations for a setting
type SettingOperations = {
	--- @field boolean Can the setting be viewed?
	view: boolean,

	--- @field boolean Can the setting be created?
	create: boolean,

	--- @field boolean Can the setting be updated?
	update: boolean,

	--- @field boolean Can the setting be deleted?
	delete: boolean
}
```

</details>

<div id="view"></div>

### view

[boolean](#boolean)

<div id="create"></div>

### create

[boolean](#boolean)

<div id="update"></div>

### update

[boolean](#boolean)

<div id="delete"></div>

### delete

[boolean](#boolean)

<div id="ColumnSuggestion"></div>

## ColumnSuggestion

A suggestion for a column. Can either be a static set of suggestions or no suggestions at all.

<details>
<summary>Raw Type</summary>

```luau
--- A suggestion for a column. Can either be a static set of suggestions or no suggestions at all.
type ColumnSuggestion = {
	type: "None"
} | {
	type: "Static",

	suggestions: {string}
}
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

```luau
"None"
```

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

```luau
"Static"
```

<div id="suggestions"></div>

#### suggestions

{[string](#string)}

</details>

<div id="ColumnType"></div>

## ColumnType

The type-specific data about a column

<details>
<summary>Raw Type</summary>

```luau
--- The type-specific data about a column
type ColumnType = {
	type: "Scalar" | "Array",

	inner: "String",

	--- @field number The minimum length of the string
	min_length: number?,

	--- @field number The maximum length of the string
	max_length: number?,

	--- @field {string} The allowed values for the string (will be rendered as either a select menu or otherwise)
	---
	--- If empty, all values are allowed.
	allowed_values: {string},

	--- @field string The kind of string this is. This will be used to determine how the string is rendered client-side.
	--- e.g. textarea, channel, user, role, kittycat-permission, uuid, interval, timestamp etc.
	kind: string
} | {
	type: "Scalar" | "Array",

	inner: "Integer"
} | {
	type: "Scalar" | "Array",

	inner: "Float"
} | {
	type: "Scalar" | "Array",

	inner: "BitFlag",

	--- @field {string} The bitflag values as a hashmap
	values: {
		[string]: number
	}
} | {
	type: "Scalar" | "Array",

	inner: "Boolean"
} | {
	type: "Scalar" | "Array",

	inner: "Json",

	--- @field The maximum size, in bytes, of the JSON object
	max_bytes: number?
}
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Scalar"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Array"
```

</details>

<div id="inner"></div>

#### inner

```luau
"String"
```

<div id="min_length"></div>

#### min_length

*This field is optional and may not be specified*

[number](#number)?

<div id="max_length"></div>

#### max_length

*This field is optional and may not be specified*

[number](#number)?

<div id="allowed_values"></div>

#### allowed_values



If empty, all values are allowed.

{[string](#string)}

<div id="kind"></div>

#### kind

e.g. textarea, channel, user, role, kittycat-permission, uuid, interval, timestamp etc.

[string](#string)

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Scalar"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Array"
```

</details>

<div id="inner"></div>

#### inner

```luau
"Integer"
```

</details>

<details>
<summary>Variant 3</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Scalar"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Array"
```

</details>

<div id="inner"></div>

#### inner

```luau
"Float"
```

</details>

<details>
<summary>Variant 4</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Scalar"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Array"
```

</details>

<div id="inner"></div>

#### inner

```luau
"BitFlag"
```

<div id="values"></div>

#### values

*This is an inline table type with the following fields*

<div id="[string]"></div>

##### [string]

[number](#number)

</details>

<details>
<summary>Variant 5</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Scalar"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Array"
```

</details>

<div id="inner"></div>

#### inner

```luau
"Boolean"
```

</details>

<details>
<summary>Variant 6</summary>

*This is an inline table type with the following fields*

<div id="type"></div>

#### type

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"Scalar"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Array"
```

</details>

<div id="inner"></div>

#### inner

```luau
"Json"
```

<div id="max_bytes"></div>

#### max_bytes

*This field is optional and may not be specified*

[number](#number)?

</details>

<div id="OperationType"></div>

## OperationType

<details>
<summary>Raw Type</summary>

```luau
type OperationType = "View" | "Create" | "Update" | "Delete"
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"View"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"Create"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"Update"
```

</details>

<details>
<summary>Variant 4</summary>

```luau
"Delete"
```

</details>

<div id="Column"></div>

## Column

A column in a setting

<details>
<summary>Raw Type</summary>

```luau
--- A column in a setting
type Column = {
	--- @field string The ID of the column
	id: string,

	--- @field string The friendly name of the column
	name: string,

	--- @field string The description of the column
	description: string,

	--- @field ColumnType The type of the column
	column_type: ColumnType,

	--- @field boolean Whether or not the column is nullable
	nullable: boolean,

	--- @field ColumnSuggestion The suggestions for the column
	suggestions: ColumnSuggestion,

	--- @field boolean Whether or not the column is a secret field
	secret: boolean,

	--- @field {string} The operations for which the field should be ignored (essentially, read only)
	---
	--- Semantics are defined by the template.
	ignored_for: {OperationType}
}
```

</details>

<div id="id"></div>

### id

[string](#string)

<div id="name"></div>

### name

[string](#string)

<div id="description"></div>

### description

[string](#string)

<div id="column_type"></div>

### column_type

[ColumnType](#ColumnType)

<div id="nullable"></div>

### nullable

[boolean](#boolean)

<div id="suggestions"></div>

### suggestions

[ColumnSuggestion](#ColumnSuggestion)

<div id="secret"></div>

### secret

[boolean](#boolean)

<div id="ignored_for"></div>

### ignored_for



Semantics are defined by the template.

{[OperationType](#OperationType)}

<div id="Setting"></div>

## Setting

<details>
<summary>Raw Type</summary>

```luau
type Setting = {
	--- @field string The ID of the option
	id: string,

	--- @field string The name of the option
	name: string,

	--- @field string The description of the option
	description: string,

	--- @field string The primary key of the table. This *should* be present in user responses to page settings
	--- as well but this is not guaranteed and must be checked for by the template.
	primary_key: string,

	--- @field string Title template, used for the title of the embed
	title_template: string,

	--- @field {Column} The columns for this option
	columns: {Column},

	--- @field SettingOperations The supported operations for this option
	supported_operations: SettingOperations
}
```

</details>

<div id="id"></div>

### id

[string](#string)

<div id="name"></div>

### name

[string](#string)

<div id="description"></div>

### description

[string](#string)

<div id="primary_key"></div>

### primary_key

as well but this is not guaranteed and must be checked for by the template.

[string](#string)

<div id="title_template"></div>

### title_template

[string](#string)

<div id="columns"></div>

### columns

{[Column](#Column)}

<div id="supported_operations"></div>

### supported_operations

[SettingOperations](#SettingOperations)

<div id="Page"></div>

## Page

<details>
<summary>Raw Type</summary>

```luau
type Page = {
	--- @field string The title of the page.
	title: string,

	--- @field string The description of the page.
	description: string,

	--- @field {Setting} The settings of the page.
	settings: {Setting}
}
```

</details>

<div id="title"></div>

### title

[string](#string)

<div id="description"></div>

### description

[string](#string)

<div id="settings"></div>

### settings

{[Setting](#Setting)}

<div id="PageExecutor"></div>

## PageExecutor

<details>
<summary>Raw Type</summary>

```luau
type PageExecutor = {
	--- @function () -> Page
	--- Returns the current page.
	get: (self: PageExecutor) -> Promise.LuaPromise<Page?>,

	--- @function () -> Promise<void>
	--- Sets a page to be the templates page. This will overwrite any existing page if one exists.
	set: (self: PageExecutor, page: Page) -> Promise.LuaPromise<{}>,

	--- @function () -> Promise<void>
	--- Deletes the templates page. This will not delete the page itself, but will remove it from the server's list of custom pages.
	delete: (self: PageExecutor) -> Promise.LuaPromise<{}>
}
```

</details>

<div id="get"></div>

### get

@function () -> Page

Returns the current page.

<details>
<summary>Function Signature</summary>

```luau
--- @function () -> Page
--- Returns the current page.
get: (self: PageExecutor) -> Promise.LuaPromise<Page?>
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[Page](#Page)?&gt;<div id="set"></div>

### set

@function () -> Promise<void>

Sets a page to be the templates page. This will overwrite any existing page if one exists.

<details>
<summary>Function Signature</summary>

```luau
--- @function () -> Promise<void>
--- Sets a page to be the templates page. This will overwrite any existing page if one exists.
set: (self: PageExecutor, page: Page) -> Promise.LuaPromise<{}>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="page"></div>

##### page

[Page](#Page)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;{}&gt;<div id="delete"></div>

### delete

@function () -> Promise<void>

Deletes the templates page. This will not delete the page itself, but will remove it from the server's list of custom pages.

<details>
<summary>Function Signature</summary>

```luau
--- @function () -> Promise<void>
--- Deletes the templates page. This will not delete the page itself, but will remove it from the server's list of custom pages.
delete: (self: PageExecutor) -> Promise.LuaPromise<{}>
```

</details>

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;{}&gt;<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

Creates a new PageExecutor

<details>
<summary>Function Signature</summary>

```luau
--- Creates a new PageExecutor
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> PageExecutor end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="token"></div>

### token

[Primitives](#module.Primitives).[TemplateContext](#TemplateContext)

<div id="scope"></div>

### scope

*This field is optional and may not be specified*

[ExecutorScope](#module.ExecutorScope).[ExecutorScope](#ExecutorScope)?

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[PageExecutor](#PageExecutor)

---

<div id="@antiraid/permissions.luau"></div>

# @antiraid/permissions.luau

<div id="Types"></div>

## Types

<div id="Permission"></div>

## Permission

<details>
<summary>Raw Type</summary>

```luau
type Permission = kittycat.Permission
```

</details>

[kittycat](#module.kittycat).[Permission](#Permission)

<div id="StaffPermissions"></div>

## StaffPermissions

<details>
<summary>Raw Type</summary>

```luau
type StaffPermissions = kittycat.StaffPermissions
```

</details>

[kittycat](#module.kittycat).[StaffPermissions](#StaffPermissions)

<div id="CheckPatchChangesError"></div>

## CheckPatchChangesError

<details>
<summary>Raw Type</summary>

```luau
type CheckPatchChangesError = kittycat.CheckPatchChangesError
```

</details>

[kittycat](#module.kittycat).[CheckPatchChangesError](#CheckPatchChangesError)

<div id="Functions"></div>

# Functions

<div id="permission_from_string"></div>

## permission_from_string

Returns a Permission object from a string.

<details>
<summary>Function Signature</summary>

```luau
--- Returns a Permission object from a string.
function permission_from_string(str: string) -> kittycat.Permission end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="str"></div>

### str

[string](#string)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[kittycat](#module.kittycat).[Permission](#Permission)<div id="permission_to_string"></div>

## permission_to_string

Returns a string from a Permission object.

<details>
<summary>Function Signature</summary>

```luau
--- Returns a string from a Permission object.
function permission_to_string(perm: kittycat.Permission) -> string end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="perm"></div>

### perm

[kittycat](#module.kittycat).[Permission](#Permission)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[string](#string)<div id="has_perm"></div>

## has_perm

Checks if a list of permissions in Permission object form contains a specific permission.

<details>
<summary>Function Signature</summary>

```luau
-- Checks if a list of permissions in Permission object form contains a specific permission.
function has_perm(permissions: {kittycat.Permission}, permission: kittycat.Permission) -> boolean end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="permissions"></div>

### permissions

{[kittycat](#module.kittycat).[Permission](#Permission)}

<div id="permission"></div>

### permission

[kittycat](#module.kittycat).[Permission](#Permission)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[boolean](#boolean)<div id="has_perm_str"></div>

## has_perm_str

Checks if a list of permissions in canonical string form contains a specific permission.

<details>
<summary>Function Signature</summary>

```luau
--- Checks if a list of permissions in canonical string form contains a specific permission.
function has_perm_str(permissions: {string}, permission: string) -> boolean end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="permissions"></div>

### permissions

{[string](#string)}

<div id="permission"></div>

### permission

[string](#string)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[boolean](#boolean)<div id="staff_permissions_resolve"></div>

## staff_permissions_resolve

Resolves a StaffPermissions object into a list of Permission objects.

<details>
<summary>Function Signature</summary>

```luau
--- Resolves a StaffPermissions object into a list of Permission objects.
function staff_permissions_resolve(sp: kittycat.StaffPermissions) -> {kittycat.Permission} end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="sp"></div>

### sp

[kittycat](#module.kittycat).[StaffPermissions](#StaffPermissions)

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

{[kittycat](#module.kittycat).[Permission](#Permission)}<div id="check_patch_changes"></div>

## check_patch_changes

Checks if a list of permissions can be patched to another list of permissions.

<details>
<summary>Function Signature</summary>

```luau
--- Checks if a list of permissions can be patched to another list of permissions.
function check_patch_changes(manager_perms: {kittycat.Permission}, current_perms: {kittycat.Permission}, new_perms: {kittycat.Permission}) -> (boolean, kittycat.CheckPatchChangesError?) end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="manager_perms"></div>

### manager_perms

{[kittycat](#module.kittycat).[Permission](#Permission)}

<div id="current_perms"></div>

### current_perms

{[kittycat](#module.kittycat).[Permission](#Permission)}

<div id="new_perms"></div>

### new_perms

{[kittycat](#module.kittycat).[Permission](#Permission)}

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[boolean](#boolean)

<div id="ret2"></div>

### ret2

*This field is optional and may not be specified*

[kittycat](#module.kittycat).[CheckPatchChangesError](#CheckPatchChangesError)?



---

<div id="@antiraid/promise.luau"></div>

# @antiraid/promise.luau

<div id="Types"></div>

## Types

<div id="LuaPromise"></div>

## LuaPromise

Opaque promise type returned by antiraid

<details>
<summary>Raw Type</summary>

```luau
--- Opaque promise type returned by antiraid
type LuaPromise<T> = {
	--- Note: this will always actually be nil but is required to enforce nominal typing properly
	__phantom_LuaPromiseT: T
}
```

</details>

<div id="__phantom_LuaPromiseT"></div>

### __phantom_LuaPromiseT

Note: this will always actually be nil but is required to enforce nominal typing properly

[T](#T)

<div id="Functions"></div>

# Functions

<div id="yield"></div>

## yield

Yields the coroutine and resumes it returning the end value of the promise when it resolves

<details>
<summary>Function Signature</summary>

```luau
--- Yields the coroutine and resumes it returning the end value of the promise when it resolves
function yield<T>(promise: LuaPromise<T>) -> T end
```

</details>

<div id="Generics"></div>

## Generics

<div id="T"></div>

### T

This generic is unconstrained and can be any type

<div id="Arguments"></div>

## Arguments

<div id="promise"></div>

### promise

[LuaPromise](#LuaPromise)&lt;[T](#T)&gt;<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[T](#T)

---

<div id="@antiraid/stings.luau"></div>

# @antiraid/stings.luau

<div id="Types"></div>

## Types

<div id="StingCreate"></div>

## StingCreate

A type representing a new sting to be created.

<details>
<summary>Raw Type</summary>

```luau
--- A type representing a new sting to be created.
type StingCreate = {
	--- The source of the sting.
	src: string?,

	--- The number of stings.
	stings: number,

	--- The reason for the stings.
	reason: string?,

	--- The reason the stings were voided.
	void_reason: string?,

	--- The guild ID the sting targets. MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON
	guild_id: string,

	--- The creator of the sting. Must be either 'system' or 'user:{user_id}'
	creator: string,

	--- The target of the sting. Must be either 'system' or 'user:{user_id}'
	target: string,

	--- The state of the sting.
	state: "active" | "voided" | "handled",

	--- When the sting expires as a duration. Format: {duration}{unit} e.g. 1d, 1h, 1m, 1s
	duration: string?,

	--- The data/metadata present within the sting, if any.
	sting_data: any?
}
```

</details>

<div id="src"></div>

### src

The source of the sting.

*This field is optional and may not be specified*

[string](#string)?

<div id="stings"></div>

### stings

The number of stings.

[number](#number)

<div id="reason"></div>

### reason

The reason for the stings.

*This field is optional and may not be specified*

[string](#string)?

<div id="void_reason"></div>

### void_reason

The reason the stings were voided.

*This field is optional and may not be specified*

[string](#string)?

<div id="guild_id"></div>

### guild_id

The guild ID the sting targets. MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON

[string](#string)

<div id="creator"></div>

### creator

The creator of the sting. Must be either 'system' or 'user:{user_id}'

[string](#string)

<div id="target"></div>

### target

The target of the sting. Must be either 'system' or 'user:{user_id}'

[string](#string)

<div id="state"></div>

### state

The state of the sting.

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"active"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"voided"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"handled"
```

</details>

<div id="duration"></div>

### duration

When the sting expires as a duration. Format: {duration}{unit} e.g. 1d, 1h, 1m, 1s

*This field is optional and may not be specified*

[string](#string)?

<div id="sting_data"></div>

### sting_data

The data/metadata present within the sting, if any.

*This field is optional and may not be specified*

[any](#any)?

<div id="Sting"></div>

## Sting

A type representing a sting.

<details>
<summary>Raw Type</summary>

```luau
--- A type representing a sting.
type Sting = {
	--- The ID of the sting.
	id: string,

	--- The source of the sting.
	src: string?,

	--- The number of stings.
	stings: number,

	--- The reason for the stings.
	reason: string?,

	--- The reason the stings were voided.
	void_reason: string?,

	--- The guild ID the sting targets. MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON
	guild_id: string,

	--- The creator of the sting. Must be either 'system' or 'user:{user_id}'
	creator: string,

	--- The target of the sting. Must be either 'system' or 'user:{user_id}'
	target: string,

	--- The state of the sting.
	state: "active" | "voided" | "handled",

	--- When the sting expires as a duration. Format: {duration}{unit} e.g. 1d, 1h, 1m, 1s
	duration: string?,

	--- The data/metadata present within the sting, if any.
	sting_data: any?,

	--- The log of the sting as it was being handled internally by AntiRaid's internal systens
	handle_log: any?
}
```

</details>

<div id="id"></div>

### id

The ID of the sting.

[string](#string)

<div id="src"></div>

### src

The source of the sting.

*This field is optional and may not be specified*

[string](#string)?

<div id="stings"></div>

### stings

The number of stings.

[number](#number)

<div id="reason"></div>

### reason

The reason for the stings.

*This field is optional and may not be specified*

[string](#string)?

<div id="void_reason"></div>

### void_reason

The reason the stings were voided.

*This field is optional and may not be specified*

[string](#string)?

<div id="guild_id"></div>

### guild_id

The guild ID the sting targets. MUST MATCH THE GUILD ID THE TEMPLATE IS RUNNING ON

[string](#string)

<div id="creator"></div>

### creator

The creator of the sting. Must be either 'system' or 'user:{user_id}'

[string](#string)

<div id="target"></div>

### target

The target of the sting. Must be either 'system' or 'user:{user_id}'

[string](#string)

<div id="state"></div>

### state

The state of the sting.

Union with variants:

<details>
<summary>Variant 1</summary>

```luau
"active"
```

</details>

<details>
<summary>Variant 2</summary>

```luau
"voided"
```

</details>

<details>
<summary>Variant 3</summary>

```luau
"handled"
```

</details>

<div id="duration"></div>

### duration

When the sting expires as a duration. Format: {duration}{unit} e.g. 1d, 1h, 1m, 1s

*This field is optional and may not be specified*

[string](#string)?

<div id="sting_data"></div>

### sting_data

The data/metadata present within the sting, if any.

*This field is optional and may not be specified*

[any](#any)?

<div id="handle_log"></div>

### handle_log

The log of the sting as it was being handled internally by AntiRaid's internal systens

*This field is optional and may not be specified*

[any](#any)?

<div id="StingExecutor"></div>

## StingExecutor

<details>
<summary>Raw Type</summary>

```luau
type StingExecutor = {
	--- Lists a page of stings. The number of stings per page is undefined at this time
	--- @param page number The page to list
	--- @return Promise.LuaPromise<{Sting}> The list of stings
	list: (self: StingExecutor, page: number) -> Promise.LuaPromise<{Sting}>,

	--- Gets a sting by its ID
	--- @param id string The ID of the sting
	--- @return Promise.LuaPromise<Sting> The sting
	get: (self: StingExecutor, id: string) -> Promise.LuaPromise<Sting>,

	--- Creates a new sting
	--- @param data StingCreate The data to create the sting with
	--- @return Promise.LuaPromise<string> The ID of the created sting
	create: (self: StingExecutor, data: StingCreate) -> Promise.LuaPromise<string>,

	--- Updates a sting 
	--- @param data Sting The data to update the sting with. Note that the ID of the sting must exist in DB and the previous sting
	--- with said ID will be replaced with ``data``.
	--- @return Promise.LuaPromise<nil>
	update: (self: StingExecutor, data: Sting) -> Promise.LuaPromise<nil>,

	--- Deletes a sting by its ID
	--- @param id string The ID of the sting
	--- @return Promise.LuaPromise<nil>
	delete: (self: StingExecutor, id: string) -> Promise.LuaPromise<nil>
}
```

</details>

<div id="list"></div>

### list

Lists a page of stings. The number of stings per page is undefined at this time

@return Promise.LuaPromise<{Sting}> The list of stings

<details>
<summary>Function Signature</summary>

```luau
--- Lists a page of stings. The number of stings per page is undefined at this time
--- @param page number The page to list
--- @return Promise.LuaPromise<{Sting}> The list of stings
list: (self: StingExecutor, page: number) -> Promise.LuaPromise<{Sting}>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="page"></div>

##### page

number The page to list

[number](#number)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;{[Sting](#Sting)}&gt;<div id="get"></div>

### get

Gets a sting by its ID

@return Promise.LuaPromise<Sting> The sting

<details>
<summary>Function Signature</summary>

```luau
--- Gets a sting by its ID
--- @param id string The ID of the sting
--- @return Promise.LuaPromise<Sting> The sting
get: (self: StingExecutor, id: string) -> Promise.LuaPromise<Sting>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the sting

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[Sting](#Sting)&gt;<div id="create"></div>

### create

Creates a new sting

@return Promise.LuaPromise<string> The ID of the created sting

<details>
<summary>Function Signature</summary>

```luau
--- Creates a new sting
--- @param data StingCreate The data to create the sting with
--- @return Promise.LuaPromise<string> The ID of the created sting
create: (self: StingExecutor, data: StingCreate) -> Promise.LuaPromise<string>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

StingCreate The data to create the sting with

[StingCreate](#StingCreate)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[string](#string)&gt;<div id="update"></div>

### update

Updates a sting

with said ID will be replaced with ``data``.

@return Promise.LuaPromise<nil>

<details>
<summary>Function Signature</summary>

```luau
--- Updates a sting 
--- @param data Sting The data to update the sting with. Note that the ID of the sting must exist in DB and the previous sting
--- with said ID will be replaced with ``data``.
--- @return Promise.LuaPromise<nil>
update: (self: StingExecutor, data: Sting) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="data"></div>

##### data

Sting The data to update the sting with. Note that the ID of the sting must exist in DB and the previous sting

[Sting](#Sting)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="delete"></div>

### delete

Deletes a sting by its ID

@return Promise.LuaPromise<nil>

<details>
<summary>Function Signature</summary>

```luau
--- Deletes a sting by its ID
--- @param id string The ID of the sting
--- @return Promise.LuaPromise<nil>
delete: (self: StingExecutor, id: string) -> Promise.LuaPromise<nil>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="id"></div>

##### id

string The ID of the sting

[string](#string)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[nil](#nil)&gt;<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

<details>
<summary>Function Signature</summary>

```luau
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> StingExecutor end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="token"></div>

### token

[Primitives](#module.Primitives).[TemplateContext](#TemplateContext)

<div id="scope"></div>

### scope

*This field is optional and may not be specified*

[ExecutorScope](#module.ExecutorScope).[ExecutorScope](#ExecutorScope)?

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[StingExecutor](#StingExecutor)

---

<div id="@antiraid/typesext.luau"></div>

# @antiraid/typesext.luau

<div id="Types"></div>

## Types

<div id="MultiOption"></div>

## MultiOption

A multi option is either T (Some(Some(T)), a empty table (Some(None)), or nil (None)

<details>
<summary>Raw Type</summary>

```luau
--- A multi option is either T (Some(Some(T)), a empty table (Some(None)), or nil (None)
type MultiOption<T> = T | {} | nil
```

</details>

Union with variants:

<details>
<summary>Variant 1</summary>

[T](#T)

</details>

<details>
<summary>Variant 2</summary>

*This is an inline table type with the following fields*

</details>

<details>
<summary>Variant 3</summary>

[nil](#nil)

</details>



---

<div id="@antiraid/userinfo.luau"></div>

# @antiraid/userinfo.luau

<div id="Types"></div>

## Types

<div id="UserInfo"></div>

## UserInfo

@class UserInfo



Represents a summary of a users permission related

information on AntiRaid





<details>
<summary>Raw Type</summary>

```luau
--- @class UserInfo
---
--- Represents a summary of a users permission related 
--- information on AntiRaid
---
--- @field discord_permissions discord.Snowflake The users discord permissions
--- @field kittycat_staff_permissions Kittycat.StaffPermissions The users kittycat staff permissions
--- @field kittycat_resolved_permissions {Kittycat.Permission} The users resolved kittycat permissions
--- @field guild_owner_id discord.Snowflake The ID of the guild owner
--- @field guild_roles {[discord.Snowflake]: discord.GuildRoleObject} The users guild roles
--- @field member_roles {discord.Snowflake} The users member roles
---
type UserInfo = {
	discord_permissions: discord.Snowflake,

	kittycat_staff_permissions: Kittycat.StaffPermissions,

	kittycat_resolved_permissions: {Kittycat.Permission},

	guild_owner_id: discord.Snowflake,

	guild_roles: {
		[discord.Snowflake]: discord.GuildRoleObject
	},

	member_roles: {discord.Snowflake}
}
```

</details>

<div id="discord_permissions"></div>

### discord_permissions

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="kittycat_staff_permissions"></div>

### kittycat_staff_permissions

[Kittycat](#module.Kittycat).[StaffPermissions](#StaffPermissions)

<div id="kittycat_resolved_permissions"></div>

### kittycat_resolved_permissions

{[Kittycat](#module.Kittycat).[Permission](#Permission)}

<div id="guild_owner_id"></div>

### guild_owner_id

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="guild_roles"></div>

### guild_roles

*This is an inline table type with the following fields*

<div id="[discord.Snowflake]"></div>

##### [discord.Snowflake]

[discord](#module.discord).[GuildRoleObject](#GuildRoleObject)

<div id="member_roles"></div>

### member_roles

{[discord](#module.discord).[Snowflake](#Snowflake)}

<div id="UserInfoExecutor"></div>

## UserInfoExecutor

@class UserInfoExecutor



Allows templates to get permission-related information about a user



<details>
<summary>Raw Type</summary>

```luau
--- @class UserInfoExecutor
---
--- Allows templates to get permission-related information about a user
---
--- @field get (user_id: discord.Snowflake): Promise.LuaPromise<UserInfo> Gets the UserInfo for a user
type UserInfoExecutor = {
	--- Gets the UserInfo for a user
	--- @param user_id discord.Snowflake The ID of the user to get the UserInfo for
	get: (self: UserInfoExecutor, user_id: discord.Snowflake) -> Promise.LuaPromise<UserInfo>
}
```

</details>

<div id="get"></div>

### get

Gets the UserInfo for a user

<details>
<summary>Function Signature</summary>

```luau
--- Gets the UserInfo for a user
--- @param user_id discord.Snowflake The ID of the user to get the UserInfo for
get: (self: UserInfoExecutor, user_id: discord.Snowflake) -> Promise.LuaPromise<UserInfo>
```

</details>

<div id="Arguments"></div>

#### Arguments

<div id="user_id"></div>

##### user_id

discord.Snowflake The ID of the user to get the UserInfo for

[discord](#module.discord).[Snowflake](#Snowflake)

<div id="Returns"></div>

#### Returns

<div id="ret1"></div>

##### ret1

[Promise](#module.Promise).[LuaPromise](#LuaPromise)&lt;[UserInfo](#UserInfo)&gt;<div id="Functions"></div>

# Functions

<div id="new"></div>

## new

<details>
<summary>Function Signature</summary>

```luau
function new(token: Primitives.TemplateContext, scope: ExecutorScope.ExecutorScope?) -> UserInfoExecutor end
```

</details>

<div id="Arguments"></div>

## Arguments

<div id="token"></div>

### token

[Primitives](#module.Primitives).[TemplateContext](#TemplateContext)

<div id="scope"></div>

### scope

*This field is optional and may not be specified*

[ExecutorScope](#module.ExecutorScope).[ExecutorScope](#ExecutorScope)?

<div id="Returns"></div>

## Returns

<div id="ret1"></div>

### ret1

[UserInfoExecutor](#UserInfoExecutor)