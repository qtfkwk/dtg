/*!
CLI Integration Tests
*/

// # Crates

use assert_cmd::Command;

// # Constants

const SECONDS: &str = "1606314757";
const NANOSECONDS: &str = "191168200";
const RFC3339: &str = "2020-11-25T14:32:37Z";
const UTC: &str = "Wed 25 Nov 2020 14:32:37 UTC";
const EST: &str = "Wed 25 Nov 2020 09:32:37 EST";
const MONTH: &str = "November";
const DOW: &str = "Wednesday";
const X: &str = "XeAOEWb";

// # Helper functions

/**
Retrieve the binary to test
*/
pub fn cmd(bin: &str) -> Command {
    Command::cargo_bin(bin).unwrap()
}

/**
Print the command
*/
fn p(bin: &str, args: &[&str]) {
    println!(
        "{} {}",
        bin,
        args.iter()
            .map(|x| {
                if x.contains(' ') {
                    format!("\"{}\"", x)
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    );
}

/**
Run command that fails
*/
fn fail(bin: &str, args: &[&str], code: i32, msg: &str) {
    p(bin, args);
    cmd(bin)
        .args(args)
        .assert()
        .failure()
        .code(code)
        .stderr(format!("ERROR: {}!\n", msg));
}

/**
Run command that succeeds
*/
fn pass(bin: &str, args: &[&str], want: &str) {
    p(bin, args);
    cmd(bin)
        .args(args)
        .assert()
        .success()
        .stdout(format!("{}\n", want));
}

/**
Compose timestamp
*/
fn nanoseconds() -> String {
    format!("{}.{}", SECONDS, NANOSECONDS)
}

// # Tests

#[test]
fn version() {
    for i in ["-V", "--version"].iter() {
        pass("dtg", &[i], &format!("dtg v{}", env!("CARGO_PKG_VERSION")));
    }
}

#[test]
fn help() {
    for i in ["-h", "--help"].iter() {
        pass(
            "dtg",
            &[i],
            &format!(
                "\
dtg v{}

{}

```text
dtg [-V|--version] [-h|--help] \\
    [-z TZ] [-f FORMAT] \\
    [-l] [-a] [-x] \\
    [TIMESTAMP]
```

Item              | Description       | Default
------------------|-------------------|---------------------
`-V`, `--version` | Print version     |
`-h`, `--help`    | Print usage       |
`-z TZ`           | Timezone (1)      | `UTC`
`-l`              | `-z local`        |
`-f FORMAT`       | Format (2)        | `%Y-%m-%dT%H:%M:%SZ`
`-a`              | Custom format (3) |
`-x`              | Custom format (4) |
`TIMESTAMP`       | `SECONDS[.NS]`    | *Now*

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

4. Compact format using base 60 (0-9, A-Z, a-x) for 2 character
   full year and 1 character each for month, day, hour, minute,
   and second.
\
            ",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_HOMEPAGE"),
            ),
        );
    }
}

#[test]
fn epoch_seconds() {
    pass("dtg", &[SECONDS], RFC3339);
}

#[test]
fn epoch_subsecond() {
    pass(
        "dtg",
        &["-f", "%Y-%m-%dT%H:%M:%S.%fZ", &nanoseconds()],
        &RFC3339.replace('Z', &format!(".{}Z", NANOSECONDS)),
    );
}

#[test]
fn zone_utc() {
    for i in ["-z", "--zone"].iter() {
        pass("dtg", &[i, "UTC", &nanoseconds()], UTC);
    }
}

#[test]
fn zone_est() {
    for i in ["-z", "--zone"].iter() {
        pass("dtg", &[i, "EST", &nanoseconds()], EST);
    }
}

#[test]
fn format_custom_day_of_week() {
    let ns = nanoseconds();
    for i in ["-f", "--format"].iter() {
        pass("dtg", &[i, "%A", &ns], DOW);
    }
}

#[test]
fn format_a() {
    let want = format!("{}.{}\n{}\n{}\n{}", SECONDS, NANOSECONDS, RFC3339, UTC, EST);
    let ns = nanoseconds();
    pass("dtg", &["-a", "-z", "EST", &ns], &want);
    pass("dtg", &["-z", "EST", "-a", &ns], &want);
    pass("dtg", &["-az", "EST", &ns], &want);
}

#[test]
fn format_x() {
    let ns = nanoseconds();
    pass("dtg", &["-x", &ns], X);
}

#[test]
fn format_ax() {
    let want = format!("{}.{}\n{}\n{}\n{}\n{}", SECONDS, NANOSECONDS, RFC3339, UTC, EST, X);
    let ns = nanoseconds();
    pass("dtg", &["-a", "-x", "-z", "EST", &ns], &want);
    pass("dtg", &["-a", "-z", "EST", "-x", &ns], &want);
    pass("dtg", &["-z", "EST", "-a", "-x", &ns], &want);
    pass("dtg", &["-ax", "-z", "EST", &ns], &want);
    pass("dtg", &["-z", "EST", "-ax", &ns], &want);
    pass("dtg", &["-axz", "EST", &ns], &want);
}

#[test]
fn format_xz() {
    let want = format!("{}\n{}.{}\n{}\n{}\n{}", X, SECONDS, NANOSECONDS, RFC3339, UTC, EST);
    let ns = nanoseconds();
    pass("dtg", &["-x", "-a", "-z", "EST", &ns], &want);
    pass("dtg", &["-x", "-z", "EST", "-a", &ns], &want);
    pass("dtg", &["-z", "EST", "-x", "-a", &ns], &want);
    pass("dtg", &["-xa", "-z", "EST", &ns], &want);
    pass("dtg", &["-z", "EST", "-xa", &ns], &want);
    pass("dtg", &["-xaz", "EST", &ns], &want);
}

#[test]
fn mulitple_f_options() {
    let want = format!("{}\n{}", MONTH, DOW);
    let ns = nanoseconds();
    pass("dtg", &["-f", "%B", "-f", "%A", "-z", "EST", &ns], &want);
    pass("dtg", &["-f", "%B", "-z", "EST", "-f", "%A", &ns], &want);
    pass("dtg", &["-z", "EST", "-f", "%B", "-f", "%A", &ns], &want);
}

// ## Errors

#[test]
fn invalid_option() {
    fail("dtg", &["-q"], 1, "Invalid option: `-q`");
}

#[test]
fn invalid_argument() {
    fail("dtg", &["blah"], 2, "Invalid argument: `blah`");
}

#[test]
fn invalid_time_zone_1() {
    fail("dtg", &["-z"], 3, "Invalid time zone: ``");
}

#[test]
fn invalid_time_zone_2() {
    fail("dtg", &["-z", "Z"], 3, "Invalid time zone: `Z`");
}

#[test]
fn invalid_format() {
    fail("dtg", &["-f"], 4, "Invalid format: ``");
}
