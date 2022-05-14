#![no_std]
use stellar_contract_sdk::{Env, EnvValConvertible, OrAbort, RawVal};

#[no_mangle]
pub fn add(e: Env, a: RawVal, b: RawVal) -> RawVal {
    let a: i32 = i32::try_from_val(&e, a).or_abort();
    let b: i32 = i32::try_from_val(&e, b).or_abort();

    let c = a + b;

    return c.into_val(&e);
}

#[cfg(test)]
mod test {
    use super::add;
    use stellar_contract_sdk::{Env, EnvValConvertible, OrAbort};

    #[test]
    fn test_add() {
        let e = Env::default();
        let x = 10i32.into_val(&e);
        let y = 12i32.into_val(&e);
        let z = add(e.clone(), x, y);
        let z = i32::try_from_val(&e, z).or_abort();
        assert!(z == 22);
    }
}
