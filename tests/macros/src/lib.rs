// A test vector for testing the interactions of the soroban-sdk macros with third-party macros,
// validating that they are composable and compatible.

#![no_std]
use proc_macros::{check_fn_is_item_fn, parse_item_fn, parse_item_impl};
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct Contract;

#[contractimpl]
#[parse_item_impl]
impl Contract {
    // Test that attribute macros that expect to be used on fns are composable with contractimpl.
    #[parse_item_fn]
    pub fn empty() {}

    // Test that attribute macros are not copied to the generated export functions. See test below.
    #[check_fn_is_item_fn]
    pub fn empty2() {}
}

#[cfg(test)]
mod test {
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_empty() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.empty();
    }

    // Test that custom attribute macros on methods in #[contractimpl] are not
    // incorrectly applied to generated wrapper functions.
    //
    // The #[check_fn_is_item_fn] macro injects code that references `Self`,
    // which is only valid inside an impl block. If #[contractimpl] incorrectly
    // passes the attribute to generated free functions (like __invoke_raw_slice),
    // the injected `Self` reference would cause a compile error:
    //   "error[E0411]: cannot find type `Self` in this scope"
    //
    // This test passing means the attribute is correctly filtered out of
    // generated wrapper functions and only applied to the original method.
    #[test]
    fn test_custom_attrs_are_not_copied_onto_generated_fns() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        client.empty2();
    }
}
