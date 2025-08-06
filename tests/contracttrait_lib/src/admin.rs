use soroban_sdk::{contracttrait, symbol_short, Address, Env, Symbol};

/// Trait for using an admin address to control access.
#[contracttrait(add_impl_type = true)]
pub trait Administratable {
    fn admin(env: &Env) -> soroban_sdk::Address;
    fn set_admin(env: &Env, new_admin: &soroban_sdk::Address);
    #[internal]
    fn require_admin(env: &Env) {
        Self::admin(env).require_auth();
    }
}

pub const STORAGE_KEY: Symbol = symbol_short!("ADMIN");

fn get(env: &Env) -> Option<Address> {
    env.storage().instance().get(&STORAGE_KEY)
}

pub struct Admin;

impl Administratable for Admin {
    type Impl = Admin;
    fn admin(env: &Env) -> soroban_sdk::Address {
        unsafe { get(env).unwrap_unchecked() }
    }
    fn set_admin(env: &Env, new_admin: &soroban_sdk::Address) {
        if let Some(admin) = get(env) {
            admin.require_auth()
        }
        env.storage().instance().set(&STORAGE_KEY, new_admin);
    }
}
