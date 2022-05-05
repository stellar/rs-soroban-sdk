#![no_std]
use stellar_contract_sdk::{Env, EnvValType, OrAbort, Val};

#[no_mangle]
pub fn add(e: Env, a: Val, b: Val) -> Val {
    let a: i64 = i64::try_from_val(e.clone(), a).or_abort();
    let b: i64 = i64::try_from_val(e.clone(), b).or_abort();

    let c = a + b;

    return c.into_val(e);
}

#[cfg(test)]
mod test {
    use super::add;
    use stellar_contract_sdk::{Env, EnvValType, Host, OrAbort, Val};

    #[test]
    fn test_add() {
        let h = &Host::default();
        let e: Env = h.into();

        let x: Val = 10i64.into_val(e.clone());
        let y: Val = 12i64.into_val(e.clone());
        let z: Val = add(e.clone(), x, y);
        let z: i64 = i64::try_from_val(e.clone(), z).or_abort();
        assert!(z == 22);
    }
}
