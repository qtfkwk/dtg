# all

* clippy
* test
* build
* doc

# check

* outdated
* audit

# update

* update-toml
* update-lock

# run

* `target/release/{dirname}`

```
target/release/{dirname}
```

# clippy

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo clippy -- -D clippy::all -D clippy::pedantic
```

# test

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo test
```

# build

* `target/release/{dirname}`

# `target/release/{dirname}`

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`
* `README.md`

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `**/*.rs`

```bash
set -xeo pipefail
cargo build --release
kapow cli/t/README.md >cli/README.md
kapow cli/EXAMPLES.md >>cli/README.md
cat FORMATS.md >>cli/README.md
kapow lib/t/README.md >lib/README.md
cat lib/EXAMPLES.md >>lib/README.md
cat FORMATS.md >>lib/README.md
kapow t/README.md >README.md
sed -n $(($(grep -n '^# Examples$' cli/README.md |cut -d: -f1)+2)),\
$(($(grep -n '^# Formats$' cli/README.md |cut -d: -f1)-1))p cli/README.md >>README.md
echo -e "# Library\n" >>README.md
sed -n $(($(grep -n '^# Examples$' lib/README.md |cut -d: -f1)+2)),\
$(($(grep -n '^# Formats$' lib/README.md |cut -d: -f1)-1))p lib/README.md >>README.md
cat FORMATS.md >>README.md
```

# doc

```
cargo doc
```

# outdated

```
cargo outdated --exit-code=1
```

# audit

```
cargo audit
```

# update-toml

```
cargo upgrade -i
```

# update-lock

```
cargo update
```

# install

* `README.md`

```
cargo install --path .
```

# uninstall

```
cargo uninstall {dirname}
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo dtg kapow tokei toml-cli
```

# clean

```
cargo clean
```

# cocomo

```bash -eo pipefail
tokei; echo
cocomo -o sloccount
cocomo
```

# commit

```bash
set -xeo pipefail
V=$(toml get -r lib/Cargo.toml package.version)
git commit -m "$V"
git tag -a "$V" -m "$V"
```

# publish

```
cargo publish -p dtg-lib
cargo publish -p dtg
git push
git push --tags
```

# full

* update
* check
* all
* install

