# About

Date/time library

*See also the [API documentation] and [dtg] crate.*

[API documentation]: https://docs.rs/dtg-lib
[dtg]: https://crates.io/crates/dtg

# Examples

```Rust
use chrono::{TimeZone, Utc};
use dtg_lib::{tz, Dtg, Format};

let epoch = 1658448142;
let nanoseconds = 936196858;
let rfc_3339 = "2022-07-22T00:02:22Z";
let default_utc = "Fri 22 Jul 2022 00:02:22 UTC";
let default_mt = "Thu 21 Jul 2022 18:02:22 MDT";
let x = "Xg6L02M";
let a_utc = format!("{epoch}.000000000\n{rfc_3339}\n{default_utc}\n{default_utc}");
let a_mt = format!("{epoch}.000000000\n{rfc_3339}\n{default_utc}\n{default_mt}");
let day_of_week_utc = "Friday";
let day_of_week_mt = "Thursday";
let tz_utc = tz("UTC").ok();
let tz_mt = tz("MST7MDT").ok();
let default_fmt = Some(Format::default());
let day_of_week_fmt = Some(Format::custom("%A"));

// Create Dtg

let dtg_1_str = format!("{}", epoch);

let dtg_1_ts = Dtg::from(&dtg_1_str).unwrap();
let dtg_1_dt = Dtg::from_dt(&Utc.timestamp(epoch, 0));
let dtg_1_x = Dtg::from_x(x).unwrap();

assert_eq!(dtg_1_ts, dtg_1_dt);
assert_eq!(dtg_1_dt, dtg_1_x);
assert_eq!(dtg_1_x, dtg_1_ts);

// Create Dtg with nanoseconds

let dtg_2_str = format!("{}.{}", epoch, nanoseconds);

let dtg_2_ts = Dtg::from(&dtg_2_str).unwrap();
let dtg_2_dt = Dtg::from_dt(&Utc.timestamp(epoch, nanoseconds));

assert_eq!(dtg_2_ts, dtg_2_dt);

// Default format

assert_eq!(dtg_1_ts.default(&None), default_utc);
assert_eq!(dtg_1_ts.default(&tz_utc), default_utc);
assert_eq!(dtg_1_ts.default(&tz_mt), default_mt);

assert_eq!(dtg_1_ts.format(&default_fmt, &None), default_utc);
assert_eq!(dtg_1_ts.format(&default_fmt, &tz_utc), default_utc);
assert_eq!(dtg_1_ts.format(&default_fmt, &tz_mt), default_mt);

// RFC 3339 format

assert_eq!(dtg_1_ts.rfc_3339(), rfc_3339);
assert_eq!(dtg_1_ts.format(&None, &None), rfc_3339);

// "x" format

assert_eq!(dtg_1_ts.x_format(), x);
assert_eq!(dtg_1_ts.format(&Some(Format::X), &None), x);

// "a" format

assert_eq!(dtg_1_ts.a_format(&None), a_utc);
assert_eq!(dtg_1_ts.a_format(&tz_utc), a_utc);
assert_eq!(dtg_1_ts.a_format(&tz_mt), a_mt);

assert_eq!(dtg_1_ts.format(&Some(Format::A), &None), a_utc);
assert_eq!(dtg_1_ts.format(&Some(Format::A), &tz_utc), a_utc);
assert_eq!(dtg_1_ts.format(&Some(Format::A), &tz_mt), a_mt);

// Custom format

assert_eq!(dtg_1_ts.format(&day_of_week_fmt, &None), day_of_week_utc);
assert_eq!(dtg_1_ts.format(&day_of_week_fmt, &tz_mt), day_of_week_mt);
```

# Formats

## Date specifiers

Spec. | Example       | Description
------|---------------|----------------------------------------------------------------------------
`%Y`  | `2001`        | The full proleptic Gregorian year, zero-padded to 4 digits.
`%C`  | `20`          | The proleptic Gregorian year divided by 100, zero-padded to 2 digits.
`%y`  | `01`          | The proleptic Gregorian year modulo 100, zero-padded to 2 digits.
`%m`  | `07`          | Month number (01--12), zero-padded to 2 digits.
`%b`  | `Jul`         | Abbreviated month name. Always 3 letters.
`%B`  | `July`        | Full month name. Also accepts corresponding abbreviation in parsing.
`%h`  | `Jul`         | Same as `%b`.
`%d`  | `08`          | Day number (01--31), zero-padded to 2 digits.
`%e`  | ` 8`          | Same as `%d` but space-padded. Same as `%_d`.
`%a`  | `Sun`         | Abbreviated weekday name. Always 3 letters.
`%A`  | `Sunday`      | Full weekday name. Also accepts corresponding abbreviation in parsing.
`%w`  | `0`           | Sunday = 0, Monday = 1, ..., Saturday = 6.
`%u`  | `7`           | Monday = 1, Tuesday = 2, ..., Sunday = 7. (ISO 8601)
`%U`  | `28`          | Week number starting with Sunday (00--53), zero-padded to 2 digits.
`%W`  | `27`          | Same as `%U`, but week 1 starts with the first Monday in that year instead.
`%G`  | `2001`        | Same as `%Y` but uses the year number in ISO 8601 week date.
`%g`  | `01`          | Same as `%y` but uses the year number in ISO 8601 week date.
`%V`  | `27`          | Same as `%U` but uses the week number in ISO 8601 week date (01--53).
`%j`  | `189`         | Day of the year (001--366), zero-padded to 3 digits.
`%D`  | `07/08/01`    | Month-day-year format. Same as `%m/%d/%y`.
`%x`  | `07/08/01`    | Locale's date representation (e.g., 12/31/99).
`%F`  | `2001-07-08`  | Year-month-day format (ISO 8601). Same as `%Y-%m-%d`.
`%v`  | ` 8-Jul-2001` | Day-month-year format. Same as `%e-%b-%Y`.

## Time specifiers

Spec.  | Example       | Description
-------|---------------|----------------------------------------------------------------------
`%H`   | `00`          | Hour number (00--23), zero-padded to 2 digits.
`%k`   | ` 0`          | Same as `%H` but space-padded. Same as `%_H`.
`%I`   | `12`          | Hour number in 12-hour clocks (01--12), zero-padded to 2 digits.
`%l`   | `12`          | Same as `%I` but space-padded. Same as `%_I`.
`%P`   | `am`          | `am` or `pm` in 12-hour clocks.
`%p`   | `AM`          | `AM` or `PM` in 12-hour clocks.
`%M`   | `34`          | Minute number (00--59), zero-padded to 2 digits.
`%S`   | `60`          | Second number (00--60), zero-padded to 2 digits.
`%f`   | `026490000`   | The fractional seconds (in nanoseconds) since last whole second.
`%.f`  | `.026490`     | Similar to `.%f` but left-aligned. These all consume the leading dot.
`%.3f` | `.026`        | Similar to `.%f` but left-aligned but fixed to a length of 3.
`%.6f` | `.026490`     | Similar to `.%f` but left-aligned but fixed to a length of 6.
`%.9f` | `.026490000`  | Similar to `.%f` but left-aligned but fixed to a length of 9.
`%3f`  | `026`         | Similar to `%.3f` but without the leading dot.
`%6f`  | `026490`      | Similar to `%.6f` but without the leading dot.
`%9f`  | `026490000`   | Similar to `%.9f` but without the leading dot.
`%R`   | `00:34`       | Hour-minute format. Same as `%H:%M`.
`%T`   | `00:34:60`    | Hour-minute-second format. Same as `%H:%M:%S`.
`%X`   | `00:34:60`    | Locale's time representation (e.g., 23:13:48).
`%r`   | `12:34:60 AM` | Hour-minute-second format in 12-hour clocks. Same as `%I:%M:%S %p`.

## Time zone specifiers

Spec. | Example  | Description
------|----------|--------------------------------------------------------------------------
`%Z`  | `ACST`   | Local time zone name. Skips all non-whitespace characters during parsing.
`%z`  | `+0930`  | Offset from the local time to UTC (with UTC being `+0000`).
`%:z` | `+09:30` | Same as `%z` but with a colon.
`%#z` | `+09`    | *Parsing only:* Same as `%z` but allows minutes to be missing or present.

## Date & time specifiers

Spec. | Example                            | Description
------|------------------------------------|------------------------------------------------------------------
`%c`  | `Sun Jul  8 00:34:60 2001`         | Locale's date and time (e.g., Thu Mar  3 23:05:25 2005).
`%+`  | `2001-07-08T00:34:60.026490+09:30` | ISO 8601 / RFC 3339 date & time format.
`%s`  | `994518299`                        | UNIX timestamp, the number of seconds since 1970-01-01 00:00 UTC.

## Special specifiers

Spec. | Description
------|------------------------
`%t`  | Literal tab (`\t`).
`%n`  | Literal newline (`\n`).
`%%`  | Literal percent sign.

