/*!
Date/time CLI utility
*/

use std::collections::HashMap;

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{TZ_VARIANTS, Tz};
use structopt::StructOpt;

/**
Print message to stderr and exit with code
*/
fn error(code: i32, msg: &str) {
    eprintln!("ERROR: {}!", msg);
    std::process::exit(code);
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
Format command
*/
enum Cmd {
    Custom(String),
    A,
    X,
}

/// Date/time CLI utility; https://github.com/qtfkwk/dtg
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// Print the readme
    #[structopt(long)]
    readme: bool,

    /// Format(s) [-z/-l: "%a %d %b %Y %H:%M:%S %Z", "%Y-%m-%dT%H:%M:%SZ"]
    #[structopt(short)]
    formats: Vec<String>,

    /// "a" format (1)
    #[structopt(short)]
    a_format: bool,

    /// "x" format (2)
    #[structopt(short)]
    x_format: bool,

    /// Give timestamp argument(s) in "x" format (2)
    #[structopt(short="X")]
    from_x: bool,

    /// Timezone [default: UTC]
    #[structopt(short)]
    zone: Option<String>,

    /// Local timezone
    #[structopt(short)]
    local_zone: bool,

    /// Search/list timezones
    #[structopt(short="Z")]
    list_zones: bool,

    /// Argument [-X: timestamp in "x" format (2), -Z: timezone search term,
    /// timestamp in "%s.%f" format, default: now]
    #[structopt(name="ARG")]
    args: Vec<String>,
}

/**
Main
*/
fn main() {
    let app = Opt::clap().set_term_width(80)
        .after_help(
            "\
NOTES:
    1. \"a\" format:

       ```
       %s.%f
       %Y-%m-%dT%H:%M:%SZ
       %a %d %b %Y %H:%M:%S %Z
       %a %d %b %Y %H:%M:%S %Z # -l implied or use -z <zone>
       ```

    2. \"x\" format (novel UTC / base 60 encoding):

       ```
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
\
            ",
        );
    let opt = Opt::from_clap(&app.get_matches());

    if opt.readme {
        let readme = include_str!("../README.md");
        print!("{}", readme);
        return;
    }

    if opt.list_zones {
        let mut found = 0;
        if opt.args.is_empty() {
            for zone in TZ_VARIANTS.iter() {
                println!("{}", zone);
                found += 1;
            }
        } else {
            let search = &opt.args[0];
            let search_lc = search.to_lowercase();
            for zone in TZ_VARIANTS.iter() {
                let name = zone.to_string().to_lowercase();
                if name.contains(&search_lc) {
                    println!("{}", zone);
                    found += 1;
                }
            }
            if found == 0 {
                error(1, &format!("Zero timezones found matching `{}`", search));
            }
        }
        return;
    }

    let mut cmds = vec![];
    for i in opt.formats.iter() {
        cmds.push(Cmd::Custom(i.to_string()));
    }
    if opt.a_format {
        cmds.push(Cmd::A);
    }
    if opt.x_format {
        cmds.push(Cmd::X);
    }
    if cmds.is_empty() {
        if opt.local_zone || opt.zone != None {
            cmds.push(Cmd::Custom(String::from("%a %d %b %Y %H:%M:%S %Z")));
        } else {
            cmds.push(Cmd::Custom(String::from("%Y-%m-%dT%H:%M:%SZ")));
        }
    }
    let zone = match opt.zone {
        Some(s) => tz(&s),
        None => if opt.local_zone || opt.a_format {
            tz("local")
        } else {
            tz("UTC")
        }
    };
    let mut args = vec![];
    for timestamp in opt.args.iter() {
        args.push(if opt.from_x {
            from_format_x(timestamp)
        } else {
            timestamp.to_string()
        });
    }
    if args.len() == 0 {
        args.push(String::from("now"));
    }
    for arg in args.iter() {
        let mut dt = None;
        if ["now", ""].contains(&arg.as_str()) {
            dt = Some(Utc::now());
        } else {
            let s = arg.split('.').collect::<Vec<&str>>();
            let n = s.len();
            if n == 1 {
                if let Ok(seconds) = s[0].parse::<i64>() {
                    if seconds > 8210298412799 {
                        error(4, &format!("Overflow: `{}`", arg));
                    }
                    dt = Some(Utc.timestamp(seconds, 0));
                }
            } else if n == 2 {
                if let Ok(seconds) = s[0].parse::<i64>() {
                    if seconds > 8210298412799 {
                        error(4, &format!("Overflow: `{}`", arg));
                    }
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
                for cmd in cmds.iter() {
                    println!("{}", match cmd {
                        Cmd::Custom(f) => d
                            .with_timezone(&zone)
                            .format(&f)
                            .to_string(),
                        Cmd::A => format_a(&d, &zone),
                        Cmd::X => format_x(&d),
                    });
                }
            }
            None => {
                error(2, &format!("Invalid argument: `{}`", arg));
            }
        }
    }
}

/**
Format "a"
*/
fn format_a<T: TimeZone>(d: &DateTime<T>, tz: &Tz) -> String
where
    T::Offset: std::fmt::Display
{
    format!(
        "{}\n{}",
        d.format("%s.%f%n%Y-%m-%dT%H:%M:%SZ%n%a %d %b %Y %H:%M:%S %Z"),
        d.with_timezone(tz).format("%a %d %b %Y %H:%M:%S %Z"),
    )
}

/**
Format "x"
*/
fn format_x<T: TimeZone>(d: &DateTime<T>) -> String
where
    T::Offset: std::fmt::Display
{
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
    let mut year = d.format("%Y").to_string().parse::<u32>().unwrap();
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
    let mut y: Vec<u8> = vec![];
    if year == 0 {
        y.push(0);
    }
    while year > 0 {
        y.push((year % 60) as u8);
        year /= 60;
    }
    let year = y
        .iter()
        .rev()
        .map(|x| c.get(x).unwrap())
        .collect::<String>();
    format!("{}{}{}{}{}{}", year, mon, day, h, m, s)
}

/**
Convert from format "x" to "%s"
*/
fn from_format_x(arg: &str) -> String {
    let c: HashMap<char, u8> = [
        ('0', 0), ('A', 10), ('K', 20), ('U', 30), ('e', 40), ('o', 50),
        ('1', 1), ('B', 11), ('L', 21), ('V', 31), ('f', 41), ('p', 51),
        ('2', 2), ('C', 12), ('M', 22), ('W', 32), ('g', 42), ('q', 52),
        ('3', 3), ('D', 13), ('N', 23), ('X', 33), ('h', 43), ('r', 53),
        ('4', 4), ('E', 14), ('O', 24), ('Y', 34), ('i', 44), ('s', 54),
        ('5', 5), ('F', 15), ('P', 25), ('Z', 35), ('j', 45), ('t', 55),
        ('6', 6), ('G', 16), ('Q', 26), ('a', 36), ('k', 46), ('u', 56),
        ('7', 7), ('H', 17), ('R', 27), ('b', 37), ('l', 47), ('v', 57),
        ('8', 8), ('I', 18), ('S', 28), ('c', 38), ('m', 48), ('w', 58),
        ('9', 9), ('J', 19), ('T', 29), ('d', 39), ('n', 49), ('x', 59),
    ].iter().cloned().collect();
    let mut v: Vec<u32> = arg.chars().rev().take(5)
        .map(|x| *c.get(&x).unwrap() as u32).collect();
    v[3] += 1;
    v[4] += 1;
    let mut y = 0;
    for (e, x) in arg.chars().rev().skip(5).enumerate() {
        y += (*c.get(&x).unwrap() as i32) * 60_i32.pow(e as u32);
    }
    if y > 262143 {
        error(4, &format!("Overflow: `{}`", arg));
    }
    format!("{}", Utc.ymd(y, v[4], v[3]).and_hms(v[2], v[1], v[0]).timestamp())
}
