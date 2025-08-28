#![doc = include_str!("../README.md")]

use clap::{Parser, builder::Styles};
use dtg_lib::{Dtg, Format, tz};
use jiff::tz::TimeZone;

#[cfg(unix)]
use pager::Pager;

const STYLES: Styles = Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);

fn error(code: i32, msg: &str) {
    eprintln!("ERROR: {msg}!");
    std::process::exit(code);
}

#[derive(Parser)]
#[command(
    about = "\
Date/time CLI utility

<https://crates.io/crates/dtg> / <https://github.com/qtfkwk/dtg>

---\
    ",
    version,
    max_term_width = 80,
    styles = STYLES,
    after_help = "\
---

Notes:

1. \"a\" format:

   ```text
   %s.%f
   %Y-%m-%dT%H:%M:%SZ
   %a %d %b %Y %H:%M:%S %Z # UTC
   %a %d %b %Y %H:%M:%S %Z # Specified or local timezone
   ```

2. \"x\" format (novel UTC / base 60 encoding):

   ```text
   0* 0 1 2 3 4 5 6 7 8 9
   1* A B C D E F G H I J
   2* K L M N O P Q R S T
   3* U V W X Y Z a b c d
   4* e f g h i j k l m n
   5* o p q r s t u v w x
   ```

   Field  | Values           | Result
   -------|------------------|----------
   Year   | 2020 => 33*60+40 | Xe
   Month  | Jan-Dec => 0-11  | 0-B
   Day    | 0-27/28/29/30    | 0-R/S/T/U
   Hour   | 0-23             | 0-N
   Minute | 0-59             | 0-x
   Second | 0-59             | 0-x

3. Prints the timestamp in each format with one or more timezones using a
   comma-separated string (`-z UTC,EST`).

4. The `-f`, `-a`, and `-x` options are processed *in that order* and do not
   enable any reordering, however the `-n` option processes its arguments in the
   order given and handles custom, \"a\", \"x\", and named formats.

5. \"bcd\" format: year, month, day, hour, minute, and second displayed like a
   binary clock with the Braille Patterns Unicode Block and `|` separators.

6. `-l` / `-z` are ignored when processing UTC-only formats like `-n rfc-3339`.
\
    ",
)]
struct Cli {
    /// Local timezone (6)
    #[arg(short)]
    local_zone: bool,

    /// "a" format (1)
    #[arg(short)]
    a_format: bool,

    /// "x" format (2)
    #[arg(short)]
    x_format: bool,

    /// Give timestamp argument(s) in "x" format (2)
    #[arg(short = 'X')]
    from_x: bool,

    /// Search/list timezones
    #[arg(short = 'Z')]
    list_zones: bool,

    /// Format(s) [-z/-l: "%a %d %b %Y %H:%M:%S %Z", "%Y-%m-%dT%H:%M:%SZ"]
    #[arg(short, value_name = "FORMAT")]
    formats: Vec<String>,

    /// Timezone(s) [default: UTC] (3) (6)
    #[arg(short)]
    zone: Option<String>,

    /// Separator [default: "\n"]
    #[arg(short)]
    separator: Option<String>,

    /// Named format(s) [all, bcd, compact-date (%Y%m%d), compact-date-time (%Y%m%d-%H%M%S),
    /// compact-time (%H%M%S), default, rfc-3339, x, or any custom format] (4) (5)
    #[arg(short, value_name = "NAME")]
    named_formats: Vec<String>,

    /// Run every N seconds
    #[arg(short, value_name = "N")]
    interval: Option<f32>,

    /// Clear and run every N seconds
    #[arg(short, value_name = "N")]
    clear: Option<f32>,

    /// Print the readme
    #[arg(short, long)]
    readme: bool,

    /// Argument [-X: "x" timestamp (2), -Z: timezone, "%s.%f" timestamp, default: now]
    #[arg(name = "ARG")]
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.readme {
        #[cfg(unix)]
        Pager::with_pager("bat -pl md").setup();

        print!("{}", include_str!("../README.md"));
        return;
    }

    if cli.list_zones {
        let mut found = 0;
        let zones = jiff::tz::db().available().filter(|x| {
            !x.to_string().starts_with("right/") && !["Factory", "posixrules"].contains(&x.as_str())
        });
        if cli.args.is_empty() {
            for zone in zones {
                println!("{zone}");
                found += 1;
            }
        } else {
            let search = &cli.args[0];
            let search_lc = search.to_lowercase();
            for zone in zones {
                let name = zone.to_string().to_lowercase();
                if name.contains(&search_lc) {
                    println!("{zone}");
                    found += 1;
                }
            }
            if found == 0 {
                error(1, &format!("Zero timezones found matching `{search}`"));
            }
        }
        return;
    }

    let clear = cli.clear.is_some();
    if cli.interval.is_some() && clear {
        error(6, "Options `-i` and `-c` are mutually exclusive");
        return;
    }
    let interval = match cli.interval {
        Some(f) => Some(std::time::Duration::from_secs_f32(f)),
        None => cli.clear.map(std::time::Duration::from_secs_f32),
    };

    let separator = match cli.separator {
        Some(s) => match s.as_str() {
            "\\n" => String::from("\n"),
            "\\t" => String::from("\t"),
            _ => s.to_string(),
        },
        None => String::from("\n"),
    };

    let mut formats = vec![];
    for i in cli.formats.iter() {
        formats.push(Format::Custom(i.to_string()));
    }
    if cli.a_format {
        formats.push(Format::A);
    }
    if cli.x_format {
        formats.push(Format::X);
    }
    for n in &cli.named_formats {
        formats.push(match n.as_str() {
            "a" | "all" => Format::A,
            "cd" | "compact-date" => Format::Custom(String::from("%Y%m%d")),
            "cdt" | "compact-date-time" => Format::Custom(String::from("%Y%m%d-%H%M%S")),
            "ct" | "compact-time" => Format::Custom(String::from("%H%M%S")),
            "d" | "default" => Format::default(),
            "i" | "r" | "iso" | "rfc" | "rfc-3339" => Format::rfc_3339(),
            "x" => Format::X,
            "bcd" => Format::BCD,
            _ => Format::Custom(n.clone()),
        });
    }
    if formats.is_empty() {
        formats.push(if cli.local_zone || cli.zone.is_some() {
            Format::default()
        } else {
            Format::rfc_3339()
        });
    }
    let mut zones = vec![];
    match cli.zone {
        Some(s) => {
            for i in s.split(',') {
                zones.push(tz_(i));
            }
        }
        None => {
            if cli.local_zone || cli.a_format {
                zones.push(tz_("local"));
            } else {
                zones.push(tz_("UTC"));
            }
        }
    }
    let formats = formats
        .iter()
        .map(|x| Some(x.clone()))
        .collect::<Vec<Option<Format>>>();
    if let Some(duration) = interval {
        loop {
            if clear {
                clearscreen::clear().unwrap();
            }
            core(&cli.args, &formats, &zones, &separator, cli.from_x);
            std::thread::sleep(duration);
        }
    } else {
        core(&cli.args, &formats, &zones, &separator, cli.from_x);
    }
}

fn core(
    args: &[String],
    formats: &[Option<Format>],
    timezones: &[Option<TimeZone>],
    separator: &str,
    from_x: bool,
) {
    let mut dtgs = vec![];
    for arg in args.iter() {
        let dtg = if from_x {
            Dtg::from_x(arg)
        } else {
            Dtg::from(arg)
        };
        if dtg.is_err() {
            error(2, &format!("Invalid timestamp: `{arg}`"));
        }
        dtgs.push(dtg.unwrap());
    }
    if dtgs.is_empty() {
        dtgs.push(Dtg::now());
    }
    for i in dtgs.iter() {
        let mut t = vec![];
        for fmt in formats {
            for tz in timezones {
                t.push(i.format(fmt, tz));
            }
        }
        println!("{}", t.join(separator));
    }
}

fn tz_(i: &str) -> Option<TimeZone> {
    let t = tz(i);
    if let Err(ref e) = t {
        match e.code {
            101 => error(5, &e.message),
            102 => error(3, &e.message),
            _ => error(1, "?"),
        }
    }
    t.ok()
}
