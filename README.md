# About

Date/time CLI utility

# Usage

```
dtg [-V|--version] [-h|--help] [-z TZ] [-f FORMAT] [TIMESTAMP]
```

Item              | Description               | Default
------------------|---------------------------|--------------------------
`-V`, `--version` | Print banner with version |
`-h`, `--help`    | Print usage               |
`-z TZ`           | Timezone[^1]              | `UTC`
`-f FORMAT`       | Strftime format[^2]       | `%Y-%m-%dT%H:%M:%SZ`[^3]
`-a`              | Use custom format[^4]     |
`TIMESTAMP`       | Timestamp `SECONDS[.NS]`  | *now*

[^1]: Implies `-f '%a %d %b %Y %H:%M:%S %Z'`

[^2]: https://docs.rs/chrono/latest/chrono/format/strftime#specifiers

[^3]: See note 1

[^4]: Similar to `%s.%f%n%Y-%m-%dT%H:%M:%SZ%n%a %d %b %Y %H:%M:%S %Z`
      except the last line is repeated for the given timezone and the
      top three lines are in UTC
