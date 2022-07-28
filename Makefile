all: check build test

export RUSTFLAGS=-Dwarnings

CARGO_DOC_ARGS?=--open

doc:
	cargo +nightly doc \
	    --no-deps \
		--package soroban-sdk \
		--features docs,testutils \
		$(CARGO_DOC_ARGS)

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

watch-doc:
	cargo +nightly watch --clear --watch-when-idle --shell '$(MAKE) doc CARGO_DOC_ARGS='

fmt:
	cargo fmt --all

clean:
	cargo clean
	CARGO_TARGET_DIR=target-tiny cargo +nightly clean
