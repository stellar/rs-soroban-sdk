use crate as soroban_sdk;
use soroban_sdk::{contractimpl, testutils::CostType, Env};

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

#[test]
fn test_budget() {
    let e = Env::default();
    let contract_id = e.register_contract(None, Contract);
    let client = ContractClient::new(&e, &contract_id);

    let a = 10i32;
    let b = 12i32;
    e.reset_budget();
    let c = client.add(&a, &b);
    assert_eq!(c, 22);

    let budget = e.budget();
    eprintln!("Cpu Insns: {}", budget.get_cpu_insns_count());
    eprintln!("Mem Bytes: {}", budget.get_mem_bytes_count());
    for cost_type in CostType::variants() {
        eprintln!("Cost ({cost_type:?}): {}", budget.get_input(*cost_type));
    }
}
