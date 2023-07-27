#![doc = include_str!("../README.md")]

//--------------------------------------------------------------------------------------------------
// Crates

use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use lazy_static::lazy_static;

use std::collections::HashMap;

//--------------------------------------------------------------------------------------------------
// Constants / lazy static

const DEFAULT: &str = "%a %d %b %Y %H:%M:%S %Z";
const EPOCH: &str = "%s.%f";
const RFC_3339: &str = "%Y-%m-%dT%H:%M:%SZ";

lazy_static! {
    #[rustfmt::skip]
    static ref ITOC: HashMap<u8, char> = {
        [
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
        ]
        .iter()
        .cloned()
        .collect()
    };

    static ref CTOI: HashMap<char, u8> = {
        ITOC.iter().map(|(i, c)| (*c, *i)).collect()
    };
}

//--------------------------------------------------------------------------------------------------
// DtgError struct

/**
Custom error

* 101: Invalid timestamp
* 102: Invalid timezone
* 103: Failed to get local timezone
*/
#[derive(Debug)]
pub struct DtgError {
    pub code: usize,
    pub message: String,
}

impl DtgError {
    /// Create error
    pub fn new(message: &str, code: usize) -> DtgError {
        DtgError {
            code,
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for DtgError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::cmp::PartialEq for DtgError {
    fn eq(&self, other: &DtgError) -> bool {
        self.code == other.code && self.message == other.message
    }
}

//--------------------------------------------------------------------------------------------------
// Dtg struct

/**
Date time group
*/
#[derive(Debug)]
pub struct Dtg {
    dt: DateTime<Utc>,
}

impl Dtg {
    /**
    Create a current [Dtg]
    */
    pub fn now() -> Dtg {
        Dtg { dt: Utc::now() }
    }

    /**
    Create a [Dtg] from a string timestamp

    ```
    use chrono::{TimeZone, Utc};
    use dtg_lib::Dtg;

    assert_eq!(
        Dtg::from("1658448142").unwrap(),
        Dtg::from_dt(&Utc.timestamp_opt(1658448142, 0).unwrap()),
    );
    assert_eq!(
        Dtg::from("1658448142.936196858").unwrap(),
        Dtg::from_dt(&Utc.timestamp_opt(1658448142, 936196858).unwrap()),
    );
    ```
    */
    pub fn from(s: &str) -> Result<Dtg, DtgError> {
        let mut x = s.split('.');
        if let Some(seconds) = x.next() {
            if let Ok(seconds) = seconds.parse::<i64>() {
                if seconds <= 8210298412799 {
                    if let Some(nanoseconds) = x.next() {
                        let mut nanoseconds = nanoseconds.to_string();
                        while nanoseconds.len() < 9 {
                            nanoseconds.push('0');
                        }
                        if let Ok(nanoseconds) = nanoseconds[..9].parse::<u32>() {
                            return Ok(Dtg {
                                dt: Utc.timestamp_opt(seconds, nanoseconds).unwrap(),
                            });
                        }
                    } else {
                        return Ok(Dtg {
                            dt: Utc.timestamp_opt(seconds, 0).unwrap(),
                        });
                    }
                }
            }
        }
        Err(DtgError::new(&format!("Invalid timestamp: `{s}`"), 101))
    }

    /**
    Create a [Dtg] from an "x" format timestamp

    ```
    use dtg_lib::{Dtg, Format};

    let dtg = Dtg::from_x("Xg6L02M").unwrap();

    assert_eq!(dtg.format(&Some(Format::custom("%s")), &None), "1658448142");
    assert_eq!(dtg.rfc_3339(), "2022-07-22T00:02:22Z");
    ```
    */
    pub fn from_x(s: &str) -> Result<Dtg, DtgError> {
        let mut v: Vec<u32> = s
            .chars()
            .rev()
            .take(5)
            .map(|x| *CTOI.get(&x).unwrap() as u32)
            .collect();
        v[3] += 1; // day
        v[4] += 1; // month
        let mut y = 0;
        for (exp, c) in s.chars().rev().skip(5).enumerate() {
            y += (*CTOI.get(&c).unwrap() as i32) * 60_i32.pow(exp as u32);
        }
        if y > 262143 {
            return Err(DtgError::new(&format!("Invalid timestamp: `{s}`"), 101));
        }
        let dt = Utc
            .with_ymd_and_hms(y, v[4], v[3], v[2], v[1], v[0])
            .unwrap();
        Ok(Dtg { dt })
    }

    /**
    Create a [Dtg] from a [`DateTime<Utc>`]

    ```
    use chrono::{TimeZone, Utc};
    use dtg_lib::Dtg;

    assert_eq!(
        Dtg::from("1658448142").unwrap(),
        Dtg::from_dt(&Utc.timestamp_opt(1658448142, 0).unwrap()),
    );
    ```
    */
    pub fn from_dt(dt: &DateTime<Utc>) -> Dtg {
        Dtg { dt: *dt }
    }

    /**
    Format as a string

    ```
    use dtg_lib::{tz, Dtg};

    let dtg = Dtg::from("1658448142").unwrap();
    let default_utc = "Fri 22 Jul 2022 00:02:22 UTC";
    let default_mt = "Thu 21 Jul 2022 18:02:22 MDT";

    assert_eq!(dtg.default(&None), default_utc);
    assert_eq!(dtg.default(&tz("UTC").ok()), default_utc);
    assert_eq!(dtg.default(&tz("MST7MDT").ok()), default_mt);
    ```
    */
    pub fn default(&self, tz: &Option<Tz>) -> String {
        self.format(&Some(Format::default()), tz)
    }

    /**
    Format as an RFC 3339 string

    ```
    use dtg_lib::Dtg;

    let dtg = Dtg::from("1658448142").unwrap();

    assert_eq!(dtg.rfc_3339(), "2022-07-22T00:02:22Z");
    ```
    */
    pub fn rfc_3339(&self) -> String {
        self.format(&None, &None)
    }

    /**
    Format as "x" format

    ```
    use dtg_lib::Dtg;

    let dtg = Dtg::from("1658448142").unwrap();

    assert_eq!(dtg.x_format(), "Xg6L02M");
    ```
    */
    pub fn x_format(&self) -> String {
        self.format(&Some(Format::X), &None)
    }

    /**
    Format as "a" format

    ```
    use dtg_lib::{tz, Dtg};

    let dtg = Dtg::from("1658448142").unwrap();
    let a_utc = "\
    1658448142.000000000
    2022-07-22T00:02:22Z
    Fri 22 Jul 2022 00:02:22 UTC
    Fri 22 Jul 2022 00:02:22 UTC";
    let a_mt = "\
    1658448142.000000000
    2022-07-22T00:02:22Z
    Fri 22 Jul 2022 00:02:22 UTC
    Thu 21 Jul 2022 18:02:22 MDT";

    assert_eq!(dtg.a_format(&None), a_utc);
    assert_eq!(dtg.a_format(&tz("UTC").ok()), a_utc);
    assert_eq!(dtg.a_format(&tz("MST7MDT").ok()), a_mt);
    ```
    */
    pub fn a_format(&self, tz: &Option<Tz>) -> String {
        self.format(&Some(Format::A), tz)
    }

    /**
    Format like a binary clock using the Braille Patterns Unicode Block and `|` separators

    ```
    use dtg_lib::Dtg;

    let dtg = Dtg::from("1658448142").unwrap();

    assert_eq!(dtg.bcd_format(), "⠄⠤|⢰|⠤|⠀|⠠|⠤"); // 2022|07|22|00|02|22
    ```
    */
    pub fn bcd_format(&self) -> String {
        self.format(&Some(Format::BCD), &None)
    }

    /**
    Format as a string with format and timezone

    ```
    use dtg_lib::{tz, Dtg, Format};

    let dtg = Dtg::from("1658448142").unwrap();

    assert_eq!(
        dtg.format(&None, &None),
        "2022-07-22T00:02:22Z",
    );
    assert_eq!(
        dtg.format(&Some(Format::X), &None),
        "Xg6L02M",
    );

    let a_fmt = Some(Format::custom("%A"));

    assert_eq!(
        dtg.format(&a_fmt, &None),
        "Friday",
    );
    assert_eq!(
        dtg.format(&a_fmt, &tz("MST7MDT").ok()),
        "Thursday",
    );
    ```
    */
    pub fn format(&self, fmt: &Option<Format>, tz: &Option<Tz>) -> String {
        let tz = tz.unwrap_or(Tz::UTC);
        match fmt {
            Some(fmt) => fmt.with(&self.dt, &tz),
            None => Format::Custom(RFC_3339.to_string()).with(&self.dt, &tz),
        }
    }
}

impl std::cmp::PartialEq for Dtg {
    fn eq(&self, other: &Dtg) -> bool {
        self.dt == other.dt
    }
}

//--------------------------------------------------------------------------------------------------
// Format enum

/**
Format

# "a" format

Four newline-separated timestamps with the epoch time in fractional seconds, RFC 3339 format / UTC,
default format / UTC, and default format / local

```text
%s.%f
%Y-%m-%dT%H:%M:%SZ
%a %d %b %Y %H:%M:%S %Z # UTC
%a %d %b %Y %H:%M:%S %Z # Specified or local timezone
```

# "x" format

Novel UTC / base 60 encoding

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

See also [Dtg::from_x]

# Custom format

See also [Dtg::format]

The following information originates from the [chrono documentation], which `dtg-lib` uses
internally.

[chrono documentation]: https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers

## Date specifiers

Spec. | Example       | Description
------|---------------|----------------------------------------------------------------------------
`%Y`  | `2001`        | The full proleptic Gregorian year, zero-padded to 4 digits.
`%C`  | `20`          | The proleptic Gregorian year divided by 100, zero-padded to 2 digits.
`%y`  | `01`          | The proleptic Gregorian year modulo 100, zero-padded to 2 digits.
`%m`  | `07`          | Month number (01--12), zero-padded to 2 digits.
`%b`  | `Jul`         | Abbreviated month name. Always 3 letters.
`%B`  | `July`        | Full month name. Also accepts corresponding abbreviation in parsing.
`%h`  | `Jul`         | Same as `%b`.
`%d`  | `08`          | Day number (01--31), zero-padded to 2 digits.
`%e`  | ` 8`          | Same as `%d` but space-padded. Same as `%_d`.
`%a`  | `Sun`         | Abbreviated weekday name. Always 3 letters.
`%A`  | `Sunday`      | Full weekday name. Also accepts corresponding abbreviation in parsing.
`%w`  | `0`           | Sunday = 0, Monday = 1, ..., Saturday = 6.
`%u`  | `7`           | Monday = 1, Tuesday = 2, ..., Sunday = 7. (ISO 8601)
`%U`  | `28`          | Week number starting with Sunday (00--53), zero-padded to 2 digits.
`%W`  | `27`          | Same as `%U`, but week 1 starts with the first Monday in that year instead.
`%G`  | `2001`        | Same as `%Y` but uses the year number in ISO 8601 week date.
`%g`  | `01`          | Same as `%y` but uses the year number in ISO 8601 week date.
`%V`  | `27`          | Same as `%U` but uses the week number in ISO 8601 week date (01--53).
`%j`  | `189`         | Day of the year (001--366), zero-padded to 3 digits.
`%D`  | `07/08/01`    | Month-day-year format. Same as `%m/%d/%y`.
`%x`  | `07/08/01`    | Locale's date representation (e.g., 12/31/99).
`%F`  | `2001-07-08`  | Year-month-day format (ISO 8601). Same as `%Y-%m-%d`.
`%v`  | ` 8-Jul-2001` | Day-month-year format. Same as `%e-%b-%Y`.

## Time specifiers

Spec.  | Example       | Description
-------|---------------|----------------------------------------------------------------------
`%H`   | `00`          | Hour number (00--23), zero-padded to 2 digits.
`%k`   | ` 0`          | Same as `%H` but space-padded. Same as `%_H`.
`%I`   | `12`          | Hour number in 12-hour clocks (01--12), zero-padded to 2 digits.
`%l`   | `12`          | Same as `%I` but space-padded. Same as `%_I`.
`%P`   | `am`          | `am` or `pm` in 12-hour clocks.
`%p`   | `AM`          | `AM` or `PM` in 12-hour clocks.
`%M`   | `34`          | Minute number (00--59), zero-padded to 2 digits.
`%S`   | `60`          | Second number (00--60), zero-padded to 2 digits.
`%f`   | `026490000`   | The fractional seconds (in nanoseconds) since last whole second.
`%.f`  | `.026490`     | Similar to `.%f` but left-aligned. These all consume the leading dot.
`%.3f` | `.026`        | Similar to `.%f` but left-aligned but fixed to a length of 3.
`%.6f` | `.026490`     | Similar to `.%f` but left-aligned but fixed to a length of 6.
`%.9f` | `.026490000`  | Similar to `.%f` but left-aligned but fixed to a length of 9.
`%3f`  | `026`         | Similar to `%.3f` but without the leading dot.
`%6f`  | `026490`      | Similar to `%.6f` but without the leading dot.
`%9f`  | `026490000`   | Similar to `%.9f` but without the leading dot.
`%R`   | `00:34`       | Hour-minute format. Same as `%H:%M`.
`%T`   | `00:34:60`    | Hour-minute-second format. Same as `%H:%M:%S`.
`%X`   | `00:34:60`    | Locale's time representation (e.g., 23:13:48).
`%r`   | `12:34:60 AM` | Hour-minute-second format in 12-hour clocks. Same as `%I:%M:%S %p`.

## Time zone specifiers

Spec. | Example  | Description
------|----------|--------------------------------------------------------------------------
`%Z`  | `ACST`   | Local time zone name. Skips all non-whitespace characters during parsing.
`%z`  | `+0930`  | Offset from the local time to UTC (with UTC being `+0000`).
`%:z` | `+09:30` | Same as `%z` but with a colon.
`%#z` | `+09`    | *Parsing only:* Same as `%z` but allows minutes to be missing or present.

## Date & time specifiers

Spec. | Example                            | Description
------|------------------------------------|------------------------------------------------------------------
`%c`  | `Sun Jul  8 00:34:60 2001`         | Locale's date and time (e.g., Thu Mar  3 23:05:25 2005).
`%+`  | `2001-07-08T00:34:60.026490+09:30` | ISO 8601 / RFC 3339 date & time format.
`%s`  | `994518299`                        | UNIX timestamp, the number of seconds since 1970-01-01 00:00 UTC.

## Special specifiers

Spec. | Description
------|------------------------
`%t`  | Literal tab (`\t`).
`%n`  | Literal newline (`\n`).
`%%`  | Literal percent sign.
*/
#[derive(Clone)]
pub enum Format {
    A,
    BCD,
    X,
    Custom(String),
}

impl Format {
    /**
    Create a default [Format]
    */
    pub fn new() -> Format {
        Format::Custom(DEFAULT.to_string())
    }

    /**
    Create an RFC 3339 [Format]
    */
    pub fn rfc_3339() -> Format {
        Format::Custom(RFC_3339.to_string())
    }

    /**
    Create a custom [Format]
    */
    pub fn custom(s: &str) -> Format {
        Format::Custom(s.to_string())
    }

    /**
    Format a [DateTime<Utc>] with a timezone
    */
    fn with(&self, dt: &DateTime<Utc>, tz: &Tz) -> String {
        match self {
            Format::Custom(f) => {
                if f == RFC_3339 {
                    dt.format(f).to_string()
                } else {
                    dt.with_timezone(tz).format(f).to_string()
                }
            }
            Format::A => format!(
                "{}\n{}\n{}\n{}",
                dt.format(EPOCH),
                dt.format(RFC_3339),
                dt.format(DEFAULT),
                dt.with_timezone(tz).format(DEFAULT),
            ),
            Format::X => self.x(dt),
            Format::BCD => self.bcd(dt, tz),
        }
    }

    /**
    Format a [DateTime<Utc>] with "x" format
    */
    fn x(&self, dt: &DateTime<Utc>) -> String {
        let mut year = dt.year() as u32;
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
            .map(|x| ITOC.get(x).unwrap())
            .collect::<String>();
        let mon = ITOC.get(&(dt.month0() as u8)).unwrap();
        let day = ITOC.get(&(dt.day0() as u8)).unwrap();
        let h = ITOC.get(&(dt.hour() as u8)).unwrap();
        let m = ITOC.get(&(dt.minute() as u8)).unwrap();
        let s = ITOC.get(&(dt.second() as u8)).unwrap();
        format!("{year}{mon}{day}{h}{m}{s}")
    }

    /**
    Format a [DateTime<Utc>] like a binary clock using the Braille Patterns Unicode Block and `|`
    separators
    */
    fn bcd(&self, dt: &DateTime<Utc>, tz: &Tz) -> String {
        let dt = dt.with_timezone(tz);
        let yyyy = dt.year();
        let (mut r, yyyy) = if yyyy < 0 {
            (String::from("-"), (-yyyy) as u32)
        } else {
            (String::new(), yyyy as u32)
        };
        let cc = (yyyy / 100) as u8;
        let yy = (yyyy - yyyy / 100 * 100) as u8;
        for (i, n) in [
            cc,
            yy,
            dt.month() as u8,
            dt.day() as u8,
            dt.hour() as u8,
            dt.minute() as u8,
            dt.second() as u8,
        ]
        .iter()
        .enumerate()
        {
            if i >= 2 {
                r.push('|');
            }
            r.push(bcd_braille(*n));
        }
        r
    }
}

impl Default for Format {
    fn default() -> Format {
        Format::new()
    }
}

//--------------------------------------------------------------------------------------------------
// Functions

/**
Get a timezone by name

```
use chrono_tz::Tz;
use dtg_lib::{tz, DtgError};

assert_eq!(tz("UTC"), Ok(Tz::UTC));
assert_eq!(tz("GMT"), Ok(Tz::GMT));
assert_eq!(tz("America/New_York"), Ok(Tz::America__New_York));
assert_eq!(tz("EST5EDT"), Ok(Tz::EST5EDT));
//assert_eq!(tz("local"), Ok(Tz::America__New_York));

assert_eq!(tz("nonexistent"), Err(DtgError::new("Invalid timezone: `nonexistent`", 102)));
```

Timezones:

```text
Africa/Abidjan
Africa/Accra
Africa/Addis_Ababa
Africa/Algiers
Africa/Asmara
Africa/Asmera
Africa/Bamako
Africa/Bangui
Africa/Banjul
Africa/Bissau
Africa/Blantyre
Africa/Brazzaville
Africa/Bujumbura
Africa/Cairo
Africa/Casablanca
Africa/Ceuta
Africa/Conakry
Africa/Dakar
Africa/Dar_es_Salaam
Africa/Djibouti
Africa/Douala
Africa/El_Aaiun
Africa/Freetown
Africa/Gaborone
Africa/Harare
Africa/Johannesburg
Africa/Juba
Africa/Kampala
Africa/Khartoum
Africa/Kigali
Africa/Kinshasa
Africa/Lagos
Africa/Libreville
Africa/Lome
Africa/Luanda
Africa/Lubumbashi
Africa/Lusaka
Africa/Malabo
Africa/Maputo
Africa/Maseru
Africa/Mbabane
Africa/Mogadishu
Africa/Monrovia
Africa/Nairobi
Africa/Ndjamena
Africa/Niamey
Africa/Nouakchott
Africa/Ouagadougou
Africa/Porto-Novo
Africa/Sao_Tome
Africa/Timbuktu
Africa/Tripoli
Africa/Tunis
Africa/Windhoek
America/Adak
America/Anchorage
America/Anguilla
America/Antigua
America/Araguaina
America/Argentina/Buenos_Aires
America/Argentina/Catamarca
America/Argentina/ComodRivadavia
America/Argentina/Cordoba
America/Argentina/Jujuy
America/Argentina/La_Rioja
America/Argentina/Mendoza
America/Argentina/Rio_Gallegos
America/Argentina/Salta
America/Argentina/San_Juan
America/Argentina/San_Luis
America/Argentina/Tucuman
America/Argentina/Ushuaia
America/Aruba
America/Asuncion
America/Atikokan
America/Atka
America/Bahia
America/Bahia_Banderas
America/Barbados
America/Belem
America/Belize
America/Blanc-Sablon
America/Boa_Vista
America/Bogota
America/Boise
America/Buenos_Aires
America/Cambridge_Bay
America/Campo_Grande
America/Cancun
America/Caracas
America/Catamarca
America/Cayenne
America/Cayman
America/Chicago
America/Chihuahua
America/Coral_Harbour
America/Cordoba
America/Costa_Rica
America/Creston
America/Cuiaba
America/Curacao
America/Danmarkshavn
America/Dawson
America/Dawson_Creek
America/Denver
America/Detroit
America/Dominica
America/Edmonton
America/Eirunepe
America/El_Salvador
America/Ensenada
America/Fort_Nelson
America/Fort_Wayne
America/Fortaleza
America/Glace_Bay
America/Godthab
America/Goose_Bay
America/Grand_Turk
America/Grenada
America/Guadeloupe
America/Guatemala
America/Guayaquil
America/Guyana
America/Halifax
America/Havana
America/Hermosillo
America/Indiana/Indianapolis
America/Indiana/Knox
America/Indiana/Marengo
America/Indiana/Petersburg
America/Indiana/Tell_City
America/Indiana/Vevay
America/Indiana/Vincennes
America/Indiana/Winamac
America/Indianapolis
America/Inuvik
America/Iqaluit
America/Jamaica
America/Jujuy
America/Juneau
America/Kentucky/Louisville
America/Kentucky/Monticello
America/Knox_IN
America/Kralendijk
America/La_Paz
America/Lima
America/Los_Angeles
America/Louisville
America/Lower_Princes
America/Maceio
America/Managua
America/Manaus
America/Marigot
America/Martinique
America/Matamoros
America/Mazatlan
America/Mendoza
America/Menominee
America/Merida
America/Metlakatla
America/Mexico_City
America/Miquelon
America/Moncton
America/Monterrey
America/Montevideo
America/Montreal
America/Montserrat
America/Nassau
America/New_York
America/Nipigon
America/Nome
America/Noronha
America/North_Dakota/Beulah
America/North_Dakota/Center
America/North_Dakota/New_Salem
America/Nuuk
America/Ojinaga
America/Panama
America/Pangnirtung
America/Paramaribo
America/Phoenix
America/Port-au-Prince
America/Port_of_Spain
America/Porto_Acre
America/Porto_Velho
America/Puerto_Rico
America/Punta_Arenas
America/Rainy_River
America/Rankin_Inlet
America/Recife
America/Regina
America/Resolute
America/Rio_Branco
America/Rosario
America/Santa_Isabel
America/Santarem
America/Santiago
America/Santo_Domingo
America/Sao_Paulo
America/Scoresbysund
America/Shiprock
America/Sitka
America/St_Barthelemy
America/St_Johns
America/St_Kitts
America/St_Lucia
America/St_Thomas
America/St_Vincent
America/Swift_Current
America/Tegucigalpa
America/Thule
America/Thunder_Bay
America/Tijuana
America/Toronto
America/Tortola
America/Vancouver
America/Virgin
America/Whitehorse
America/Winnipeg
America/Yakutat
America/Yellowknife
Antarctica/Casey
Antarctica/Davis
Antarctica/DumontDUrville
Antarctica/Macquarie
Antarctica/Mawson
Antarctica/McMurdo
Antarctica/Palmer
Antarctica/Rothera
Antarctica/South_Pole
Antarctica/Syowa
Antarctica/Troll
Antarctica/Vostok
Arctic/Longyearbyen
Asia/Aden
Asia/Almaty
Asia/Amman
Asia/Anadyr
Asia/Aqtau
Asia/Aqtobe
Asia/Ashgabat
Asia/Ashkhabad
Asia/Atyrau
Asia/Baghdad
Asia/Bahrain
Asia/Baku
Asia/Bangkok
Asia/Barnaul
Asia/Beirut
Asia/Bishkek
Asia/Brunei
Asia/Calcutta
Asia/Chita
Asia/Choibalsan
Asia/Chongqing
Asia/Chungking
Asia/Colombo
Asia/Dacca
Asia/Damascus
Asia/Dhaka
Asia/Dili
Asia/Dubai
Asia/Dushanbe
Asia/Famagusta
Asia/Gaza
Asia/Harbin
Asia/Hebron
Asia/Ho_Chi_Minh
Asia/Hong_Kong
Asia/Hovd
Asia/Irkutsk
Asia/Istanbul
Asia/Jakarta
Asia/Jayapura
Asia/Jerusalem
Asia/Kabul
Asia/Kamchatka
Asia/Karachi
Asia/Kashgar
Asia/Kathmandu
Asia/Katmandu
Asia/Khandyga
Asia/Kolkata
Asia/Krasnoyarsk
Asia/Kuala_Lumpur
Asia/Kuching
Asia/Kuwait
Asia/Macao
Asia/Macau
Asia/Magadan
Asia/Makassar
Asia/Manila
Asia/Muscat
Asia/Nicosia
Asia/Novokuznetsk
Asia/Novosibirsk
Asia/Omsk
Asia/Oral
Asia/Phnom_Penh
Asia/Pontianak
Asia/Pyongyang
Asia/Qatar
Asia/Qostanay
Asia/Qyzylorda
Asia/Rangoon
Asia/Riyadh
Asia/Saigon
Asia/Sakhalin
Asia/Samarkand
Asia/Seoul
Asia/Shanghai
Asia/Singapore
Asia/Srednekolymsk
Asia/Taipei
Asia/Tashkent
Asia/Tbilisi
Asia/Tehran
Asia/Tel_Aviv
Asia/Thimbu
Asia/Thimphu
Asia/Tokyo
Asia/Tomsk
Asia/Ujung_Pandang
Asia/Ulaanbaatar
Asia/Ulan_Bator
Asia/Urumqi
Asia/Ust-Nera
Asia/Vientiane
Asia/Vladivostok
Asia/Yakutsk
Asia/Yangon
Asia/Yekaterinburg
Asia/Yerevan
Atlantic/Azores
Atlantic/Bermuda
Atlantic/Canary
Atlantic/Cape_Verde
Atlantic/Faeroe
Atlantic/Faroe
Atlantic/Jan_Mayen
Atlantic/Madeira
Atlantic/Reykjavik
Atlantic/South_Georgia
Atlantic/St_Helena
Atlantic/Stanley
Australia/ACT
Australia/Adelaide
Australia/Brisbane
Australia/Broken_Hill
Australia/Canberra
Australia/Currie
Australia/Darwin
Australia/Eucla
Australia/Hobart
Australia/LHI
Australia/Lindeman
Australia/Lord_Howe
Australia/Melbourne
Australia/NSW
Australia/North
Australia/Perth
Australia/Queensland
Australia/South
Australia/Sydney
Australia/Tasmania
Australia/Victoria
Australia/West
Australia/Yancowinna
Brazil/Acre
Brazil/DeNoronha
Brazil/East
Brazil/West
CET
CST6CDT
Canada/Atlantic
Canada/Central
Canada/Eastern
Canada/Mountain
Canada/Newfoundland
Canada/Pacific
Canada/Saskatchewan
Canada/Yukon
Chile/Continental
Chile/EasterIsland
Cuba
EET
EST
EST5EDT
Egypt
Eire
Etc/GMT
Etc/GMT+0
Etc/GMT+1
Etc/GMT+10
Etc/GMT+11
Etc/GMT+12
Etc/GMT+2
Etc/GMT+3
Etc/GMT+4
Etc/GMT+5
Etc/GMT+6
Etc/GMT+7
Etc/GMT+8
Etc/GMT+9
Etc/GMT-0
Etc/GMT-1
Etc/GMT-10
Etc/GMT-11
Etc/GMT-12
Etc/GMT-13
Etc/GMT-14
Etc/GMT-2
Etc/GMT-3
Etc/GMT-4
Etc/GMT-5
Etc/GMT-6
Etc/GMT-7
Etc/GMT-8
Etc/GMT-9
Etc/GMT0
Etc/Greenwich
Etc/UCT
Etc/UTC
Etc/Universal
Etc/Zulu
Europe/Amsterdam
Europe/Andorra
Europe/Astrakhan
Europe/Athens
Europe/Belfast
Europe/Belgrade
Europe/Berlin
Europe/Bratislava
Europe/Brussels
Europe/Bucharest
Europe/Budapest
Europe/Busingen
Europe/Chisinau
Europe/Copenhagen
Europe/Dublin
Europe/Gibraltar
Europe/Guernsey
Europe/Helsinki
Europe/Isle_of_Man
Europe/Istanbul
Europe/Jersey
Europe/Kaliningrad
Europe/Kiev
Europe/Kirov
Europe/Lisbon
Europe/Ljubljana
Europe/London
Europe/Luxembourg
Europe/Madrid
Europe/Malta
Europe/Mariehamn
Europe/Minsk
Europe/Monaco
Europe/Moscow
Europe/Nicosia
Europe/Oslo
Europe/Paris
Europe/Podgorica
Europe/Prague
Europe/Riga
Europe/Rome
Europe/Samara
Europe/San_Marino
Europe/Sarajevo
Europe/Saratov
Europe/Simferopol
Europe/Skopje
Europe/Sofia
Europe/Stockholm
Europe/Tallinn
Europe/Tirane
Europe/Tiraspol
Europe/Ulyanovsk
Europe/Uzhgorod
Europe/Vaduz
Europe/Vatican
Europe/Vienna
Europe/Vilnius
Europe/Volgograd
Europe/Warsaw
Europe/Zagreb
Europe/Zaporozhye
Europe/Zurich
GB
GB-Eire
GMT
GMT+0
GMT-0
GMT0
Greenwich
HST
Hongkong
Iceland
Indian/Antananarivo
Indian/Chagos
Indian/Christmas
Indian/Cocos
Indian/Comoro
Indian/Kerguelen
Indian/Mahe
Indian/Maldives
Indian/Mauritius
Indian/Mayotte
Indian/Reunion
Iran
Israel
Jamaica
Japan
Kwajalein
Libya
MET
MST
MST7MDT
Mexico/BajaNorte
Mexico/BajaSur
Mexico/General
NZ
NZ-CHAT
Navajo
PRC
PST8PDT
Pacific/Apia
Pacific/Auckland
Pacific/Bougainville
Pacific/Chatham
Pacific/Chuuk
Pacific/Easter
Pacific/Efate
Pacific/Enderbury
Pacific/Fakaofo
Pacific/Fiji
Pacific/Funafuti
Pacific/Galapagos
Pacific/Gambier
Pacific/Guadalcanal
Pacific/Guam
Pacific/Honolulu
Pacific/Johnston
Pacific/Kanton
Pacific/Kiritimati
Pacific/Kosrae
Pacific/Kwajalein
Pacific/Majuro
Pacific/Marquesas
Pacific/Midway
Pacific/Nauru
Pacific/Niue
Pacific/Norfolk
Pacific/Noumea
Pacific/Pago_Pago
Pacific/Palau
Pacific/Pitcairn
Pacific/Pohnpei
Pacific/Ponape
Pacific/Port_Moresby
Pacific/Rarotonga
Pacific/Saipan
Pacific/Samoa
Pacific/Tahiti
Pacific/Tarawa
Pacific/Tongatapu
Pacific/Truk
Pacific/Wake
Pacific/Wallis
Pacific/Yap
Poland
Portugal
ROC
ROK
Singapore
Turkey
UCT
US/Alaska
US/Aleutian
US/Arizona
US/Central
US/East-Indiana
US/Eastern
US/Hawaii
US/Indiana-Starke
US/Michigan
US/Mountain
US/Pacific
US/Samoa
UTC
Universal
W-SU
WET
Zulu
```
*/
pub fn tz(s: &str) -> Result<Tz, DtgError> {
    match s {
        "local" => match iana_time_zone::get_timezone() {
            Ok(local) => tz(&local),
            Err(_) => Err(DtgError::new("Failed to get local timezone", 103)),
        },
        _ => match s.parse() {
            Ok(z) => Ok(z),
            Err(_) => Err(DtgError::new(&format!("Invalid timezone: `{s}`"), 102)),
        },
    }
}

/**
Translate a [u8] in the range 0 to 99 to a "binary clock style" / Binary Code Decimal (BCD)
representation using a single character from the Braille Patterns Unicode Block
*/
fn bcd_braille(x: u8) -> char {
    if x > 99 {
        panic!("Invalid BCD value: {x}! Must in range `0..=99`.")
    }
    let tens = x / 10;
    let ones = x - tens * 10;
    char::from_u32(
        0x2800
            + match tens {
                0 => 0x00,
                1 => 0x40,
                2 => 0x04,
                3 => 0x44,
                4 => 0x02,
                5 => 0x42,
                6 => 0x06,
                7 => 0x46,
                8 => 0x01,
                9 => 0x41,
                _ => unreachable!(),
            }
            + match ones {
                0 => 0x00,
                1 => 0x80,
                2 => 0x20,
                3 => 0xA0,
                4 => 0x10,
                5 => 0x90,
                6 => 0x30,
                7 => 0xB0,
                8 => 0x08,
                9 => 0x88,
                _ => unreachable!(),
            },
    )
    .unwrap()
}
