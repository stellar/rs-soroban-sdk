#![no_std]
use sdk::{OrAbort, Val};
use stellar_contract_sdk as sdk;

#[no_mangle]
pub fn add(a: Val, b: Val) -> Val {
    sdk::log_value(a);
    let a: u64 = a.try_into().or_abort();

    sdk::log_value(b);
    let b: u64 = b.try_into().or_abort();

    let c = a + b;
    let d: Val = c.try_into().or_abort();
    sdk::log_value(d);

    d
}

#[cfg(test)]
mod test {
    use super::add;
    use sdk::Val;
    use stellar_contract_sdk as sdk;
    extern crate alloc;
    extern crate std;

    #[test]
    fn test_add() {
        let x: Val = Val::from(10 as u64);
        let y: Val = Val::from((1u64 << 63) as u64);
        let z: Val = add(x, y);
        let z: u64 = z.try_into().unwrap();
        let expect: u64 = (1u64 << 63) + 10;
        assert_eq!(z, expect);
    }

    #[test]
    fn test_add_explicitly_using_from() {
        let x: Val = Val::from_u64(10);
        let y: Val = Val::from_u64(1 << 63);
        let z: Val = add(x, y);
        let z: u64 = z.try_into().unwrap();
        let expect: u64 = (1u64 << 63) + 10;
        assert_eq!(z, expect);
    }
}
