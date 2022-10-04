#![no_std]
use soroban_sdk::{contractimpl, Env, Invoker};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn whoami(e: Env) -> Invoker {
        e.invoker()
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use soroban_sdk::{testutils::Accounts, BytesN, Env, Invoker};

    use crate::{Contract, ContractClient};

    #[test]
    fn test() {
        let e = Env::default();
        let contract_id = BytesN::from_array(&e, &[0; 32]);
        e.register_contract(&contract_id, Contract);
        let client = ContractClient::new(&e, &contract_id);

        std::println!("default: {:?}", e.source_account());

        let invoker_1 = e.accounts().generate();
        std::println!("invoker 1: {:?}", invoker_1);

        let invoker_2 = e.accounts().generate();
        std::println!("invoker 2: {:?}", invoker_2);

        assert_ne!(&invoker_1, &invoker_2);
        assert_ne!(&invoker_1, &e.source_account());
        assert_ne!(&invoker_2, &e.source_account());

        let invoke_1a = client.r#as(&invoker_1).whoami();
        std::println!("invoke 1a: {:?}", invoke_1a);

        let invoke_1b = client.r#as(&invoker_1).whoami();
        std::println!("invoke 1b: {:?}", invoke_1b);

        assert_eq!(invoke_1a, invoke_1b);
        assert_eq!(invoke_1a, Invoker::Account(invoker_1.clone()));
        assert_eq!(invoke_1b, Invoker::Account(invoker_1.clone()));

        let invoke_2 = client.r#as(&invoker_2).whoami();
        std::println!("invoke 2: {:?}", invoke_2);
        assert_ne!(invoke_1a, invoke_2);
        assert_ne!(invoke_1b, invoke_2);
    }
}
