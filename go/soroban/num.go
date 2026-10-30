package soroban

import (
	"math/bits"

	"github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"
)

func valFromU64(x uint64) Val {
	if x <= maxSmallU64 {
		return ValFromBody(TagU64Small, x)
	}
	return Val(host.ObjFromU64(x))
}

func valToU64(v Val) uint64 {
	switch v.Tag() {
	case TagU64Small:
		return v.Body()
	case TagU64Object:
		return host.ObjToU64(uint64(v))
	}
	conversionError()
	return 0
}

func valFromI64(x int64) Val {
	if x >= minSmallI64 && x <= maxSmallI64 {
		return ValFromBody(TagI64Small, uint64(x))
	}
	return Val(host.ObjFromI64(x))
}

func valToI64(v Val) int64 {
	switch v.Tag() {
	case TagI64Small:
		return v.SignedBody()
	case TagI64Object:
		return host.ObjToI64(uint64(v))
	}
	conversionError()
	return 0
}

// Timepoint is a point in time, in seconds since the Unix epoch.
type Timepoint uint64

// ToVal implements ToValer.
func (t Timepoint) ToVal() Val {
	if uint64(t) <= maxSmallU64 {
		return ValFromBody(TagTimepointSmall, uint64(t))
	}
	return Val(host.TimepointObjFromU64(uint64(t)))
}

// FromVal implements FromValer.
func (t *Timepoint) FromVal(v Val) {
	switch v.Tag() {
	case TagTimepointSmall:
		*t = Timepoint(v.Body())
	case TagTimepointObject:
		*t = Timepoint(host.TimepointObjToU64(uint64(v)))
	default:
		conversionError()
	}
}

// Duration is a duration of time, in seconds.
type Duration uint64

// ToVal implements ToValer.
func (d Duration) ToVal() Val {
	if uint64(d) <= maxSmallU64 {
		return ValFromBody(TagDurationSmall, uint64(d))
	}
	return Val(host.DurationObjFromU64(uint64(d)))
}

// FromVal implements FromValer.
func (d *Duration) FromVal(v Val) {
	switch v.Tag() {
	case TagDurationSmall:
		*d = Duration(v.Body())
	case TagDurationObject:
		*d = Duration(host.DurationObjToU64(uint64(v)))
	default:
		conversionError()
	}
}

// U128 is an unsigned 128-bit integer.
type U128 struct {
	Hi, Lo uint64
}

// U128FromU64 returns a U128 with the value.
func U128FromU64(x uint64) U128 { return U128{Lo: x} }

// ToVal implements ToValer.
func (x U128) ToVal() Val {
	if x.Hi == 0 && x.Lo <= maxSmallU64 {
		return ValFromBody(TagU128Small, x.Lo)
	}
	return Val(host.ObjFromU128Pieces(x.Hi, x.Lo))
}

// FromVal implements FromValer.
func (x *U128) FromVal(v Val) {
	switch v.Tag() {
	case TagU128Small:
		*x = U128{Lo: v.Body()}
	case TagU128Object:
		*x = U128{Hi: host.ObjToU128Hi64(uint64(v)), Lo: host.ObjToU128Lo64(uint64(v))}
	default:
		conversionError()
	}
}

// Add returns x+y, trapping on overflow.
func (x U128) Add(y U128) U128 {
	lo, c := bits.Add64(x.Lo, y.Lo, 0)
	hi, c := bits.Add64(x.Hi, y.Hi, c)
	if c != 0 {
		overflow()
	}
	return U128{Hi: hi, Lo: lo}
}

// Mul returns x*y, trapping on overflow.
func (x U128) Mul(y U128) U128 {
	if x.Hi != 0 && y.Hi != 0 {
		overflow()
	}
	hi, lo := bits.Mul64(x.Lo, y.Lo)
	h1, l1 := bits.Mul64(x.Hi, y.Lo)
	h2, l2 := bits.Mul64(x.Lo, y.Hi)
	if h1 != 0 || h2 != 0 {
		overflow()
	}
	hi, c := bits.Add64(hi, l1, 0)
	if c != 0 {
		overflow()
	}
	hi, c = bits.Add64(hi, l2, 0)
	if c != 0 {
		overflow()
	}
	return U128{Hi: hi, Lo: lo}
}

// Sub returns x-y, trapping on underflow.
func (x U128) Sub(y U128) U128 {
	if x.Cmp(y) < 0 {
		overflow()
	}
	lo := x.Lo - y.Lo
	borrow := uint64(0)
	if x.Lo < y.Lo {
		borrow = 1
	}
	return U128{Hi: x.Hi - y.Hi - borrow, Lo: lo}
}

// Cmp returns -1, 0, or 1 if x is less than, equal to, or greater than y.
func (x U128) Cmp(y U128) int {
	switch {
	case x.Hi < y.Hi:
		return -1
	case x.Hi > y.Hi:
		return 1
	case x.Lo < y.Lo:
		return -1
	case x.Lo > y.Lo:
		return 1
	}
	return 0
}

// I128 is a signed 128-bit integer.
type I128 struct {
	Hi int64
	Lo uint64
}

// I128FromI64 returns an I128 with the value.
func I128FromI64(x int64) I128 {
	hi := int64(0)
	if x < 0 {
		hi = -1
	}
	return I128{Hi: hi, Lo: uint64(x)}
}

// IsI64 reports whether the value fits in an int64.
func (x I128) IsI64() bool {
	return (x.Hi == 0 && int64(x.Lo) >= 0) || (x.Hi == -1 && int64(x.Lo) < 0)
}

// I64 returns the value as an int64, trapping if it does not fit.
func (x I128) I64() int64 {
	if !x.IsI64() {
		overflow()
	}
	return int64(x.Lo)
}

// ToVal implements ToValer.
func (x I128) ToVal() Val {
	if x.IsI64() {
		i := int64(x.Lo)
		if i >= minSmallI64 && i <= maxSmallI64 {
			return ValFromBody(TagI128Small, uint64(i))
		}
	}
	return Val(host.ObjFromI128Pieces(x.Hi, x.Lo))
}

// FromVal implements FromValer.
func (x *I128) FromVal(v Val) {
	switch v.Tag() {
	case TagI128Small:
		*x = I128FromI64(v.SignedBody())
	case TagI128Object:
		*x = I128{Hi: host.ObjToI128Hi64(uint64(v)), Lo: host.ObjToI128Lo64(uint64(v))}
	default:
		conversionError()
	}
}

// IsNeg reports whether x is negative.
func (x I128) IsNeg() bool { return x.Hi < 0 }

// Add returns x+y, trapping on overflow.
func (x I128) Add(y I128) I128 {
	lo := x.Lo + y.Lo
	carry := int64(0)
	if lo < x.Lo {
		carry = 1
	}
	hi := x.Hi + y.Hi + carry
	// Overflow if operands have the same sign and the result's sign differs.
	if (x.Hi >= 0) == (y.Hi >= 0) && (hi >= 0) != (x.Hi >= 0) {
		overflow()
	}
	return I128{Hi: hi, Lo: lo}
}

// Neg returns -x, trapping on overflow.
func (x I128) Neg() I128 {
	if x.Hi == -1<<63 && x.Lo == 0 {
		overflow()
	}
	lo := ^x.Lo + 1
	hi := ^x.Hi
	if lo == 0 {
		hi++
	}
	return I128{Hi: hi, Lo: lo}
}

// Sub returns x-y, trapping on overflow.
func (x I128) Sub(y I128) I128 {
	lo := x.Lo - y.Lo
	borrow := int64(0)
	if x.Lo < y.Lo {
		borrow = 1
	}
	hi := x.Hi - y.Hi - borrow
	if (x.Hi >= 0) != (y.Hi >= 0) && (hi >= 0) != (x.Hi >= 0) {
		overflow()
	}
	return I128{Hi: hi, Lo: lo}
}

// Cmp returns -1, 0, or 1 if x is less than, equal to, or greater than y.
func (x I128) Cmp(y I128) int {
	switch {
	case x.Hi < y.Hi:
		return -1
	case x.Hi > y.Hi:
		return 1
	case x.Lo < y.Lo:
		return -1
	case x.Lo > y.Lo:
		return 1
	}
	return 0
}

// U256 is an unsigned 256-bit integer, stored as four 64-bit pieces from most
// to least significant.
type U256 struct {
	HiHi, HiLo, LoHi, LoLo uint64
}

// U256FromU64 returns a U256 with the value.
func U256FromU64(x uint64) U256 { return U256{LoLo: x} }

// ToVal implements ToValer.
func (x U256) ToVal() Val {
	if x.HiHi == 0 && x.HiLo == 0 && x.LoHi == 0 && x.LoLo <= maxSmallU64 {
		return ValFromBody(TagU256Small, x.LoLo)
	}
	return Val(host.ObjFromU256Pieces(x.HiHi, x.HiLo, x.LoHi, x.LoLo))
}

// FromVal implements FromValer.
func (x *U256) FromVal(v Val) {
	switch v.Tag() {
	case TagU256Small:
		*x = U256{LoLo: v.Body()}
	case TagU256Object:
		o := uint64(v)
		*x = U256{host.ObjToU256HiHi(o), host.ObjToU256HiLo(o), host.ObjToU256LoHi(o), host.ObjToU256LoLo(o)}
	default:
		conversionError()
	}
}

// Add returns x+y using the host, trapping on overflow.
func (x U256) Add(y U256) U256 {
	return FromVal[U256](Val(host.U256Add(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Sub returns x-y using the host, trapping on underflow.
func (x U256) Sub(y U256) U256 {
	return FromVal[U256](Val(host.U256Sub(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Mul returns x*y using the host, trapping on overflow.
func (x U256) Mul(y U256) U256 {
	return FromVal[U256](Val(host.U256Mul(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Div returns x/y using the host, trapping on division by zero.
func (x U256) Div(y U256) U256 {
	return FromVal[U256](Val(host.U256Div(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Rem returns x%y (Euclidean) using the host.
func (x U256) Rem(y U256) U256 {
	return FromVal[U256](Val(host.U256RemEuclid(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Pow returns x**pow using the host.
func (x U256) Pow(pow uint32) U256 {
	return FromVal[U256](Val(host.U256Pow(uint64(x.ToVal()), uint64(valFromU32(pow)))))
}

// Shl returns x<<bits using the host.
func (x U256) Shl(bits uint32) U256 {
	return FromVal[U256](Val(host.U256Shl(uint64(x.ToVal()), uint64(valFromU32(bits)))))
}

// Shr returns x>>bits using the host.
func (x U256) Shr(bits uint32) U256 {
	return FromVal[U256](Val(host.U256Shr(uint64(x.ToVal()), uint64(valFromU32(bits)))))
}

// ToBytes returns the big-endian bytes of the value.
func (x U256) ToBytes() Bytes { return Bytes{obj: Val(host.U256ValToBeBytes(uint64(x.ToVal())))} }

// U256FromBytes returns the value of the 32 big-endian bytes.
func U256FromBytes(b Bytes) U256 {
	return FromVal[U256](Val(host.U256ValFromBeBytes(uint64(b.obj))))
}

// I256 is a signed 256-bit integer, stored as four 64-bit pieces from most to
// least significant.
type I256 struct {
	HiHi             int64
	HiLo, LoHi, LoLo uint64
}

// I256FromI64 returns an I256 with the value.
func I256FromI64(x int64) I256 {
	ext := uint64(0)
	if x < 0 {
		ext = ^uint64(0)
	}
	return I256{HiHi: int64(ext), HiLo: ext, LoHi: ext, LoLo: uint64(x)}
}

// ToVal implements ToValer.
func (x I256) ToVal() Val {
	small := (x.HiHi == 0 && x.HiLo == 0 && x.LoHi == 0 && int64(x.LoLo) >= 0) ||
		(x.HiHi == -1 && x.HiLo == ^uint64(0) && x.LoHi == ^uint64(0) && int64(x.LoLo) < 0)
	if small {
		i := int64(x.LoLo)
		if i >= minSmallI64 && i <= maxSmallI64 {
			return ValFromBody(TagI256Small, uint64(i))
		}
	}
	return Val(host.ObjFromI256Pieces(x.HiHi, x.HiLo, x.LoHi, x.LoLo))
}

// FromVal implements FromValer.
func (x *I256) FromVal(v Val) {
	switch v.Tag() {
	case TagI256Small:
		*x = I256FromI64(v.SignedBody())
	case TagI256Object:
		o := uint64(v)
		*x = I256{host.ObjToI256HiHi(o), host.ObjToI256HiLo(o), host.ObjToI256LoHi(o), host.ObjToI256LoLo(o)}
	default:
		conversionError()
	}
}

// Add returns x+y using the host, trapping on overflow.
func (x I256) Add(y I256) I256 {
	return FromVal[I256](Val(host.I256Add(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Sub returns x-y using the host, trapping on overflow.
func (x I256) Sub(y I256) I256 {
	return FromVal[I256](Val(host.I256Sub(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Mul returns x*y using the host, trapping on overflow.
func (x I256) Mul(y I256) I256 {
	return FromVal[I256](Val(host.I256Mul(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Div returns x/y using the host, trapping on division by zero.
func (x I256) Div(y I256) I256 {
	return FromVal[I256](Val(host.I256Div(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Rem returns x%y (Euclidean) using the host.
func (x I256) Rem(y I256) I256 {
	return FromVal[I256](Val(host.I256RemEuclid(uint64(x.ToVal()), uint64(y.ToVal()))))
}

// Pow returns x**pow using the host.
func (x I256) Pow(pow uint32) I256 {
	return FromVal[I256](Val(host.I256Pow(uint64(x.ToVal()), uint64(valFromU32(pow)))))
}

// Shl returns x<<bits using the host.
func (x I256) Shl(bits uint32) I256 {
	return FromVal[I256](Val(host.I256Shl(uint64(x.ToVal()), uint64(valFromU32(bits)))))
}

// Shr returns x>>bits using the host.
func (x I256) Shr(bits uint32) I256 {
	return FromVal[I256](Val(host.I256Shr(uint64(x.ToVal()), uint64(valFromU32(bits)))))
}

// ToBytes returns the big-endian bytes of the value.
func (x I256) ToBytes() Bytes { return Bytes{obj: Val(host.I256ValToBeBytes(uint64(x.ToVal())))} }

// I256FromBytes returns the value of the 32 big-endian bytes.
func I256FromBytes(b Bytes) I256 {
	return FromVal[I256](Val(host.I256ValFromBeBytes(uint64(b.obj))))
}

// DivRem returns x/y and x%y, trapping if y is zero.
func (x U128) DivRem(y U128) (U128, U128) {
	if y.Hi == 0 && y.Lo == 0 {
		panicStr("division by zero")
	}
	if y.Hi == 0 && x.Hi < y.Lo {
		q, r := bits.Div64(x.Hi, x.Lo, y.Lo)
		return U128{Lo: q}, U128{Lo: r}
	}
	// Shift-subtract long division.
	var q, r U128
	for i := 127; i >= 0; i-- {
		r = U128{Hi: r.Hi<<1 | r.Lo>>63, Lo: r.Lo << 1}
		if i >= 64 {
			r.Lo |= (x.Hi >> uint(i-64)) & 1
		} else {
			r.Lo |= (x.Lo >> uint(i)) & 1
		}
		if r.Cmp(y) >= 0 {
			r = r.Sub(y)
			if i >= 64 {
				q.Hi |= 1 << uint(i-64)
			} else {
				q.Lo |= 1 << uint(i)
			}
		}
	}
	return q, r
}

// Div returns x/y, trapping if y is zero.
func (x U128) Div(y U128) U128 { q, _ := x.DivRem(y); return q }

// Rem returns x%y, trapping if y is zero.
func (x U128) Rem(y U128) U128 { _, r := x.DivRem(y); return r }

func (x I128) abs() U128 {
	if x.IsNeg() {
		n := I128{Hi: ^x.Hi, Lo: ^x.Lo}
		lo, c := bits.Add64(n.Lo, 1, 0)
		return U128{Hi: uint64(n.Hi) + c, Lo: lo}
	}
	return U128{Hi: uint64(x.Hi), Lo: x.Lo}
}

func i128FromMag(m U128, neg bool) I128 {
	limit := uint64(1) << 63
	if (!neg && m.Hi >= limit) || (neg && (m.Hi > limit || (m.Hi == limit && m.Lo != 0))) {
		overflow()
	}
	v := I128{Hi: int64(m.Hi), Lo: m.Lo}
	if neg && (m.Hi != 0 || m.Lo != 0) {
		lo, c := bits.Add64(^m.Lo, 1, 0)
		v = I128{Hi: int64(^m.Hi + c), Lo: lo}
	}
	return v
}

// Mul returns x*y, trapping on overflow.
func (x I128) Mul(y I128) I128 { return i128FromMag(x.abs().Mul(y.abs()), x.IsNeg() != y.IsNeg()) }

// Div returns x/y truncated towards zero, trapping if y is zero or on
// overflow.
func (x I128) Div(y I128) I128 { return i128FromMag(x.abs().Div(y.abs()), x.IsNeg() != y.IsNeg()) }

// Rem returns x%y with the sign of x, trapping if y is zero.
func (x I128) Rem(y I128) I128 { return i128FromMag(x.abs().Rem(y.abs()), x.IsNeg()) }
