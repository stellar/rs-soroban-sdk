//go:build tinygo.wasm

package soroban

// wasmEntryReactor is the TinyGo runtime's _initialize entry point. It
// initializes the heap and runs package initializers.
//
//export _initialize
func wasmEntryReactor()

var initialized bool

// EnsureInit initializes the Go runtime on the first call into a contract.
// The Soroban VM does not call the _initialize export TinyGo generates and
// does not allow a start function, so the code soroban-go generates for each
// contract function calls EnsureInit before anything else.
func EnsureInit() {
	if !initialized {
		initialized = true
		wasmEntryReactor()
	}
}
