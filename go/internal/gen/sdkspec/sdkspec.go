// Package sdkspec holds the spec entries of the SDK's own contract types, such
// as the custom account auth types, as the Rust SDK defines them.
package sdkspec

import (
	_ "embed"

	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

//go:embed sdk.xdr
var sdkXDR []byte

// Names maps the Go SDK's contract types to their qualified Rust SDK names.
var Names = map[string]string{
	"Context":                     "::soroban_sdk::auth::Context",
	"ContractContext":             "::soroban_sdk::auth::ContractContext",
	"CreateContractHostFnContext": "::soroban_sdk::auth::CreateContractHostFnContext",
	"CreateContractWithConstructorHostFnContext": "::soroban_sdk::auth::CreateContractWithConstructorHostFnContext",
	"SubContractInvocation":                      "::soroban_sdk::auth::SubContractInvocation",
	"InvokerContractAuthEntry":                   "::soroban_sdk::auth::InvokerContractAuthEntry",
	"ContractExecutable":                         "::soroban_sdk::ContractExecutable",
	"ContractExecutableRef":                      "::soroban_sdk::ContractExecutableRef",
	"Executable":                                 "::soroban_sdk::address::Executable",
}

// Entries returns the spec entries of the SDK's contract types.
func Entries() []xdr.Entry {
	e, err := xdr.DecodeEntries(sdkXDR)
	if err != nil {
		panic(err)
	}
	return e
}
