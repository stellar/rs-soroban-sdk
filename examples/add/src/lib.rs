#![no_std]
use sdk::{OrAbort, Val};
use stellar_contract_host as host;
use stellar_contract_sdk as sdk;

#[no_mangle]
pub fn add(h: impl host::Host, a: Val, b: Val) -> Val {
    let a: i64 = a.try_into().or_abort();
    let b: i64 = b.try_into().or_abort();

    let c = a + b;

    return c.try_into().or_abort();
}

#[cfg(test)]
mod test {
    use super::add;
    use sdk::Val;
    use stellar_contract_sdk as sdk;

    #[test]
    fn test_add() {
        let h: Host = host::HostContext {};
        let x: Val = Val::from_i64(10);
        let y: Val = Val::from_i64(12);
        let z: Val = add(h, x, y);
        let z: i64 = z.try_into().unwrap();
        assert!(z == 22);
    }
}
