use soroban_sdk::{contracttype, Address, Env, Map};

#[contracttype]
#[derive(Clone)]
pub struct Roles {
    pub admin: Address,
    pub operators: Map<Address, bool>,
}

pub fn only_admin(e: &Env, roles: &Roles, caller: &Address) {
    if &roles.admin != caller {
        panic!("Not authorized: Admin only");
    }
}

pub fn only_operator(e: &Env, roles: &Roles, caller: &Address) {
    if roles.operators.get(caller.clone()).unwrap_or(false) != true {
        panic!("Not authorized: Operator only");
    }
}
