#![no_std]
use soroban_sdk::{contractimpl, contracttype, Vec};

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

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: UdtEnum, b: UdtEnum) -> i64 {
        let a = match a {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        let b = match b {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b,
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        a + b
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{vec, xdr::ScVal, Bytes, BytesN, Env, TryFromVal};

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
            let mut bin = Bytes::new(&e);
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
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
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
        let roundtrip = UdtStruct::try_from_val(&e, val).unwrap();
        assert_eq!(udt, roundtrip);
    }
}
