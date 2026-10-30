// Package soroban is the Go port of the Rust soroban-sdk crate. It provides the
// types and functions for writing Soroban smart contracts in Go, compiled to
// Wasm with TinyGo by the soroban-go toolchain.
//
// A contract is a Go package main that declares a contract type and exports
// methods on it:
//
//	//soroban:contract
//	type Contract struct{}
//
//	func (Contract) Hello(env soroban.Env, to soroban.Symbol) soroban.Vec[soroban.Symbol] {
//		return soroban.VecOf(soroban.SymbolFromStr("Hello"), to)
//	}
//
// Build it with:
//
//	soroban-go build ./path/to/contract
package soroban

// Val is the 64-bit value type that crosses the boundary between a contract
// and the host. The low 8 bits are a tag and the high 56 bits are the body.
type Val uint64

// Tag is the low 8 bits of a Val that identify the type of the value.
type Tag uint8

const (
	TagFalse          Tag = 0
	TagTrue           Tag = 1
	TagVoid           Tag = 2
	TagError          Tag = 3
	TagU32Val         Tag = 4
	TagI32Val         Tag = 5
	TagU64Small       Tag = 6
	TagI64Small       Tag = 7
	TagTimepointSmall Tag = 8
	TagDurationSmall  Tag = 9
	TagU128Small      Tag = 10
	TagI128Small      Tag = 11
	TagU256Small      Tag = 12
	TagI256Small      Tag = 13
	TagSymbolSmall    Tag = 14

	TagU64Object          Tag = 64
	TagI64Object          Tag = 65
	TagTimepointObject    Tag = 66
	TagDurationObject     Tag = 67
	TagU128Object         Tag = 68
	TagI128Object         Tag = 69
	TagU256Object         Tag = 70
	TagI256Object         Tag = 71
	TagBytesObject        Tag = 72
	TagStringObject       Tag = 73
	TagSymbolObject       Tag = 74
	TagVecObject          Tag = 75
	TagMapObject          Tag = 76
	TagAddressObject      Tag = 77
	TagMuxedAddressObject Tag = 78
	TagExecutableObject   Tag = 79
)

const (
	tagBits   = 8
	tagMask   = (1 << tagBits) - 1
	bodyBits  = 64 - tagBits
	minorBits = 24
	majorBits = 32
	minorMask = (1 << minorBits) - 1

	maxSmallU64 = (uint64(1) << bodyBits) - 1
	minSmallI64 = -(int64(1) << (bodyBits - 1))
	maxSmallI64 = (int64(1) << (bodyBits - 1)) - 1
)

// Well known constant Vals.
const (
	ValFalse Val = Val(TagFalse)
	ValTrue  Val = Val(TagTrue)
	ValVoid  Val = Val(TagVoid)
)

// Tag returns the tag of the Val.
func (v Val) Tag() Tag { return Tag(v & tagMask) }

// Body returns the 56-bit body of the Val.
func (v Val) Body() uint64 { return uint64(v) >> tagBits }

// SignedBody returns the 56-bit body of the Val sign-extended to 64 bits.
func (v Val) SignedBody() int64 { return int64(v) >> tagBits }

// Major returns the high 32 bits of the body.
func (v Val) Major() uint32 { return uint32(uint64(v) >> (tagBits + minorBits)) }

// Minor returns the low 24 bits of the body.
func (v Val) Minor() uint32 { return uint32((uint64(v) >> tagBits) & minorMask) }

// IsVoid reports whether the Val is void.
func (v Val) IsVoid() bool { return v.Tag() == TagVoid }

// IsObject reports whether the Val is a handle to a host object.
func (v Val) IsObject() bool { t := v.Tag(); return t >= TagU64Object && t <= TagExecutableObject }

// ToVal implements ToValer.
func (v Val) ToVal() Val { return v }

// FromVal implements FromValer.
func (v *Val) FromVal(x Val) { *v = x }

// ValFromBody constructs a Val from a tag and a body.
func ValFromBody(tag Tag, body uint64) Val { return Val(body<<tagBits | uint64(tag)) }

// ValFromMajorMinor constructs a Val from a tag and major and minor parts.
func ValFromMajorMinor(tag Tag, major, minor uint32) Val {
	return Val(uint64(major)<<(tagBits+minorBits) | uint64(minor&minorMask)<<tagBits | uint64(tag))
}

func valFromBool(b bool) Val {
	if b {
		return ValTrue
	}
	return ValFalse
}

func valFromU32(x uint32) Val { return ValFromMajorMinor(TagU32Val, x, 0) }
func valFromI32(x int32) Val  { return ValFromMajorMinor(TagI32Val, uint32(x), 0) }

// expectTag traps if the Val does not have the tag.
func (v Val) expectTag(t Tag) {
	if v.Tag() != t {
		conversionError()
	}
}
