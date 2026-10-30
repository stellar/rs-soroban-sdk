# Build and test the Go SDK and the soroban-go toolchain.
#
# Requires Go and TinyGo (https://tinygo.org). The parity tests also require
# the Rust test wasms, built from the repository root with `make build-test-wasms`
# or `cargo build --release --target wasm32v1-none -p test_<name>`.

TEST_CONTRACTS = $(wildcard tests/test_*)

all: install build-test-contracts test

install:
	go install ./cmd/soroban-go

generate-host:
	go run ./internal/genhost -env internal/genhost/env.json -out soroban/internal/host

build-test-contracts:
	for c in $(TEST_CONTRACTS); do go run ./cmd/soroban-go build $$c || exit 1; done

test: build-test-contracts
	go vet ./...
	go test ./...

# Deploy and invoke the test contracts on a network with the stellar-cli.
# NETWORK=local for a quickstart network, see scripts/e2e.sh.
e2e:
	scripts/e2e.sh $(or $(NETWORK),testnet)

.PHONY: all install generate-host build-test-contracts test e2e
