package soroban

import (
	"unsafe"

	"github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"
)

// Symbol is a short identifier string of up to 32 characters from the set
// [a-zA-Z0-9_]. Symbols of up to 9 characters are stored inline in the Val.
type Symbol struct {
	val Val
}

// MaxSmallSymbolLen is the maximum length of a symbol stored inline in a Val.
const MaxSmallSymbolLen = 9

// MaxSymbolLen is the maximum length of a symbol.
const MaxSymbolLen = 32

const symbolCodeBits = 6

// SymbolShort returns a symbol of up to 9 characters, computed without calling
// the host. It traps if the string is longer than 9 characters or contains
// invalid characters. It corresponds to symbol_short! in the Rust SDK.
func SymbolShort(s string) Symbol {
	v, ok := smallSymbolVal(s)
	if !ok {
		panicStr("invalid short symbol")
	}
	return Symbol{val: v}
}

// SymbolFromStr returns a symbol for the string, which must be at most 32
// characters from the set [a-zA-Z0-9_]. It corresponds to Symbol::new in the
// Rust SDK.
func SymbolFromStr(s string) Symbol {
	if v, ok := smallSymbolVal(s); ok {
		return Symbol{val: v}
	}
	if len(s) > MaxSymbolLen {
		panicStr("symbol too long")
	}
	for i := 0; i < len(s); i++ {
		if symbolCharCode(s[i]) == 0 {
			panicStr("invalid symbol character")
		}
	}
	return Symbol{val: Val(host.SymbolNewFromLinearMemory(strPtr(s), uint64(valFromU32(uint32(len(s))))))}
}

func symbolCharCode(c byte) uint64 {
	switch {
	case c == '_':
		return 1
	case c >= '0' && c <= '9':
		return 2 + uint64(c-'0')
	case c >= 'A' && c <= 'Z':
		return 12 + uint64(c-'A')
	case c >= 'a' && c <= 'z':
		return 38 + uint64(c-'a')
	}
	return 0
}

func smallSymbolVal(s string) (Val, bool) {
	if len(s) > MaxSmallSymbolLen {
		return 0, false
	}
	var accum uint64
	for i := 0; i < len(s); i++ {
		code := symbolCharCode(s[i])
		if code == 0 {
			return 0, false
		}
		accum = accum<<symbolCodeBits | code
	}
	return ValFromBody(TagSymbolSmall, accum), true
}

// String returns the symbol as a Go string.
func (s Symbol) String() string {
	if s.val.Tag() == TagSymbolSmall {
		var buf [MaxSmallSymbolLen]byte
		n := 0
		body := s.val.Body()
		for i := MaxSmallSymbolLen - 1; i >= 0; i-- {
			code := byte((body >> (uint(i) * symbolCodeBits)) & 63)
			var c byte
			switch {
			case code == 0:
				continue
			case code == 1:
				c = '_'
			case code < 12:
				c = '0' + code - 2
			case code < 38:
				c = 'A' + code - 12
			default:
				c = 'a' + code - 38
			}
			buf[n] = c
			n++
		}
		return string(buf[:n])
	}
	l := host.SymbolLen(uint64(s.val))
	n := Val(l).Major()
	buf := make([]byte, n)
	if n > 0 {
		host.SymbolCopyToLinearMemory(uint64(s.val), uint64(valFromU32(0)), slicePtr(buf), uint64(valFromU32(n)))
	}
	return string(buf)
}

// Equal reports whether the symbols are equal.
func (s Symbol) Equal(o Symbol) bool {
	if s.val == o.val {
		return true
	}
	if s.val.Tag() == TagSymbolSmall || o.val.Tag() == TagSymbolSmall {
		return false
	}
	return host.ObjCmp(uint64(s.val), uint64(o.val)) == 0
}

// ToVal implements ToValer.
func (s Symbol) ToVal() Val { return s.val }

// FromVal implements FromValer.
func (s *Symbol) FromVal(v Val) {
	if t := v.Tag(); t != TagSymbolSmall && t != TagSymbolObject {
		conversionError()
	}
	s.val = v
}

// strPtr returns the linear memory position of the string as a U32Val.
func strPtr(s string) uint64 {
	if len(s) == 0 {
		return uint64(valFromU32(0))
	}
	return uint64(valFromU32(uint32(uintptr(unsafe.Pointer(unsafe.StringData(s))))))
}

// slicePtr returns the linear memory position of the slice as a U32Val.
func slicePtr[T any](b []T) uint64 {
	if len(b) == 0 {
		return uint64(valFromU32(0))
	}
	return uint64(valFromU32(uint32(uintptr(unsafe.Pointer(&b[0])))))
}

func u32Val(x uint32) uint64 { return uint64(valFromU32(x)) }

func u32FromRaw(raw uint64) uint32 { return Val(raw).Major() }

func boolFromRaw(raw uint64) bool { return Val(raw) == ValTrue }
