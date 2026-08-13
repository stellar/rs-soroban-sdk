#![no_std]
use soroban_sdk::{contract, contractimpl, U256};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn run(a: U256, b: U256) {
        if a < b {
            panic!("unexpected")
        }
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use soroban_sdk::{
        testutils::{arbitrary::SorobanArbitrary, proptest::arb, EnvTestConfig},
        Env, IntoVal, U256,
    };

    use crate::{Contract, ContractClient};

    proptest! {
        /// The prototype is named in the parameter list, in the same way that a
        /// fuzz test names it in its input struct.
        #[test]
        fn test_run_prototype(
            a: <U256 as SorobanArbitrary>::Prototype,
            b: <U256 as SorobanArbitrary>::Prototype,
        ) {
            let env = Env::new_with_config(EnvTestConfig {
                capture_snapshot_at_drop: false,
            });

            let a: U256 = a.into_val(&env);
            let b: U256 = b.into_val(&env);

            let contract_id = env.register(Contract, ());
            let client = ContractClient::new(&env, &contract_id);

            assert_eq!(client.try_run(&a, &b).is_ok(), a >= b);
        }

        /// The `arb` strategy names the contract type rather than its prototype.
        #[test]
        fn test_run(a in arb::<U256>(), b in arb::<U256>()) {
            // The snapshot of this test is a list of randomly generated values
            // that has no review value, so don't capture one.
            let env = Env::new_with_config(EnvTestConfig {
                capture_snapshot_at_drop: false,
            });

            let a: U256 = a.into_val(&env);
            let b: U256 = b.into_val(&env);

            let contract_id = env.register(Contract, ());
            let client = ContractClient::new(&env, &contract_id);

            assert_eq!(client.try_run(&a, &b).is_ok(), a >= b);
        }
    }
}
