// Port of tests/contract_data.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

func (Contract) Put(e soroban.Env, key, val soroban.Symbol) {
	e.Storage().Persistent().Set(key, val)
}

func (Contract) Get(e soroban.Env, key soroban.Symbol) soroban.Option[soroban.Symbol] {
	v, ok := soroban.Get[soroban.Symbol](e.Storage().Persistent(), key)
	return soroban.Option[soroban.Symbol]{Value: v, Some: ok}
}

func (Contract) Del(e soroban.Env, key soroban.Symbol) {
	e.Storage().Persistent().Remove(key)
}

func main() {}
