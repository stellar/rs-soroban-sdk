LIB_CRATES = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.publish == null) | .name' | tr '\n' ' ')
TEST_CRATES = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name | startswith("test_")) | .name' | tr '\n' ' ')

MSRV = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name == "soroban-sdk") | .rust_version')
TEST_CRATES_RUSTUP_TOOLCHAIN?=$(MSRV)

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
	SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2=1 \
		cargo hack --feature-powerset --ignore-unknown-features --features testutils \
			--exclude-features docs \
			--exclude-features hazmat-crypto \
			--exclude-features hazmat-address \
			test

build: build-libs build-test-wasms

build-libs: fmt
	cargo hack build --release $(foreach c,$(LIB_CRATES),--package $(c))

# Cargo derives rustc's `-C metadata` from each package's version, and that
# value seeds every symbol hash. Fat LTO merges the upstream modules in an
# order taken from those names, so bumping the workspace version reshuffles
# functions in the built wasms, and so in tests-expanded, without any code
# change. Pin the version for these builds so their output depends on the code
# alone. Registry dependencies keep their real versions, so a dependency bump
# still changes the output, as it should.
#
# Prefix a recipe with this. It restores Cargo.toml and Cargo.lock on exit,
# including when the build fails. The backups live under target/ so that a
# hard kill cannot leave an untracked file in the working tree.
PINNED_VERSION = 0.0.0
# Absolute paths throughout: a recipe may cd elsewhere before the trap runs.
PIN_WORKSPACE_VERSION = \
	mkdir -p $(CURDIR)/target && \
	cp $(CURDIR)/Cargo.toml $(CURDIR)/target/.Cargo.toml.unpinned && \
	cp $(CURDIR)/Cargo.lock $(CURDIR)/target/.Cargo.lock.unpinned && \
	trap 'mv -f $(CURDIR)/target/.Cargo.toml.unpinned $(CURDIR)/Cargo.toml; \
	      mv -f $(CURDIR)/target/.Cargo.lock.unpinned $(CURDIR)/Cargo.lock' EXIT && \
	perl -0pi -e 'my ($$v) = /^\[workspace\.package\]\s*\nversion = "([^"]+)"/m; \
	              s/^version = "\Q$$v\E"$$/version = "$(PINNED_VERSION)"/mg; \
	              s/\{ version = "\Q$$v\E",/{ version = "$(PINNED_VERSION)",/g' \
		$(CURDIR)/Cargo.toml

build-test-wasms: fmt
	# Build the test wasms with MSRV by default, with some meta disabled for
	# binary stability for tests.
	$(PIN_WORKSPACE_VERSION) && \
	SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2=1 \
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
	$(PIN_WORKSPACE_VERSION) && \
	for package in $(TEST_CRATES); do \
		if [ "$$package" = "test_alloc" ]; then \
			continue; \
		fi; \
		echo "Expanding $$package for linux target including tests"; \
    RUSTUP_TOOLCHAIN=$(TEST_CRATES_RUSTUP_TOOLCHAIN) \
      RUSTFLAGS='--cfg soroban_sdk_internal_no_rssdkver_meta' \
      cargo expand --package $$package --tests --target x86_64-unknown-linux-gnu | rustfmt > tests-expanded/$${package}_tests.rs; \
		echo "Expanding $$package for wasm32v1-none target without tests"; \
    SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2=1 \
    RUSTUP_TOOLCHAIN=$(TEST_CRATES_RUSTUP_TOOLCHAIN) \
      RUSTFLAGS='--cfg soroban_sdk_internal_no_rssdkver_meta' \
			cargo expand --package $$package --release --target wasm32v1-none | rustfmt > tests-expanded/$${package}_wasm32v1-none.rs; \
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
