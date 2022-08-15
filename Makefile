all: check build test

export RUSTFLAGS=-Dwarnings

CARGO_TEST_SUBCOMMAND:=$(shell type -p cargo-nextest >/dev/null && echo nextest run || echo test)
CARGO_DOC_ARGS?=--open

doc: fmt
	cargo test --doc --features testutils
	cargo +nightly doc \
	    --no-deps \
		--package soroban-sdk \
		--features docs,testutils \
		$(CARGO_DOC_ARGS)

test: fmt build
	cargo hack --feature-powerset --exclude-features docs $(CARGO_TEST_SUBCOMMAND)

build: fmt
	cargo hack build --target wasm32-unknown-unknown --release
	CARGO_TARGET_DIR=target-tiny cargo +nightly build --target wasm32-unknown-unknown --release \
		-Z build-std=std,panic_abort \
		-Z build-std-features=panic_immediate_abort
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done
	cd target-tiny/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

check: fmt
	cargo hack --feature-powerset --exclude-features docs check --all-targets
	cargo hack check --release --target wasm32-unknown-unknown

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

watch-doc:
	cargo +nightly watch --clear --watch-when-idle --shell '$(MAKE) doc CARGO_DOC_ARGS='

fmt:
	cargo fmt --all

clean:
	cargo clean
	CARGO_TARGET_DIR=target-tiny cargo +nightly clean

# Build all projects as if they are being published to crates.io, and do so for
# all feature and target combinations.
publish-dry-run-sdk:
	cargo +stable hack --feature-powerset publish --locked --dry-run --exclude-features docs --package soroban-sdk
	cargo +stable hack --feature-powerset publish --locked --dry-run --exclude-features docs,testutils --package soroban-sdk --target wasm32-unknown-unknown

publish-dry-run-sdk-macros:
	cd macros && cargo +stable hack --feature-powerset publish --locked --dry-run --package soroban-sdk-macros

# Publish publishes the crate to crates.io. The dry-run is a dependency because
# the dry-run target will verify all feature set combinations.
publish-sdk: publish-dry-run-sdk
	cargo +stable publish --locked --package soroban-sdk

publish-sdk-macros: publish-dry-run-sdk-macros
	cd macros && cargo +stable publish --locked --package soroban-sdk-macros
