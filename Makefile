LIB_CRATES = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name | startswith("test_") | not) | .name' | tr '\n' ' ')
TEST_CRATES = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name | startswith("test_")) | .name' | tr '\n' ' ')

MSRV = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "soroban-sdk") | .rust_version')
TEST_CRATES_RUST_VERSION?="+$(MSRV)"

all: check test

export RUSTFLAGS=-Dwarnings

CARGO_DOC_ARGS?=--open

doc: fmt
	cargo test --doc $(foreach c,$(LIB_CRATES),--package $(c)) --features testutils,alloc,hazmat
	cargo +nightly doc --no-deps $(foreach c,$(LIB_CRATES),--package $(c)) --all-features $(CARGO_DOC_ARGS)

test: fmt build-test-wasms
	cargo hack --feature-powerset --ignore-unknown-features --features testutils --exclude-features docs test

build: build-libs build-test-wasms

build-libs: fmt
	cargo hack build --release $(foreach c,$(LIB_CRATES),--package $(c))

build-test-wasms: fmt
	# Build the test wasms with MSRV with some meta disabled for binary stability for tests.
	RUSTFLAGS='--cfg soroban_sdk_internal_no_rssdkver_meta' \
		cargo $(TEST_CRATES_RUST_VERSION) hack build --release --target wasm32v1-none $(foreach c,$(TEST_CRATES),--package $(c)) ; \
	cd target/wasm32v1-none/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

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

print-features:
	cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | "Crate: \(.name)\n" + (.features | to_entries | map("  \(.key) -> \(.value)") | join("\n"))'
