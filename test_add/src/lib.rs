#![no_std]
use sdk::{OrAbort, Val};
use stellar_contract_sdk as sdk;
use stellar_contract_sdk_macros as sdkmacros;

#[no_mangle]
#[sdkmacros::contractfn]
pub fn add(a: i64, b: i64) -> i64 {
    let a: i64 = a.try_into().or_abort();
    let b: i64 = b.try_into().or_abort();

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
        let x: Val = Val::from_u63(10);
        let y: Val = Val::from_u63(12);
        let r: Val = Val::from_u63(22);
        assert_eq!(__cf_add(x, y), r);
    }
}
