#![no_std]
use stellar_contract_sdk::{contract, contractimpl, contracttype, IntoEnvVal, Vec};

contract!();

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

// #[cfg(any(test, feature = "testutils"))]
// impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScMap>> for UdtStruct {
//     type Error = stellar_contract_sdk::xdr::Error;
//     #[inline(always)]
//     fn try_from(
//         ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScMap>,
//     ) -> Result<Self, Self::Error> {
//         use stellar_contract_sdk::xdr::Validate;
//         use stellar_contract_sdk::EnvType;
//         use stellar_contract_sdk::TryIntoEnvVal;
//         let map = ev.val;
//         map.validate()?;
//         Ok(Self {
//             a: {
//                 let key = &"a"
//                     .try_into()
//                     .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
//                 let idx = map
//                     .binary_search_by_key(key, |entry| entry.key.clone())
//                     .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
//                 let ev: stellar_contract_sdk::EnvVal = (&map[idx].val.clone())
//                     .try_into_env_val(&ev.env)
//                     .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
//                 ev.try_into()
//                     .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?
//             },
//             b: {
//                 let key = &"b"
//                     .try_into()
//                     .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
//                 let idx = map
//                     .binary_search_by_key(key, |entry| entry.key.clone())
//                     .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
//                 let ev: EnvType<i64> = map[idx]
//                     .val
//                     .clone()
//                     .try_into_env_val(&ev.env)
//                     .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?;
//                 ev.val
//             },
//         })
//     }
// }
// #[cfg(any(test, feature = "testutils"))]
// impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScObject>> for UdtStruct {
//     type Error = stellar_contract_sdk::xdr::Error;
//     #[inline(always)]
//     fn try_from(
//         ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScObject>,
//     ) -> Result<Self, Self::Error> {
//         if let stellar_contract_sdk::xdr::ScObject::Map(map) = ev.val {
//             stellar_contract_sdk::EnvType {
//                 env: ev.env,
//                 val: map,
//             }
//             .try_into()
//         } else {
//             Err(stellar_contract_sdk::xdr::Error::Invalid)
//         }
//     }
// }
// #[cfg(any(test, feature = "testutils"))]
// impl TryFrom<stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScVal>> for UdtStruct {
//     type Error = stellar_contract_sdk::xdr::Error;
//     #[inline(always)]
//     fn try_from(
//         ev: stellar_contract_sdk::EnvType<stellar_contract_sdk::xdr::ScVal>,
//     ) -> Result<Self, Self::Error> {
//         if let stellar_contract_sdk::xdr::ScVal::Object(Some(obj)) = ev.val {
//             stellar_contract_sdk::EnvType {
//                 env: ev.env,
//                 val: obj,
//             }
//             .try_into()
//         } else {
//             Err(stellar_contract_sdk::xdr::Error::Invalid)
//         }
//     }
// }
// #[cfg(any(test, feature = "testutils"))]
// impl TryInto<stellar_contract_sdk::xdr::ScMap> for UdtStruct {
//     type Error = stellar_contract_sdk::xdr::Error;
//     #[inline(always)]
//     fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScMap, Self::Error> {
//         extern crate alloc;
//         Ok(stellar_contract_sdk::xdr::ScMap(
//             <[_]>::into_vec(
//                 #[rustc_box]
//                 ::alloc::boxed::Box::new([
//                     stellar_contract_sdk::xdr::ScMapEntry {
//                         key: "a"
//                             .try_into()
//                             .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?,
//                         val: self
//                             .a
//                             .try_into()
//                             .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?,
//                     },
//                     stellar_contract_sdk::xdr::ScMapEntry {
//                         key: "b"
//                             .try_into()
//                             .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?,
//                         val: self
//                             .b
//                             .try_into()
//                             .map_err(|_| stellar_contract_sdk::xdr::Error::Invalid)?,
//                     },
//                 ]),
//             )
//             .try_into()?,
//         ))
//     }
// }
// #[cfg(any(test, feature = "testutils"))]
// impl TryInto<stellar_contract_sdk::xdr::ScObject> for UdtStruct {
//     type Error = stellar_contract_sdk::xdr::Error;
//     #[inline(always)]
//     fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScObject, Self::Error> {
//         Ok(stellar_contract_sdk::xdr::ScObject::Map(self.try_into()?))
//     }
// }
// #[cfg(any(test, feature = "testutils"))]
// impl TryInto<stellar_contract_sdk::xdr::ScVal> for UdtStruct {
//     type Error = stellar_contract_sdk::xdr::Error;
//     #[inline(always)]
//     fn try_into(self) -> Result<stellar_contract_sdk::xdr::ScVal, Self::Error> {
//         Ok(stellar_contract_sdk::xdr::ScVal::Object(Some(
//             self.try_into()?,
//         )))
//     }
// }

pub struct Contract;

#[contractimpl(export_if = "export", tests_if = "testutils")]
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
    use stellar_contract_sdk::{vec, xdr::ScVal, Env, IntoVal, TryFromVal, TryIntoVal, EnvVal};

    #[test]
    fn test_add() {
        let e = Env::default();
        let udt = UdtStruct {
            a: 10,
            b: 12,
            c: vec![&e, 1],
        };
        let z = __add::call_raw(
            e.clone(),
            UdtEnum::UdtA.into_val(&e),
            UdtEnum::UdtB(udt).into_val(&e),
        );
        let z = i64::try_from_val(&e, z).unwrap();
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
        let roundtrip: EnvVal = (&val).try_into().unwrap();
        let roundtrip: UdtStruct = (&val).try_into_val(&e).unwrap();
        assert_eq!(udt, roundtrip);
    }
}
