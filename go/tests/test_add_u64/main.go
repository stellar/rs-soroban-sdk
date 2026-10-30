// Port of tests/add_u64.
package main

import "math/bits"

//soroban:contract
type Contract struct{}

//soroban:contracterror
type Error uint32

const ErrorOverflow Error = 1

//soroban:contracterror
type MyError uint32

const MyErrorOverflow MyError = 1

func (Contract) Add(a, b uint64) uint64 {
	return a + b
}

func (Contract) SafeAdd(a, b uint64) (uint64, *Error) {
	sum, carry := bits.Add64(a, b, 0)
	if carry != 0 {
		err := ErrorOverflow
		return 0, &err
	}
	return sum, nil
}

func (Contract) SafeAddTwo(a, b uint64) (uint64, *MyError) {
	sum, carry := bits.Add64(a, b, 0)
	if carry != 0 {
		err := MyErrorOverflow
		return 0, &err
	}
	return sum, nil
}

func main() {}
