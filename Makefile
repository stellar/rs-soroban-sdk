all: check test

export RUSTFLAGS=-Dwarnings

CARGO_TEST_SUBCOMMAND:=$(shell type -p cargo-nextest >/dev/null && echo nextest run || echo test)
CARGO_DOC_ARGS?=--open

doc: fmt
	cargo test --doc -p soroban-sdk -p soroban-sdk-macros -p soroban-auth --features testutils
	cargo +nightly doc -p soroban-sdk -p soroban-auth --no-deps --features docs,testutils $(CARGO_DOC_ARGS)

test: fmt build
	cargo hack --feature-powerset --ignore-unknown-features --features testutils --exclude-features docs $(CARGO_TEST_SUBCOMMAND)

build: fmt
	cargo hack build --target wasm32-unknown-unknown --release
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			ls -l "$$i"; \
		done

faa:
	@echo "----"
	cargo hack build -p test_1 -p test_2 -p test_3 --target wasm32-unknown-unknown --release
	@echo "----"
	cd target/wasm32-unknown-unknown/release/ && ls -l test_[0-9].wasm
	@echo "----"
	soroban deploy --id 1 --wasm target/wasm32-unknown-unknown/release/test_1.wasm
	soroban invoke --id 1 --fn set --arg 1000 --cost
	soroban read --id 1
	soroban invoke --id 1 --fn counter
	soroban invoke --id 1 --fn balance
	@echo "----"
	soroban deploy --id 2 --wasm target/wasm32-unknown-unknown/release/test_2.wasm
	soroban invoke --id 2 --fn set --arg 1000 --cost
	soroban read --id 2
	soroban invoke --id 2 --fn counter
	soroban invoke --id 2 --fn balance
	@echo "----"
	soroban deploy --id 3 --wasm target/wasm32-unknown-unknown/release/test_3.wasm
	soroban invoke --id 3 --fn set --arg 1000 --cost
	soroban read --id 3
	soroban invoke --id 3 --fn counter
	soroban invoke --id 3 --fn balance
	@echo "----"

build-optimized: fmt
	cargo +nightly hack build  --target wasm32-unknown-unknown --release \
		--workspace \
		--exclude soroban-spec \
		--exclude soroban-sdk \
		--exclude soroban-sdk-macros \
		--exclude soroban-auth \
		-Z build-std=std,panic_abort \
		-Z build-std-features=panic_immediate_abort
	cd target/wasm32-unknown-unknown/release/ && \
		for i in *.wasm ; do \
			wasm-opt -Oz "$$i" -o "$$i.tmp" && mv "$$i.tmp" "$$i"; \
			ls -l "$$i"; \
		done

check: build fmt
	cargo hack --feature-powerset --exclude-features docs check
	cargo hack check --release --target wasm32-unknown-unknown

readme:
	cd soroban-sdk \
		&& cargo +nightly rustdoc -- -Zunstable-options -wjson \
		&& cat ../target/doc/soroban_sdk.json \
		| jq -r '.index[.root].docs' \
		> README.md
	cd soroban-auth \
		&& cargo +nightly rustdoc -- -Zunstable-options -wjson \
		&& cat ../target/doc/soroban_auth.json \
		| jq -r '.index[.root].docs' \
		> README.md

watch:
	cargo watch --clear --watch-when-idle --shell '$(MAKE)'

watch-doc:
	cargo +nightly watch --clear --watch-when-idle --shell '$(MAKE) doc CARGO_DOC_ARGS='

fmt:
	cargo fmt --all

clean:
	cargo clean

bump-version:
	cargo workspaces version --all --force '*' --no-git-commit --yes custom $(VERSION)

publish:
	cargo workspaces publish --all --force '*' --from-git --yes
