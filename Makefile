all: check test

export RUSTFLAGS=-Dwarnings

CARGO_DOC_ARGS?=--open

doc: fmt
	cargo test --doc -p soroban-sdk -p soroban-sdk-macros --features testutils,hazmat
	cargo +nightly doc -p soroban-sdk --no-deps --all-features $(CARGO_DOC_ARGS)

test: fmt build
	cargo hack --feature-powerset --ignore-unknown-features --features testutils --exclude-features docs test

build: fmt
	cargo hack build --target wasm32-unknown-unknown --release
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

check: build fmt
	cargo hack --feature-powerset --exclude-features docs check
	cargo hack check --release --target wasm32-unknown-unknown

build-fuzz:
	cd tests/fuzz/fuzz && cargo +nightly fuzz check

readme:
	cd soroban-sdk \
		&& cargo +nightly rustdoc --features testutils -- -Zunstable-options -wjson \
		&& cat ../target/doc/soroban_sdk.json \
		| jq -r '.index[.root|tostring].docs' \
		> README.md

fmt:
	cargo fmt --all

clean:
	cargo clean
