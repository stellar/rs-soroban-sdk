#![no_std]
use stellar_contract_sdk::{Env, EnvValType, OrAbort, RawVal};

#[no_mangle]
pub fn add(e: &Env, a: RawVal, b: RawVal) -> RawVal {
    let a: i64 = i64::try_from_raw_val(e, a).or_abort();
    let b: i64 = i64::try_from_raw_val(e, b).or_abort();

    let c = a + b;

    return c.into_raw_val(e);
}

#[cfg(test)]
mod test {
    use super::add;
    use stellar_contract_sdk::{Env, EnvValType, OrAbort};

    #[test]
    fn test_add() {
        let e = &Env::default();
        let x = 10i64.into_raw_val(e);
        let y = 12i64.into_raw_val(e);
        let z = add(e, x, y);
        let z = i64::try_from_raw_val(e, z).or_abort();
        assert!(z == 22);
    }
}
