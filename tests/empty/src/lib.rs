#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    // Function with void argument and void return type
    pub fn void_fn(env: Env, void_argument: ()) -> () {
        void_argument
    }
    
    // Function with no explicit return (implicitly returns ())
    pub fn empty() {}
    
    // Function with tuple argument and tuple return type for comparison
    pub fn tuple_fn(env: Env, tuple_arg: (i32, u32)) -> (i32, u32) {
        tuple_arg
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_void_function() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        // This should work with our fix - () parameters and return values
        client.void_fn(&());
        client.empty();
        
        // This should still work for tuple types
        let result = client.tuple_fn(&(42, 100u32));
        assert_eq!(result, (42, 100u32));
    }
}