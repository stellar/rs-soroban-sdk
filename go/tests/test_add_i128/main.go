// Port of tests/add_i128.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

func (Contract) Add(a, b soroban.I128) soroban.I128 {
	return a.Add(b)
}

func main() {}
