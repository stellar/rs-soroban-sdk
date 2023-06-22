use crate as soroban_sdk;
use soroban_sdk::{contractimpl, map, xdr::ContractCostType, Env, Map};

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

    // Here the cost of 5 for `MapEntry` is broken down into
    // 2 - charge for adding the two elements
    // 1 - charge for binary search of map with len == 0
    // 2 - charge for binary search of map with len == 1
    assert_eq!(e.budget().tracker(ContractCostType::MapEntry), (8, None));
    assert_eq!(b, map![&e, (1, 10), (2, 20)]);
}
