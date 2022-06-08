#![no_std]
use stellar_contract_sdk::contractfn;
use stellar_contract_sdk::Env;

#[contractfn]
pub fn add(_e: Env, a: i64, b: i64) -> i64 {
    a + b
}

#[contractfn]
pub fn add2(_e: Env, a: i64, b: i64, c: i64) -> i64 {
    a + b + c
}

#[cfg(test)]
mod test {
    use super::{_add, _add2};
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add() {
        let e = Env::default();
        let x = 10i64.into_val(&e);
        let y = 12i64.into_val(&e);
        let z = _add(e.clone(), x, y);
        let z = i64::try_from_val(&e, z).unwrap();
        assert!(z == 22);
    }

    #[test]
    fn test_add2() {
        let e = Env::default();
        let x = 10i64.into_val(&e);
        let y = 12i64.into_val(&e);
        let y2 = 12i64.into_val(&e);
        let z = _add2(e.clone(), x, y, y2);
        let z = i64::try_from_val(&e, z).unwrap();
        assert!(z == 34);
    }
}
