all: check test

export RUSTFLAGS=-Dwarnings

CARGO_TEST_SUBCOMMAND:=$(shell type -p cargo-nextest >/dev/null && echo nextest run || echo test)
CARGO_DOC_ARGS?=--open

doc: fmt
	cargo test --doc -p soroban-sdk -p soroban-sdk-macros -p soroban-auth --features testutils
	cargo +nightly doc -p soroban-sdk -p soroban-auth --no-deps --features docs,testutils $(CARGO_DOC_ARGS)

test: fmt build
	cargo hack --feature-powerset --exclude-features docs $(CARGO_TEST_SUBCOMMAND)

build: fmt
	cargo hack build --target wasm32-unknown-unknown --release
	cp target/wasm32-unknown-unknown/release/soroban_built_in_token_spec.wasm soroban-built-in-token-spec/

build-optimized: fmt
	CARGO_TARGET_DIR=target-tiny cargo +nightly hack build  --target wasm32-unknown-unknown --release \
		--workspace \
		--exclude soroban-spec \
		--exclude soroban-sdk \
		--exclude soroban-sdk-macros \
		--exclude soroban-auth \
		-Z build-std=std,panic_abort \
		-Z build-std-features=panic_immediate_abort
	cd target-tiny/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

	cp target-tiny/wasm32-unknown-unknown/release/soroban_built_in_token_spec.wasm soroban-built-in-token-spec/

check: build fmt
	cargo hack --feature-powerset --exclude-features docs check
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

bump-version:
	cargo workspaces version --all --force '*' --no-git-commit --yes custom $(VERSION)

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
