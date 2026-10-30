package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// Address is a universal opaque identifier of an account or a contract.
type Address struct {
	obj Val
}

// AddressFromString returns the Address for a strkey (G... or C...) string.
func AddressFromString(strkey String) Address {
	return Address{obj: Val(host.StrkeyToAddress(uint64(strkey.ToVal())))}
}

// AddressFromStr returns the Address for a strkey (G... or C...) Go string.
func AddressFromStr(strkey string) Address { return AddressFromString(StringFromStr(strkey)) }

// AddressFromStringBytes returns the Address for a strkey in bytes.
func AddressFromStringBytes(strkey Bytes) Address {
	return Address{obj: Val(host.StrkeyToAddress(uint64(strkey.ToVal())))}
}

// ToVal implements ToValer.
func (a Address) ToVal() Val {
	if a.obj == 0 {
		conversionError()
	}
	return a.obj
}

// FromVal implements FromValer.
func (a *Address) FromVal(v Val) {
	v.expectTag(TagAddressObject)
	a.obj = v
}

// RequireAuth ensures the Address has authorized the current invocation,
// including all its arguments. It traps if not authorized.
func (a Address) RequireAuth() { host.RequireAuth(uint64(a.ToVal())) }

// RequireAuthForArgs ensures the Address has authorized the current
// invocation with the args. It traps if not authorized.
func (a Address) RequireAuthForArgs(args Vec[Val]) {
	host.RequireAuthForArgs(uint64(a.ToVal()), args.handle())
}

// String returns the strkey of the Address.
func (a Address) String() String { return String{obj: Val(host.AddressToStrkey(uint64(a.ToVal())))} }

// Equal reports whether the addresses are equal.
func (a Address) Equal(o Address) bool { return host.ObjCmp(uint64(a.ToVal()), uint64(o.ToVal())) == 0 }

// Cmp compares the addresses, returning -1, 0 or 1.
func (a Address) Cmp(o Address) int { return int(host.ObjCmp(uint64(a.ToVal()), uint64(o.ToVal()))) }

// Executable is the executable behind an Address: the Wasm hash of a
// contract, the Stellar Asset Contract, or an account. Exactly one field is
// set.
type Executable struct {
	Wasm         *BytesN[N32]
	StellarAsset bool
	Account      bool
}

var executableCases = []string{"Wasm", "StellarAsset", "Account"}

// ToVal implements ToValer.
func (e Executable) ToVal() Val {
	switch {
	case e.Wasm != nil:
		return UnionToVal("Wasm", e.Wasm.ToVal())
	case e.StellarAsset:
		return UnionToVal("StellarAsset")
	case e.Account:
		return UnionToVal("Account")
	}
	conversionError()
	return ValVoid
}

// FromVal implements FromValer.
func (e *Executable) FromVal(v Val) {
	idx, vec := UnionFromVal(v, executableCases)
	*e = Executable{}
	switch idx {
	case 0:
		var f [1]Val
		UnionCaseFromVal(vec, f[:])
		h := FromVal[BytesN[N32]](f[0])
		e.Wasm = &h
	case 1:
		UnionCaseFromVal(vec, nil)
		e.StellarAsset = true
	case 2:
		UnionCaseFromVal(vec, nil)
		e.Account = true
	default:
		conversionError()
	}
}

// Executable returns the executable of the Address, and false if the address
// does not exist.
func (a Address) Executable() (Executable, bool) {
	r := Val(host.GetAddressExecutable(uint64(a.ToVal())))
	if r.IsVoid() {
		return Executable{}, false
	}
	return FromVal[Executable](r), true
}

// Exists reports whether the Address exists on the ledger.
func (a Address) Exists() bool {
	_, ok := a.Executable()
	return ok
}

// MuxedAddress is an Address that may carry a 64-bit multiplexing id. It
// accepts both Address and muxed address Vals.
type MuxedAddress struct {
	obj Val
}

// MuxedAddressFromAddress returns a MuxedAddress with no id.
func MuxedAddressFromAddress(a Address) MuxedAddress { return MuxedAddress{obj: a.ToVal()} }

// ToVal implements ToValer.
func (m MuxedAddress) ToVal() Val { return m.obj }

// FromVal implements FromValer.
func (m *MuxedAddress) FromVal(v Val) {
	if t := v.Tag(); t != TagAddressObject && t != TagMuxedAddressObject {
		conversionError()
	}
	m.obj = v
}

// Address returns the Address without the id.
func (m MuxedAddress) Address() Address {
	if m.obj.Tag() == TagAddressObject {
		return Address{obj: m.obj}
	}
	return Address{obj: Val(host.GetAddressFromMuxedAddress(uint64(m.obj)))}
}

// ID returns the multiplexing id, if any.
func (m MuxedAddress) ID() Option[uint64] {
	if m.obj.Tag() == TagAddressObject {
		return None[uint64]()
	}
	return Some(valToU64(Val(host.GetIdFromMuxedAddress(uint64(m.obj)))))
}
