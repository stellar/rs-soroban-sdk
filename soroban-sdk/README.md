Soroban SDK supports writing programs for the Soroban smart contract
platform.

### Docs

See [soroban.stellar.org](https://soroban.stellar.org) for documentation.

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
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));

    assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
}
# #[cfg(not(feature = "testutils"))]
# fn main() { }
```

More examples are available at <https://soroban.stellar.org/docs/category/basic-tutorials>
and <https://soroban.stellar.org/docs/category/advanced-tutorials>.
