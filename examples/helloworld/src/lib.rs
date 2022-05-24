#![no_std]
use stellar_contract_sdk::{Env, IntoVal, RawVal};

pub struct Abc;

impl Abc {
    pub fn abc(e: Env) -> RawVal {
        return (b'w' as u32).into_val(&e);
    }
}

#[no_mangle]
pub fn hello(e: Env) -> RawVal {
    return (b'w' as u32).into_val(&e);
}

#[cfg(test)]
mod test {
    use super::hello;
    use stellar_contract_sdk::Env;

    #[test]
    fn test_hello() {
        let e = Env::default();
        let r = hello(e.clone());
        let r: u32 = r.try_into().unwrap();
        assert_eq!(r, (b'w' as u32));
    }
}
