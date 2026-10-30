// Port of tests/udt.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contracttype
type UdtEnum2 uint32

const (
	UdtEnum2A UdtEnum2 = 10
	UdtEnum2B UdtEnum2 = 15
)

//soroban:contracttype union
type UdtEnum struct {
	UdtA bool
	UdtB *UdtStruct
	UdtC *UdtEnum2
	UdtD *UdtTuple
}

//soroban:contracttype tuple
type UdtTuple struct {
	V0 int64
	V1 soroban.Vec[int64]
}

//soroban:contracttype
type UdtStruct struct {
	a int64
	b int64
	C soroban.Vec[int64]
}

//soroban:contracttype
type UdtRecursive struct {
	A soroban.Symbol
	B soroban.Vec[UdtRecursive]
}

//soroban:contracttype
type RecursiveToEnum struct {
	A soroban.Symbol
	B soroban.Map[uint32, RecursiveEnum]
}

//soroban:contracttype union
type RecursiveEnum struct {
	NotRecursive bool
	Recursive    *RecursiveToEnum
}

//soroban:contract
type Contract struct{}

func value(e UdtEnum) int64 {
	switch {
	case e.UdtA:
		return 0
	case e.UdtB != nil:
		return e.UdtB.a + e.UdtB.b
	case e.UdtC != nil:
		return int64(*e.UdtC)
	case e.UdtD != nil:
		sum := e.UdtD.V0
		for v := range e.UdtD.V1.Values() {
			sum += v
		}
		return sum
	}
	return 0
}

func (Contract) Add(a, b UdtEnum) int64 {
	return value(a) + value(b)
}

func (Contract) Recursive(a UdtRecursive) soroban.Option[UdtRecursive] {
	first, ok := a.B.First()
	if !ok {
		return soroban.None[UdtRecursive]()
	}
	return soroban.Some(first)
}

func (Contract) RecursiveEnum(a RecursiveEnum, key uint32) (soroban.Option[RecursiveEnum], *soroban.Error) {
	if a.NotRecursive {
		return soroban.None[RecursiveEnum](), nil
	}
	v, ok := a.Recursive.B.Get(key)
	if !ok {
		return soroban.None[RecursiveEnum](), nil
	}
	return soroban.Some(v), nil
}

func main() {}
