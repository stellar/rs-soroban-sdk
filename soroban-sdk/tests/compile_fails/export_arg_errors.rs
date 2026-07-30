use soroban_sdk::{contracterror, contractevent, contracttype, Symbol};

#[contracttype(export = false)]
pub struct S {
    pub val: u32,
}

#[contracttype(export = true)]
pub struct S2 {
    pub val: u32,
}

#[contracterror(export = false)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum E {
    Fail = 1,
}

#[contractevent(export = false)]
pub struct Ev {
    #[topic]
    pub kind: Symbol,
}

fn main() {}
