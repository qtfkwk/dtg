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

