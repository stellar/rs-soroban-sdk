// Port of tests/account.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contracterror
type Error uint32

const ErrorFail Error = 1

//soroban:contract
type Contract struct{}

func (Contract) CheckAuth(_ soroban.Env, _signaturePayload soroban.Hash, _signatures soroban.Void, _authContexts soroban.Vec[soroban.Context]) *Error {
	return nil
}

func main() {}
