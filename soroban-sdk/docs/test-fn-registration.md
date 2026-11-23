# Test Function Registration Internals

This document describes how contract functions are registered with the test environment using module init functions, so that native tests can call them. It is intended for SDK developers who need to understand or modify the macro implementation.

## Overview

With the `testutils` feature, tests run a contract as native Rust rather than as Wasm. Building is fast, and the contract can be stepped through in a debugger:

```rust
#[test]
fn test_transfer() {
    let env = Env::default();
    let addr = env.register(Token, ());
    let client = TokenClient::new(&env, &addr);
    client.transfer(&from, &to, &10);
}
```

When a contract is invoked, the Soroban host has only the function's name and a list of `Val` arguments. To call a native contract it looks the function up by name in a map of the contract's functions.

Each `#[contractimpl]` block registers its own functions in that map from a module init function: a function that runs when the binary is loaded, before `main` and before any test. Registration is done this way, rather than by one macro building the whole map, because a contract's functions can be spread across multiple impl blocks, including blocks generated for trait default functions (see [contracttrait.md](contracttrait.md)), and each macro call sees only its own block. By the time the first test runs, the map holds every function from every impl block.

```rust
#[contractimpl]
impl Token {
    pub fn transfer(...) { ... }
}

#[contractimpl(contracttrait)]
impl Pause for Token {} // paused, pause, unpause
```

generates, simplified:

```rust
// From `impl Token`
#[ctor]
fn __Token____<hash>_ctor() {
    <Token as ContractFunctionRegister>::register("transfer", &__Token__transfer__invoke_raw_slice);
}

// From `impl Pause for Token`
#[ctor]
fn __Token__Pause__<hash>_ctor() {
    <Token as ContractFunctionRegister>::register("paused", &__Token__paused__invoke_raw_slice);
    <Token as ContractFunctionRegister>::register("pause", &__Token__pause__invoke_raw_slice);
    <Token as ContractFunctionRegister>::register("unpause", &__Token__unpause__invoke_raw_slice);
}
```

Each platform runs module init functions differently (e.g. the `.init_array` section on ELF), and the [`ctor`](https://crates.io/crates/ctor) crate abstracts that away.

All of this code is only generated when the `testutils` feature is enabled, and so is never part of a Wasm build.

## How it works

### Stage 1: `#[contract]` generates the registry

**Source:** `soroban-sdk-macros/src/lib.rs` (`contract`)

`#[contract]` generates a module per contract type holding a static map, and implements two traits from `soroban-sdk/src/testutils.rs` on the contract type that forward to it:

```rust
mod __token_fn_set_registry {
    pub type F = soroban_sdk::testutils::ContractFunctionF; // dyn Send + Sync + Fn(Env, &[Val]) -> Val

    static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());

    pub fn register(name: &'static str, func: &'static F) { /* insert into FUNCS */ }
    pub fn call(name: &str, env: Env, args: &[Val]) -> Option<Val> { /* look up in FUNCS and call */ }
}

impl soroban_sdk::testutils::ContractFunctionRegister for Token { /* register */ }
impl soroban_sdk::testutils::ContractFunctionSet for Token { /* call */ }
```

### Stage 2: `#[contractimpl]` generates slice wrappers

**Source:** `soroban-sdk-macros/src/derive_fn.rs` (`derive_pub_fns`)

For each function, alongside the `extern "C"` Wasm export, the macro generates an `__<Type>__<fn>__invoke_raw_slice` function that takes the arguments as a `&[Val]`, checks the argument count, and calls the same conversion and dispatch code the Wasm export uses. This gives every function the same signature so they can be stored in one map.

### Stage 3: `#[contractimpl]` generates the ctor

**Source:** `soroban-sdk-macros/src/derive_fn.rs` (`derive_contract_function_registration_ctor`)

The macro generates a `#[ctor]` function that registers each of the block's functions by name with its slice wrapper. `contractimpl_trait_default_fns_not_overridden!` generates one for the trait default functions in the same way.

The ctor's name must be unique, because every impl block for the contract type generates one in the same module. It is built from the type, the trait if any, and a SHA-256 hash of the function names. The function names in the hash are prefixed with any `cfg` attributes on the function, so that a cfg-gated override and the trait default generated under the inverse cfg produce different names. The `cfg` attributes are also applied to each `register` call, so that only functions compiled in are registered.

`ctor` is re-exported from `soroban_sdk::reexports_for_macros` so that contract crates don't need to depend on it themselves.

### Stage 4: the test environment dispatches through the map

**Source:** `soroban-sdk/src/env.rs` (`InternalContractFunctionSet`)

`env.register(Token, ())` hands the contract type, as a `ContractFunctionSet`, to the host. When the contract is invoked, the host calls `InternalContractFunctionSet::call` with the function's `Symbol`, which converts the name to a string and calls `Token::call`, which looks up the function in the map and calls straight into the contract's code.

## Things to know

- **Registration is global per contract type.** The map is a static, shared by every `Env` and every test in the binary. Registration is idempotent, so this is fine, but a function registered by any impl block is callable in every test.
- **Ctors run before `main`.** The code in them should be minimal and must not depend on anything else being initialized. The SDK's ctors only insert into a `Mutex<BTreeMap>`, which can be constructed in const context.
- **Platform support comes from `ctor`.** Native tests only work on platforms the `ctor` crate supports.

## Source Files

| File | Purpose |
|------|---------|
| `soroban-sdk-macros/src/lib.rs` | `#[contract]` generates the registry, and `#[contractimpl]` calls the ctor generation |
| `soroban-sdk-macros/src/derive_fn.rs` | Generates the slice wrappers and the registration ctor |
| `soroban-sdk-macros/src/derive_contractimpl_trait_default_fns_not_overridden.rs` | Generates the registration ctor for trait default functions |
| `soroban-sdk/src/testutils.rs` | `ContractFunctionRegister`, `ContractFunctionSet` and `ContractFunctionF` |
| `soroban-sdk/src/env.rs` | Adapts a `ContractFunctionSet` to the host |
