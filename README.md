# About

Date/time CLI utility

# Usage

```text
dtg [-V|--version] [-h|--help] \
    [-z TZ] [-f FORMAT] \
    [-l] [-a] [-x] \
    [TIMESTAMP]
```

Item              | Description             | Default
------------------|-------------------------|---------------------
`-V`, `--version` | Print version           |
`-h`, `--help`    | Print usage             |
`-z TZ`           | Timezone (1)            | `UTC`
`-l`              | `-z local`              |
`-f FORMAT`       | Format (2)              | `%Y-%m-%dT%H:%M:%SZ`
`-a`              | Custom format (3)       |
`-x`              | Custom format           |
`TIMESTAMP`       | `SECONDS[.NS]`          | *Now*

1. Implies `-f '%a %d %b %Y %H:%M:%S %Z'`
2. Format fields are roughly equivalent to strftime but with some
   enhancements; for details, see:
   https://docs.rs/chrono/latest/chrono/format/strftime#specifiers
3. Equivalent to the following; implies `-l`, override via `-z TZ`

    ```text
    dtg -f '%s.%f'
    dtg -f '%Y-%m-%dT%H:%M:%SZ'
    dtg -f '%a %d %b %Y %H:%M:%S %Z'
    dig -f '%a %d %b %Y %H:%M:%S %Z' -z TZ
    ```

# Examples

Get current date/time in UTC and RFC 3339 format:

```text
$ dtg
2020-11-27T03:21:16Z
```

Get current date/time in the local timezone and default format:

```text
$ dtg -l
Thu 26 Nov 2020 22:21:16 EST
```

Get current date/time in explicit timezone and default format:

```text
$ dtg -z MST
Thu 26 Nov 2020 20:21:16 MST
```

```text
$ dtg -z America/Los_Angeles
Thu 26 Nov 2020 19:21:16 PST
```

Get current date/time in UTC and specific format:

```text
$ dtg -f %A
Friday
```

Get current date/time in local timezone and specific format:

```text
$ dtg -l -f %A
Thursday
```

Get current date/time in explicit timezone and specific format:

```text
$ dtg -z MST -f %A
Thursday
```

Get current date/time in custom format (see note 3 above):

```text
$ dtg -a
1606447276.941324100
2020-11-27T03:21:16Z
Fri 27 Nov 2020 03:21:16 UTC
Thu 26 Nov 2020 22:21:16 EST
```

Get current date/time in explicit timezone and custom format (see note 3 above):

```text
$ dtg -a -z MST
1606447276.941324100
2020-11-27T03:21:16Z
Fri 27 Nov 2020 03:21:16 UTC
Thu 26 Nov 2020 20:21:16 MST
```

Get current date/time in custom format:

```text
$ dtg -x
XeAQ3LG
```

Get a specific date / time in UTC and RFC 3339 format:

```text
$ dtg 1606447276.941324100
2020-11-27T03:21:16Z
```

Get a specific date/time in the local timezone and default format:

```text
$ dtg -l 1606447276.941324100
Thu 26 Nov 2020 22:21:16 EST
```

Get a specific date/time in explicit timezone and default format:

```text
$ dtg -z MST 1606447276.941324100
Thu 26 Nov 2020 20:21:16 MST
```

```text
$ dtg -z America/Los_Angeles 1606447276.941324100
Thu 26 Nov 2020 19:21:16 PST
```

Get specific date/time in UTC and specific format:

```text
$ dtg -f %A 1606447276.941324100
Friday
```

Get specific date/time in local timezone and specific format:

```text
$ dtg -l -f %A 1606447276.941324100
Thursday
```

Get specific date/time in explicit timezone and specific format:

```text
$ dtg -z MST -f %A 1606447276.941324100
Thursday
```

Get a specific date/time in custom format (see note 3 above):

```text
$ dtg -a 1606447276.941324100
1606447276.941324100
2020-11-27T03:21:16Z
Fri 27 Nov 2020 03:21:16 UTC
Thu 26 Nov 2020 22:21:16 EST
```

Get a specific date/time in explicit timezone and custom format (see note 3 above):

```text
$ dtg -a -z MST 1606447276.941324100
1606447276.941324100
2020-11-27T03:21:16Z
Fri 27 Nov 2020 03:21:16 UTC
Thu 26 Nov 2020 20:21:16 MST
```

Get a specific date/time in custom format:

```text
$ dtg -x 1606447276.941324100
XeAQ3LG
```

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

