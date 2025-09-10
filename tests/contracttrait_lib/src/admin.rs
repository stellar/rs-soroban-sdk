use soroban_sdk::{contracttrait, symbol_short, Address, Env, Symbol};

/// Trait for using an admin address to control access.
#[contracttrait]
pub trait Administratable {
    fn admin(env: &Env) -> soroban_sdk::Address {
        unsafe { get(env).unwrap_unchecked() }
    }
    fn set_admin(env: &Env, new_admin: &soroban_sdk::Address) {
        if let Some(admin) = get(env) {
            admin.require_auth()
        }
        env.storage().instance().set(&STORAGE_KEY, new_admin);
    }
    #[internal]
    fn require_admin(env: &Env) {
        Self::admin(env).require_auth();
    }
}

pub const STORAGE_KEY: Symbol = symbol_short!("ADMIN");

fn get(env: &Env) -> Option<Address> {
    env.storage().instance().get(&STORAGE_KEY)
}
