#![no_std]
use soroban_sdk::{contractimpl, symbol, Bytes, Env};

pub struct Contract;

#[derive(Default)]
pub struct Mem {
    counter: u32,
    balance: i128,
}
impl Mem {
    fn to_bytes(&self, env: &Env) -> Bytes {
        let mut b = Bytes::new(env);
        b.extend_from_array(&self.counter.to_be_bytes());
        b.extend_from_array(&self.balance.to_be_bytes());
        b
    }
    fn from_bytes(b: &Bytes) -> Mem {
        let mut buf = [0u8; 20];
        b.copy_into_slice(&mut buf);
        let buf_counter: [u8; 4] = buf[..4].try_into().unwrap();
        let buf_balance: [u8; 16] = buf[4..].try_into().unwrap();
        Mem {
            counter: u32::from_be_bytes(buf_counter),
            balance: i128::from_be_bytes(buf_balance),
        }
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
