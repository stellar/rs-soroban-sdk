#![no_std]
use stellar_contract_sdk::{Env, IntoVal, RawVal};

#[no_mangle]
pub fn pixel(e: Env) -> RawVal {
    0x957dad00u32.into_val(&e)
}

#[cfg(test)]
mod test {
    use stellar_contract_sdk::{Env, TryFromVal};

    #[test]
    fn pixel() {
        let e = Env::default();
        let rgba = super::pixel(e.clone());
        let rgba = u32::try_from_val(&e, rgba).unwrap();
        assert_eq!(rgba, 0x957dad00);
    }
}
