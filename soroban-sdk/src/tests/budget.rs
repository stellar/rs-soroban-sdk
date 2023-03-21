use crate as soroban_sdk;
use soroban_sdk::{contractimpl, map, testutils::budget::CostType, Env, Map};

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

    assert_eq!(e.budget().input(CostType::MapNew), 1);
    assert_eq!(e.budget().input(CostType::MapEntry), 5);
    assert_eq!(b, map![&e, (1, 10), (2, 20)]);
}
