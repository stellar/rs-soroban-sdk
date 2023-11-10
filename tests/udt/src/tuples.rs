use soroban_sdk::{contract, contractimpl, contracttype, Address, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UdtEnum {
    UdtA((u32, Address)),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtTuple((u32, Address));

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UdtStruct {
    f1: (u32, Address),
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn id(v: (UdtEnum, UdtTuple, UdtStruct)) -> (UdtEnum, UdtTuple, UdtStruct) {
        (v.0, v.1, v.2)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{vec, xdr::ScVal, Bytes, Env, TryFromVal};

    #[test]
    fn test_tuples() {
        let e = Env::default();
        let contract_id = e.register_contract(None, Contract);
        let client = ContractClient::new(&e, &contract_id);

        let v1 = (
            UdtEnum::UdtA((1, Address::random(&e))),
            UdtTuple((2, Address::random(&e))),
            UdtStruct {
                f1: (3, Address::random(&e)),
            },
        );

        let v2 = client.id(&v1);

        assert_eq!(v1, v2);
    }
}
