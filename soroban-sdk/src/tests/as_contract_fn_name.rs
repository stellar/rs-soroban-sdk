//! Demonstrates using the function name derived by
//! [`crate::Env::as_contract`] to mock an authorization by name.
//!
//! When a named function is passed to `as_contract`, its name becomes the
//! contract frame's function name. Authorizations performed inside the frame
//! are recorded under that name, so a [`MockAuth`] can target them by name,
//! the same way it targets a call to a named contract function.

use crate as soroban_sdk;

use soroban_sdk::{
    auth::{Context, CustomAccountInterface},
    contract, contracterror, contractimpl,
    crypto::Hash,
    testutils::{MockAuth, MockAuthInvoke},
    Address, Env, Error, IntoVal, Vec,
};

#[contracterror]
#[derive(Debug, Eq, PartialEq)]
enum AccountError {
    Fail = 1,
}

// A custom account that approves every authorization, used as the address
// whose auth is required inside the frame.
#[contract]
struct Account;

#[contractimpl]
impl CustomAccountInterface for Account {
    type Signature = ();
    type Error = AccountError;
    #[allow(non_snake_case)]
    fn __check_auth(
        _env: Env,
        _signature_payload: Hash<32>,
        _signature: (),
        _auth_contexts: Vec<Context>,
    ) -> Result<(), AccountError> {
        Ok(())
    }
}

#[contract]
struct Contract;

#[contractimpl]
impl Contract {}

#[test]
fn mock_auth_targets_derived_fn_name() {
    // The named function to run inside the frame. Its name, `mint`, becomes the
    // frame's function name. A `fn` can't capture, so the address it authorizes
    // is passed in through a thread-local.
    std::thread_local! {
        static ADMIN: std::cell::RefCell<Option<Address>> =
            const { std::cell::RefCell::new(None) };
    }
    fn mint() {
        ADMIN.with(|a| a.borrow().as_ref().unwrap().require_auth());
    }

    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let admin = env.register(Account, ());
    ADMIN.with(|a| *a.borrow_mut() = Some(admin.clone()));

    // Authorize `admin` for a call named `mint`, the name derived from the
    // `mint` function passed to `as_contract`. The `require_auth` inside the
    // frame is satisfied because the frame's function name matches.
    env.set_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "mint",
            args: ().into_val(&env),
            sub_invokes: &[],
        },
    }
    .into()]);
    env.as_contract(&contract_id, mint);

    // Mocking a different name leaves the `require_auth` unauthorized, showing
    // the match above was on the derived name and not a coincidence.
    env.set_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "not_mint",
            args: ().into_val(&env),
            sub_invokes: &[],
        },
    }
    .into()]);
    assert!(env
        .try_as_contract::<(), Error>(&contract_id, mint)
        .is_err());
}
