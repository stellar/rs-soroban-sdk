// Port of tests/invoke_contract.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

func (Contract) AddWith(env soroban.Env, x, y int32, contractID soroban.Address) int32 {
	return soroban.InvokeContract[int32](contractID, soroban.SymbolShort("add"),
		soroban.VecFromVals(soroban.ValFromI32(x), soroban.ValFromI32(y)))
}

//soroban:contract
type AddContract struct{}

func (AddContract) Add(_ soroban.Env, a, b int32) int32 {
	return a + b
}

func main() {}
