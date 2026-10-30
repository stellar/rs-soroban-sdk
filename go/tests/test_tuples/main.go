// Port of tests/tuples.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

func (Contract) VoidFn(_voidArg soroban.Void) soroban.Void { return soroban.Void{} }

func (Contract) Tuple1(arg soroban.Tuple1[uint32]) soroban.Tuple1[uint32] { return arg }

func (Contract) Tuple2(arg soroban.Tuple2[uint32, int64]) soroban.Tuple2[uint32, int64] { return arg }

func main() {}
