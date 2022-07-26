all: check build test

export RUSTFLAGS=-Dwarnings

doc:
	cargo +nightly doc --open --no-deps \
		--package stellar-contract-sdk \
		--features docs,testutils

test:
	cargo hack --feature-powerset --exclude-features docs test

build:
	cargo build --target wasm32-unknown-unknown --release
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
	cargo check --release --target wasm32-unknown-unknown

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

fmt:
	cargo fmt --all

clean:
	cargo clean
	CARGO_TARGET_DIR=target-tiny cargo +nightly clean
