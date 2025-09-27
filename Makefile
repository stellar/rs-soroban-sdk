all: check test

export RUSTFLAGS=-Dwarnings

CARGO_DOC_ARGS?=--open
NATIVE_ONLY_CRATES:=soroban-spec soroban-spec-rust soroban-ledger-snapshot
NATIVE_PACKAGE_ARGS:=$(foreach i,$(NATIVE_ONLY_CRATES), --package $(i))
WASM_EXCLUDE_ARGS:=$(foreach i,$(NATIVE_ONLY_CRATES), --exclude $(i))

doc: fmt
	cargo test --doc -p soroban-sdk -p soroban-sdk-macros --features testutils,hazmat
	cargo +nightly doc -p soroban-sdk --no-deps --all-features $(CARGO_DOC_ARGS)

test: fmt build
	cargo hack --feature-powerset --ignore-unknown-features --features testutils --exclude-features docs test

build: fmt
	cargo hack build --release $(NATIVE_PACKAGE_ARGS)
	cargo hack build --target wasm32v1-none --release --workspace $(WASM_EXCLUDE_ARGS)
	cd target/wasm32v1-none/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

check: build fmt
	cargo hack --feature-powerset --exclude-features docs check
	cargo hack check --release --target wasm32v1-none --workspace $(WASM_EXCLUDE_ARGS)

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
expand-tests:
	rm -fr tests-expanded
	mkdir -p tests-expanded
	cargo metadata --format-version 1 | jq -r '.packages[] | select(.manifest_path | contains("/tests/")) | "\(.name) \(.manifest_path | split("/") | .[:-1] | join("/")) \(any(.targets[]; any(.kind[]; . == "cdylib")))"' | while read package dir is_cdylib; do \
		echo "Expanding $$package for host target including tests"; \
		cargo expand --package $$package --tests --target x86_64-unknown-linux-gnu | rustfmt > tests-expanded/$${package}_tests.rs; \
		if [ "$$is_cdylib" = "true" ]; then \
			echo "Expanding $$package for wasm32v1-none target without tests"; \
			cargo expand --package $$package --release --target wasm32v1-none | rustfmt > tests-expanded/$${package}_wasm32v1-none.rs; \
		fi; \
	done

fmt:
	cargo fmt --all

clean:
	cargo clean
