LIB_CRATES = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name | startswith("test_") | not) | .name' | tr '\n' ' ')
TEST_CRATES = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name | startswith("test_")) | .name' | tr '\n' ' ')

MSRV = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "soroban-sdk") | .rust_version')
TEST_CRATES_RUSTUP_TOOLCHAIN?=$(MSRV)

CARGO_DOC_ARGS?=--open

default: test

doc: fmt
	cargo test --doc $(foreach c,$(LIB_CRATES),--package $(c)) --features testutils,alloc,hazmat
	cargo +nightly doc --no-deps $(foreach c,$(LIB_CRATES),--package $(c)) --all-features $(CARGO_DOC_ARGS)

test: fmt build-test-wasms test-only

test-only:
	cargo hack --feature-powerset --ignore-unknown-features --features testutils --exclude-features docs test

build: build-libs build-test-wasms

build-libs: fmt
	cargo hack build --release $(foreach c,$(LIB_CRATES),--package $(c))

build-test-wasms: fmt
	# Build the test wasms with MSRV by default, with some meta disabled for
	# binary stability for tests.
	RUSTUP_TOOLCHAIN=$(TEST_CRATES_RUSTUP_TOOLCHAIN) \
	RUSTFLAGS='--cfg soroban_sdk_internal_no_rssdkver_meta' \
		cargo hack build --release --target wasm32v1-none $(foreach c,$(TEST_CRATES),--package $(c)) ; \
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

# Expands the generated code within each test vector contract that lives in the
# tests/ directory. Serves to surface visible changes in generated code that
# may not be obvious when making changes to sdk macros.
expand-tests: build-test-wasms
	rm -fr tests-expanded
	mkdir -p tests-expanded
	for package in $(TEST_CRATES); do \
		echo "Expanding $$package for linux target including tests"; \
		cargo expand --package $$package --tests --target x86_64-unknown-linux-gnu | rustfmt > tests-expanded/$${package}_tests.rs; \
		echo "Expanding $$package for wasm32v1-none target without tests"; \
		RUSTFLAGS='--cfg soroban_sdk_internal_no_rssdkver_meta' \
			cargo expand --package $$package --release --target wasm32v1-none | rustfmt > tests-expanded/$${package}_wasm32v1-none.rs; \
	done

fmt:
	cargo fmt --all

clean:
	cargo clean

msrv:
	@echo $(MSRV)
