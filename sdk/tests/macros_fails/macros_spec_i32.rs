use stellar_contract_sdk::contractfn;

pub struct MyType<A>(pub A);

#[contractfn]
pub fn add(a: MyType<i32>, b: MyType<i32>) -> i8 {
    unimplemented!()
}

pub fn main() {}
