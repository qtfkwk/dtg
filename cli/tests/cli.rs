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
const CST: &str = "Wed 25 Nov 2020 08:32:37 CST";
const MST: &str = "Wed 25 Nov 2020 07:32:37 MST";
const PST: &str = "Wed 25 Nov 2020 06:32:37 PST";
const MONTH: &str = "November";
const DOW: &str = "Wednesday";
const X: &str = "XeAOEWb";
const MAX: &str = "+262143-12-31T23:59:59Z";
const MAX_SECONDS: &str = "8210298412799";
const MAX_X: &str = "1Cn3BUNxx";
const OVERFLOW_SECONDS: &str = "8210298412800";
const OVERFLOW_X: &str = "1Cn400000";

// # Helper functions

/// Retrieve the binary to test
pub fn cmd(bin: &str) -> Command {
    Command::cargo_bin(bin).unwrap()
}

/// Print the command
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

/// Run command that fails
fn fail(bin: &str, args: &[&str], code: i32, msg: &str) {
    p(bin, args);
    cmd(bin)
        .args(args)
        .assert()
        .failure()
        .code(code)
        .stderr(format!("ERROR: {}!\n", msg));
}

/// Run command that succeeds
fn pass(bin: &str, args: &[&str], want: &str) {
    p(bin, args);
    cmd(bin)
        .args(args)
        .assert()
        .success()
        .stdout(format!("{}\n", want));
}

/// Compose timestamp
fn nanoseconds() -> String {
    format!("{}.{}", SECONDS, NANOSECONDS)
}

// # Tests

#[test]
fn version() {
    for i in ["-V", "--version"].iter() {
        pass("dtg", &[i], &format!("dtg {}", env!("CARGO_PKG_VERSION")));
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
        &["-f", "%Y-%m-%dT%H:%M:%S.%fZ", "--", &nanoseconds()],
        &RFC3339.replace('Z', &format!(".{}Z", NANOSECONDS)),
    );
}

#[test]
fn zone_utc() {
    pass("dtg", &["-z", "UTC", &nanoseconds()], UTC);
}

#[test]
fn zone_est() {
    pass("dtg", &["-z", "EST5EDT", &nanoseconds()], EST);
}

#[test]
fn zone_cst() {
    pass("dtg", &["-z", "CST6CDT", &nanoseconds()], CST);
}

#[test]
fn zone_mst() {
    pass("dtg", &["-z", "MST7MDT", &nanoseconds()], MST);
}

#[test]
fn zone_pst() {
    pass("dtg", &["-z", "PST8PDT", &nanoseconds()], PST);
}

#[test]
fn zone_multi() {
    pass(
        "dtg",
        &["-z", "UTC,EST5EDT,CST6CDT,MST7MDT,PST8PDT", &nanoseconds()],
        &[UTC, EST, CST, MST, PST].join("\n"),
    );
}

#[test]
fn format_custom_day_of_week() {
    let ns = nanoseconds();
    pass("dtg", &["-f", "%A", "--", &ns], DOW);
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
    pass("dtg", &["-X", X], RFC3339);
    pass("dtg", &["-X", "-f", "%s", "--", X], SECONDS);
    pass(
        "dtg",
        &["-X", "-f", "%a %d %b %Y %H:%M:%S %Z", "--", X],
        UTC,
    );
    pass(
        "dtg",
        &["-X", "-f", "%a %d %b %Y %H:%M:%S %Z", "-z", "EST", X],
        EST,
    );
}

#[test]
fn format_ax() {
    let want = format!(
        "{}.{}\n{}\n{}\n{}\n{}",
        SECONDS, NANOSECONDS, RFC3339, UTC, EST, X
    );
    let ns = nanoseconds();
    pass("dtg", &["-a", "-x", "-z", "EST", &ns], &want);
    pass("dtg", &["-a", "-z", "EST", "-x", &ns], &want);
    pass("dtg", &["-z", "EST", "-a", "-x", &ns], &want);
    pass("dtg", &["-ax", "-z", "EST", &ns], &want);
    pass("dtg", &["-z", "EST", "-ax", &ns], &want);
    pass("dtg", &["-axz", "EST", &ns], &want);
}

#[test]
fn mulitple_f_options() {
    let want = format!("{}\n{}", MONTH, DOW);
    let ns = nanoseconds();
    pass("dtg", &["-f", "%B", "-f", "%A", "-z", "EST", &ns], &want);
    pass(
        "dtg",
        &["-f", "%B", "-z", "EST", "-f", "%A", "--", &ns],
        &want,
    );
    pass(
        "dtg",
        &["-z", "EST", "-f", "%B", "-f", "%A", "--", &ns],
        &want,
    );
}

#[test]
fn timezone_list() {
    cmd("dtg").args(&["-Z"]).assert().success();
}

#[test]
fn timezone_search() {
    pass(
        "dtg",
        &["-Z", "new_"],
        "America/New_York\nAmerica/North_Dakota/New_Salem",
    );
}

#[test]
fn max_seconds() {
    pass("dtg", &[MAX_SECONDS], MAX);
}

#[test]
fn max_x() {
    pass("dtg", &["-X", MAX_X], MAX);
}

// ## Errors

#[test]
fn timezone_search_found_zero() {
    fail(
        "dtg",
        &["-Z", "blah"],
        1,
        "Zero timezones found matching `blah`",
    );
}

#[test]
fn invalid_argument() {
    fail("dtg", &["blah"], 2, "Invalid timestamp: `blah`");
}

#[test]
fn invalid_time_zone() {
    fail("dtg", &["-z", "Z"], 3, "Invalid timezone: `Z`");
}

#[test]
fn overflow_seconds() {
    fail(
        "dtg",
        &[OVERFLOW_SECONDS],
        2,
        &format!("Invalid timestamp: `{}`", OVERFLOW_SECONDS),
    );
}

#[test]
fn overflow_x() {
    fail(
        "dtg",
        &["-X", OVERFLOW_X],
        2,
        &format!("Invalid timestamp: `{}`", OVERFLOW_X),
    );
}

#[test]
fn separator() {
    let sep = " | ";
    pass(
        "dtg",
        &[
            "-z",
            "UTC,EST5EDT,CST6CDT,MST7MDT,PST8PDT",
            "-s",
            sep,
            &nanoseconds(),
        ],
        &[UTC, EST, CST, MST, PST].join(sep),
    );
}
