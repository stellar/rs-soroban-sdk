#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Vec};

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
    pub d: Option<i64>,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: UdtEnum, b: UdtEnum) -> i64 {
        let a = match a {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b + udt.d.unwrap_or(0),
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        let b = match b {
            UdtEnum::UdtA => 0,
            UdtEnum::UdtB(udt) => udt.a + udt.b + udt.d.unwrap_or(0),
            UdtEnum::UdtC(val) => val as i64,
            UdtEnum::UdtD(tup) => tup.0 + tup.1.try_iter().fold(0i64, |sum, i| sum + i.unwrap()),
        };
        a + b
    }

    pub fn add_struct(_env: Env, a: UdtStruct, b: UdtStruct) -> UdtStruct {
        let mut c = a.c.clone();
        c.append(&b.c);
        UdtStruct {
            a: a.a + b.a,
            b: a.b + b.b,
            c,
            // d is Some only if both inputs have Some
            d: match (a.d, b.d) {
                (Some(x), Some(y)) => Some(x + y),
                _ => None,
            },
        }
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
            d: Some(99),
        };
        let bin = udt.to_xdr(&e);
        // Note: The serialized format includes all present fields (a, b, c, d)
        // When d is None, it would be omitted from the map
        let expected_bytes = [
            0u8, 0, 0, 17, 0, 0, 0, 1, 0, 0, 0, 4, // Map with 4 entries
            0, 0, 0, 15, 0, 0, 0, 1, 97, 0, 0, 0, // key "a"
            0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 10, // val 10
            0, 0, 0, 15, 0, 0, 0, 1, 98, 0, 0, 0, // key "b"
            0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 12, // val 12
            0, 0, 0, 15, 0, 0, 0, 1, 99, 0, 0, 0, // key "c"
            0, 0, 0, 16, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0,
            1, // val vec![1]
            0, 0, 0, 15, 0, 0, 0, 1, 100, 0, 0, 0, // key "d"
            0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 99, // val 99
        ];
        let expected_bytes = Bytes::from_array(&e, &expected_bytes);
        assert_eq!(bin, expected_bytes);
    }

    #[test]
    fn test_add() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        // Test with d = None (should not affect sum)
        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
            d: None,
        };
        let z = client.add(&UdtEnum::UdtA, &UdtEnum::UdtB(udt));
        assert_eq!(z, 22);

        // Test with d = Some(5) (should add 5 to the sum)
        let udt_with_d = UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
            d: Some(5),
        };
        let z = client.add(&UdtEnum::UdtA, &UdtEnum::UdtB(udt_with_d));
        assert_eq!(z, 27); // 10 + 12 + 5 = 27

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
            d: Some(42),
        };
        let val: ScVal = udt.clone().try_into().unwrap();
        let roundtrip = UdtStruct::try_from_val(&e, &val).unwrap();
        assert_eq!(udt, roundtrip);
    }

    #[test]
    fn test_add_struct() {
        let e = Env::default();
        let contract_id = e.register(Contract, ());
        let client = ContractClient::new(&e, &contract_id);

        // Both have Some - result should have Some
        let a = UdtStruct {
            a: 1,
            b: 2,
            c: vec![&e, 10],
            d: Some(100),
        };
        let b = UdtStruct {
            a: 3,
            b: 4,
            c: vec![&e, 20],
            d: Some(200),
        };
        let result = client.add_struct(&a, &b);
        assert_eq!(result.a, 4);
        assert_eq!(result.b, 6);
        assert_eq!(result.c, vec![&e, 10, 20]);
        assert_eq!(result.d, Some(300));

        // One has None - result should have None
        let a = UdtStruct {
            a: 1,
            b: 2,
            c: vec![&e, 10],
            d: Some(100),
        };
        let b = UdtStruct {
            a: 3,
            b: 4,
            c: vec![&e, 20],
            d: None,
        };
        let result = client.add_struct(&a, &b);
        assert_eq!(result.a, 4);
        assert_eq!(result.b, 6);
        assert_eq!(result.c, vec![&e, 10, 20]);
        assert_eq!(result.d, None);

        // Both have None - result should have None
        let a = UdtStruct {
            a: 1,
            b: 2,
            c: vec![&e, 10],
            d: None,
        };
        let b = UdtStruct {
            a: 3,
            b: 4,
            c: vec![&e, 20],
            d: None,
        };
        let result = client.add_struct(&a, &b);
        assert_eq!(result.a, 4);
        assert_eq!(result.b, 6);
        assert_eq!(result.c, vec![&e, 10, 20]);
        assert_eq!(result.d, None);
    }
}
