Soroban SDK supports writing programs for the Soroban smart contract
platform.

### Docs

See [soroban.stellar.org](https://soroban.stellar.org) for documentation.

### Examples

```rust
use soroban_sdk::{contractimpl, symbol, vec, BytesN, Env, Symbol, Vec};

pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol!("Hello"), to]
    }
}

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol!("Dev"));

    assert_eq!(words, vec![&env, symbol!("Hello"), symbol!("Dev"),]);
}
```

More examples are available at <https://soroban.stellar.org/docs/category/examples>.
