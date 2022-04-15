#![no_std]
use sdk::{OrAbort, Val};
use stellar_contract_sdk as sdk;

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
    use sdk::testing::mem::{MemHost, MemObj};
    use sdk::testing::swap_mock_host;
    use sdk::Val;
    use stellar_contract_sdk as sdk;
    extern crate alloc;
    extern crate std;
    use std::boxed::Box;

    #[test]
    fn test_add_implicitly_using_objs() {
        let x: Val = Val::from_u63(10);
        let y: Val = Val::from(-7 as i64);
        let z: Val = add(x, y);
        let z: i64 = z.try_into().unwrap();
        assert!(z == 3);
    }

    #[test]
    fn test_add_explicitly_using_objs() {
        let mut host = Box::new(MemHost::new());
        let seven = host.put_obj(MemObj::I64(-7));
        swap_mock_host(host);

        let x: Val = Val::from_u63(10);
        let y: Val = seven.into();
        let z: Val = add(x, y);
        let z: i64 = z.try_into().unwrap();
        assert!(z == 3);
    }
}
