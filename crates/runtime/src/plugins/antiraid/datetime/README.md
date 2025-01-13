# @antiraid/datetime

This plugin allows for the managing timezones.

## Types

<div id="type.Timezone" />

### Timezone

A timezone object.

#### Methods

##### Timezone:utcToTz

```lua
function Timezone:utcToTz(year: number, month: number, day: number, hours: number, minutes: number, secs: number, all: boolean?)
```

Translates a timestamp in UTC time to a datetime in the said specific timezone.

###### Parameters

- `year` ([number](#type.number)): The year to translate.
- `month` ([number](#type.number)): The month to translate.
- `day` ([number](#type.number)): The day to translate.
- `hours` ([number](#type.number)): The hours to translate.
- `minutes` ([number](#type.number)): The minutes to translate.
- `secs` ([number](#type.number)): The seconds to translate.
- `all` ([boolean?](#type.boolean)): Whether to return both offsets if the time is ambiguous.


###### Returns

- `date` ([DateTime](#type.DateTime)): The translated datetime.- `date2` ([DateTime?](#type.DateTime)): The second translated datetime if the time is ambiguous.
##### Timezone:tzToUtc

```lua
function Timezone:tzToUtc(year: number, month: number, day: number, hours: number, minutes: number, secs: number, all: boolean?)
```

Translates a timestamp in the specified timezone to a datetime in UTC.

###### Parameters

- `year` ([number](#type.number)): The year to translate.
- `month` ([number](#type.number)): The month to translate.
- `day` ([number](#type.number)): The day to translate.
- `hours` ([number](#type.number)): The hours to translate.
- `minutes` ([number](#type.number)): The minutes to translate.
- `secs` ([number](#type.number)): The seconds to translate.
- `all` ([boolean?](#type.boolean)): Whether to return both offsets if the time is ambiguous.


###### Returns

- `date` ([DateTime](#type.DateTime)): The translated datetime.- `date2` ([DateTime?](#type.DateTime)): The second translated datetime if the time is ambiguous.
##### Timezone:timeUtcToTz

```lua
function Timezone:timeUtcToTz(hours: number, minutes: number, secs: number): DateTime
```

Translates a time of the current day in UTC time to a datetime in the said specific timezone.

###### Parameters

- `hours` ([number](#type.number)): The hours to translate.
- `minutes` ([number](#type.number)): The minutes to translate.
- `secs` ([number](#type.number)): The seconds to translate.


###### Returns

- `date` ([DateTime](#type.DateTime)): The translated datetime.
##### Timezone:timeTzToUtc

```lua
function Timezone:timeTzToUtc(hours: number, minutes: number, secs: number): DateTime
```

Translates a time of the current day in the said specific timezone to a datetime in UTC.

###### Parameters

- `hours` ([number](#type.number)): The hours to translate.
- `minutes` ([number](#type.number)): The minutes to translate.
- `secs` ([number](#type.number)): The seconds to translate.


###### Returns

- `date` ([DateTime](#type.DateTime)): The translated datetime.
##### Timezone:now

```lua
function Timezone:now(): DateTime
```

Translates the current timestamp to a datetime in the said specific timezone.

###### Returns

- `date` ([DateTime](#type.DateTime)): The translated datetime.


<div id="type.TimeDelta" />

### TimeDelta

A time delta object. Supports addition/subtraction with another TimeDelta object as well as comparisons with them.



#### Fields

- `nanos` ([number](#type.number)): The number of nanoseconds in the time delta.
- `micros` ([number](#type.number)): The number of microseconds in the time delta.
- `millis` ([number](#type.number)): The number of milliseconds in the time delta.
- `seconds` ([number](#type.number)): The number of seconds in the time delta.
- `minutes` ([number](#type.number)): The number of minutes in the time delta.
- `hours` ([number](#type.number)): The number of hours in the time delta.
- `days` ([number](#type.number)): The number of days in the time delta.
- `weeks` ([number](#type.number)): The number of weeks in the time delta.


#### Methods

##### TimeDelta:offset_string

```lua
function TimeDelta:offset_string(): string
```

Returns the offset as a string.

###### Returns

- `offset` ([string](#type.string)): The offset as a string.


<div id="type.DateTime" />

### DateTime

A datetime object. Supports addition/subtraction with TimeDelta objects as well as comparisons with other DateTime objects.



#### Fields

- `year` ([number](#type.number)): The year of the datetime.
- `month` ([number](#type.number)): The month of the datetime.
- `day` ([number](#type.number)): The day of the datetime.
- `hour` ([number](#type.number)): The hour of the datetime.
- `minute` ([number](#type.number)): The minute of the datetime.
- `second` ([number](#type.number)): The second of the datetime.
- `timestamp_seconds` ([number](#type.number)): The timestamp in seconds of the datetime from the Unix epoch.
- `timestamp_millis` ([number](#type.number)): The timestamp in milliseconds of the datetime from the Unix epoch.
- `timestamp_micros` ([number](#type.number)): The timestamp in microseconds of the datetime from the Unix epoch.
- `timestamp_nanos` ([number](#type.number)): The timestamp in nanoseconds of the datetime from the Unix epoch.
- `tz` ([Timezone](#type.Timezone)): The timezone of the datetime.
- `offset` ([TimeDelta](#type.TimeDelta)): The offset of the datetime.


#### Methods

##### DateTime:with_timezone

```lua
function DateTime:with_timezone(tz: Timezone): DateTime
```

Converts the datetime to the specified timezone.

###### Parameters

- `tz` ([Timezone](#type.Timezone)): The timezone to convert to.


###### Returns

- `dt` ([DateTime](#type.DateTime)): The converted datetime.
##### DateTime:format

```lua
function DateTime:format(format: string): string
```

Formats the datetime using the specified format string.

###### Parameters

- `format` ([string](#type.string)): The format string to use.


###### Returns

- `formatted` ([string](#type.string)): The formatted datetime.
##### DateTime:duration_since

```lua
function DateTime:duration_since(other: DateTime): TimeDelta
```

Calculates the duration between the current datetime and another datetime.

###### Parameters

- `other` ([DateTime](#type.DateTime)): The other datetime to calculate the duration to.


###### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The duration between the two datetimes.


## Methods

### new

```lua
function new(timezone: string): Timezone
```

Returns a new Timezone object if the timezone is recognized/supported.

#### Parameters

- `timezone` ([string](#type.string)): The timezone to get the offset for.


#### Returns

- `tzobj` ([Timezone](#type.Timezone)): The timezone userdata object.

### timedelta_weeks

```lua
function timedelta_weeks(weeks: number): TimeDelta
```

Creates a new TimeDelta object with the specified number of weeks.

#### Parameters

- `weeks` ([number](#type.number)): The number of weeks.


#### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The TimeDelta object.

### timedelta_days

```lua
function timedelta_days(days: number): TimeDelta
```

Creates a new TimeDelta object with the specified number of days.

#### Parameters

- `days` ([number](#type.number)): The number of days.


#### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The TimeDelta object.

### timedelta_hours

```lua
function timedelta_hours(hours: number): TimeDelta
```

Creates a new TimeDelta object with the specified number of hours.

#### Parameters

- `hours` ([number](#type.number)): The number of hours.


#### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The TimeDelta object.

### timedelta_minutes

```lua
function timedelta_minutes(minutes: number): TimeDelta
```

Creates a new TimeDelta object with the specified number of minutes.

#### Parameters

- `minutes` ([number](#type.number)): The number of minutes.


#### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The TimeDelta object.

### timedelta_seconds

```lua
function timedelta_seconds(seconds: number): TimeDelta
```

Creates a new TimeDelta object with the specified number of seconds.

#### Parameters

- `seconds` ([number](#type.number)): The number of seconds.


#### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The TimeDelta object.

### timedelta_millis

```lua
function timedelta_millis(millis: number): TimeDelta
```

Creates a new TimeDelta object with the specified number of milliseconds.

#### Parameters

- `millis` ([number](#type.number)): The number of milliseconds.


#### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The TimeDelta object.

### timedelta_micros

```lua
function timedelta_micros(micros: number): TimeDelta
```

Creates a new TimeDelta object with the specified number of microseconds.

#### Parameters

- `micros` ([number](#type.number)): The number of microseconds.


#### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The TimeDelta object.

### timedelta_nanos

```lua
function timedelta_nanos(nanos: number): TimeDelta
```

Creates a new TimeDelta object with the specified number of nanoseconds.

#### Parameters

- `nanos` ([number](#type.number)): The number of nanoseconds.

#### Returns

- `td` ([TimeDelta](#type.TimeDelta)): The TimeDelta object.
