# About

Date/time library

*See also the [dtg] crate.*

[dtg]: https://crates.io/crates/dtg

# Examples

```
use chrono::{TimeZone, Utc};
use dtg_lib::{tz, Dtg, Format};

let epoch = 1658448142;
let nanoseconds = 936196858;
let x = "Xg6L02M";
let rfc_3339 = "2022-07-22T00:02:22Z";
let day_of_week_utc = "Friday";
let day_of_week_mt = "Thursday";

let dtg_1_str = format!("{}", epoch);
let dtg_2_str = format!("{}.{}", epoch, nanoseconds);

let dtg_1_ts = Dtg::from(&dtg_1_str).unwrap();
let dtg_1_dt = Dtg::from_dt(&Utc.timestamp(epoch, 0));
let dtg_1_x = Dtg::from_x(x).unwrap();

assert_eq!(dtg_1_ts, dtg_1_dt);
assert_eq!(dtg_1_dt, dtg_1_x);
assert_eq!(dtg_1_x, dtg_1_ts);

let dtg_2_ts = Dtg::from(&dtg_2_str).unwrap();
let dtg_2_dt = Dtg::from_dt(&Utc.timestamp(epoch, nanoseconds));

assert_eq!(dtg_2_ts, dtg_2_dt);

assert_eq!(dtg_1_ts.rfc_3339(), rfc_3339);
assert_eq!(dtg_1_ts.format(&None, &None), rfc_3339);
assert_eq!(dtg_1_ts.format(&Some(Format::X), &None), x);

let day_of_week_fmt = Some(Format::Custom(String::from("%A")));
let tz_mt = tz("MST7MDT").ok();

assert_eq!(dtg_1_ts.format(&day_of_week_fmt, &None), day_of_week_utc);
assert_eq!(dtg_1_ts.format(&day_of_week_fmt, &tz_mt), day_of_week_mt);
```

