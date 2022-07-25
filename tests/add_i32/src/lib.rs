#![no_std]
use stellar_contract_sdk::contractimpl;

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod test {
    extern crate std;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    use super::__add::call_raw as add;
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add() {
        let e = Env::default();
        let x = 10i32.into_val(&e);
        let y = 12i32.into_val(&e);
        let z = add(e.clone(), x, y);
        let z = i32::try_from_val(&e, z).unwrap();
        assert!(z == 22);
    }

    #[test]
    fn test_add_overflow() {
        let e = Env::default();
        let x = (-241823608i32).into_val(&e);
        let y = (-1905660041i32).into_val(&e);
        let res = catch_unwind(AssertUnwindSafe(|| {
            add(e, x, y);
        }));
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod proptest {
    extern crate std;
    use core::panic::AssertUnwindSafe;
    use proptest::prelude::*;
    use std::{format, panic};

    use super::__add::call_raw as add;
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    proptest! {
        #[test]
        fn test_add(a in any::<i32>(), b in any::<i32>()) {
            let e = Env::default();
            match a.checked_add(b) {
                // If a + b would result in overflow, assert that the add fn
                // will panic.
                None => {
                    let res = panic::catch_unwind(AssertUnwindSafe(move || {
                        add(e.clone(), a.into_val(&e), b.into_val(&e));
                    }));
                    prop_assert!(res.is_err());
                },
                // If a + b would not result in overflow, assert that the add fn
                // returns the sum of a and b.
                Some(expected_sum) => {
                    let vsum = add(e.clone(), a.into_val(&e), b.into_val(&e));
                    let sum = i32::try_from_val(&e, vsum).unwrap();
                    prop_assert_eq!(sum, expected_sum);
                },
            }
        }
    }
}
