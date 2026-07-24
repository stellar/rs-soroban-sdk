// Setting `lib` on `contracttype` or `contracterror` is deprecated. With the
// `deprecated` lint denied, using it must fail to compile, proving the
// deprecation warning is emitted.
#![deny(deprecated)]

use soroban_sdk::{contracterror, contracttype};

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
