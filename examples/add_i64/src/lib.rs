#![no_std]
use stellar_contract_sdk::{Env, IntoVal, RawVal, TryFromVal};

#[cfg(target_family = "wasm")]
#[link_section = "jcv1"]
pub static SCV1: [u8; 10] = *b"abcdefghij";

#[no_mangle]
pub fn add(e: Env, a: RawVal, b: RawVal) -> RawVal {
    let a: i64 = i64::try_from_val(&e, a).unwrap();
    let b: i64 = i64::try_from_val(&e, b).unwrap();

    let c = a + b;

    return c.into_val(&e);
}

#[cfg(test)]
mod test {
    use super::add;
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add() {
        let e = Env::default();
        let x = 10i64.into_val(&e);
        let y = 12i64.into_val(&e);
        let z = add(e.clone(), x, y);
        let z = i64::try_from_val(&e, z).unwrap();
        assert!(z == 22);
    }
}
