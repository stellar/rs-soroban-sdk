use soroban_sdk::contractimpl;

pub struct MyType<A>(pub A);

pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn add(a: MyType<i32>, b: MyType<i32>) -> i8 {
        unimplemented!()
    }
}

pub fn main() {}
