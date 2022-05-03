#![no_std]
use stellar_contract_sdk::{OrAbort, Val};

#[no_mangle]
pub fn add(a: Val, b: Val) -> Val {
    let a: i64 = a.try_into().or_abort();
    let b: i64 = b.try_into().or_abort();

    let c = a + b;

    return c.try_into().or_abort();
}

#[cfg(test)]
mod test {
    use super::add;
    use stellar_contract_sdk::Val;

    #[test]
    fn test_add() {
        let x: Val = Val::from_i64(10);
        let y: Val = Val::from_i64(12);
        let z: Val = add(x, y);
        let z: i64 = z.try_into().unwrap();
        assert!(z == 22);
    }
}
