package soroban

import (
	"unsafe"

	"github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"
)

// Env provides access to the environment the contract is executing within. It
// corresponds to Env in the Rust SDK. Env is a zero size value; contract
// functions may declare an Env as their first parameter to receive one, or
// construct one with Env{}.
type Env struct{}

// Storage returns access to the contract's data storage.
func (Env) Storage() Storage { return Storage{} }

// Events returns access to publishing events.
func (Env) Events() Events { return Events{} }

// Ledger returns access to information about the current ledger.
func (Env) Ledger() Ledger { return Ledger{} }

// Crypto returns access to cryptographic functions.
func (Env) Crypto() Crypto { return Crypto{} }

// Deployer returns access to deploying and updating contracts.
func (Env) Deployer() Deployer { return Deployer{} }

// Prng returns access to the pseudo-random number generator.
func (Env) Prng() Prng { return Prng{} }

// Logs returns access to diagnostic logging.
func (Env) Logs() Logs { return Logs{} }

// CurrentContractAddress returns the address of the currently executing
// contract.
func (Env) CurrentContractAddress() Address {
	return Address{obj: Val(host.GetCurrentContractAddress())}
}

// PanicWithError fails the current contract invocation with the error.
func (Env) PanicWithError(err ErrorValuer) { PanicWithError(err) }

// InvokeContract invokes the function on the contract with the args,
// converting the return value to T. If the invocation fails, the current
// invocation fails too.
func InvokeContract[T any](contract Address, fn Symbol, args Vec[Val]) T {
	return FromVal[T](Val(host.Call(uint64(contract.ToVal()), uint64(fn.ToVal()), args.handle())))
}

// TryInvokeContract invokes the function on the contract with the args. If the
// invocation fails the error is returned and the current invocation continues.
// Errors that are not contract errors may still abort the current invocation.
func TryInvokeContract[T any](contract Address, fn Symbol, args Vec[Val]) (T, *Error) {
	r := Val(host.TryCall(uint64(contract.ToVal()), uint64(fn.ToVal()), args.handle()))
	if r.Tag() == TagError {
		var zero T
		var e Error
		e.FromVal(r)
		return zero, &e
	}
	return FromVal[T](r), nil
}

// InvokeContract invokes the function on the contract with the args, returning
// the raw Val result.
func (Env) InvokeContract(contract Address, fn Symbol, args Vec[Val]) Val {
	return InvokeContract[Val](contract, fn, args)
}

// AuthorizeAsCurrentContract authorizes sub-contract calls made on behalf of
// the current contract. Each entry is an InvokerContractAuthEntry value, which
// can be constructed with SubContractInvocation.
func (Env) AuthorizeAsCurrentContract(entries Vec[InvokerContractAuthEntry]) {
	host.AuthorizeAsCurrContract(entries.handle())
}

// Ledger provides information about the current ledger.
type Ledger struct{}

// ProtocolVersion returns the protocol version of the ledger.
func (Ledger) ProtocolVersion() uint32 { return u32FromRaw(host.GetLedgerVersion()) }

// Sequence returns the sequence number of the ledger.
func (Ledger) Sequence() uint32 { return u32FromRaw(host.GetLedgerSequence()) }

// Timestamp returns the close time of the ledger in seconds since the epoch.
func (Ledger) Timestamp() uint64 { return valToU64(Val(host.GetLedgerTimestamp())) }

// MaxLiveUntilLedger returns the maximum ledger sequence that an entry can live
// until.
func (Ledger) MaxLiveUntilLedger() uint32 { return u32FromRaw(host.GetMaxLiveUntilLedger()) }

// NetworkID returns the network id, the SHA-256 hash of the network passphrase.
func (Ledger) NetworkID() BytesN[N32] {
	return BytesN[N32]{Bytes: Bytes{obj: Val(host.GetLedgerNetworkId())}}
}

// Logs provides diagnostic logging. Logs are only emitted as diagnostic
// events, which are recorded only when diagnostics are enabled.
type Logs struct{}

// Log emits a diagnostic event with the message and values.
func (Logs) Log(msg string, args ...any) {
	vals := make([]Val, len(args))
	for i, a := range args {
		vals[i] = ToVal(a)
	}
	host.LogFromLinearMemory(strPtr(msg), u32Val(uint32(len(msg))), slicePtr(vals), u32Val(uint32(len(vals))))
}

// Log emits a diagnostic event with the message and values. It corresponds to
// the log! macro in the Rust SDK.
func Log(msg string, args ...any) { Logs{}.Log(msg, args...) }

// Ensure the Val slice layout matches the host's expectation of 8 byte Vals.
var _ [8 - unsafe.Sizeof(Val(0))]struct{}
