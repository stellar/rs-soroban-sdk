use crate::testutils::storage::{Persistent, Temporary};
use crate::testutils::{Address as _, Ledger};
use crate::{self as soroban_sdk};
use soroban_sdk::{
    contract, contractdata, contractimpl, Address, ContractDataKey, Env, FixedContractData, Symbol,
    UpdateError,
};

#[contractdata(
    storage = persistent,
    namespace = "USER_DEPOSIT",
    ttl = bump_when_below(100, 500),
)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct UserDeposit {
    #[key]
    user: Address,
    amount: u32,
    nonce: u32,
}

#[contractdata(storage = instance)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct Admin(u32);

#[contractdata(storage = persistent, ttl = bump_to(700))]
#[derive(Clone, Debug, Eq, PartialEq)]
struct PoolConfig {
    fee_bps: u32,
}

#[contractdata(
    storage = temporary,
    key = "custom_config",
    ttl = bump_when_below(100, 500),
)]
#[derive(Clone, Debug, Eq, PartialEq)]
struct ExplicitConfig(u32);

#[contractdata(storage = persistent, namespace = "ALLOWANCE")]
#[derive(Clone, Debug, Eq, PartialEq)]
struct Allowance {
    #[key]
    owner: Address,
    #[key]
    spender: Address,
    amount: i128,
}

#[contractdata(storage = temporary, namespace = "SESSION")]
#[derive(Clone, Debug, Eq, PartialEq)]
struct Session {
    #[key]
    id: u32,
    issued_at: u32,
}

#[contract]
struct ContractDataContract;

#[contractimpl]
impl ContractDataContract {}

fn env() -> (Env, Address) {
    let env = Env::default();
    env.ledger().set_min_persistent_entry_ttl(100);
    env.ledger().set_min_temp_entry_ttl(50);
    env.ledger().set_max_entry_ttl(20_000);
    let contract_id = env.register(ContractDataContract, ());
    (env, contract_id)
}

#[test]
fn typed_contract_data() {
    let (env, contract_id) = env();

    env.as_contract(&contract_id, || {
        assert!(!Admin::has(&env));
        assert_eq!(Admin::get(&env), None);
        Admin(7).save(&env);
        assert!(Admin::has(&env));
        assert_eq!(Admin::get(&env), Some(Admin(7)));
        assert_eq!(
            env.storage()
                .instance()
                .get::<Symbol, Admin>(&Symbol::new(&env, "admin")),
            Some(Admin(7))
        );
        Admin(7).bump(&env);
        assert_eq!(Admin::get(&env), Some(Admin(7)));
        Admin::remove(&env);
        assert!(!Admin::has(&env));
        Admin(8).save(&env).bump(&env);
        assert_eq!(Admin::get(&env), Some(Admin(8)));
        Admin::update(&env, |admin| {
            admin.0 = 9;
            Ok::<_, ()>(())
        })
        .unwrap();
        assert_eq!(Admin::get(&env), Some(Admin(9)));

        let user = Address::generate(&env);
        assert!(UserDeposit::get(&env, &user).is_none());
        assert!(!UserDeposit::has(&env, &user));
        let deposit = UserDeposit {
            user: user.clone(),
            amount: 9,
            nonce: 1,
        };
        deposit.save(&env);
        assert!(UserDeposit::has(&env, &user));
        assert_eq!(UserDeposit::get(&env, &user), Some(deposit.clone()));
        deposit.bump(&env);
        assert_eq!(UserDeposit::get(&env, &user), Some(deposit));
        UserDeposit::update(&env, &user, |deposit| {
            deposit.amount = 10;
            Ok::<_, ()>(())
        })
        .unwrap();
        assert_eq!(UserDeposit::get(&env, &user).unwrap().amount, 10);
        let other_user = Address::generate(&env);
        assert_eq!(
            UserDeposit::update(&env, &user, |deposit| {
                deposit.user = other_user.clone();
                Ok::<_, ()>(())
            }),
            Err(UpdateError::KeyChanged)
        );
        assert_eq!(UserDeposit::get(&env, &user).unwrap().amount, 10);
        UserDeposit::remove(&env, &user);
        assert!(!UserDeposit::has(&env, &user));
        UserDeposit {
            user: user.clone(),
            amount: 9,
            nonce: 1,
        }
        .save(&env)
        .bump(&env);
        assert_eq!(
            UserDeposit::get(&env, &user),
            Some(UserDeposit {
                user,
                amount: 9,
                nonce: 1,
            })
        );
    });
}

#[test]
fn macro_combinations() {
    let (env, contract_id) = env();

    env.as_contract(&contract_id, || {
        // Fixed persistent data: implicit `snake_case` key and `bump_to`.
        assert_eq!(PoolConfig::get(&env), None);
        PoolConfig { fee_bps: 30 }.save(&env);
        assert!(PoolConfig::has(&env));
        assert_eq!(PoolConfig::get(&env), Some(PoolConfig { fee_bps: 30 }));
        assert_eq!(
            env.storage()
                .persistent()
                .get::<Symbol, PoolConfig>(&Symbol::new(&env, "pool_config")),
            Some(PoolConfig { fee_bps: 30 })
        );
        PoolConfig { fee_bps: 30 }.bump(&env);
        assert_eq!(
            env.storage()
                .persistent()
                .get_ttl(&Symbol::new(&env, "pool_config")),
            700
        );
        assert_eq!(PoolConfig::get(&env), Some(PoolConfig { fee_bps: 30 }));
        PoolConfig::remove(&env);
        assert!(!PoolConfig::has(&env));
        PoolConfig { fee_bps: 50 }.save(&env).bump(&env);
        assert_eq!(PoolConfig::get(&env), Some(PoolConfig { fee_bps: 50 }));

        // Fixed temporary data: explicit key and `bump_when_below`.
        assert_eq!(ExplicitConfig::get(&env), None);
        ExplicitConfig(7).save(&env);
        assert!(ExplicitConfig::has(&env));
        assert_eq!(ExplicitConfig::get(&env), Some(ExplicitConfig(7)));
        assert_eq!(
            env.storage()
                .temporary()
                .get::<Symbol, ExplicitConfig>(&Symbol::new(&env, "custom_config")),
            Some(ExplicitConfig(7))
        );
        ExplicitConfig(7).bump(&env);
        assert_eq!(
            env.storage()
                .temporary()
                .get_ttl(&Symbol::new(&env, "custom_config")),
            500
        );
        assert_eq!(ExplicitConfig::get(&env), Some(ExplicitConfig(7)));
        ExplicitConfig::remove(&env);
        assert!(!ExplicitConfig::has(&env));
        ExplicitConfig(9).save(&env).bump(&env);
        assert_eq!(ExplicitConfig::get(&env), Some(ExplicitConfig(9)));

        // Persistent dynamic data: two borrowed key components.
        let allowance_key = (Address::generate(&env), Address::generate(&env));
        assert_eq!(Allowance::get(&env, &allowance_key), None);
        assert!(!Allowance::has(&env, &allowance_key));
        let allowance = Allowance {
            owner: allowance_key.0.clone(),
            spender: allowance_key.1.clone(),
            amount: 100,
        };
        allowance.save(&env);
        assert!(Allowance::has(&env, &allowance_key));
        assert!(env.storage().persistent().has(&(
            Symbol::new(&env, "ALLOWANCE"),
            &allowance_key.0,
            &allowance_key.1,
        )));
        assert_eq!(
            Allowance::get(&env, &allowance_key),
            Some(allowance.clone())
        );
        allowance.bump(&env);
        assert_eq!(Allowance::get(&env, &allowance_key), Some(allowance));
        Allowance::remove(&env, &allowance_key);
        assert!(!Allowance::has(&env, &allowance_key));
        Allowance {
            owner: allowance_key.0.clone(),
            spender: allowance_key.1.clone(),
            amount: 200,
        }
        .save(&env)
        .bump(&env);
        assert_eq!(
            Allowance::get(&env, &allowance_key),
            Some(Allowance {
                owner: allowance_key.0.clone(),
                spender: allowance_key.1.clone(),
                amount: 200,
            })
        );

        // Temporary dynamic data: a single scalar key and no TTL policy.
        let session_id = 7;
        assert_eq!(Session::get(&env, &session_id), None);
        assert!(!Session::has(&env, &session_id));
        let session = Session {
            id: session_id,
            issued_at: 42,
        };
        session.save(&env);
        assert!(Session::has(&env, &session_id));
        assert!(env
            .storage()
            .temporary()
            .has(&(Symbol::new(&env, "SESSION"), session_id)));
        assert_eq!(Session::get(&env, &session_id), Some(session.clone()));
        session.bump(&env);
        assert_eq!(Session::get(&env, &session_id), Some(session));
        Session::remove(&env, &session_id);
        assert!(!Session::has(&env, &session_id));
        Session {
            id: session_id,
            issued_at: 84,
        }
        .save(&env)
        .bump(&env);
        assert_eq!(
            Session::get(&env, &session_id),
            Some(Session {
                id: session_id,
                issued_at: 84,
            })
        );
    });
}
