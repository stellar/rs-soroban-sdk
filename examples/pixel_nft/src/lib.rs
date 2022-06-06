#![no_std]
use stellar_contract_sdk::{Env, IntoVal, RawVal, Symbol};

#[no_mangle]
pub fn pixel(e: Env) -> RawVal {
    0x957dad00u32.into_val(&e)
}

#[no_mangle]
pub fn owner(e: Env) -> RawVal {
    Symbol::from_str("GBKLMQVNCR").into_val(&e)
}

#[cfg(test)]
mod test {
    use stellar_contract_sdk::{Env, Symbol, TryFromVal};

    #[test]
    fn pixel() {
        let e = Env::default();
        let rgba = super::pixel(e.clone());
        let rgba = u32::try_from_val(&e, rgba).unwrap();
        assert_eq!(rgba, 0x957dad00);
    }

    #[test]
    fn owner() {
        let e = Env::default();
        let owner = super::owner(e.clone());
        let owner = Symbol::try_from_val(&e, owner).unwrap().to_str();
        let owner: &str = owner.as_ref();
        assert_eq!("GBKLMQVNCR", owner);
    }
}
