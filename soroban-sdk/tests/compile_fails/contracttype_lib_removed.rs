// The `lib` argument on `contracttype`, `contracterror`, and `contractevent`
// has been removed and must be rejected as an unknown argument.
use soroban_sdk::{contracterror, contractevent, contracttype};

#[contracttype(lib = "libname")]
pub struct S {
    pub a: u32,
}

#[contracterror(lib = "libname")]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum E {
    A = 1,
}

#[contractevent(lib = "libname")]
pub struct Ev {
    pub a: u32,
}

fn main() {}
