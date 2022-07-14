#![no_std]
use stellar_contract_sdk::{contract, contractimpl, contracttype, ConversionError, IntoEnvVal};

contract!();

#[contracttype]
pub enum UdtEnum {
    UdtA,
    UdtB(UdtStruct),
}

#[contracttype]
pub struct UdtStruct {
    pub a: i64,
    pub b: i64,
}

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: UdtEnum, b: UdtEnum) -> i64 {
        let a = match a {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
        };
        let b = match b {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
        };
        a + b
    }
}

#[cfg(test)]
mod test {
    use super::{UdtEnum, UdtStruct, __add};
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add() {
        let e = Env::default();
        let udt = UdtStruct { a: 10, b: 12 };
        let z = __add(
            e.clone(),
            UdtEnum::UdtA.into_val(&e),
            UdtEnum::UdtB(udt).into_val(&e),
        );
        let z = i64::try_from_val(&e, z).unwrap();
        assert_eq!(z, 22);
    }
}
