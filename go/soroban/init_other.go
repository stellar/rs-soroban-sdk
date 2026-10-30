//go:build !tinygo.wasm

package soroban

// EnsureInit initializes the Go runtime on the first call into a contract. It
// is a no-op when not compiled to a contract.
func EnsureInit() {}
