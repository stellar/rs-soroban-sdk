// Port of tests/constructor.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

//soroban:contracttype union
type DataKey struct {
	Persistent *uint32
	Temp       *uint32
	Instance   *uint32
}

func (Contract) Constructor(env soroban.Env, initKey uint32, initValue int64) {
	p, t, i := initKey, initKey*2, initKey*3
	env.Storage().Persistent().Set(DataKey{Persistent: &p}, initValue)
	env.Storage().Temporary().Set(DataKey{Temp: &t}, initValue*2)
	env.Storage().Instance().Set(DataKey{Instance: &i}, initValue*3)
}

func (Contract) GetData(env soroban.Env, key DataKey) soroban.Option[int64] {
	var s soroban.StorageGetter
	switch {
	case key.Persistent != nil:
		s = env.Storage().Persistent()
	case key.Temp != nil:
		s = env.Storage().Temporary()
	default:
		s = env.Storage().Instance()
	}
	v, ok := soroban.Get[int64](s, key)
	return soroban.Option[int64]{Value: v, Some: ok}
}

func main() {}
