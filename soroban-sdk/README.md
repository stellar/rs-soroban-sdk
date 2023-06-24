Soroban SDK supports writing programs for the Soroban smart contract
platform.

### Docs

See [soroban.stellar.org](https://soroban.stellar.org) for documentation.

### Examples

```rust
use soroban_sdk::{contractimpl, vec, BytesN, Env, Symbol, Vec};

pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));

    assert_eq!(words, vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]);
}
```

More examples are available at <https://soroban.stellar.org/docs/category/how-to-guides>.
