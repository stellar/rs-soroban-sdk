package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// Map is an ordered map of keys of type K to values of type V stored in the
// host. It corresponds to Map<K, V> in the Rust SDK.
type Map[K, V any] struct {
	obj Val
}

// NewMap returns an empty Map.
func NewMap[K, V any]() Map[K, V] { return Map[K, V]{obj: Val(host.MapNew())} }

func (m Map[K, V]) handle() uint64 {
	if m.obj == 0 {
		return host.MapNew()
	}
	return uint64(m.obj)
}

// ToVal implements ToValer.
func (m Map[K, V]) ToVal() Val { return Val(m.handle()) }

// FromVal implements FromValer. The entries are not checked until accessed.
func (m *Map[K, V]) FromVal(v Val) {
	v.expectTag(TagMapObject)
	m.obj = v
}

// Len returns the number of entries.
func (m Map[K, V]) Len() uint32 { return u32FromRaw(host.MapLen(m.handle())) }

// IsEmpty reports whether the map has no entries.
func (m Map[K, V]) IsEmpty() bool { return m.Len() == 0 }

// Has reports whether the map contains the key.
func (m Map[K, V]) Has(k K) bool { return boolFromRaw(host.MapHas(m.handle(), uint64(ToVal(any(k))))) }

// Get returns the value for the key, and false if not present.
func (m Map[K, V]) Get(k K) (V, bool) {
	kv := uint64(ToVal(any(k)))
	if !boolFromRaw(host.MapHas(m.handle(), kv)) {
		var zero V
		return zero, false
	}
	return FromVal[V](Val(host.MapGet(m.handle(), kv))), true
}

// MustGet returns the value for the key, trapping if not present.
func (m Map[K, V]) MustGet(k K) V {
	return FromVal[V](Val(host.MapGet(m.handle(), uint64(ToVal(any(k))))))
}

// Set sets the value for the key.
func (m *Map[K, V]) Set(k K, v V) {
	m.obj = Val(host.MapPut(m.handle(), uint64(ToVal(any(k))), uint64(ToVal(any(v)))))
}

// Remove removes the key, returning false if it was not present.
func (m *Map[K, V]) Remove(k K) bool {
	kv := uint64(ToVal(any(k)))
	if !boolFromRaw(host.MapHas(m.handle(), kv)) {
		return false
	}
	m.obj = Val(host.MapDel(m.handle(), kv))
	return true
}

// KeyAt returns the key at the position in the ordered map.
func (m Map[K, V]) KeyAt(i uint32) K { return FromVal[K](Val(host.MapKeyByPos(m.handle(), u32Val(i)))) }

// ValueAt returns the value at the position in the ordered map.
func (m Map[K, V]) ValueAt(i uint32) V {
	return FromVal[V](Val(host.MapValByPos(m.handle(), u32Val(i))))
}

// Keys returns the keys in order.
func (m Map[K, V]) Keys() Vec[K] { return Vec[K]{obj: Val(host.MapKeys(m.handle()))} }

// Values returns the values in key order.
func (m Map[K, V]) Values() Vec[V] { return Vec[V]{obj: Val(host.MapValues(m.handle()))} }

// All returns an iterator over the entries in key order, for use with range.
func (m Map[K, V]) All() func(yield func(K, V) bool) {
	return func(yield func(K, V) bool) {
		n := m.Len()
		for i := uint32(0); i < n; i++ {
			if !yield(m.KeyAt(i), m.ValueAt(i)) {
				return
			}
		}
	}
}

// Equal reports whether the maps are structurally equal.
func (m Map[K, V]) Equal(o Map[K, V]) bool { return host.ObjCmp(m.handle(), o.handle()) == 0 }
