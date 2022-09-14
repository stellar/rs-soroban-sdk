#![no_std]
use soroban_sdk::{contractimpl, symbol, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env) {
        env.events().publish((), symbol!("hello"));
    }
}

#[cfg(test)]
mod test {
    extern crate alloc;
    use soroban_sdk::{
        symbol,
        testutils::EnvEvent,
        xdr::{
            ContractEvent, ContractEventBody, ContractEventType, ContractEventV0, ExtensionPoint,
        },
        BytesN, Env,
    };

    use crate::{Contract, ContractClient};

    #[test]
    fn test_pub_event() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        client.hello();

        assert_eq!(
            e.events().get(),
            alloc::vec![EnvEvent::Contract(ContractEvent {
                ext: ExtensionPoint::V0,
                contract_id: None,
                type_: ContractEventType::Contract,
                body: ContractEventBody::V0(ContractEventV0 {
                    topics: ().try_into().unwrap(),
                    data: (symbol!("hello")).try_into().unwrap(),
                })
            })]
        );
    }
}
