#![no_std]
use soroban_sdk::{contract, contractimpl, MuxedAddress, String};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn parse_address(strkey: String) -> MuxedAddress {
        MuxedAddress::from_string(&strkey)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    use crate::{Contract, ContractClient};

    #[test]
    fn test_parse_g_address() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let strkey = String::from_str(
            &env,
            "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5",
        );
        let result = client.parse_address(&strkey);
        assert_eq!(result.id(), None);
    }

    #[test]
    fn test_parse_m_address() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let strkey = String::from_str(
            &env,
            "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU",
        );
        let result = client.parse_address(&strkey);
        assert!(result.id().is_some());
    }

    #[test]
    fn test_parse_c_address() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let strkey = String::from_str(
            &env,
            "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE",
        );
        let result = client.parse_address(&strkey);
        assert_eq!(result.id(), None);
    }
}
