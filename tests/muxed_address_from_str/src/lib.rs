#![no_std]
use soroban_sdk::{contract, contractimpl, Address, MuxedAddress, String};

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

        let strkey_str = "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5";
        let strkey = String::from_str(&env, strkey_str);
        let result = client.parse_address(&strkey);
        let expected_address = Address::from_str(&env, strkey_str);
        assert_eq!(result.address(), expected_address);
        assert_eq!(result.id(), None);
    }

    #[test]
    fn test_parse_m_address() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let muxed_strkey = String::from_str(
            &env,
            "MA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAAAAAAAAAPCICBKU",
        );
        let base_strkey = "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5";
        let result = client.parse_address(&muxed_strkey);
        let expected_address = Address::from_str(&env, base_strkey);
        assert_eq!(result.address(), expected_address);
        assert_eq!(result.id(), Some(123456));
    }

    #[test]
    fn test_parse_c_address() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        let strkey_str = "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE";
        let strkey = String::from_str(&env, strkey_str);
        let result = client.parse_address(&strkey);
        let expected_address = Address::from_str(&env, strkey_str);
        assert_eq!(result.address(), expected_address);
        assert_eq!(result.id(), None);
    }
}
