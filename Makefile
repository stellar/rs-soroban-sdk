all: check test

export RUSTFLAGS=-Dwarnings

CARGO_DOC_ARGS?=--open

doc: fmt
	cargo test --doc -p soroban-sdk -p soroban-sdk-macros --features testutils,hazmat
	cargo +nightly-2024-10-10 doc -p soroban-sdk --no-deps --all-features $(CARGO_DOC_ARGS)

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
	# Allow warnings when using nightly until https://github.com/rust-lang/rust/issues/131643 is fixed.
	cd tests/fuzz/fuzz && cargo +nightly-2024-10-10 fuzz check

readme:
	# Allow warnings when using nightly until https://github.com/rust-lang/rust/issues/131643 is fixed.
	cd soroban-sdk \
		&& cargo +nightly-2024-10-10 rustdoc -- -Zunstable-options -wjson \
		&& cat ../target/doc/soroban_sdk.json \
		| jq -r '.index[.root|tostring].docs' \
		> README.md

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

watch-doc:
	cargo +nightly-2024-10-10 watch --clear --watch-when-idle --shell '$(MAKE) doc CARGO_DOC_ARGS='

fmt:
	cargo fmt --all

clean:
	cargo clean

bump-version:
	cargo workspaces version --all --force '*' --no-git-commit --yes custom $(VERSION)

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
