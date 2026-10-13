use core::marker::PhantomData;
use soroban_sdk::contractimpl;

pub struct Generic<T> {
    _t: PhantomData<T>,
}

#[contractimpl]
impl Generic<u32> {
    pub fn a() {}
}

#[contractimpl]
impl<T> Generic<T> {
    pub fn b() {}
}

#[contractimpl]
impl self::Generic<u64> {
    pub fn c() {}
}

fn main() {}
