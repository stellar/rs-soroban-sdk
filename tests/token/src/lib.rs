#![no_std]
use soroban_auth::{Identifier, Signature};
use soroban_sdk::{
    contracterror, contractimpl, contracttype, symbol, BigInt, Env, IntoVal, Status,
};

pub struct Contract;

#[contracterror]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    AlreadyInitialized = 1,
    AmountNegative = 2,
    InsufficientBalance = 3,
}

#[contracttype]
pub struct Config {
    pub admin: Identifier,
}

#[contracttype]
pub enum DataKey {
    Config,
    Balance(Identifier),
}

#[contractimpl]
impl Contract {
    pub fn initialize(env: Env, config: Config) -> Result<(), Error> {
        if env.contract_data().has(DataKey::Config) {
            Err(Error::AlreadyInitialized)
        } else {
            env.contract_data().set(DataKey::Config, config);
            Ok(())
        }
    }

    pub fn balance(env: Env, id: Identifier) -> BigInt {
        let data = env.contract_data();

        data.get(DataKey::Balance(id.clone()))
            .unwrap_or_else(|| Ok(BigInt::zero(&env)))
            .unwrap()
    }

    pub fn send(
        env: Env,
        seq: BigInt,
        from: Signature,
        to: Identifier,
        amount: BigInt,
    ) -> Result<(), Error> {
        soroban_auth::verify(&env, &from, symbol!("send"), (&seq, &from, &to, &amount));

        let from = from.get_identifier(&env);

        if amount < 0 {
            return Err(Error::AmountNegative);
        }

        let data = env.contract_data();

        let from_balance_key = DataKey::Balance(from);
        let mut from_balance: BigInt = data
            .get(&from_balance_key)
            .unwrap_or_else(|| Ok(BigInt::zero(&env)))
            .unwrap();

        let to_balance_key = DataKey::Balance(to);
        let mut to_balance: BigInt = data
            .get(&to_balance_key)
            .unwrap_or_else(|| Ok(BigInt::zero(&env)))
            .unwrap();

        from_balance -= &amount;
        to_balance += &amount;

        if from_balance >= 0 {
            data.set(&from_balance_key, from_balance);
            data.set(&to_balance_key, to_balance);
            Ok(())
        } else {
            Err(Error::InsufficientBalance)
        }
    }

    pub fn mint(
        env: Env,
        seq: BigInt,
        admin: Signature,
        to: Identifier,
        amount: BigInt,
    ) -> Result<(), Error> {
        soroban_auth::verify(&env, &admin, symbol!("send"), (&seq, &to, &amount));

        if amount < 0 {
            return Err(Error::AmountNegative);
        }

        let data = env.contract_data();

        let to_balance_key = DataKey::Balance(to);
        let mut to_balance: BigInt = data
            .get(&to_balance_key)
            .unwrap_or_else(|| Ok(BigInt::zero(&env)))
            .unwrap();

        to_balance += amount;

        data.set(&to_balance_key, to_balance);
        Ok(())
    }
}

#[cfg(test)]
mod test;
