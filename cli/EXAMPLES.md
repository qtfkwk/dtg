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

