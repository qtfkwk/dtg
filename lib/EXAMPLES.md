# Examples

```Rust
use chrono::{TimeZone, Utc};
use dtg_lib::{tz, Dtg, Format};

let epoch = 1658448142;
let nanoseconds = 936196858;
let rfc_3339 = "2022-07-22T00:02:22Z";
let default_utc = "Fri 22 Jul 2022 00:02:22 UTC";
let default_mt = "Thu 21 Jul 2022 18:02:22 MDT";
let x = "Xg6L02M";
let a_utc = format!("{epoch}.000000000\n{rfc_3339}\n{default_utc}\n{default_utc}");
let a_mt = format!("{epoch}.000000000\n{rfc_3339}\n{default_utc}\n{default_mt}");
let day_of_week_utc = "Friday";
let day_of_week_mt = "Thursday";
let tz_utc = tz("UTC").ok();
let tz_mt = tz("MST7MDT").ok();
let default_fmt = Some(Format::default());
let day_of_week_fmt = Some(Format::custom("%A"));

// Create Dtg

let dtg_1_str = format!("{}", epoch);

let dtg_1_ts = Dtg::from(&dtg_1_str).unwrap();
let dtg_1_dt = Dtg::from_dt(&Utc.timestamp(epoch, 0));
let dtg_1_x = Dtg::from_x(x).unwrap();

assert_eq!(dtg_1_ts, dtg_1_dt);
assert_eq!(dtg_1_dt, dtg_1_x);
assert_eq!(dtg_1_x, dtg_1_ts);

// Create Dtg with nanoseconds

let dtg_2_str = format!("{}.{}", epoch, nanoseconds);

let dtg_2_ts = Dtg::from(&dtg_2_str).unwrap();
let dtg_2_dt = Dtg::from_dt(&Utc.timestamp(epoch, nanoseconds));

assert_eq!(dtg_2_ts, dtg_2_dt);

// Default format

assert_eq!(dtg_1_ts.default(&None), default_utc);
assert_eq!(dtg_1_ts.default(&tz_utc), default_utc);
assert_eq!(dtg_1_ts.default(&tz_mt), default_mt);

assert_eq!(dtg_1_ts.format(&default_fmt, &None), default_utc);
assert_eq!(dtg_1_ts.format(&default_fmt, &tz_utc), default_utc);
assert_eq!(dtg_1_ts.format(&default_fmt, &tz_mt), default_mt);

// RFC 3339 format

assert_eq!(dtg_1_ts.rfc_3339(), rfc_3339);
assert_eq!(dtg_1_ts.format(&None, &None), rfc_3339);

// "x" format

assert_eq!(dtg_1_ts.x_format(), x);
assert_eq!(dtg_1_ts.format(&Some(Format::X), &None), x);

// "a" format

assert_eq!(dtg_1_ts.a_format(&None), a_utc);
assert_eq!(dtg_1_ts.a_format(&tz_utc), a_utc);
assert_eq!(dtg_1_ts.a_format(&tz_mt), a_mt);

assert_eq!(dtg_1_ts.format(&Some(Format::A), &None), a_utc);
assert_eq!(dtg_1_ts.format(&Some(Format::A), &tz_utc), a_utc);
assert_eq!(dtg_1_ts.format(&Some(Format::A), &tz_mt), a_mt);

// Custom format

assert_eq!(dtg_1_ts.format(&day_of_week_fmt, &None), day_of_week_utc);
assert_eq!(dtg_1_ts.format(&day_of_week_fmt, &tz_mt), day_of_week_mt);
```

