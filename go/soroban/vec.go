package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// Vec is a growable array of values of type T stored in the host. It
// corresponds to Vec<T> in the Rust SDK. Values are converted to and from Vals
// as they are stored and retrieved, trapping on a value of the wrong type.
//
// Mutating methods have pointer receivers because the host returns a new
// object handle for each modification.
type Vec[T any] struct {
	obj Val
}

// NewVec returns an empty Vec.
func NewVec[T any]() Vec[T] { return Vec[T]{obj: Val(host.VecNew())} }

// VecOf returns a Vec containing the values.
func VecOf[T any](items ...T) Vec[T] {
	if len(items) == 0 {
		return NewVec[T]()
	}
	vals := make([]Val, len(items))
	for i := range items {
		vals[i] = ToVal(any(items[i]))
	}
	return Vec[T]{obj: Val(host.VecNewFromLinearMemory(slicePtr(vals), u32Val(uint32(len(vals)))))}
}

// VecFromVals returns a Vec containing the Vals.
func VecFromVals(vals ...Val) Vec[Val] {
	if len(vals) == 0 {
		return NewVec[Val]()
	}
	return Vec[Val]{obj: Val(host.VecNewFromLinearMemory(slicePtr(vals), u32Val(uint32(len(vals)))))}
}

func (v Vec[T]) handle() uint64 {
	if v.obj == 0 {
		return host.VecNew()
	}
	return uint64(v.obj)
}

// ToVal implements ToValer.
func (v Vec[T]) ToVal() Val { return Val(v.handle()) }

// FromVal implements FromValer. The elements are not checked until accessed.
func (v *Vec[T]) FromVal(x Val) {
	x.expectTag(TagVecObject)
	v.obj = x
}

// ToVals returns the Vec as a Vec[Val].
func (v Vec[T]) ToVals() Vec[Val] { return Vec[Val]{obj: Val(v.handle())} }

// Len returns the number of elements.
func (v Vec[T]) Len() uint32 { return u32FromRaw(host.VecLen(v.handle())) }

// IsEmpty reports whether the Vec has no elements.
func (v Vec[T]) IsEmpty() bool { return v.Len() == 0 }

// Get returns the element at the index, trapping if out of bounds.
func (v Vec[T]) Get(i uint32) T { return FromVal[T](Val(host.VecGet(v.handle(), u32Val(i)))) }

// TryGet returns the element at the index, and false if out of bounds.
func (v Vec[T]) TryGet(i uint32) (T, bool) {
	if i >= v.Len() {
		var zero T
		return zero, false
	}
	return v.Get(i), true
}

// Set sets the element at the index, trapping if out of bounds.
func (v *Vec[T]) Set(i uint32, x T) {
	v.obj = Val(host.VecPut(v.handle(), u32Val(i), uint64(ToVal(any(x)))))
}

// PushBack appends an element.
func (v *Vec[T]) PushBack(x T) { v.obj = Val(host.VecPushBack(v.handle(), uint64(ToVal(any(x))))) }

// PushFront prepends an element.
func (v *Vec[T]) PushFront(x T) { v.obj = Val(host.VecPushFront(v.handle(), uint64(ToVal(any(x))))) }

// PopBack removes and returns the last element, and false if empty.
func (v *Vec[T]) PopBack() (T, bool) {
	last, ok := v.Last()
	if ok {
		v.obj = Val(host.VecPopBack(v.handle()))
	}
	return last, ok
}

// PopFront removes and returns the first element, and false if empty.
func (v *Vec[T]) PopFront() (T, bool) {
	first, ok := v.First()
	if ok {
		v.obj = Val(host.VecPopFront(v.handle()))
	}
	return first, ok
}

// First returns the first element, and false if empty.
func (v Vec[T]) First() (T, bool) {
	if v.IsEmpty() {
		var zero T
		return zero, false
	}
	return FromVal[T](Val(host.VecFront(v.handle()))), true
}

// Last returns the last element, and false if empty.
func (v Vec[T]) Last() (T, bool) {
	if v.IsEmpty() {
		var zero T
		return zero, false
	}
	return FromVal[T](Val(host.VecBack(v.handle()))), true
}

// Insert inserts an element at the index.
func (v *Vec[T]) Insert(i uint32, x T) {
	v.obj = Val(host.VecInsert(v.handle(), u32Val(i), uint64(ToVal(any(x)))))
}

// Remove removes the element at the index.
func (v *Vec[T]) Remove(i uint32) { v.obj = Val(host.VecDel(v.handle(), u32Val(i))) }

// Append appends the elements of the other Vec.
func (v *Vec[T]) Append(o Vec[T]) { v.obj = Val(host.VecAppend(v.handle(), o.handle())) }

// Slice returns the elements in [start, end).
func (v Vec[T]) Slice(start, end uint32) Vec[T] {
	return Vec[T]{obj: Val(host.VecSlice(v.handle(), u32Val(start), u32Val(end)))}
}

// Contains reports whether the Vec contains the element.
func (v Vec[T]) Contains(x T) bool {
	return !Val(host.VecFirstIndexOf(v.handle(), uint64(ToVal(any(x))))).IsVoid()
}

// FirstIndexOf returns the index of the first occurrence of the element, and
// false if not found.
func (v Vec[T]) FirstIndexOf(x T) (uint32, bool) {
	r := Val(host.VecFirstIndexOf(v.handle(), uint64(ToVal(any(x)))))
	if r.IsVoid() {
		return 0, false
	}
	return r.Major(), true
}

// LastIndexOf returns the index of the last occurrence of the element, and
// false if not found.
func (v Vec[T]) LastIndexOf(x T) (uint32, bool) {
	r := Val(host.VecLastIndexOf(v.handle(), uint64(ToVal(any(x)))))
	if r.IsVoid() {
		return 0, false
	}
	return r.Major(), true
}

// BinarySearch searches a sorted Vec for the element. It returns the index of
// the element and true if found, or the index it would be inserted at and
// false if not found.
func (v Vec[T]) BinarySearch(x T) (uint32, bool) {
	r := host.VecBinarySearch(v.handle(), uint64(ToVal(any(x))))
	return uint32(r), r>>32 == 1
}

// All returns an iterator over the index and elements, for use with range.
func (v Vec[T]) All() func(yield func(uint32, T) bool) {
	return func(yield func(uint32, T) bool) {
		n := v.Len()
		for i := uint32(0); i < n; i++ {
			if !yield(i, v.Get(i)) {
				return
			}
		}
	}
}

// Values returns an iterator over the elements, for use with range.
func (v Vec[T]) Values() func(yield func(T) bool) {
	return func(yield func(T) bool) {
		n := v.Len()
		for i := uint32(0); i < n; i++ {
			if !yield(v.Get(i)) {
				return
			}
		}
	}
}

// ToSlice returns the elements as a newly allocated slice.
func (v Vec[T]) ToSlice() []T {
	n := v.Len()
	if n == 0 {
		return nil
	}
	vals := make([]Val, n)
	host.VecUnpackToLinearMemory(v.handle(), slicePtr(vals), u32Val(n))
	out := make([]T, n)
	for i := range vals {
		FromValInto(&out[i], vals[i])
	}
	return out
}

// Equal reports whether the Vecs are structurally equal.
func (v Vec[T]) Equal(o Vec[T]) bool { return host.ObjCmp(v.handle(), o.handle()) == 0 }
