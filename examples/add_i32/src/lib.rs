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
    extern crate std;
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

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let e = Env::default();
        let x = (-241823608i32).into_val(&e);
        let y = (-1905660041i32).into_val(&e);
        add(e, x, y);
    }
}

#[cfg(test)]
mod proptest {
    extern crate std;
    use proptest::prelude::*;
    use std::{format, panic};

    use super::add;
    use stellar_contract_sdk::{Env, EnvValConvertible, OrAbort};

    proptest! {
        #[test]
        fn test_add(a in any::<i32>(), b in any::<i32>()) {
            match a.checked_add(b) {
                // If a + b would result in overflow, assert that the add fn
                // will panic.
                None => {
                    let res = panic::catch_unwind(|| {
                        let e = Env::default();
                        let a = a.into_val(&e);
                        let b = b.into_val(&e);
                        add(e, a, b);
                    });
                    prop_assert!(res.is_err());
                },
                // If a + b would not result in overflow, assert that the add fn
                // returns the sum of a and b.
                Some(expected_sum) => {
                    let e = Env::default();
                    let vsum = add(e.clone(), a.into_val(&e), b.into_val(&e));
                    let sum = i32::try_from_val(&e, vsum).or_abort();
                    prop_assert_eq!(sum, expected_sum);
                },
            }
        }
    }
}
