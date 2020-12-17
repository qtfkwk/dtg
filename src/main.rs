/*!
Date/time CLI utility
*/

use std::collections::HashMap;

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
    let mut opt_x = false;
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
    [-l] [-a] [-x] \\
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
        } else if arg == "-lf" {
            opt_f = true;
            zone = tz("local");
            use_fmt = true;
        } else if ["-al", "-la"].contains(&a) {
            opt_a = true;
            zone = tz("local");
            use_fmt = true;
        } else if arg == "-a" {
            opt_a = true;
            if zone == Tz::UTC {
                zone = tz("local");
            }
        } else if arg == "-az" {
            opt_a = true;
            opt_z = true;
            if zone == Tz::UTC {
                zone = tz("local");
            }
        } else if arg == "-x" {
            opt_x = true;
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
                if opt_x {
                    let c: HashMap<u8, char> = [
                        (0, '0'), (10, 'A'), (20, 'K'), (30, 'U'), (40, 'e'), (50, 'o'),
                        (1, '1'), (11, 'B'), (21, 'L'), (31, 'V'), (41, 'f'), (51, 'p'),
                        (2, '2'), (12, 'C'), (22, 'M'), (32, 'W'), (42, 'g'), (52, 'q'),
                        (3, '3'), (13, 'D'), (23, 'N'), (33, 'X'), (43, 'h'), (53, 'r'),
                        (4, '4'), (14, 'E'), (24, 'O'), (34, 'Y'), (44, 'i'), (54, 's'),
                        (5, '5'), (15, 'F'), (25, 'P'), (35, 'Z'), (45, 'j'), (55, 't'),
                        (6, '6'), (16, 'G'), (26, 'Q'), (36, 'a'), (46, 'k'), (56, 'u'),
                        (7, '7'), (17, 'H'), (27, 'R'), (37, 'b'), (47, 'l'), (57, 'v'),
                        (8, '8'), (18, 'I'), (28, 'S'), (38, 'c'), (48, 'm'), (58, 'w'),
                        (9, '9'), (19, 'J'), (29, 'T'), (39, 'd'), (49, 'n'), (59, 'x'),
                    ].iter().cloned().collect();
                    let year = d.format("%Y").to_string().parse::<u32>().unwrap();
                    let mut mon = d.format("%m").to_string().parse::<u8>().unwrap();
                    let mut day = d.format("%d").to_string().parse::<u8>().unwrap();
                    let h = d.format("%H").to_string().parse::<u8>().unwrap();
                    let m = d.format("%M").to_string().parse::<u8>().unwrap();
                    let s = d.format("%S").to_string().parse::<u8>().unwrap();
                    mon -= 1;
                    day -= 1;
                    let mon = c.get(&mon).unwrap();
                    let day = c.get(&day).unwrap();
                    let h = c.get(&h).unwrap();
                    let m = c.get(&m).unwrap();
                    let s = c.get(&s).unwrap();
                    done(&format!("{:03X}{}{}{}{}{}", year, mon, day, h, m, s));
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
