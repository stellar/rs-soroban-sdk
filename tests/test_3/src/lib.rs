#![no_std]
use soroban_sdk::{contractimpl, symbol, Bytes, Env};

pub struct Contract;

#[derive(Default)]
#[repr(packed)]
pub struct Mem {
    counter: u32,
    balance: i128,
}
impl Mem {
    fn to_bytes(&self, env: &Env) -> Bytes {
        Bytes::from_slice(env, unsafe {
            core::slice::from_raw_parts(
                (self as *const Mem) as *const u8,
                ::core::mem::size_of::<Mem>(),
            )
        })
    }
    fn from_bytes(b: &Bytes) -> Mem {
        let mut buf = [0u8; core::mem::size_of::<Mem>()];
        b.copy_into_slice(&mut buf);
        unsafe { core::mem::transmute(buf) }
    }
}

#[contractimpl]
impl Contract {
    fn get(env: &Env) -> Mem {
        match env.storage().get(symbol!("MEM")) {
            Some(Ok(bytes)) => Mem::from_bytes(&bytes),
            _ => Mem::default(),
        }
    }

    pub fn set(env: Env, b: i128) {
        let mem = Self::get(&env);
        env.storage().set(
            symbol!("MEM"),
            Mem {
                counter: mem.counter + 1,
                balance: b,
            }
            .to_bytes(&env),
        )
    }

    pub fn counter(env: Env) -> u32 {
        let mem = Self::get(&env);
        mem.counter
    }

    pub fn balance(env: Env) -> i128 {
        let mem = Self::get(&env);
        mem.balance
    }
}
