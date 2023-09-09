# Changelog

* 1.0.2: Initial public release
* 2.0.0
    * General improvements to readme and code
    * Replaced `-u` with `-l` and ability to get local timezone via the
      [iana-time-zone crate](https://crates.io/crates/iana-time-zone)
* 2.0.1: Improve readme (add `-f` examples...); `s/-r/-l/` in usage
* 2.1.0: Enable bundling options (`-a -z TZ`: `-az TZ`, `-a -l`: `-al`, `-l -a`: `-la`,
  `-l -f FORMAT`: `-lf FORMAT`)
* 2.2.0: Add `-x` option
* 2.2.1: Fix `-x` option year from hex to base 60
* 2.2.2: Update doc; allow `-ax`, `-axz`, `-xa`, `-xaz`
* 2.2.3: Enable multiple `-f` options
* 2.2.4: Add `-X` option
* 3.0.0: Rewrite with structopt; add `-Z`
* 3.1.0: Add `--readme`
* 3.2.0: Improve doc
* 3.2.1: Fix `-a` option (yanked)
* 3.2.2: Fix readme
* 3.3.0: Improve doc; upgrade dependencies
* 3.3.1: Fix tables in readme
* 3.4.0: Catch overflows; upgrade dependencies
* 3.5.0: Multiple timezones; cargo fmt; upgrade dependencies
* 3.6.0: Separator `-s` option
* 3.7.0: Interval `-i`, `-c` options
* 4.0.0
    * Redesign as lib and cli crates in a workspace
    * Replace structopt with clap v3 derive API
* 4.1.0: Added `Dtg::{a_format,x_format,default}`, `Format::custom` methods to lib
* 4.1.1: Fix broken `-i`, `-c` in cli
* 4.1.2: Generate readme files
* 4.1.3: Update dependencies (see
  [comment](https://github.com/chronotope/chrono/issues/602#issuecomment-1242149249))
* 5.0.0: Update dependencies (clap v3 to v4, chrono 0.4.22 to 0.4.23); apply clippy fixes; replace
  `Makefile` with a cargo-make `Makefile.toml`
* 5.1.0: Update dependencies; fix readme
* 5.2.0: Add bat pager to `-r` option; clean up; update dependencies
* 5.3.0: Update dependencies; fix usage
* 5.3.1: Fix changelog
* 5.4.0: Neuter pager on windows; update dependencies
* 5.5.0: Add `-n` / named format option; update dependencies
* 5.5.1: Fix CLI examples
* 5.6.0: Add "bcd" format and cargo-make install task; fix bug when using `-l` or `-z` with
  `-n rfc-3339`; update dependencies
* 5.7.0: Use `bbd_lib::encode_bcd`; update dependencies
* 5.8.0: Update dependencies

