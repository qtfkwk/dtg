NAME=$(shell basename $(realpath .))

all: check build doc
	@echo $@ done

check: .check
	@echo $@ done

.check: Cargo.lock Cargo.toml
	cargo outdated --exit-code 1
	@touch $@

build: README.md target/release/${NAME}
	@echo $@ done

target/release/${NAME}: $(shell find */src -type f) Makefile Cargo.toml Cargo.lock
	cargo build --release

README.md: README.master.md cli/README.md lib/README.md target/release/${NAME} Cargo.toml Cargo.lock
	@perl -ne 'print;if(/^\$$ /){$$c=$$_;$$c=~s/^\$$ //;system "./target/release/$$c"}' <$< >$@
	@sed -n $$(($$(grep -n '^# Examples$$' cli/README.md |cut -d: -f1)+2)),$$(($$(grep -n '^# Formats$$' cli/README.md |cut -d: -f1)-1))p cli/README.md >>$@
	@echo "# Library\n" >>$@
	@sed -n $$(($$(grep -n '^# Examples$$' lib/README.md |cut -d: -f1)+2)),$$(($$(grep -n '^# Formats$$' lib/README.md |cut -d: -f1)-1))p lib/README.md >>$@
	@cat FORMATS.md >>$@

cli/README.md: cli/README.master.md target/release/${NAME} Cargo.toml Cargo.lock
	@perl -ne 'print;if(/^\$$ /){$$c=$$_;$$c=~s/^\$$ //;system "./target/release/$$c"}' <$< >$@
	@perl -ne 'print;if(/^\$$ /){$$c=$$_;$$c=~s/^\$$ //;system "./target/release/$$c"}' <cli/EXAMPLES.md >>$@
	@cat FORMATS.md >>$@

lib/README.md: lib/README.master.md target/release/${NAME} Cargo.toml Cargo.lock
	@cp $< $@
	@cat lib/EXAMPLES.md >>$@
	@cat FORMATS.md >>$@

doc: .doc README.md
	@echo $@ done

.doc: $(shell find */src -type f)
	cargo doc --no-deps
	@touch $@

test:
	cargo test --release

install:
	cargo install --force --path cli
	@echo $@ done

uninstall:
	cargo uninstall ${NAME}
	@echo $@ done

clean:
	cargo clean
	rm -f .check .doc
	@echo $@ done

rebuild: clean build
	@echo $@ done

.PHONY: all check build doc test install uninstall clean rebuild

