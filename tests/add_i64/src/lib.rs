#![no_std]
use soroban_sdk::{contractimpl, Env};

// There are two ways to export contract fns:

// 1. Using the `contractimpl` macro on a struct impl.

pub struct Add1;

#[contractimpl]
impl Add1 {
    fn addimpl(a: i64, b: i64) -> i64 {
        a + b
    }
    pub fn add1(a: i64, b: i64) -> i64 {
        Self::addimpl(a, b)
    }
}

// 2. Using the `contractimpl` macro on a trait impl.

pub trait Add2Trait {
    fn add2(e: Env, a: i64, b: i64) -> i64;
}

pub struct Add2;

#[contractimpl]
impl Add2Trait for Add2 {
    fn add2(_e: Env, a: i64, b: i64) -> i64 {
        a + b
    }
}

#[cfg(test)]
mod test {
    use super::{Add1, Add2, Add2Trait};
    use soroban_sdk::Env;

    #[test]
    fn test_add() {
        let x = 10i64;
        let y = 12i64;
        let z = Add1::add1(x, y);
        assert_eq!(z, 22);

        let e = Env::default();
        let z = Add2::add2(e, x, y);
        assert_eq!(z, 22);
    }
}

#[cfg(test)]
mod test_via_val {
    use super::*;
    use soroban_sdk::{BytesN, Env};

    #[test]
    fn test_add_1() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, [0; 32]);
        e.register_contract(&contract_id, Add1);

        let x = 10i64;
        let y = 12i64;
        let z = add1::invoke(&e, &contract_id, &x, &y);
        assert!(z == 22);
    }

    #[test]
    fn test_add_2() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, [0; 32]);
        e.register_contract(&contract_id, Add2);

        let x = 10i64;
        let y = 12i64;
        let z = add2::invoke(&e, &contract_id, &x, &y);
        assert!(z == 22);
    }
}
