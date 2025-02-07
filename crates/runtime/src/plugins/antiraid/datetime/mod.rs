use std::str::FromStr;

use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::OffsetComponents;
use mlua::prelude::*;

use crate::primitives::create_userdata_iterator_with_fields;

pub struct TimeDelta {
    pub timedelta: chrono::TimeDelta,
}

impl LuaUserData for TimeDelta {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(this.timedelta.to_string())
        });

        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<TimeDelta>| Ok(this.timedelta == other.timedelta),
        );

        methods.add_meta_method(
            LuaMetaMethod::Add,
            |_, this, other: LuaUserDataRef<TimeDelta>| {
                Ok(TimeDelta {
                    timedelta: this.timedelta + other.timedelta,
                })
            },
        );

        methods.add_meta_method(
            LuaMetaMethod::Sub,
            |_, this, other: LuaUserDataRef<TimeDelta>| {
                Ok(TimeDelta {
                    timedelta: this.timedelta - other.timedelta,
                })
            },
        );

        methods.add_meta_method(
            LuaMetaMethod::Le,
            |_, this, other: LuaUserDataRef<TimeDelta>| Ok(this.timedelta <= other.timedelta),
        );

        methods.add_meta_method(
            LuaMetaMethod::Lt,
            |_, this, other: LuaUserDataRef<TimeDelta>| Ok(this.timedelta < other.timedelta),
        );

        methods.add_method("offset_string", |_, this, ()| {
            Ok(format!(
                "{}{:02}:{:02}",
                if this.timedelta.num_seconds() < 0 {
                    "-"
                } else {
                    "+"
                },
                this.timedelta.num_hours(),
                this.timedelta.num_minutes() % 60
            ))
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<TimeDelta>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "nanos",
                    "micros",
                    "millis",
                    "seconds",
                    "minutes",
                    "hours",
                    "days",
                    "weeks",
                    // Methods
                    "offset_string",
                ],
            )
        });
    }

    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_meta_field(LuaMetaMethod::Type, "TimeDelta".to_string());
        fields.add_field_method_get("nanos", |_, this| Ok(this.timedelta.num_nanoseconds()));
        fields.add_field_method_get("micros", |_, this| Ok(this.timedelta.num_microseconds()));
        fields.add_field_method_get("millis", |_, this| Ok(this.timedelta.num_milliseconds()));
        fields.add_field_method_get("seconds", |_, this| Ok(this.timedelta.num_seconds()));
        fields.add_field_method_get("minutes", |_, this| Ok(this.timedelta.num_minutes()));
        fields.add_field_method_get("hours", |_, this| Ok(this.timedelta.num_hours()));
        fields.add_field_method_get("days", |_, this| Ok(this.timedelta.num_days()));
        fields.add_field_method_get("weeks", |_, this| Ok(this.timedelta.num_weeks()));
    }
}

pub struct DateTime<Tz>
where
    Tz: chrono::TimeZone + 'static + From<chrono_tz::Tz>,
    chrono_tz::Tz: From<Tz>,
    Tz::Offset: std::fmt::Display,
{
    pub dt: chrono::DateTime<Tz>,
}

impl<Tz> LuaUserData for DateTime<Tz>
where
    Tz: chrono::TimeZone + 'static + From<chrono_tz::Tz>,
    chrono_tz::Tz: From<Tz>,
    Tz::Offset: std::fmt::Display,
{
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(this.dt.to_rfc3339())
        });

        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<DateTime<Tz>>| Ok(this.dt == other.dt),
        );

        methods.add_meta_method(
            LuaMetaMethod::Add,
            |_, this, td: LuaUserDataRef<TimeDelta>| {
                Ok(DateTime {
                    dt: this.dt.clone() + td.timedelta,
                })
            },
        );

        methods.add_meta_method(
            LuaMetaMethod::Sub,
            |_, this, td: LuaUserDataRef<TimeDelta>| {
                Ok(DateTime {
                    dt: this.dt.clone() - td.timedelta,
                })
            },
        );

        methods.add_meta_method(
            LuaMetaMethod::Le,
            |_, this, other: LuaUserDataRef<DateTime<Tz>>| Ok(this.dt <= other.dt),
        );

        methods.add_meta_method(
            LuaMetaMethod::Lt,
            |_, this, other: LuaUserDataRef<DateTime<Tz>>| Ok(this.dt < other.dt),
        );

        methods.add_method("with_timezone", |_, this, tz: LuaUserDataRef<Timezone>| {
            Ok(DateTime {
                dt: this.dt.with_timezone(&tz.tz.into()),
            })
        });

        methods.add_method("format", |_, this, format: String| {
            Ok(this.dt.format(&format).to_string())
        });

        methods.add_method(
            "duration_since",
            |_, this, other: LuaUserDataRef<DateTime<Tz>>| {
                Ok(TimeDelta {
                    timedelta: this.dt.clone().signed_duration_since(other.dt.clone()),
                })
            },
        );

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<DateTime<Tz>>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    "year",
                    "month",
                    "day",
                    "hour",
                    "minute",
                    "second",
                    "timestamp_seconds",
                    "timestamp_millis",
                    "timestamp_micros",
                    "timestamp_nanos",
                    "tz",
                    "base_offset",
                    "dst_offset",
                    // Methods
                    "with_timezone",
                    "format",
                    "duration_since",
                ],
            )
        });
    }

    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("year", |_, this| Ok(this.dt.year()));
        fields.add_field_method_get("month", |_, this| Ok(this.dt.month()));
        fields.add_field_method_get("day", |_, this| Ok(this.dt.day()));
        fields.add_field_method_get("hour", |_, this| Ok(this.dt.hour()));
        fields.add_field_method_get("minute", |_, this| Ok(this.dt.minute()));
        fields.add_field_method_get("second", |_, this| Ok(this.dt.second()));
        fields.add_field_method_get("timestamp_seconds", |_, this| Ok(this.dt.timestamp()));
        fields.add_field_method_get("timestamp_millis", |_, this| Ok(this.dt.timestamp_millis()));
        fields.add_field_method_get("timestamp_micros", |_, this| {
            Ok(this.dt.timestamp_subsec_micros())
        });
        fields.add_field_method_get("timestamp_nanos", |_, this| {
            Ok(this.dt.timestamp_subsec_nanos())
        });
        fields.add_field_method_get("tz", |_, this| {
            Ok(Timezone {
                tz: this.dt.timezone().into(),
            })
        });
        fields.add_field_method_get("base_offset", |_, this| {
            let tz: chrono_tz::Tz = this.dt.timezone().into();

            let td = tz
                .offset_from_utc_datetime(&this.dt.naive_utc())
                .base_utc_offset();

            Ok(TimeDelta { timedelta: td })
        });
        fields.add_field_method_get("dst_offset", |_, this| {
            let tz: chrono_tz::Tz = this.dt.timezone().into();

            let td = tz
                .offset_from_utc_datetime(&this.dt.naive_utc())
                .dst_offset();

            Ok(TimeDelta { timedelta: td })
        });
    }
}

pub struct Timezone {
    pub tz: chrono_tz::Tz,
}

impl LuaUserData for Timezone {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(this.tz.to_string())
        });

        methods.add_meta_method(
            LuaMetaMethod::Eq,
            |_, this, other: LuaUserDataRef<Timezone>| Ok(this.tz == other.tz),
        );

        // Translates a timestamp in UTC time to a datetime in the said specific timezone
        methods.add_method(
            "utcToTz",
            |_,
             this,
             (year, month, day, hours, minutes, secs, all): (
                i32,
                u32,
                u32,
                u32,
                u32,
                u32,
                Option<bool>,
            )| {
                match chrono_tz::Tz::UTC.with_ymd_and_hms(year, month, day, hours, minutes, secs) {
                    chrono::LocalResult::Single(ymd_hms) => {
                        let ymd_hms = ymd_hms.with_timezone(&this.tz);
                        Ok((Some(DateTime { dt: ymd_hms }), None))
                    }
                    chrono::LocalResult::Ambiguous(tz, t2) => {
                        if all.unwrap_or(false) {
                            let tz = tz.with_timezone(&this.tz);
                            let t2 = t2.with_timezone(&this.tz);
                            Ok((Some(DateTime { dt: tz }), Some(DateTime { dt: t2 })))
                        } else {
                            let tz = tz.with_timezone(&this.tz);
                            Ok((Some(DateTime { dt: tz }), None))
                        }
                    }
                    chrono::LocalResult::None => {
                        Err(mlua::Error::RuntimeError("Invalid date".to_string()))
                    }
                }
            },
        );

        // Translates a timestamp in the specified timezone to a datetime in UTC
        //
        // Returns a tuple of the first offset and second offset (if the time is ambiguous)
        methods.add_method(
            "tzToUtc",
            |_,
             this,
             (year, month, day, hours, minutes, secs, all): (
                i32,
                u32,
                u32,
                u32,
                u32,
                u32,
                Option<bool>,
            )| {
                match this
                    .tz
                    .with_ymd_and_hms(year, month, day, hours, minutes, secs)
                {
                    chrono::LocalResult::Single(ymd_hms) => {
                        let ymd_hms = ymd_hms.with_timezone(&chrono_tz::Tz::UTC);
                        Ok((Some(DateTime { dt: ymd_hms }), None))
                    }
                    chrono::LocalResult::Ambiguous(tz, t2) => {
                        if all.unwrap_or(false) {
                            let tz = tz.with_timezone(&chrono_tz::Tz::UTC);
                            let t2 = t2.with_timezone(&chrono_tz::Tz::UTC);
                            Ok((Some(DateTime { dt: tz }), Some(DateTime { dt: t2 })))
                        } else {
                            let tz = tz.with_timezone(&chrono_tz::Tz::UTC);
                            Ok((Some(DateTime { dt: tz }), None))
                        }
                    }
                    chrono::LocalResult::None => {
                        Err(mlua::Error::RuntimeError("Invalid date".to_string()))
                    }
                }
            },
        );

        // Translates a time of the current day in UTC time to a datetime in the said specific timezone
        methods.add_method(
            "timeUtcToTz",
            |_, this, (hours, minutes, secs): (u32, u32, u32)| {
                let now = chrono::Utc::now();
                let now = now
                    .with_hour(hours)
                    .ok_or(mlua::Error::RuntimeError("Invalid time".to_string()))?
                    .with_minute(minutes)
                    .ok_or(mlua::Error::RuntimeError("Invalid time".to_string()))?
                    .with_second(secs)
                    .ok_or(mlua::Error::RuntimeError("Invalid time".to_string()))?
                    .with_timezone(&this.tz);
                Ok(DateTime { dt: now })
            },
        );

        // Translates a time of the current day in the said specific timezone to a datetime in UTC
        methods.add_method(
            "timeTzToUtc",
            |_, this, (hours, minutes, secs): (u32, u32, u32)| {
                let now = this.tz.from_utc_datetime(&chrono::Utc::now().naive_utc());
                let now = now
                    .with_hour(hours)
                    .ok_or(mlua::Error::RuntimeError("Invalid time".to_string()))?
                    .with_minute(minutes)
                    .ok_or(mlua::Error::RuntimeError("Invalid time".to_string()))?
                    .with_second(secs)
                    .ok_or(mlua::Error::RuntimeError("Invalid time".to_string()))?
                    .with_timezone(&chrono_tz::Tz::UTC);

                Ok(DateTime { dt: now })
            },
        );

        // Translates the current timestamp to a datetime in the said specific timezone
        methods.add_method("now", |_, this, (): ()| {
            let now = chrono::Utc::now();
            let now = now.with_timezone(&this.tz);
            Ok(DateTime { dt: now })
        });

        // Given a unix time, returns a DateTime object with this timezone
        methods.add_method("fromTime", |_, this, time: i64| {
            let Some(dt) = chrono::DateTime::from_timestamp(time, 0) else {
                return Err(mlua::Error::RuntimeError(
                    "Invalid time (might exceed bounds?)".to_string(),
                ));
            };
            let dt = dt.with_timezone(&this.tz);
            Ok(DateTime { dt })
        });

        // Given a unix time in milliseconds, returns a DateTime object with this timezone
        methods.add_method("fromTimeMillis", |_, this, time: i64| {
            let Some(dt) = chrono::DateTime::from_timestamp_millis(time) else {
                return Err(mlua::Error::RuntimeError(
                    "Invalid time (might exceed bounds?)".to_string(),
                ));
            };
            let dt = dt.with_timezone(&this.tz);
            Ok(DateTime { dt })
        });

        // Given a unix time in microseconds, returns a DateTime object with this timezone
        methods.add_method("fromTimeMicros", |_, this, time: i64| {
            let Some(dt) = chrono::DateTime::from_timestamp_micros(time) else {
                return Err(mlua::Error::RuntimeError(
                    "Invalid time (might exceed bounds?)".to_string(),
                ));
            };
            let dt = dt.with_timezone(&this.tz);
            Ok(DateTime { dt })
        });

        // Given a unix time in nanoseconds, returns a DateTime object with this timezone
        methods.add_method("fromTimeNanos", |_, this, epoch: i64| {
            let dt = chrono::DateTime::from_timestamp_nanos(epoch);
            let dt = dt.with_timezone(&this.tz);
            Ok(DateTime { dt })
        });

        methods.add_meta_function(LuaMetaMethod::Iter, |lua, ud: LuaAnyUserData| {
            if !ud.is::<Timezone>() {
                return Err(mlua::Error::external("Invalid userdata type"));
            }

            create_userdata_iterator_with_fields(
                lua,
                ud,
                [
                    // Fields
                    // Methods
                    "utcToTz",
                    "tzToUtc",
                    "timeUtcToTz",
                    "timeTzToUtc",
                    "now",
                    "fromTime",
                    "fromTimeMillis",
                    "fromTimeMicros",
                    "fromTimeNanos",
                ],
            )
        });
    }
}

pub fn init_plugin(lua: &Lua) -> LuaResult<LuaTable> {
    let module = lua.create_table()?;

    module.set(
        "new",
        lua.create_function(|_, tz: String| {
            // Map some common timezones automatically
            match tz.as_str() {
                "UTC" => Ok(Timezone { tz: chrono_tz::UTC }),
                "IST" => Ok(Timezone {
                    tz: chrono_tz::Asia::Kolkata, // Most people in India call it IST though...
                }),
                "PST" | "PDT" => Ok(Timezone {
                    tz: chrono_tz::America::Los_Angeles, // Somehow not included in from_str here?
                }),
                "EDT" => Ok(Timezone {
                    tz: chrono_tz::America::New_York, // Many people use EDT
                }),
                _ => {
                    if let Ok(tz) = chrono_tz::Tz::from_str(&tz) {
                        Ok(Timezone { tz })
                    } else {
                        Err(mlua::Error::RuntimeError("Invalid timezone".to_string()))
                    }
                }
            }
        })?,
    )?;

    // The standard UTC timezone
    module.set("UTC", lua.create_userdata(Timezone { tz: chrono_tz::UTC })?)?;

    // Creates a new TimeDelta object
    module.set(
        "timedelta_weeks",
        lua.create_function(|_, weeks: i64| {
            Ok(TimeDelta {
                timedelta: chrono::Duration::try_weeks(weeks).ok_or(mlua::Error::RuntimeError(
                    "Invalid number of weeks".to_string(),
                ))?,
            })
        })?,
    )?;

    module.set(
        "timedelta_days",
        lua.create_function(|_, days: i64| {
            Ok(TimeDelta {
                timedelta: chrono::Duration::try_days(days).ok_or(mlua::Error::RuntimeError(
                    "Invalid number of days".to_string(),
                ))?,
            })
        })?,
    )?;

    module.set(
        "timedelta_hours",
        lua.create_function(|_, hours: i64| {
            Ok(TimeDelta {
                timedelta: chrono::Duration::try_hours(hours).ok_or(mlua::Error::RuntimeError(
                    "Invalid number of hours".to_string(),
                ))?,
            })
        })?,
    )?;

    module.set(
        "timedelta_minutes",
        lua.create_function(|_, minutes: i64| {
            Ok(TimeDelta {
                timedelta: chrono::Duration::try_minutes(minutes).ok_or(
                    mlua::Error::RuntimeError("Invalid number of minutes".to_string()),
                )?,
            })
        })?,
    )?;

    module.set(
        "timedelta_seconds",
        lua.create_function(|_, seconds: i64| {
            Ok(TimeDelta {
                timedelta: chrono::Duration::try_seconds(seconds).ok_or(
                    mlua::Error::RuntimeError("Invalid number of seconds".to_string()),
                )?,
            })
        })?,
    )?;

    module.set(
        "timedelta_millis",
        lua.create_function(|_, millis: i64| {
            Ok(TimeDelta {
                timedelta: chrono::Duration::try_milliseconds(millis).ok_or(
                    mlua::Error::RuntimeError("Invalid number of milliseconds".to_string()),
                )?,
            })
        })?,
    )?;

    module.set(
        "timedelta_micros",
        lua.create_function(|_, micros: i64| {
            Ok(TimeDelta {
                timedelta: chrono::Duration::microseconds(micros),
            })
        })?,
    )?;

    module.set(
        "timedelta_nanos",
        lua.create_function(|_, nanos: i64| {
            Ok(TimeDelta {
                timedelta: chrono::Duration::nanoseconds(nanos),
            })
        })?,
    )?;

    module.set_readonly(true); // Block any attempt to modify this table
    Ok(module)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timezone() {
        let lua = Lua::new();
        let module = init_plugin(&lua).unwrap();

        lua.load(
            r#"
            local tz = ...
            local myTz = tz.new("IST")
            assert(tostring(myTz) == "Asia/Kolkata", "0: Expected Asia/Kolkata, got " .. tostring(myTz))
            assert(myTz:utcToTz(2021, 1, 1, 8, 0, 0):format("%I:%M %p") == "01:30 PM", "0: Basic sanity test failed") -- Should be 1:30 PM IST
            
            -- Should be 1:30 PM IST
            local date = myTz:utcToTz(2021, 1, 1, 8, 0, 0):format("%I%M%S")
            assert(tonumber(date) == 013000, "1: Expected 013000, got " .. date)

            -- The same thing should work with timeUtcToTz
            local time = myTz:timeUtcToTz(8, 0, 0):format("%I%M%S")
            assert(tonumber(time) == 13000, "2: Expected 13000, got " .. time)

            -- Test the other way around (timeUtcToTz)
            local time = myTz:timeTzToUtc(13, 30, 0):format("%I%M%S")
            assert(tonumber(time) == 80000, "3: Expected 80000, got " .. time)

            -- Test the other way around (tzToUtc)
            local date = myTz:tzToUtc(2021, 1, 1, 13, 30, 0):format("%I:%M %p")
            assert(date == "08:00 AM", "4: Expected 08:00 AM, got " .. date)

            -- Test datetime methods
            local date = myTz:utcToTz(2021, 1, 1, 8, 0, 0)
            assert(date.year == 2021, "5: Expected 2021, got " .. date.year)
            assert(date.month == 1, "6: Expected 1, got " .. date.month)
            assert(date.day == 1, "7: Expected 1, got " .. date.day)
            assert(date.hour == 13, "8: Expected 13, got " .. date.hour)
            assert(date.minute == 30, "9: Expected 30, got " .. date.minute)
            assert(date.second == 0, "10: Expected 0, got " .. date.second)
            assert(date.base_offset:offset_string() == "+05:30", "11: Expected +05:30, got " .. date.base_offset:offset_string())
            assert(date.base_offset.seconds == 19800, "11: Expected 19800, got " .. date.base_offset.seconds)
            assert(date.base_offset.millis == 19800000, "12: Expected 19800000, got " .. date.base_offset.millis)
            assert(date.dst_offset.seconds == 0, "12: Expected 0, got " .. date.dst_offset.seconds)
            assert(date.dst_offset.millis == 0, "12: Expected 0, got " .. date.dst_offset.millis)
            assert(date.tz == myTz, "12: Expected myTz, got " .. tostring(date.tz))
            
            -- Make a new timedelta object
            local td = tz.timedelta_seconds(10)
            assert(td.seconds == 10, "13: Expected 10, got " .. td.seconds)
            assert(td.millis == 10000, "14: Expected 10000, got " .. td.millis)
            assert(td.minutes == 0, "15: Expected 0, got " .. td.minutes)
            assert(td.hours == 0, "16: Expected 0, got " .. td.hours)
            assert(td.days == 0, "17: Expected 0, got " .. td.days)
            assert(td:offset_string() == "+00:00", "18: Expected +00:00, got " .. td:offset_string())
            assert(date + td == myTz:utcToTz(2021, 1, 1, 8, 0, 10), "19: Expected 2021-01-01T13:30:10+05:30, got " .. tostring(date + td))
            
            local td2 = tz.timedelta_weeks(10)
            assert(td2.weeks == 10, "20: Expected 10, got " .. td2.weeks)
            local date33 = myTz:utcToTz(2021, 1, 1, 8, 0, 0) + td2
            assert(date33:format("%Y-%m-%dT%H:%M:%S%z") == "2021-03-12T13:30:00+0530", "21: Expected 2021-03-12T13:30:00+0530, got " .. date33:format("%Y-%m-%dT%H:%M:%S%z"))

            -- Test with_timezone
            local newDate = date:with_timezone(tz.new("UTC"))
            assert(newDate:format("%Y-%m-%dT%H:%M:%S%z") == "2021-01-01T08:00:00+0000", "20: Expected 2021-01-01T08:00:00+0000, got " .. newDate:format("%Y-%m-%dT%H:%M:%S%z"))
            local newDateInEST = date:with_timezone(tz.new("EST"))
            -- In EST, its 3:00 AM
            assert(newDateInEST:format("%Y-%m-%dT%H:%M:%S%z") == "2021-01-01T03:00:00-0500", "21: Expected 2021-01-01T03:00:00-0500, got " .. newDateInEST:format("%Y-%m-%dT%H:%M:%S%z"))

            -- Test dst_offset with DST
            local est = tz.new("Europe/London")
            local date = est:utcToTz(2016, 5, 10, 12, 0, 0)
            assert(date.dst_offset.seconds == 3600, "22: Expected 3600, got " .. date.dst_offset.seconds)
        "#,
        )
        .call::<()>(module)
        .unwrap();
    }
}
