// Port of tests/events.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

//soroban:contractevent
type Transfer struct {
	From      soroban.Address `soroban:",topic"`
	To        soroban.Address `soroban:",topic"`
	Amount    soroban.I128
	ToMuxedID soroban.Option[uint64] `soroban:"to_muxed_id"`
}

// An event whose data is a single value, rather than a map.
//
//soroban:contractevent data_format=single-value
type SingleValue struct {
	From   soroban.Address `soroban:",topic"`
	Amount soroban.I128
}

// An event whose data is a single value, and that has no data fields, and so
// whose data is void.
//
//soroban:contractevent data_format=single-value
type SingleValueVoid struct {
	From soroban.Address `soroban:",topic"`
}

// An event whose data is a vec, rather than a map.
//
//soroban:contractevent data_format=vec
type VecValues struct {
	From soroban.Address `soroban:",topic"`
	A    uint32
	B    uint32
}

// An event whose data is a map, which is the default.
//
//soroban:contractevent data_format=map
type MapValues struct {
	From soroban.Address `soroban:",topic"`
	A    uint32
	B    uint32
}

func (Contract) SingleValue(env soroban.Env, from soroban.Address, amount soroban.I128) {
	SingleValue{From: from, Amount: amount}.Publish(env)
}

func (Contract) SingleValueVoid(env soroban.Env, from soroban.Address) {
	SingleValueVoid{From: from}.Publish(env)
}

func (Contract) VecValues(env soroban.Env, from soroban.Address, a, b uint32) {
	VecValues{From: from, A: a, B: b}.Publish(env)
}

func (Contract) MapValues(env soroban.Env, from soroban.Address, a, b uint32) {
	MapValues{From: from, A: a, B: b}.Publish(env)
}

func (Contract) Transfer(env soroban.Env, from soroban.Address, to soroban.MuxedAddress, amount soroban.I128) {
	Transfer{From: from, To: to.Address(), Amount: amount, ToMuxedID: to.ID()}.Publish(env)
}

func (Contract) FailedTransfer(env soroban.Env, from, to soroban.Address, amount soroban.I128) {
	Transfer{From: from, To: to, Amount: amount}.Publish(env)
	panic("fail")
}

func main() {}
