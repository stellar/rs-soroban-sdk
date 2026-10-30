// Port of tests/auth.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type ContractA struct{}

func (ContractA) Fn1(a soroban.Address) uint64 {
	a.RequireAuth()
	return 2
}

func main() {}

// ContractAInterface is the interface of ContractA, for calling it.
//
//soroban:contractclient name=ContractAClient
type ContractAInterface interface {
	Fn1(a soroban.Address) uint64
}

//soroban:contract
type ContractB struct{}

func (ContractB) Fn2(e soroban.Env, a, sub soroban.Address) uint64 {
	a.RequireAuthForArgs(soroban.VecFromVals(soroban.ValFromI32(1), soroban.ValFromI32(2)))
	client := NewContractAClient(sub)
	return client.Fn1(a)
}
