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
    use core::ops::DerefMut;

    use super::add;
    use stellar_contract_sdk::{HostConvertable, Val, HOST};

    #[test]
    fn test_add() {
        let x: Val = HOST.with(|h| 10i64.val_from(h.borrow_mut().deref_mut().as_mut()));
        let y: Val = HOST.with(|h| 12i64.val_from(h.borrow_mut().deref_mut().as_mut()));
        let z: Val = add(x, y);
        let z: i64 =
            HOST.with(|h| i64::try_val_into(z, h.borrow_mut().deref_mut().as_mut()).unwrap());
        assert!(z == 22);
    }
}
