#![no_std]
use stellar_contract_sdk::{contractimpl, Env};

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
    use stellar_contract_sdk::Env;

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
    use super::{__add1, __add2};
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add_val() {
        for f in [__add1, __add2] {
            let e = Env::default();
            let x = 10i64.into_val(&e);
            let y = 12i64.into_val(&e);
            let z = f(e.clone(), x, y);
            let z = i64::try_from_val(&e, z).unwrap();
            assert_eq!(z, 22);
        }
    }
}
