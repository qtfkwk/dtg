/*!
Date/time CLI utility
*/

use chrono::{TimeZone, Utc};
use chrono_tz::Tz;

/**
Main
*/
fn main() {
    fn error(code: i32, msg: &str) {
        eprintln!("ERROR: {}!", msg);
        std::process::exit(code);
    }
    fn done(msg: &str) {
        println!("{}", msg);
        std::process::exit(0);
    }
    let mut opt_f = false;
    let mut fmt = String::from("%a %d %b %Y %H:%M:%S %Z");
    let mut use_fmt = false;
    let mut opt_z = false;
    let mut zone = Tz::UTC;
    let mut opt_a = false;
    let mut args = vec![];
    for arg in std::env::args().skip(1) {
        let a = arg.as_str();
        if ["-V", "--version"].contains(&a) {
            println!("dtg v{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        } else if ["-h", "--help"].contains(&a) {
            println!(
                "\
dtg v{}

{}

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

\
                ",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_HOMEPAGE"),
            );
            std::process::exit(0);
        } else if ["-f", "--format"].contains(&a) {
            opt_f = true;
        } else if opt_f {
            opt_f = false;
            fmt = arg;
            use_fmt = true;
        } else if ["-z", "--zone"].contains(&a) {
            opt_z = true;
        } else if opt_z {
            if let Ok(z) = arg.parse() {
                zone = z;
            } else {
                error(3, &format!("Invalid time zone: `{}`", arg));
            }
            opt_z = false;
            use_fmt = true;
        } else if arg == "-u" {
            zone = Tz::UTC;
            use_fmt = true;
        } else if arg == "-a" {
            opt_a = true;
            fmt = String::from("%a %d %b %Y %H:%M:%S %Z");
            use_fmt = true;
        } else if arg.starts_with('-') {
            error(1, &format!("Invalid option: `{}`", arg));
        } else {
            args.push(arg);
        }
    }
    if opt_z {
        error(3, "Invalid time zone: ``");
    }
    if opt_f {
        error(4, "Invalid format: ``");
    }
    if args.len() == 0 {
        args.push(String::from(""));
    }
    for arg in args.iter() {
        let mut dt = None;
        if arg == "" {
            dt = Some(Utc::now());
        } else {
            let s = arg.split('.').collect::<Vec<&str>>();
            let n = s.len();
            if n == 1 {
                if let Ok(seconds) = s[0].parse::<i64>() {
                    dt = Some(Utc.timestamp(seconds, 0));
                }
            } else if n == 2 {
                if let Ok(seconds) = s[0].parse::<i64>() {
                    let mut ss = s[1].to_string();
                    while ss.len() < 9 {
                        ss.push_str("0");
                    }
                    if let Ok(nanoseconds) = ss.parse::<u32>() {
                        dt = Some(Utc.timestamp(seconds, nanoseconds));
                    }
                }
            }
        }
        match dt {
            Some(d) => {
                if opt_a {
                    println!("{}", d.format("%s.%f%n%Y-%m-%dT%H:%M:%SZ%n%a %d %b %Y %H:%M:%S %Z"));
                }
                let d = d.with_timezone(&zone);
                done(&if use_fmt {
                    format!("{}", d.format(&fmt))
                } else {
                    format!("{}", d.format("%Y-%m-%dT%H:%M:%SZ"))
                });
            }
            None => {
                error(2, &format!("Invalid argument: `{}`", arg));
            }
        }
    }
}
