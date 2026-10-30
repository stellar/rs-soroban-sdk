//go:build ignore

// Command extract writes the spec entries of the Rust SDK's own contract
// types, those named ::soroban_sdk::..., from a contract Wasm built with the
// Rust SDK whose spec references them, such as tests/account:
//
//	cargo build --release --target wasm32v1-none -p test_account
//	go run ./internal/gen/sdkspec/extract.go ../target/wasm32v1-none/release/test_account.wasm internal/gen/sdkspec/sdk.xdr
package main

import (
	"os"
	"strings"

	"github.com/stellar/rs-soroban-sdk/go/internal/wasm"
	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

func main() {
	b, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	data, _, err := wasm.CustomSection(b, "contractspecv0")
	if err != nil {
		panic(err)
	}
	entries, err := xdr.DecodeEntries(data)
	if err != nil {
		panic(err)
	}
	var out []byte
	for _, e := range entries {
		if strings.HasPrefix(e.Name, "::soroban_sdk::") {
			out = append(out, e.Encode()...)
		}
	}
	if err := os.WriteFile(os.Args[2], out, 0o644); err != nil {
		panic(err)
	}
}
