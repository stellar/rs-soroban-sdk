use soroban_sdk::{contracterror, contracttype};

// The `lib` argument is no longer supported on `contracttype` or
// `contracterror` and should be rejected as an unknown field.

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

fn main() {}
