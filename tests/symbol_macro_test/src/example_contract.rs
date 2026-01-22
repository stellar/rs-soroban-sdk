use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol, symbol_short};

#[contract]
pub struct SymbolDemoContract;

#[contractimpl]
impl SymbolDemoContract {
    /// Demonstrates the different ways to create symbols
    pub fn symbol_demo(env: Env) -> (Symbol, Symbol, Symbol, Symbol) {
        // Method 1: symbol_short! - for constants ≤9 chars (compile-time)
        let short_const = symbol_short!("constant");
        
        // Method 2: Symbol::new - runtime optimization attempt  
        let using_new = Symbol::new(&env, "runtime_new");
        
        // Method 3: symbol! macro - optimal choice made at compile-time
        let short_optimal = symbol!(&env, "optimal");    // → compile-time constant  
        let long_optimal = symbol!(&env, "very_long_symbol_name"); // → runtime host call
        
        (short_const, using_new, short_optimal, long_optimal)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symbol_demo() {
        let env = Env::default();
        let contract_id = env.register(SymbolDemoContract, ());
        let client = SymbolDemoContractClient::new(&env, &contract_id);

        let (short_const, using_new, short_optimal, long_optimal) = client.symbol_demo();
        
        // All methods should create valid symbols
        assert_eq!(short_const, symbol_short!("constant"));
        assert_eq!(using_new, Symbol::new(&env, "runtime_new"));
        assert_eq!(short_optimal, symbol_short!("optimal"));
        assert_eq!(long_optimal, Symbol::new(&env, "very_long_symbol_name"));
    }
}