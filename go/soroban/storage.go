package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// storageType values are passed to the host as raw integers.
const (
	storageTemporary  uint64 = 0
	storagePersistent uint64 = 1
	storageInstance   uint64 = 2
)

// Storage provides access to the contract's data storage. It corresponds to
// Storage in the Rust SDK.
type Storage struct{}

// Persistent returns access to persistent storage.
func (Storage) Persistent() Persistent { return Persistent{} }

// Temporary returns access to temporary storage.
func (Storage) Temporary() Temporary { return Temporary{} }

// Instance returns access to instance storage.
func (Storage) Instance() Instance { return Instance{} }

// MaxTTL returns the maximum TTL that an entry can be extended to.
func (Storage) MaxTTL() uint32 {
	seq := u32FromRaw(host.GetLedgerSequence())
	max := u32FromRaw(host.GetMaxLiveUntilLedger())
	if max < seq {
		return 0
	}
	return max - seq
}

// Persistent is persistent storage, for data that must be kept until removed.
type Persistent struct{}

// Temporary is temporary storage, for data that may be removed when its TTL
// expires.
type Temporary struct{}

// Instance is instance storage, stored with the contract instance and sharing
// its TTL.
type Instance struct{}

func has(t uint64, k Val) bool { return boolFromRaw(host.HasContractData(uint64(k), t)) }
func get(t uint64, k Val) Val  { return Val(host.GetContractData(uint64(k), t)) }
func set(t uint64, k, v Val)   { host.PutContractData(uint64(k), uint64(v), t) }
func del(t uint64, k Val)      { host.DelContractData(uint64(k), t) }

// Has reports whether there is a value for the key.
func (Persistent) Has(key any) bool { return has(storagePersistent, ToVal(key)) }

// Set stores the value for the key.
func (Persistent) Set(key, val any) { set(storagePersistent, ToVal(key), ToVal(val)) }

// Remove removes the value for the key.
func (Persistent) Remove(key any) { del(storagePersistent, ToVal(key)) }

// GetVal returns the raw value for the key, and false if there is no value.
func (Persistent) GetVal(key any) (Val, bool) { return getVal(storagePersistent, key) }

// ExtendTTL extends the TTL of the entry for the key to extendTo ledgers if its
// TTL is below threshold.
func (Persistent) ExtendTTL(key any, threshold, extendTo uint32) {
	host.ExtendContractDataTtl(uint64(ToVal(key)), storagePersistent, u32Val(threshold), u32Val(extendTo))
}

// ExtendTTLWithLimits extends the TTL of the entry for the key towards
// extendTo, only if the extension is at least minExtension ledgers, and by at
// most maxExtension ledgers.
func (Persistent) ExtendTTLWithLimits(key any, extendTo, minExtension, maxExtension uint32) {
	host.ExtendContractDataTtlV2(uint64(ToVal(key)), storagePersistent, u32Val(extendTo), u32Val(minExtension), u32Val(maxExtension))
}

// Has reports whether there is a value for the key.
func (Temporary) Has(key any) bool { return has(storageTemporary, ToVal(key)) }

// Set stores the value for the key.
func (Temporary) Set(key, val any) { set(storageTemporary, ToVal(key), ToVal(val)) }

// Remove removes the value for the key.
func (Temporary) Remove(key any) { del(storageTemporary, ToVal(key)) }

// GetVal returns the raw value for the key, and false if there is no value.
func (Temporary) GetVal(key any) (Val, bool) { return getVal(storageTemporary, key) }

// ExtendTTL extends the TTL of the entry for the key to extendTo ledgers if its
// TTL is below threshold.
func (Temporary) ExtendTTL(key any, threshold, extendTo uint32) {
	host.ExtendContractDataTtl(uint64(ToVal(key)), storageTemporary, u32Val(threshold), u32Val(extendTo))
}

// ExtendTTLWithLimits extends the TTL of the entry for the key towards
// extendTo, only if the extension is at least minExtension ledgers, and by at
// most maxExtension ledgers.
func (Temporary) ExtendTTLWithLimits(key any, extendTo, minExtension, maxExtension uint32) {
	host.ExtendContractDataTtlV2(uint64(ToVal(key)), storageTemporary, u32Val(extendTo), u32Val(minExtension), u32Val(maxExtension))
}

// Has reports whether there is a value for the key.
func (Instance) Has(key any) bool { return has(storageInstance, ToVal(key)) }

// Set stores the value for the key.
func (Instance) Set(key, val any) { set(storageInstance, ToVal(key), ToVal(val)) }

// Remove removes the value for the key.
func (Instance) Remove(key any) { del(storageInstance, ToVal(key)) }

// GetVal returns the raw value for the key, and false if there is no value.
func (Instance) GetVal(key any) (Val, bool) { return getVal(storageInstance, key) }

// ExtendTTL extends the TTL of the contract instance and code to extendTo
// ledgers if the TTL is below threshold.
func (Instance) ExtendTTL(threshold, extendTo uint32) {
	host.ExtendCurrentContractInstanceAndCodeTtl(u32Val(threshold), u32Val(extendTo))
}

// ExtendTTLWithLimits extends the TTL of the contract instance and code towards
// extendTo, only if the extension is at least minExtension ledgers, and by at
// most maxExtension ledgers.
func (Instance) ExtendTTLWithLimits(extendTo, minExtension, maxExtension uint32) {
	host.ExtendContractInstanceAndCodeTtlV2(host.GetCurrentContractAddress(), ttlExtensionInstanceAndCode, u32Val(extendTo), u32Val(minExtension), u32Val(maxExtension))
}

func getVal(t uint64, key any) (Val, bool) {
	k := ToVal(key)
	if !has(t, k) {
		return ValVoid, false
	}
	return get(t, k), true
}

// StorageGetter is implemented by the storage types.
type StorageGetter interface {
	GetVal(key any) (Val, bool)
}

// Get returns the value for the key in the storage, and false if there is no
// value. It traps if the stored value is not of type V.
//
//	count, ok := soroban.Get[uint32](env.Storage().Instance(), soroban.SymbolShort("COUNTER"))
func Get[V any](s StorageGetter, key any) (V, bool) {
	v, ok := s.GetVal(key)
	if !ok {
		var zero V
		return zero, false
	}
	return FromVal[V](v), true
}

// GetOr returns the value for the key in the storage, or def if there is no
// value.
func GetOr[V any](s StorageGetter, key any, def V) V {
	v, ok := Get[V](s, key)
	if !ok {
		return def
	}
	return v
}

// StorageSetter is implemented by the storage types.
type StorageSetter interface {
	StorageGetter
	Set(key, val any)
}

// Update reads the value for the key, calls f with it, stores the result, and
// returns it.
func Update[V any](s StorageSetter, key any, f func(Option[V]) V) V {
	v, ok := Get[V](s, key)
	n := f(Option[V]{Value: v, Some: ok})
	s.Set(key, n)
	return n
}

// ContractTtlExtension values for ExtendTTLWithLimits on contracts.
const (
	ttlExtensionInstanceAndCode uint64 = 0
	ttlExtensionInstance        uint64 = 1
	ttlExtensionCode            uint64 = 2
)
