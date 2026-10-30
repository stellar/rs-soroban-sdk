// Command test_sdk exercises the breadth of the Go SDK on a network: ledger
// info, crypto, prng, collections, numbers, storage, deployment, addresses,
// and token contracts. It has no Rust counterpart in tests/.
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:meta source=github.com/stellar/rs-soroban-sdk/go/tests/test_sdk

//soroban:contract
type Contract struct{}

// LedgerInfo describes the current ledger.
//
//soroban:contracttype
type LedgerInfo struct {
	Protocol  uint32
	Sequence  uint32
	Timestamp uint64
	NetworkID soroban.BytesN[soroban.N32]
}

//soroban:contracterror
type Error uint32

const (
	ErrorNotFound Error = 1
	ErrorTooBig   Error = 2
)

// Counted is published each time the counter is incremented.
//
//soroban:contractevent topics=counter,incr data_format=single-value
type Counted struct {
	Count uint32
}

var counterKey = soroban.SymbolShort("COUNTER")

func (Contract) Ledger(env soroban.Env) LedgerInfo {
	l := env.Ledger()
	return LedgerInfo{Protocol: l.ProtocolVersion(), Sequence: l.Sequence(), Timestamp: l.Timestamp(), NetworkID: l.NetworkID()}
}

func (Contract) CurrentAddress(env soroban.Env) soroban.Address {
	return env.CurrentContractAddress()
}

func (Contract) Sha256(env soroban.Env, data soroban.Bytes) soroban.Hash {
	return env.Crypto().Sha256(data)
}

func (Contract) Keccak256(env soroban.Env, data soroban.Bytes) soroban.Hash {
	return env.Crypto().Keccak256(data)
}

func (Contract) Ed25519Verify(env soroban.Env, publicKey soroban.BytesN[soroban.N32], message soroban.Bytes, signature soroban.BytesN[soroban.N64]) bool {
	env.Crypto().Ed25519Verify(publicKey, message, signature)
	return true
}

func (Contract) Random(env soroban.Env, lo, hi uint64) uint64 {
	return env.Prng().U64InRange(lo, hi)
}

// Shuffle returns the values shuffled, which keeps their sum.
func (Contract) ShuffleSum(env soroban.Env, v soroban.Vec[uint32]) uint32 {
	var sum uint32
	for x := range soroban.Shuffle(v).Values() {
		sum += x
	}
	return sum
}

// Histogram counts the occurrences of each symbol.
func (Contract) Histogram(words soroban.Vec[soroban.Symbol]) soroban.Map[soroban.Symbol, uint32] {
	m := soroban.NewMap[soroban.Symbol, uint32]()
	for w := range words.Values() {
		n, _ := m.Get(w)
		m.Set(w, n+1)
	}
	return m
}

// BytesOps reverses the bytes and appends their length.
func (Contract) BytesOps(b soroban.Bytes) soroban.Bytes {
	s := b.ToSlice()
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
	out := soroban.BytesFromSlice(s)
	out.Push(byte(b.Len()))
	return out
}

func (Contract) Greet(name soroban.String) soroban.String {
	return soroban.StringFromStr("Hello, " + name.String() + "!")
}

func (Contract) U256Math(a, b soroban.U256) soroban.U256 {
	return a.Mul(b).Add(a)
}

func (Contract) I128Math(a, b soroban.I128) soroban.Tuple3[soroban.I128, soroban.I128, soroban.I128] {
	return soroban.NewTuple3(a.Mul(b), a.Div(b), a.Rem(b))
}

func (Contract) Increment(env soroban.Env) uint32 {
	s := env.Storage().Instance()
	n := soroban.Update(s, counterKey, func(v soroban.Option[uint32]) uint32 { return v.UnwrapOr(0) + 1 })
	s.ExtendTTL(100, 1000)
	Counted{Count: n}.Publish(env)
	return n
}

func (Contract) PutTemp(env soroban.Env, key soroban.Symbol, val uint64) {
	t := env.Storage().Temporary()
	t.Set(key, val)
	t.ExtendTTL(key, 10, 100)
}

func (Contract) GetTemp(env soroban.Env, key soroban.Symbol) (uint64, *Error) {
	v, ok := soroban.Get[uint64](env.Storage().Temporary(), key)
	if !ok {
		err := ErrorNotFound
		return 0, &err
	}
	return v, nil
}

func (Contract) MaxTTL(env soroban.Env) uint32 {
	return env.Storage().MaxTTL()
}

// Checked fails with an error from a soroban:contracterror type.
func (Contract) Checked(x uint32) uint32 {
	soroban.AssertWithError(x <= 100, ErrorTooBig)
	return x * 2
}

// Deploy deploys the uploaded Wasm with the salt, passing the constructor
// args, and returns the new contract's address.
func (Contract) Deploy(env soroban.Env, wasmHash soroban.BytesN[soroban.N32], salt soroban.BytesN[soroban.N32], args soroban.Vec[soroban.Val]) soroban.Address {
	d := env.Deployer().WithCurrentContract(salt)
	addr := d.DeployV2(wasmHash, args)
	if !addr.Equal(d.DeployedAddress()) {
		panic("unexpected address")
	}
	return addr
}

func (Contract) AddressString(a soroban.Address) soroban.String {
	return a.String()
}

func (Contract) AddressFromString(s soroban.String) soroban.Address {
	return soroban.AddressFromString(s)
}

func (Contract) Muxed(m soroban.MuxedAddress) soroban.Tuple2[soroban.Address, soroban.Option[uint64]] {
	return soroban.NewTuple2(m.Address(), m.ID())
}

func (Contract) TokenInfo(token soroban.Address, id soroban.Address) soroban.Tuple3[soroban.String, uint32, soroban.I128] {
	c := soroban.NewTokenClient(token)
	return soroban.NewTuple3(c.Symbol(), c.Decimals(), c.Balance(id))
}

// Pay transfers amount of the token from from to to, which from must
// authorize.
func (Contract) Pay(token, from soroban.Address, to soroban.MuxedAddress, amount soroban.I128) soroban.I128 {
	from.RequireAuth()
	c := soroban.NewTokenClient(token)
	c.Transfer(from, to, amount)
	return c.Balance(to.Address())
}

func main() {}
