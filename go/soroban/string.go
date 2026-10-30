package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// String is a string stored in the host.
type String struct {
	obj Val
}

// StringFromStr returns a String containing the Go string.
func StringFromStr(s string) String {
	return String{obj: Val(host.StringNewFromLinearMemory(strPtr(s), u32Val(uint32(len(s)))))}
}

// StringFromBytes returns a String containing the bytes.
func StringFromBytes(b Bytes) String { return b.ToString() }

func (s String) handle() uint64 {
	if s.obj == 0 {
		return host.StringNewFromLinearMemory(u32Val(0), u32Val(0))
	}
	return uint64(s.obj)
}

// ToVal implements ToValer.
func (s String) ToVal() Val { return Val(s.handle()) }

// FromVal implements FromValer.
func (s *String) FromVal(v Val) {
	v.expectTag(TagStringObject)
	s.obj = v
}

// Len returns the length of the string in bytes.
func (s String) Len() uint32 { return u32FromRaw(host.StringLen(s.handle())) }

// IsEmpty reports whether the string is empty.
func (s String) IsEmpty() bool { return s.Len() == 0 }

// CopyInto copies the string's bytes into the slice, which must be the length
// of the string.
func (s String) CopyInto(b []byte) {
	if len(b) == 0 {
		return
	}
	host.StringCopyToLinearMemory(s.handle(), u32Val(0), slicePtr(b), u32Val(uint32(len(b))))
}

// String returns the string as a Go string.
func (s String) String() string {
	b := make([]byte, s.Len())
	s.CopyInto(b)
	return string(b)
}

// ToBytes returns the string's bytes.
func (s String) ToBytes() Bytes { return Bytes{obj: Val(host.StringToBytes(s.handle()))} }

// Equal reports whether the strings are equal.
func (s String) Equal(o String) bool { return host.ObjCmp(s.handle(), o.handle()) == 0 }
