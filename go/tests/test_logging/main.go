// Port of tests/logging.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

func (Contract) Hello(env soroban.Env) {
	one, two := soroban.SymbolShort("one"), soroban.SymbolShort("two")
	env.Logs().Log("none")
	env.Logs().Log("none")
	env.Logs().Log("one:", one)
	env.Logs().Log("one:", one)
	env.Logs().Log("one and two:", one, two)
	env.Logs().Log("one and two:", one, two)
}

func main() {}
