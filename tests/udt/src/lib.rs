#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Vec};

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UdtEnum2 {
    A = 10,
    B = 15,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UdtEnum {
    UdtA,
    UdtB(UdtStruct),
    UdtC(UdtEnum2),
    UdtD(UdtTuple),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtTuple(pub i64, pub Vec<i64>);

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtStruct {
    a: i64,
    b: i64,
    pub c: Vec<i64>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: UdtEnum, b: UdtEnum) -> i64 {
        let a = match a {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        let b = match b {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        a + b
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{vec, xdr::ScVal, Bytes, Env, TryFromVal};

    #[test]
    fn test_serializing() {
        use soroban_sdk::xdr::ToXdr;
        let e = Env::default();
        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
        };
        let bin = udt.to_xdr(&e);
        let expected_bytes = [
            0u8, 0, 0, 17, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 15, 0, 0, 0, 1, 97, 0, 0, 0, 0, 0, 0,
            6, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 15, 0, 0, 0, 1, 98, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0,
            0, 0, 0, 0, 12, 0, 0, 0, 15, 0, 0, 0, 1, 99, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 1, 0, 0, 0,
            1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 1,
        ];
        let expected_bytes = Bytes::from_array(&e, &expected_bytes);
        assert_eq!(bin, expected_bytes);
    }

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
        };
        let z = client.add(&UdtEnum::UdtA, &UdtEnum::UdtB(udt));
        assert_eq!(z, 22);

        let udt1 = UdtEnum2::A;
        let udt2 = UdtTuple(1, vec![&e, 2, 3]);
        let z = client.add(&UdtEnum::UdtC(udt1), &UdtEnum::UdtD(udt2));
        assert_eq!(z, 16);
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
        let roundtrip = UdtStruct::try_from_val(&e, &val).unwrap();
        assert_eq!(udt, roundtrip);
    }
}
