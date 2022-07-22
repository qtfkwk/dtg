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

README.md: README.master.md target/release/${NAME} Cargo.toml Cargo.lock
	@# Run each example command in README.master.md
	@perl -ne 'print;if(/^\$$ /){$$c=$$_;$$c=~s/^\$$ //;system "./target/release/$$c"}' <$< >$@
	cp $@ cli/

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

