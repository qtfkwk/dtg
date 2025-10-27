# Changelog

* 1.0.2 (2020-11-25): Initial public release
* 2.0.0 (2020-11-26): General improvements to readme and code; replaced `-u` with `-l` and ability
  to get local timezone via the [iana-time-zone crate](https://crates.io/crates/iana-time-zone)
    * 2.0.1 (2020-11-26): Improve readme (add `-f` examples...); `s/-r/-l/` in usage
    * 2.1.0 (2020-11-28): Enable bundling options (`-a -z TZ`: `-az TZ`, `-a -l`: `-al`, `-l -a`:
      `-la`, `-l -f FORMAT`: `-lf FORMAT`)
    * 2.2.0 (2020-12-17): Add `-x` option
        * 2.2.1 (2020-12-19): Fix `-x` option year from hex to base 60
        * 2.2.2 (2020-12-20): Update doc; allow `-ax`, `-axz`, `-xa`, `-xaz`
        * 2.2.3 (2020-12-20): Enable multiple `-f` options
        * 2.2.4 (2020-12-21): Add `-X` option
* 3.0.0 (2020-12-23): Rewrite with structopt; add `-Z`
    * 3.1.0 (2020-12-23): Add `--readme`
    * 3.2.0 (2020-12-23): Improve doc
        * 3.2.1 (2020-12-26): Fix `-a` option (yanked)
        * 3.2.2 (2020-12-26): Fix readme
    * 3.3.0 (2021-05-01): Improve doc; upgrade dependencies
        * 3.3.1 (2021-05-01): Fix tables in readme
    * 3.4.0 (2021-09-02): Catch overflows; upgrade dependencies
    * 3.5.0 (2021-09-10): Multiple timezones; cargo fmt; upgrade dependencies
    * 3.6.0 (2021-09-10): Separator `-s` option
    * 3.7.0 (2021-09-10): Interval `-i`, `-c` options
* 4.0.0 (2022-07-21): Redesign as lib and cli crates in a workspace; replace structopt with clap v3
  derive API
    * 4.1.0 (2022-07-22): Added `Dtg::{a_format,x_format,default}`, `Format::custom` methods to lib
        * 4.1.1 (2022-07-22): Fix broken `-i`, `-c` in cli
        * 4.1.2 (2022-07-23): Generate readme files
        * 4.1.3 (2022-09-12): Update dependencies (see
          [comment](https://github.com/chronotope/chrono/issues/602#issuecomment-1242149249))
* 5.0.0 (2023-02-21): Update dependencies (clap v3 to v4, chrono 0.4.22 to 0.4.23); apply clippy fixes; replace
  `Makefile` with a cargo-make `Makefile.toml`
    * 5.1.0 (2023-05-24): Update dependencies; fix readme
    * 5.2.0 (2023-05-30): Add bat pager to `-r` option; clean up; update dependencies
    * 5.3.0 (2023-05-31): Update dependencies; fix usage
        * 5.3.1 (2023-05-31): Fix changelog
    * 5.4.0 (2023-07-06): Neuter pager on windows; update dependencies
    * 5.5.0 (2023-07-22): Add `-n` / named format option; update dependencies
        * 5.5.1 (2023-07-22): Fix CLI examples
    * 5.6.0 (2023-07-27): Add "bcd" format and cargo-make install task; fix bug when using `-l` or `-z` with
      `-n rfc-3339`; update dependencies
    * 5.7.0 (2023-07-30): Use `bbd_lib::encode_bcd`; update dependencies
    * 5.8.0 (2023-09-09): Update dependencies
        * 5.8.1 (2023-10-21): Update dependencies
        * 5.8.2 (2024-07-23): Fix changelog; update dependencies; improve error handling; fix issue
          w/ chrono: maximum timestamp is now a year earlier:
            * Was: 8210298412799 (262143-12-31T23:59:59Z)
            * Now: 8210266876799 (262142-12-31T23:59:59Z)
* 6.0.0 (2024-07-25): Replace chrono and chrono-tz for jiff; maximum timestamp is now 253402207200
  (9999-12-30T22:00:00Z); update dependencies
    * 6.0.1 (2024-07-25): Fix documentation
    * 6.1.0 (2024-08-01): Add `Dtg::from_ymd_hms()` function to apply [BurntSushi's suggestion](https://github.com/BurntSushi/jiff/discussions/43#discussioncomment-10160135) to create a Timestamp from components instead of allocating a string and parsing it; improve `Dtg::from_x()`; fix changelog; update dependencies
        * 6.1.1 (2024-08-22): Update dependencies; add `commit` target to makefile
    * 6.2.0 (2024-08-22): Add `Dtg.elapsed()` method returning a new `Duration` struct (wrapper around a `jiff::Span`); fix makefile
    * 6.3.0 (2024-10-24): Add clap color; update dependencies
        * 6.3.1 (2024-12-04): Update dependencies
        * 6.3.2 (2025-02-20): Update dependencies
        * 6.3.3 (2025-04-16): Update dependencies
        * 6.3.4 (2025-04-16): Update dependencies
    * 6.4.0 (2025-08-28): Update dependencies; 2024 edition
        * 6.4.1 (2025-10-27): Update dependencies; use `pager2`

