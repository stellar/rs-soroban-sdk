#![no_std]
use soroban_sdk::contractimpl;

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod test {
    use soroban_sdk::{Env, FixedBinary};

    use crate::{add, Contract};

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = FixedBinary::from_array(&e, [0; 32]);
        e.register_contract(&contract_id, Contract);

        let x = 10i32;
        let y = 12i32;
        let z = add::invoke(&e, &contract_id, &x, &y);
        assert!(z == 22);
    }
}
