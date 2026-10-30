# soroban-sdk for Go

A Go port of the [`soroban-sdk`](../soroban-sdk) crate, and `soroban-go`, a
toolchain that builds Soroban smart contracts written in Go into contract Wasm
that is deployable with the [stellar-cli](https://github.com/stellar/stellar-cli).

Contracts are compiled with [TinyGo](https://tinygo.org), for a target limited
to the Wasm features the Soroban VM supports. `soroban-go` generates the glue
the Rust SDK's macros generate — the exported contract functions, the
conversions of contract types to and from `Val`s, events and contract clients —
and writes the `contractspecv0`, `contractenvmetav0` and `contractmetav0`
custom sections.

The emitted contract spec matches the Rust SDK's: for each Go port of the Rust
SDK's test contracts in [`tests/`](tests), the spec entries are byte for byte
identical to those of the Rust contract built with `stellar contract build`
(see [Parity](#parity)).

## Example

```go
package main

import "github.com/stellar/rs-soroban-sdk/go/soroban"

//soroban:contract
type Contract struct{}

//soroban:contracttype
type State struct {
	Count uint32
	Last  soroban.Address
}

//soroban:contracterror
type Error uint32

const ErrorLimitReached Error = 1

// Increment increments the counter for the caller.
func (Contract) Increment(env soroban.Env, caller soroban.Address) (uint32, *Error) {
	caller.RequireAuth()
	key := soroban.SymbolShort("STATE")
	state, _ := soroban.Get[State](env.Storage().Instance(), key)
	if state.Count == 100 {
		err := ErrorLimitReached
		return 0, &err
	}
	state.Count++
	state.Last = caller
	env.Storage().Instance().Set(key, state)
	return state.Count, nil
}

func main() {}
```

```sh
soroban-go build ./counter        # writes ./counter/counter.wasm
stellar contract deploy --wasm counter/counter.wasm --source me --network testnet
stellar contract invoke --id C... --source me --network testnet -- increment --caller me
```

## Install

1. Install [Go](https://go.dev) 1.24 and [TinyGo](https://tinygo.org) 0.40 or
   later, and put `tinygo` on `PATH` (or set `TINYGOROOT`).
2. Install the toolchain:

   ```sh
   go install github.com/stellar/rs-soroban-sdk/go/cmd/soroban-go@latest
   ```

3. In the contract's module: `go get github.com/stellar/rs-soroban-sdk/go`.

## The toolchain

```
soroban-go build [flags] [dir]     build a contract package to Wasm
soroban-go generate [dir]          generate the contract glue code only
soroban-go spec [-json] file.wasm  print the contract spec and meta of a Wasm
soroban-go version                 print the SDK version
```

`build` flags:

| Flag | |
|---|---|
| `-o file` | Output path. Defaults to `<dir>/<dir name>.wasm`. |
| `-protocol n` | Protocol version recorded in `contractenvmetav0`. Defaults to 30, the version the SDK targets, as the Rust SDK does. Use the network's version to deploy to a network on an older protocol, e.g. `-protocol 29`. |
| `-meta key=value` | Adds an entry to `contractmetav0`. Repeatable. |
| `-raw` | Writes the raw spec: every entry, with fully qualified names, as the Rust SDK writes it before `stellar contract build` post-processes it. |
| `-opt level` | TinyGo optimization level. Defaults to `z`. |
| `-tinygo path` | The TinyGo binary. |

`build`:

1. Analyzes the package for the directives below, and writes
   `soroban_generated.go` into it. The file should be committed, like other
   generated Go code.
2. Compiles the package with TinyGo for a `wasm32-unknown-unknown` target with
   only the features the Soroban VM supports (MVP + mutable globals + sign
   extension; no floats, no saturating float conversions, no bulk memory, no
   multi-value, no reference types), the leaking (bump) allocator, no
   scheduler, and panics as traps.
3. Checks the Wasm contains no instructions or types the VM rejects (such as
   floating point from `fmt`, `strconv` or `math`), with an explanatory error.
4. Removes TinyGo's custom sections and `_initialize` export, and writes the
   contract's custom sections.

### Custom sections

| Section | Contents |
|---|---|
| `contractspecv0` | The contract spec. Unused types, errors and events are removed and type names are reduced to their simple names, as `stellar contract build` does for the Rust SDK (`-raw` to skip). |
| `contractenvmetav0` | The interface version: protocol 30, pre-release 0. |
| `contractmetav0` | `gover` (the TinyGo and Go versions, like the Rust SDK's `rsver`), `gosdkver` (the SDK version and revision, like `rssdkver`), and entries from `//soroban:meta` and `-meta`. |

## Writing contracts

A contract is a Go `package main`. Rust attribute macros are directives in
comments.

| Rust | Go |
|---|---|
| `#[contract] pub struct C;` + `#[contractimpl]` | `//soroban:contract` on a type. Its exported methods are contract functions. A package can have several contract types. |
| `pub fn hello_world(env: Env, ...)` | `func (C) HelloWorld(env soroban.Env, ...)`. Names are converted to snake_case, as are parameter names; a leading `_` is dropped. `//soroban:name n` overrides a function name, `//soroban:skip` excludes a method. The `soroban.Env` parameter is optional. |
| `fn __constructor` | `func (C) Constructor(...)` |
| `fn __check_auth` | `func (C) CheckAuth(...)` |
| `-> T` | one result `T` |
| `-> Result<T, E>` | results `(T, *E)` with `E` a contract error, `(T, error)` or `(T, *soroban.Error)`; `nil` is `Ok` |
| `-> Result<(), E>` | one result `*E`, `error` or `*soroban.Error` |
| `#[contracttype] struct` | `//soroban:contracttype` on a struct. Field names are snake_case, or set with a `soroban:"name"` tag. |
| `#[contracttype] struct T(A, B)` | `//soroban:contracttype tuple` on a struct |
| `#[contracttype] enum E { A = 1 }` | `//soroban:contracttype` on a `uint32` type, with constants of the type. The type name prefix is removed from case names: `const EA E = 1` is case `A`. |
| `#[contracttype] enum U { A, B(T), C(T1, T2) }` | `//soroban:contracttype union` on a struct with a field per case: `A bool` for a void case, `B *T` for a single value, `C *soroban.Tuple2[T1, T2]` for several. Exactly one field is set. |
| `#[contracterror] enum E { A = 1 }` | `//soroban:contracterror` on a `uint32` type with constants. The type implements `error` and `soroban.ErrorValuer`. |
| `#[contractevent(topics = [..], data_format = ..)] struct` | `//soroban:contractevent topics=a,b data_format=single-value\|vec\|map` on a struct; topic fields have the tag `soroban:",topic"`. Generates `Topics`, `Data` and `Publish(env)`. |
| `#[contractclient(name = "C")] trait T` | `//soroban:contractclient name=C` on an interface. Generates a client with a method and a `Try` method per interface method. |
| `contractmeta!(key = "k", val = "v")` | `//soroban:meta k=v` |
| `panic_with_error!(&env, E::A)` | `env.PanicWithError(EA)` / `soroban.PanicWithError(EA)` |
| `assert_with_error!(&env, c, E::A)` | `soroban.AssertWithError(c, EA)` |
| `log!(&env, "msg", a, b)` | `env.Logs().Log("msg", a, b)` |
| `symbol_short!("abc")` | `soroban.SymbolShort("abc")` |

Types:

| Rust | Go |
|---|---|
| `bool`, `u32`, `i32`, `u64`, `i64` | `bool`, `uint32`, `int32`, `uint64`, `int64` |
| `u128`, `i128`, `U256`, `I256` | `soroban.U128`, `soroban.I128` (with checked `Add`, `Sub`, `Mul`, `Div`, `Rem`), `soroban.U256`, `soroban.I256` |
| `Timepoint`, `Duration` | `soroban.Timepoint`, `soroban.Duration` |
| `Bytes`, `BytesN<32>`, `Hash<32>` | `soroban.Bytes`, `soroban.BytesN[soroban.N32]`, `soroban.Hash` |
| `String`, `Symbol` | `soroban.String`, `soroban.Symbol` |
| `Address`, `MuxedAddress` | `soroban.Address`, `soroban.MuxedAddress` |
| `Vec<T>`, `Map<K, V>` | `soroban.Vec[T]`, `soroban.Map[K, V]` |
| `Option<T>` | `soroban.Option[T]` |
| `(A, B)`, `()` | `soroban.Tuple2[A, B]` (up to `Tuple6`), `soroban.Void` |
| `Val`, `Error` | `soroban.Val`, `soroban.Error` |

The environment:

| Rust | Go |
|---|---|
| `env.storage().persistent()/temporary()/instance()` | `env.Storage().Persistent()/Temporary()/Instance()` with `Has`, `Set`, `Remove`, `ExtendTTL`, `ExtendTTLWithLimits`, and `soroban.Get[V](s, key)`, `soroban.GetOr`, `soroban.Update` |
| `env.events().publish(..)` | `env.Events().Publish(topics, data)`, `event.Publish(env)` |
| `env.ledger()` | `env.Ledger()`: `ProtocolVersion`, `Sequence`, `Timestamp`, `MaxLiveUntilLedger`, `NetworkID` |
| `env.crypto()` | `env.Crypto()`: `Sha256`, `Keccak256`, `Ed25519Verify`, `Secp256k1Recover`, `Secp256r1Verify`, `MlDsa44/65/87Verify`, `Bls12381()`, `Bn254()`, Poseidon permutations |
| `env.deployer()` | `env.Deployer()`: `WithCurrentContract`, `WithAddress`, `WithStellarAsset`, `UploadContractWasm`, `UpdateCurrentContractWasm`, TTL extension |
| `env.prng()` | `env.Prng()`: `Seed`, `Bytes`, `U64`, `U64InRange`, and `soroban.Shuffle` |
| `env.invoke_contract`, `try_invoke_contract` | `soroban.InvokeContract[T]`, `soroban.TryInvokeContract[T]` |
| `env.authorize_as_current_contract` | `env.AuthorizeAsCurrentContract` with `soroban.InvokerContractAuthEntry` |
| `address.require_auth()` | `address.RequireAuth()`, `RequireAuthForArgs` |
| `token::TokenClient`, `StellarAssetClient` | `soroban.NewTokenClient`, `soroban.NewStellarAssetClient` |
| `auth::Context`, `ContractExecutable`, ... | `soroban.Context`, `soroban.ContractExecutable`, ... with the same spec as the Rust SDK's types |

Go language notes:

- Avoid floating point (`float32`, `float64`) and packages that use it, such
  as `fmt` and `strconv`; the VM rejects floating point instructions and the
  build fails with an error explaining so. `math/bits`, `errors`, `strings`
  and `bytes` are fine.
- Memory is allocated with a bump allocator that never frees, which suits a
  contract invocation's short life. Prefer the host types (`soroban.Vec`,
  `soroban.Map`, `soroban.Bytes`) over large Go slices and maps.
- Goroutines are not supported.
- A Go `panic` traps the contract, like a Rust panic.

## Host functions

[`soroban/internal/host`](soroban/internal/host) declares every host function
in [`env.json`](internal/genhost/env.json) (copied from rs-soroban-env) as a
`//go:wasmimport`. Regenerate with `make generate-host`. Off-chain builds use
stubs that panic, so packages using the SDK build and vet natively.

## Testing

```sh
make test   # builds the test contracts, runs go vet and go test
make e2e    # deploys and invokes every test contract on testnet
NETWORK=local make e2e  # ... or on a quickstart network
```

- `soroban`: unit tests of the `Val` encodings and 128-bit arithmetic.
- `internal/gen`: analysis and generation for every test contract, and that
  the committed generated code is up to date and compiles.
- `internal/spec`, `internal/xdr`, `internal/wasm`: spec shaking and name
  reduction, XDR round trips of the Rust SDK's specs, and Wasm rewriting and
  feature checks against the Rust SDK's wasms.
- `internal/parity`: spec parity, see below.
- [`scripts/e2e.sh`](scripts/e2e.sh): builds every test contract, deploys it
  with the stellar-cli, and invokes its functions, checking results, errors,
  events, auth, storage, cross-contract calls, deployment and token
  transfers. It uses js-stellar-sdk for the calls stellar-cli can't make
  (arguments of recursive types and `()`).

### Parity

[`tests/`](tests) has a Go port of each of these test contracts of the Rust SDK:
`empty`, `add_u64`, `add_i128`, `udt`, `errors`, `events`, `auth`,
`constructor`, `logging`, `contract_data`, `invoke_contract`, `tuples` and
`account`. The
directories are named like the Rust crates (`test_udt`), so the qualified type
names (`::test_udt::UdtStruct`) match.

`go test ./internal/parity` compares the spec of each Go contract with the
Rust contract's, as a set of XDR encoded entries:

- `TestSpecParity` applies the `stellar contract build` post-processing
  (shaking by references and the Rust SDK's markers, then name reduction) to
  the Rust wasms in `../target/wasm32v1-none/release`.
- `TestSpecParityStellarCLI` compares with Rust wasms built by
  `stellar contract build` itself, in the directory `STELLAR_CLI_BUILT_WASMS`.

[`tests/test_sdk`](tests/test_sdk) exercises the rest of the SDK on a network
and has no Rust counterpart.

## Differences from the Rust SDK

- There is no native test environment (`testutils`): Go contracts are tested
  by deploying them, or by running them in the Rust SDK's test environment
  with `env.register(wasm, ..)`.
- `contractimport!` (generating a client and types from a Wasm) is not
  implemented; declare a `//soroban:contractclient` interface instead.
- The meta keys are `gover` and `gosdkver` rather than `rsver` and
  `rssdkver`.
- Contract types must be declared in the contract's package.

## Layout

| Path | |
|---|---|
| [`soroban`](soroban) | The SDK. |
| [`cmd/soroban-go`](cmd/soroban-go) | The toolchain. |
| [`internal/gen`](internal/gen) | Directive analysis, code generation, and spec generation. |
| [`internal/build`](internal/build) | TinyGo invocation and Wasm post-processing. |
| [`internal/spec`](internal/spec) | Spec shaking and name reduction, ported from `soroban-spec`. |
| [`internal/xdr`](internal/xdr) | XDR of the spec and meta types. |
| [`internal/wasm`](internal/wasm) | Wasm section rewriting and feature checks. |
| [`internal/genhost`](internal/genhost) | Generator of the host function bindings. |
| [`tests`](tests) | Test contracts. |
| [`scripts`](scripts) | End-to-end tests. |
