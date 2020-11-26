/*!
Date/time CLI utility
*/

use chrono::{TimeZone, Utc};
use chrono_tz::Tz;

fn error(code: i32, msg: &str) {
    eprintln!("ERROR: {}!", msg);
    std::process::exit(code);
}

fn done(msg: &str) {
    println!("{}", msg);
    std::process::exit(0);
}

/**
Process timezone name
*/
fn tz(name: &str) -> Tz {
    if name == "local" {
        if let Ok(local) = iana_time_zone::get_timezone() {
            return tz(&local);
        } else {
            error(5, "Couldn't get local timezone");
        }
    }
    if let Ok(z) = name.parse() {
        return z;
    } else {
        error(3, &format!("Invalid time zone: `{}`", name));
    }
    Tz::UTC
}

/**
Main
*/
fn main() {
    let mut opt_z = false;
    let mut opt_f = false;
    let mut opt_a = false;
    let mut use_fmt = false;
    let mut fmt = String::from("%a %d %b %Y %H:%M:%S %Z");
    let mut zone = Tz::UTC;
    let mut args = vec![];
    for arg in std::env::args().skip(1) {
        let a = arg.as_str();
        if ["-V", "--version"].contains(&a) {
            done(&format!("dtg v{}", env!("CARGO_PKG_VERSION")));
        } else if ["-h", "--help"].contains(&a) {
            done(&format!(
                "\
dtg v{}

{}

```text
dtg [-V|--version] [-h|--help] \\
    [-z TZ] [-f FORMAT] \\
    [-l] [-a] \\
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
\
                ",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_HOMEPAGE"),
            ));
        } else if ["-f", "--format"].contains(&a) {
            opt_f = true;
        } else if opt_f {
            opt_f = false;
            use_fmt = true;
            fmt = arg;
        } else if ["-z", "--zone"].contains(&a) {
            opt_z = true;
        } else if opt_z {
            opt_z = false;
            zone = tz(&arg);
            use_fmt = true;
        } else if arg == "-l" {
            zone = tz("local");
            use_fmt = true;
        } else if arg == "-a" {
            opt_a = true;
            if zone == Tz::UTC {
                zone = tz("local");
            }
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
                    done(&format!(
                        "{}\n{}",
                        d.format("%s.%f%n%Y-%m-%dT%H:%M:%SZ%n%a %d %b %Y %H:%M:%S %Z"),
                        d.with_timezone(&zone).format("%a %d %b %Y %H:%M:%S %Z"),
                    ));
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
