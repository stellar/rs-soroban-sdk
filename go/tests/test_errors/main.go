// Port of tests/errors.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

//soroban:contracttype
type Flag uint32

const (
	FlagA Flag = 0
	FlagB Flag = 1
	FlagC Flag = 2
	FlagD Flag = 3
	FlagE Flag = 4
)

//soroban:contracterror
type Error uint32

const ErrorAnError Error = 1

func (Contract) Hello(env soroban.Env, flag Flag) (soroban.Symbol, *Error) {
	env.Storage().Persistent().Set(soroban.SymbolShort("persisted"), true)
	switch flag {
	case FlagA:
		return soroban.SymbolShort("hello"), nil
	case FlagB:
		err := ErrorAnError
		return soroban.Symbol{}, &err
	case FlagC:
		env.PanicWithError(ErrorAnError)
	case FlagD:
		panic("an error")
	case FlagE:
		env.PanicWithError(soroban.ErrorFromContractCode(9))
	}
	panic("unimplemented")
}

func main() {}
