LIB_CRATES = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.publish == null) | .name' | tr '\n' ' ')
TEST_CRATES = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name | startswith("test_")) | .name' | tr '\n' ' ')

MSRV = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "soroban-sdk") | .rust_version')
TEST_CRATES_RUSTUP_TOOLCHAIN?=$(MSRV)

VERSION_MAJOR = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "soroban-sdk") | .version | split(".")[0]')

CARGO_DOC_ARGS?=--open

default: test

doc: fmt
	cargo test --doc $(foreach c,$(LIB_CRATES),--package $(c)) --features testutils,alloc,hazmat
	$(MAKE) doc-only

# Build the docs for all the library crates, without running the doc tests.
doc-only:
	cargo +nightly doc --no-deps $(foreach c,$(LIB_CRATES),--package $(c)) --all-features $(CARGO_DOC_ARGS)

test: fmt build-test-wasms test-only

# Run tests. 
# The docs feature is excluded because it is a market for docs builds. The
# hazmat granular features are excluded because all hazmat features are tested
# together with the umbrella hazmat feature.
test-only:
	STELLAR_CLI_VERSION_MAJOR=$(VERSION_MAJOR) \
		cargo hack --feature-powerset --ignore-unknown-features --features testutils \
			--exclude-features docs \
			--exclude-features hazmat-crypto \
			--exclude-features hazmat-address \
			test

build: build-libs build-test-wasms

build-libs: fmt
	cargo hack build --release $(foreach c,$(LIB_CRATES),--package $(c))

build-test-wasms: fmt
	# Build the test wasms with MSRV by default, with some meta disabled for
	# binary stability for tests.
	STELLAR_CLI_VERSION_MAJOR=$(VERSION_MAJOR) \
	RUSTUP_TOOLCHAIN=$(TEST_CRATES_RUSTUP_TOOLCHAIN) \
	RUSTFLAGS='--cfg soroban_sdk_internal_no_rssdkver_meta' \
		cargo hack build --release --target wasm32v1-none $(foreach c,$(TEST_CRATES),--package $(c)) ; \
	cd target/wasm32v1-none/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

# Builds the fuzz tests. Requires cargo-fuzz and cargo-afl.
build-fuzz:
	cd tests/fuzz/fuzz && cargo +nightly fuzz check
	cd tests/fuzz_afl/fuzz && cargo afl build

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
		if [ "$$package" = "test_alloc" ]; then \
			continue; \
		fi; \
		echo "Expanding $$package for linux target including tests"; \
    RUSTUP_TOOLCHAIN=$(TEST_CRATES_RUSTUP_TOOLCHAIN) \
      RUSTFLAGS='--cfg soroban_sdk_internal_no_rssdkver_meta' \
      cargo expand --package $$package --tests --target x86_64-unknown-linux-gnu | rustfmt > tests-expanded/$${package}_tests.rs; \
		echo "Expanding $$package for wasm32v1-none target without tests"; \
    STELLAR_CLI_VERSION_MAJOR=$(VERSION_MAJOR) \
    RUSTUP_TOOLCHAIN=$(TEST_CRATES_RUSTUP_TOOLCHAIN) \
      RUSTFLAGS='--cfg soroban_sdk_internal_no_rssdkver_meta' \
			cargo expand --package $$package --release --target wasm32v1-none | rustfmt > tests-expanded/$${package}_wasm32v1-none.rs; \
	done

# Dumps the contractspecv0 section of each test vector contract in the tests/
# directory, as built by build-test-wasms and so before any spec shaking, as a
# pretty formatted JSON array of XDR-JSON values, sorted by kind and name then
# entry so that moving items around in the source does not reorder them. Serves
# to surface changes to the spec the SDK embeds, which the expanded code does
# not show because the spec is encoded from it rather than written out by it.
spec-snapshots: build-test-wasms
	$(MAKE) spec-snapshots-from-built-wasms

spec-snapshots-from-built-wasms:
	rm -fr tests-specs
	mkdir -p tests-specs
	for name in $(TEST_CRATES); do \
		wasm=target/wasm32v1-none/release/$$name.wasm; \
		[ -f "$$wasm" ] || continue; \
		echo "Dumping spec of $$name"; \
		cargo run --quiet --package spec-json -- $$wasm > tests-specs/$$name.json || exit 1; \
	done

miri:
	RUST_BACKTRACE=1 \
	MIRIFLAGS="-Zmiri-disable-isolation -Zmiri-strict-provenance" \
	PROPTEST_CASES=1 \
	cargo +nightly miri nextest run

fmt:
	cargo fmt --all

clean:
	cargo clean

msrv:
	@echo $(MSRV)

version-major:
	@echo $(VERSION_MAJOR)
