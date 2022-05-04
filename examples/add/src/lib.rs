#![no_std]
use stellar_contract_env::EnvValType;
use stellar_contract_sdk::{Env, OrAbort, Val};

#[no_mangle]
pub fn add(e: Env, a: Val, b: Val) -> Val {
    let a: i64 = i64::try_from_val(e.clone(), a).or_abort();
    let b: i64 = i64::try_from_val(e.clone(), b).or_abort();

    let c = a + b;

    return c.into_env_val(e).val;
}

#[cfg(test)]
mod test {
    use super::add;
    extern crate alloc;
    use alloc::rc::Rc;
    use stellar_contract_env::{EnvValType, Host, OrAbort};
    use stellar_contract_sdk::{Env, Val};

    #[test]
    fn test_add() {
        let h = Rc::new(Host::default());
        let e: Env = Env::from(&h);

        let x: Val = 10i64.into_env_val(e.clone()).val;
        let y: Val = 12i64.into_env_val(e.clone()).val;
        let z: Val = add(e.clone(), x, y);
        let z: i64 = i64::try_from_val(e.clone(), z).or_abort();
        assert!(z == 22);
    }
}
