package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// Bytes is a growable array of bytes stored in the host.
type Bytes struct {
	obj Val
}

// NewBytes returns an empty Bytes.
func NewBytes() Bytes { return Bytes{obj: Val(host.BytesNew())} }

// BytesFromSlice returns a Bytes containing a copy of the slice.
func BytesFromSlice(b []byte) Bytes {
	return Bytes{obj: Val(host.BytesNewFromLinearMemory(slicePtr(b), u32Val(uint32(len(b)))))}
}

// BytesFromString returns a Bytes containing the bytes of the string.
func BytesFromString(s string) Bytes {
	return Bytes{obj: Val(host.BytesNewFromLinearMemory(strPtr(s), u32Val(uint32(len(s)))))}
}

func (b Bytes) handle() uint64 {
	if b.obj == 0 {
		// Zero value of Bytes is an empty bytes.
		return host.BytesNew()
	}
	return uint64(b.obj)
}

// ToVal implements ToValer.
func (b Bytes) ToVal() Val { return Val(b.handle()) }

// FromVal implements FromValer.
func (b *Bytes) FromVal(v Val) {
	v.expectTag(TagBytesObject)
	b.obj = v
}

// Len returns the number of bytes.
func (b Bytes) Len() uint32 { return u32FromRaw(host.BytesLen(b.handle())) }

// IsEmpty reports whether there are no bytes.
func (b Bytes) IsEmpty() bool { return b.Len() == 0 }

// Get returns the byte at the index, trapping if out of bounds.
func (b Bytes) Get(i uint32) byte { return byte(u32FromRaw(host.BytesGet(b.handle(), u32Val(i)))) }

// Set sets the byte at the index, trapping if out of bounds.
func (b *Bytes) Set(i uint32, x byte) {
	b.obj = Val(host.BytesPut(b.handle(), u32Val(i), u32Val(uint32(x))))
}

// Push appends a byte.
func (b *Bytes) Push(x byte) { b.obj = Val(host.BytesPush(b.handle(), u32Val(uint32(x)))) }

// Pop removes and returns the last byte, and false if empty.
func (b *Bytes) Pop() (byte, bool) {
	if b.IsEmpty() {
		return 0, false
	}
	last := byte(u32FromRaw(host.BytesBack(b.handle())))
	b.obj = Val(host.BytesPop(b.handle()))
	return last, true
}

// Insert inserts a byte at the index.
func (b *Bytes) Insert(i uint32, x byte) {
	b.obj = Val(host.BytesInsert(b.handle(), u32Val(i), u32Val(uint32(x))))
}

// Remove removes the byte at the index.
func (b *Bytes) Remove(i uint32) { b.obj = Val(host.BytesDel(b.handle(), u32Val(i))) }

// Append appends the other bytes.
func (b *Bytes) Append(o Bytes) { b.obj = Val(host.BytesAppend(b.handle(), o.handle())) }

// Extend appends the bytes of the slice.
func (b *Bytes) Extend(s []byte) {
	l := b.Len()
	b.obj = Val(host.BytesCopyFromLinearMemory(b.handle(), u32Val(l), slicePtr(s), u32Val(uint32(len(s)))))
}

// Slice returns the bytes in [start, end).
func (b Bytes) Slice(start, end uint32) Bytes {
	return Bytes{obj: Val(host.BytesSlice(b.handle(), u32Val(start), u32Val(end)))}
}

// First returns the first byte, and false if empty.
func (b Bytes) First() (byte, bool) {
	if b.IsEmpty() {
		return 0, false
	}
	return byte(u32FromRaw(host.BytesFront(b.handle()))), true
}

// Last returns the last byte, and false if empty.
func (b Bytes) Last() (byte, bool) {
	if b.IsEmpty() {
		return 0, false
	}
	return byte(u32FromRaw(host.BytesBack(b.handle()))), true
}

// CopyInto copies bytes starting at the offset into the slice, trapping if out
// of bounds.
func (b Bytes) CopyInto(offset uint32, s []byte) {
	if len(s) == 0 {
		return
	}
	host.BytesCopyToLinearMemory(b.handle(), u32Val(offset), slicePtr(s), u32Val(uint32(len(s))))
}

// ToSlice returns the bytes as a newly allocated slice.
func (b Bytes) ToSlice() []byte {
	s := make([]byte, b.Len())
	b.CopyInto(0, s)
	return s
}

// Equal reports whether the bytes are equal.
func (b Bytes) Equal(o Bytes) bool { return host.ObjCmp(b.handle(), o.handle()) == 0 }

// Cmp compares the bytes, returning -1, 0 or 1.
func (b Bytes) Cmp(o Bytes) int { return int(host.ObjCmp(b.handle(), o.handle())) }

// ToString converts the bytes to a String.
func (b Bytes) ToString() String { return String{obj: Val(host.BytesToString(b.handle()))} }

// Len is implemented by marker types that give the length of a BytesN.
type Len interface {
	Len() uint32
}

// BytesN is a fixed length array of bytes stored in the host. The length is
// given by the type parameter L, for example BytesN[N32] for 32 bytes. It
// corresponds to BytesN<N> in the Rust SDK.
type BytesN[L Len] struct {
	Bytes
}

// BytesNFromSlice returns a BytesN containing a copy of the slice, trapping if
// the slice is not the length.
func BytesNFromSlice[L Len](b []byte) BytesN[L] {
	var l L
	if uint32(len(b)) != l.Len() {
		conversionError()
	}
	return BytesN[L]{Bytes: BytesFromSlice(b)}
}

// BytesNFromBytes returns a BytesN containing the bytes, trapping if the bytes
// are not the length.
func BytesNFromBytes[L Len](b Bytes) BytesN[L] {
	var l L
	if b.Len() != l.Len() {
		conversionError()
	}
	return BytesN[L]{Bytes: b}
}

// ToVal implements ToValer.
func (b BytesN[L]) ToVal() Val {
	if b.obj == 0 {
		var l L
		return BytesFromSlice(make([]byte, l.Len())).ToVal()
	}
	return b.obj
}

// FromVal implements FromValer, trapping if the length is incorrect.
func (b *BytesN[L]) FromVal(v Val) {
	b.Bytes.FromVal(v)
	var l L
	if b.Bytes.Len() != l.Len() {
		conversionError()
	}
}

// ToBytes returns the BytesN as Bytes.
func (b BytesN[L]) ToBytes() Bytes { return Bytes{obj: b.ToVal()} }

// Length markers for BytesN.
type (
	N1   struct{}
	N2   struct{}
	N4   struct{}
	N8   struct{}
	N16  struct{}
	N20  struct{}
	N24  struct{}
	N28  struct{}
	N32  struct{}
	N33  struct{}
	N48  struct{}
	N64  struct{}
	N65  struct{}
	N96  struct{}
	N128 struct{}
	N192 struct{}
	N256 struct{}
)

func (N1) Len() uint32   { return 1 }
func (N2) Len() uint32   { return 2 }
func (N4) Len() uint32   { return 4 }
func (N8) Len() uint32   { return 8 }
func (N16) Len() uint32  { return 16 }
func (N20) Len() uint32  { return 20 }
func (N24) Len() uint32  { return 24 }
func (N28) Len() uint32  { return 28 }
func (N32) Len() uint32  { return 32 }
func (N33) Len() uint32  { return 33 }
func (N48) Len() uint32  { return 48 }
func (N64) Len() uint32  { return 64 }
func (N65) Len() uint32  { return 65 }
func (N96) Len() uint32  { return 96 }
func (N128) Len() uint32 { return 128 }
func (N192) Len() uint32 { return 192 }
func (N256) Len() uint32 { return 256 }
