#![no_std]
use stellar_contract_sdk::Env;
use stellar_contract_sdk::{contractfn, contractimpl};

#[contractfn]
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

pub struct Add2;

#[contractimpl]
impl Add2 {
    fn addimpl(a: i64, b: i64) -> i64 {
        a + b
    }
    pub fn add2(a: i64, b: i64) -> i64 {
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
    use super::{add, Add2, Add3, Add3Trait};
    use stellar_contract_sdk::Env;

    #[test]
    fn test_add() {
        [add, Add2::add2].iter().for_each(|f| {
            let x = 10i64;
            let y = 12i64;
            let z = f(x, y);
            assert_eq!(z, 22);
        });
        [Add3::add3].iter().for_each(|f| {
            let e = Env::default();
            let x = 10i64;
            let y = 12i64;
            let z = f(e, x, y);
            assert_eq!(z, 22);
        });
    }
}

#[cfg(test)]
mod test_via_val {
    use super::{__add, __add2, __add3};
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add_val() {
        [__add, __add2, __add3].iter().for_each(|f| {
            let e = Env::default();
            let x = 10i64.into_val(&e);
            let y = 12i64.into_val(&e);
            let z = f(e.clone(), x, y);
            let z = i64::try_from_val(&e, z).unwrap();
            assert_eq!(z, 22);
        });
    }
}
