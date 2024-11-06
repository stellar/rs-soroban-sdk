Soroban SDK supports writing smart contracts for the [Soroban] smart contract
Wasm-powered runtime, that is deployed on [Stellar].

### Docs

See [developers.stellar.org] for documentation about building smart contracts for [Stellar].

[developers.stellar.org]: https://developers.stellar.org
[Stellar]: https://stellar.org
[Soroban]: https://stellar.org/soroban

### Migrating Major Versions

See [_migrating] for a summary of how to migrate from one major version to another.

### Examples

```rust
use soroban_sdk::{contract, contractimpl, vec, symbol_short, BytesN, Env, Symbol, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}

#[test]
fn test() {
# }
# #[cfg(feature = "testutils")]
# fn main() {
    let env = Env::default();
    let contract_id = env.register(HelloContract, ());
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));

    assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
}
# #[cfg(not(feature = "testutils"))]
# fn main() { }
```

More examples are available at:
- <https://developers.stellar.org/docs/build/smart-contracts/example-contracts>
- <https://developers.stellar.org/docs/build/guides>
