#![no_std]
use soroban_sdk::{contractimpl, contracttype, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UdtEnum {
    UdtA,
    UdtB(UdtStruct),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtStruct {
    pub a: i64,
    pub b: i64,
    pub c: Vec<i64>,
}

pub struct Contract;

#[contractimpl(export_if = "export")]
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
    use super::*;
    use soroban_sdk::{vec, xdr::ScVal, Binary, Env, TryFromVal};

    #[test]
    fn test_serializing() {
        use soroban_sdk::serde::Serialize;
        let e = Env::default();
        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
        };
        let bin = udt.serialize(&e);
        assert_eq!(bin, {
            let mut bin = Binary::new(&e);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(4);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(1);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(1);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(3);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(5);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(1);
            bin.push(97);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(10);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(5);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(1);
            bin.push(98);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(12);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(5);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(1);
            bin.push(99);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(4);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(1);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(1);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(0);
            bin.push(1);
            bin
        })
    }

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id =Binary::from_array(&e, [0; 32]);
        e.register_contract(&contract_id, Contract);

        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
        };
        let z = add::invoke(
            &e,
            &contract_id,
            &UdtEnum::UdtA,
            &UdtEnum::UdtB(udt),
        );
        assert_eq!(z, 22);
    }

    #[test]
    fn test_scval_accessibility_from_udt_types() {
        let e = Env::default();
        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
        };
        let val: ScVal = udt.clone().try_into().unwrap();
        let roundtrip = UdtStruct::try_from_val(&e, val).unwrap();
        assert_eq!(udt, roundtrip);
    }
}
