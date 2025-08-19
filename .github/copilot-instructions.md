# Stellar Soroban SDK for Rust

Soroban SDK is a Rust workspace for writing smart contracts for Stellar blockchain's [smart contract platform](https://developers.stellar.org/docs/learn/fundamentals/contract-development) known as Soroban. The SDK includes the core library, macros, specifications, token utilities, and comprehensive test contracts demonstrating various features.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Bootstrap and Setup
- Install Rust toolchain:
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  - `rustup install stable`
  - `rustup +stable target add wasm32v1-none`
  - `rustup +stable component add rust-src`
- Install Rust nightly (required for documentation):
  - `rustup install nightly`
  - `rustup +nightly target add wasm32v1-none`
  - `rustup +nightly component add rust-src`
- Install required cargo tools:
  - `cargo install --locked cargo-hack`
  - `cargo install --locked cargo-nextest`
  - `cargo install --locked cargo-watch`

### Build Commands
- **CRITICAL**: Format code first: `make fmt` -- takes ~1 second
- **CRITICAL**: Build: `make build` -- takes 3 minutes. NEVER CANCEL. Set timeout to 10+ minutes.
- **CRITICAL**: Run tests: `make test` -- takes 6 minutes. NEVER CANCEL. Set timeout to 15+ minutes.
- **CRITICAL**: Generate docs: `make doc` -- takes 2 minutes. NEVER CANCEL. Set timeout to 10+ minutes.
- **CRITICAL**: Check code: `make check` -- takes 3.5 minutes. NEVER CANCEL. Set timeout to 10+ minutes.

### Development Workflow
- Watch for changes during development:
  - `cargo watch --clear --exec "check --package soroban-sdk"` for core SDK changes
  - `cargo watch --clear --exec "test --package soroban-sdk"` for test-driven development
  - `cargo watch --clear --shell "make build"` to continuously build all targets
- **ALWAYS** run `make fmt` before committing changes or the CI will fail
- **ALWAYS** run `make check` and `make test` before submitting changes

### Manual Testing Scenarios
After making changes to the SDK, always validate with these scenarios:

1. **Basic Smart Contract Creation**:
   - Create a simple contract with the SDK macros (`#[contract]`, `#[contractimpl]`)
   - **CRITICAL**: Include `#![no_std]` at the top of contract files
   - Verify it compiles to Wasm successfully: `cargo build --target wasm32v1-none --release`
   - Test contract functions using the test framework: `cargo test`
   - Verify Wasm file is generated in `target/wasm32v1-none/release/` (typically 500B-3KB)

2. **SDK Feature Testing**:
   - Use core types like `Env`, `Symbol`, `Map`, `Vec`, `BytesN`
   - Test storage operations (instance, persistent, temporary)
   - Verify auth and event functionality
   - Test token operations if making token-related changes
   - Use `log!` macro for debugging in tests

3. **Contract Compilation Validation**:
   - Build all test contracts: `make build` or `cargo hack build --target wasm32v1-none --release --workspace --exclude soroban-spec --exclude soroban-spec-rust --exclude soroban-ledger-snapshot`
   - Verify all Wasm files are generated correctly
   - Check that Wasm file sizes are reasonable (200B-3KB range)
   - Ensure no compilation warnings for the main SDK packages

4. **Integration Testing**:
   - Run specific test contract: `cargo test -p test_[name]` (e.g., `cargo test -p test_auth`)
   - Test contract interactions using the client generation
   - Verify contract state management and data persistence
   - Test error handling with custom error types

### Pre-commit Checklist
- Run `make fmt` to format code
- Run `make check` to validate compilation
- Run `make test` to execute test suite
- Build documentation with `make doc` if modifying public APIs
- Test specific contracts in `tests/` directory if making core changes

## Repository Structure

### Key Directories
- `soroban-sdk/` - Main SDK crate with core functionality for creating smart contracts
- `soroban-sdk-macros/` - Procedural macros for generating contract clients and types
- `soroban-spec/` - Contract specification utilities
- `soroban-spec-rust/` - Rust code generation from contract specs
- `soroban-ledger-snapshot/` - Ledger snapshot utilities for testing
- `soroban-token-sdk/` - Token-specific SDK functionality
- `tests/` - Comprehensive test contracts demonstrating SDK features:
  - `tests/auth/` - Authentication examples
  - `tests/events/` - Event emission examples
  - `tests/storage/` - Storage pattern examples
  - `tests/constructor/` - Contract initialization examples
  - `tests/errors/` - Error handling examples

### Important Files
- `Cargo.toml` - Workspace configuration
- `Makefile` - Common development commands
- `rust-toolchain.toml` - Specifies stable toolchain with wasm32v1-none target
- `.github/workflows/rust.yml` - CI pipeline configuration
- `deny.toml` - Dependency and license validation

### Common Code Patterns
- Contracts use `#[contract]` and `#[contractimpl]` macros
- Test utilities available with `testutils` feature
- All contracts compile to Wasm target `wasm32v1-none`
- Tests use `Env::default()` for simulation environment

## Build System Details

### Targets and Features
- **Native target**: `x86_64-unknown-linux-gnu` for tooling and tests
- **Wasm target**: `wasm32v1-none` for smart contracts
- **Key features**: `testutils` enables testing utilities, `hazmat` enables low-level APIs
- **Native-only crates**: `soroban-spec`, `soroban-spec-rust`, `soroban-ledger-snapshot`

### Cargo Tools Usage
- `cargo-hack` - Feature powerset testing and multi-target builds
- `cargo-nextest` - Fast test execution (alternative to `cargo test`)
- `cargo-watch` - File watching for development

### Build Artifacts
- Wasm contracts output to `target/wasm32v1-none/release/*.wasm`
- Documentation generates to `target/doc/soroban_sdk/`
- Test snapshots stored in various `test_snapshots/` directories

## Troubleshooting

### Common Issues
- **Missing Wasm target**: Run `rustup target add wasm32v1-none` if build fails
- **Cargo tool missing**: Install with `cargo install --locked [tool-name]`
- **CI failures**: Always run `make fmt` before committing

### Performance Notes
- Initial builds download and compile many dependencies (~3 minutes)
- Incremental builds are much faster (~10-30 seconds)
- Test execution includes both unit tests and integration tests
- Documentation build requires nightly toolchain

## Example Contract Workflow

```rust
#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(_env: Env, _name: Symbol) -> Symbol {
        symbol_short!("Hello")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn test_hello() {
        let env = Env::default();
        let contract_id = env.register(HelloContract, ());
        let client = HelloContractClient::new(&env, &contract_id);

        let result = client.hello(&symbol_short!("world"));
        assert_eq!(result, symbol_short!("Hello"));
    }
}
```

**CRITICAL**: All smart contracts must include `#![no_std]` at the top to
produce minimal binaries. All the non-test SDK features must support building with `#![no_std]` as well.

After creating a contract:
1. Build Wasm: `cargo build --target wasm32v1-none --release`
2. Test: `cargo test` (tests run on native target with std)
3. Verify Wasm output in `target/wasm32v1-none/release/` (typically 500B-3KB)
4. Check contract functions work as expected in test environment