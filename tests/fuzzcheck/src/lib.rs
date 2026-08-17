#![no_std]
// The code that fuzzcheck's derive macros generate requires this feature and the
// std library, and so crates with `#[contracttype]` types built with the sdk's
// "fuzzcheck" feature must declare them.
#![cfg_attr(feature = "fuzzcheck", feature(coverage_attribute))]

#[cfg(feature = "fuzzcheck")]
#[macro_use]
extern crate std;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Map, String, Symbol, Vec, U256};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Deposit {
    pub depositor: Address,
    pub amount: u128,
    pub memo: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum Instruction {
    Deposit(Deposit),
    Withdraw(u128),
    Close,
}

/// A contract type that is a tuple struct.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Tagged(pub Symbol, pub u32);

/// A contract type that is an enum with integer values.
#[contracttype]
#[derive(Clone, Copy, Debug)]
pub enum Priority {
    Low = 1,
    High = 2,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Panics if the instructions deposit and withdraw the same amount, which a
    /// fuzzer is expected to be able to find.
    pub fn run(instructions: Vec<Instruction>) -> u32 {
        let mut deposited: Option<u128> = None;
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Deposit(deposit) => deposited = Some(deposit.amount),
                Instruction::Withdraw(amount) => {
                    if deposited == Some(amount) && amount != 0 {
                        panic!("unexpected")
                    }
                }
                Instruction::Close => break,
            }
        }
        instructions.len()
    }

    /// Accepts a value of every builtin type, so that the mutators of all of
    /// them are exercised.
    pub fn builtins(
        a: u32,
        b: i64,
        c: u128,
        d: i128,
        e: U256,
        f: Vec<Address>,
        g: Map<String, u32>,
        h: Option<u64>,
        i: (u32, u64),
        j: Tagged,
        k: Priority,
    ) -> u32 {
        let _ = (a, b, c, d, e, f, g, h, i, j, k);
        0
    }
}
