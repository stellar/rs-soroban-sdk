#![no_std]
use stellar_contract_sdk::Env;
use stellar_contract_sdk::{contractfn, contractimpl};

#[contractfn]
pub fn add(_e: Env, a: i64, b: i64) -> i64 {
    a + b
}

pub struct Add2;

#[contractimpl]
impl Add2 {
    fn addimpl(a: i64, b: i64) -> i64 {
        a + b
    }
    pub fn add2(_e: Env, a: i64, b: i64) -> i64 {
        Self::addimpl(a, b)
    }
}

pub trait Add3Trait {
    fn add3(e: Env, a: i64, b: i64) -> i64;
}

pub struct Add3;

#[contractimpl]
impl Add3Trait for Add3 {
    fn add3(_e: Env, a: i64, b: i64) -> i64 {
        a + b
    }
}

#[cfg(test)]
mod test {
    use super::{__add, __add2, __add3};
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add() {
        [__add, __add2, __add3].iter().for_each(|f| {
            let e = Env::default();
            let x = 10i64.into_val(&e);
            let y = 12i64.into_val(&e);
            let z = f(e.clone(), x, y);
            let z = i64::try_from_val(&e, z).unwrap();
            assert!(z == 22);
        });
    }
}
