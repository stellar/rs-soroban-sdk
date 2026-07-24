// Setting `lib` on `contracttype`, `contracterror`, or `contractevent` is
// deprecated. With the `deprecated` lint denied, using it must fail to compile,
// proving the deprecation warning is emitted.
#![deny(deprecated)]

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
