#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Error, Map, Symbol, Vec};

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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtRecursive {
    pub a: Symbol,
    pub b: Vec<UdtRecursive>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecursiveToEnum {
    pub a: Symbol,
    pub b: Map<u32, RecursiveEnum>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecursiveEnum {
    NotRecursive,
    Recursive(RecursiveToEnum),
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

    pub fn recursive(a: UdtRecursive) -> Option<UdtRecursive> {
        if a.b.is_empty() {
            None
        } else {
            Some(a.b.first_unchecked())
        }
    }

    pub fn recursive_enum(a: RecursiveEnum, key: u32) -> Result<Option<RecursiveEnum>, Error> {
        match a {
            RecursiveEnum::NotRecursive => Ok(None),
            RecursiveEnum::Recursive(router) => Ok(router.b.get(key)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{symbol_short, vec, xdr::ScVal, Bytes, Env, TryFromVal};

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
        let contract_id = e.register(Contract, ());
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

    #[test]
    fn test_recursive() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let recursive_udt_0 = UdtRecursive {
            a: symbol_short!("0"),
            b: vec![&e],
        };
        let recursive_udt_1 = UdtRecursive {
            a: symbol_short!("1"),
            b: vec![&e, recursive_udt_0.clone()],
        };
        let recursive_udt_2 = UdtRecursive {
            a: symbol_short!("2"),
            b: vec![&e, recursive_udt_1.clone()],
        };

        let result_0 = client.recursive(&recursive_udt_2);
        assert_eq!(result_0, Some(recursive_udt_1));

        let result_1 = client.recursive(&result_0.unwrap());
        assert_eq!(result_1, Some(recursive_udt_0));

        let result_2 = client.recursive(&result_1.unwrap());
        assert_eq!(result_2, None);
    }

    #[test]
    fn test_recursive_enum() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        let entry = RecursiveEnum::Recursive(RecursiveToEnum {
            a: symbol_short!("test"),
            b: Map::from_array(&e, [(42u32, RecursiveEnum::NotRecursive)]),
        });
        let result = client.recursive_enum(&entry, &42);
        assert_eq!(result, Some(RecursiveEnum::NotRecursive));
        let none_result = client.recursive_enum(&entry, &43);
        assert_eq!(none_result, None);
    }
}

#[cfg(test)]
mod test_with_wasm {
    use soroban_sdk::{symbol_short, vec, Env, Map};

    mod contract {
        soroban_sdk::contractimport!(file = "../../target/wasm32v1-none/release/test_udt.wasm");
    }

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register(contract::WASM, ());
        let client = contract::Client::new(&e, &contract_id);

        let udt = contract::UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
        };
        let z = client.add(&contract::UdtEnum::UdtA, &contract::UdtEnum::UdtB(udt));
        assert_eq!(z, 22);

        let udt1 = contract::UdtEnum2::A;
        let udt2 = contract::UdtTuple(1, vec![&e, 2, 3]);
        let z = client.add(
            &contract::UdtEnum::UdtC(udt1),
            &contract::UdtEnum::UdtD(udt2),
        );
        assert_eq!(z, 16);
    }

    #[test]
    fn test_recursive() {
        let e = Env::default();
        let contract_id = e.register(contract::WASM, ());
        let client = contract::Client::new(&e, &contract_id);

        let recursive_udt_0 = contract::UdtRecursive {
            a: symbol_short!("0"),
            b: vec![&e],
        };
        let recursive_udt_1 = contract::UdtRecursive {
            a: symbol_short!("1"),
            b: vec![&e, recursive_udt_0.clone()],
        };
        let recursive_udt_2 = contract::UdtRecursive {
            a: symbol_short!("2"),
            b: vec![&e, recursive_udt_1.clone()],
        };

        let result_0 = client.recursive(&recursive_udt_2);
        assert_eq!(result_0, Some(recursive_udt_1));

        let result_1 = client.recursive(&result_0.unwrap());
        assert_eq!(result_1, Some(recursive_udt_0));

        let result_2 = client.recursive(&result_1.unwrap());
        assert_eq!(result_2, None);
    }

    #[test]
    fn test_recursive_enum() {
        let e = Env::default();
        let contract_id = e.register(contract::WASM, ());
        let client = contract::Client::new(&e, &contract_id);

        let entry = contract::RecursiveEnum::Recursive(contract::RecursiveToEnum {
            a: symbol_short!("test"),
            b: Map::from_array(&e, [(42u32, contract::RecursiveEnum::NotRecursive)]),
        });
        let result = client.recursive_enum(&entry, &42);
        assert_eq!(result, Some(contract::RecursiveEnum::NotRecursive));
        let none_result = client.recursive_enum(&entry, &43);
        assert_eq!(none_result, None);
    }
}
