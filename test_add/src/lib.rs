#![no_std]
use sdk::{OrAbort, Val};
use stellar_contract_sdk as sdk;
use stellar_contract_sdk_macros as sdkmacros;

#[no_mangle]
#[sdkmacros::contractfn]
pub fn add(a: i32, b: i32) -> i32 {
    let a: i32 = a.try_into().or_abort();
    let b: i32 = b.try_into().or_abort();

    let c = a + b;

    return c.try_into().or_abort();
}

#[cfg(test)]
mod test {
    use super::__cf_add;
    use super::add;
    use sdk::Val;
    use stellar_contract_sdk as sdk;

    #[test]
    fn test_add() {
        assert_eq!(add(10, 12), 22);
        let x: Val = Val::from_i32(10);
        let y: Val = Val::from_i32(12);
        let r: Val = Val::from_i32(22);
        assert_eq!(__cf_add(x, y), r);
    }
}
