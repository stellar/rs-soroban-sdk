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
    use stellar_contract_host::Host;
    use stellar_contract_sdk::{HostConvertable, Val, HOST};

    #[test]
    fn test_add() {
        let x: Val = HOST.with(|h| 10i64.val_from(&mut h.borrow_mut()));
        let y: Val = 12i64.val_from(&mut HOST);
        let z: Val = add(x, y);
        let z: i64 = z.try_into().unwrap();
        assert!(z == 22);
    }
}
