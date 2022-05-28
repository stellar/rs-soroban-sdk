#![no_std]
use stellar_contract_sdk::contractfn;
use stellar_contract_sdk::Env;

#[contractfn]
pub fn add(_e: Env, a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod test {
    use super::__add;
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add() {
        let e = Env::default();
        let x = 10i64.into_val(&e);
        let y = 12i64.into_val(&e);
        let z = __add(e.clone(), x, y);
        let z = i64::try_from_val(&e, z).unwrap();
        assert!(z == 22);
    }
}
