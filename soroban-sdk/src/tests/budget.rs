use crate as soroban_sdk;
use soroban_env_host::budget::CostTracker;
use soroban_sdk::{contract, contractimpl, map, xdr::ContractCostType, Env, Map};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(e: Env) -> Map<i32, i32> {
        let mut map = Map::new(&e);
        map.set(1, 10);
        map.set(2, 20);
        map
    }
}

#[test]
fn test_budget() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    e.budget().reset_default();
    let b = client.add();
    e.budget().print();

    assert_eq!(
        e.budget().tracker(ContractCostType::VisitObject),
        CostTracker {
            iterations: 13,
            inputs: None,
            cpu: 793,
            mem: 0,
        }
    );
    assert_eq!(b, map![&e, (1, 10), (2, 20)]);
}
