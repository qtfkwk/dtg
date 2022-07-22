# About

Date/time CLI utility

# Usage

~~~text
$ dtg -h
~~~

# Examples

Get current date/time in UTC and RFC 3339 format:

```text
$ dtg
```

Get current date/time in the local timezone and default format:

```text
$ dtg -l
```

Get current date/time in explicit timezone and default format:

```text
$ dtg -z MST
```

```text
$ dtg -z America/Los_Angeles
```

Get current date/time in UTC and specific format:

```text
$ dtg -f %A
```

```text
$ dtg -f %s.%f
```

Get current date/time in local timezone and specific format:

```text
$ dtg -l -f %A
```

Get current date/time in explicit timezone and specific format:

```text
$ dtg -z MST -f %A
```

Get current date/time in "a" format:

```text
$ dtg -a
```

Get current date/time in explicit timezone and "a" format:

```text
$ dtg -a -z MST
```

Get current date/time in "x" format:

```text
$ dtg -x
```

Get a specific date / time in UTC and RFC 3339 format:

```text
$ dtg 1606447276.941324100
```

Get a specific date/time in the local timezone and default format:

```text
$ dtg -l 1606447276.941324100
```

Get a specific date/time in explicit timezone and default format:

```text
$ dtg -z MST 1606447276.941324100
```

```text
$ dtg -z America/Los_Angeles 1606447276.941324100
```

Get specific date/time in UTC and specific format:

```text
$ dtg -f %A 1606447276.941324100
```

Get specific date/time in local timezone and specific format:

```text
$ dtg -l -f %A -- 1606447276.941324100
```

Get specific date/time in explicit timezone and specific format:

```text
$ dtg -z MST -f %A -- 1606447276.941324100
```

Get a specific date/time in "a" format:

```text
$ dtg -a 1606447276.941324100
```

Get a specific date/time in explicit timezone and "a" format:

```text
$ dtg -a -z MST 1606447276.941324100
```

Get a specific date/time in "x" format:

```text
$ dtg -x 1606447276.941324100
```

Get a specific date / time from "x" format in RFC 3339 format:

```text
$ dtg -X XeAQ3LG
```

Get a specific date / time from "x" format in the local timezone and default format:

```text
$ dtg -X -l XeAQ3LG
```

Get a specific date / time from "x" format in explicit timezone and default format:

```text
$ dtg -X -z MST XeAQ3LG
```

Get a specific date / time from "x" format in specific format:

```text
$ dtg -f '%a %d %b %Y %H:%M:%S %Z' -X XeAQ3LG
```

Get a specific date / time from "x" format in local timezone and specific
format:

```text
$ dtg -l -f '%a %d %b %Y %H:%M:%S %Z' -X XeAQ3LG
```

Get a specific date / time from "x" format in explicit timezone and specific
format:

```text
$ dtg -z MST -f '%a %d %b %Y %H:%M:%S %Z' -X XeAQ3LG
```

List available time zones:

```text
$ dtg -Z
```

Search for a timezone:

```text
$ dtg -Z ca/w
```

Multiple timezones:

```text
$ dtg -z UTC,EST5EDT,CST6CDT,MST7MDT,PST8PDT -f '%Z%n%H:%M:%S%n' -f '%Z%n%Y-%m-%d%n'
```

*Note the above prints each format for each timezone... to print each timezone for each format,
use a single format and `%n`:*

```text
$ dtg -z UTC,EST5EDT,CST6CDT,MST7MDT,PST8PDT -f '%Z%n%H:%M:%S%n%n%Z%n%Y-%m-%d%n'
```

Use a custom separator between formats/timezones:

```text
$ dtg -z PST8PDT,MST7MDT,CST6CDT,EST5EDT,UTC -f '[%Z %H:%M:%S]' -s ' '
```

# Formats

The following information originates from the [chrono documentation], which `dtg` uses internally.

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

# Changelog

* 1.0.2: Initial public release
* 2.0.0
    * General improvements to readme and code
    * Replaced `-u` with `-l` and ability to get local timezone via the
      [iana-time-zone crate](https://crates.io/crates/iana-time-zone)
* 2.0.1: Improve readme (add `-f` examples...); `s/-r/-l/` in usage
* 2.1.0: Enable bundling options (`-a -z TZ`: `-az TZ`, `-a -l`: `-al`, `-l -a`: `-la`,
  `-l -f FORMAT`: `-lf FORMAT`)
* 2.2.0: Add `-x` option
* 2.2.1: Fix `-x` option year from hex to base 60
* 2.2.2: Update doc; allow `-ax`, `-axz`, `-xa`, `-xaz`
* 2.2.3: Enable multiple `-f` options
* 2.2.4: Add `-X` option
* 3.0.0: Rewrite with structopt; add `-Z`
* 3.1.0: Add `--readme`
* 3.2.0: Improve doc
* 3.2.1: Fix `-a` option (yanked)
* 3.2.2: Fix readme
* 3.3.0: Improve doc; upgrade dependencies
* 3.3.1: Fix tables in readme
* 3.4.0: Catch overflows; upgrade dependencies
* 3.5.0: Multiple timezones; cargo fmt; upgrade dependencies
* 3.6.0: Separator `-s` option
* 3.7.0: Interval `-i`, `-c` options
* 4.0.0
    * Redesign as lib and cli crates in a workspace
    * Replace structopt with clap v3 derive API

[chrono documentation]: https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers

