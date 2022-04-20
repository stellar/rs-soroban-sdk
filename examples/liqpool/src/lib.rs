#![no_std]
use stellar_contract_sdk as sdk;
use stellar_contract_sdk::{Object, OrAbort, Val};

mod config;
mod datakeys;
mod state;
use config::Config;
use state::State;

#[no_mangle]
pub fn init(acc: Val, asset_pool: Val, asset_a: Val, asset_b: Val) -> Val {
    _init(
        acc.try_into().or_abort(),
        asset_pool.try_into().or_abort(),
        asset_a.try_into().or_abort(),
        asset_b.try_into().or_abort(),
    )
    .into()
}

#[no_mangle]
pub fn deposit(src_acc: Val, amount_a: Val, amount_b: Val) -> Val {
    _deposit(
        src_acc.try_into().or_abort(),
        amount_a.try_into().or_abort(),
        amount_b.try_into().or_abort(),
    )
    .try_into()
    .or_abort()
}

#[no_mangle]
pub fn withdraw(src_acc: Val, amount_pool: Val) -> Val {
    _withdraw(
        src_acc.try_into().or_abort(),
        amount_pool.try_into().or_abort(),
    )
    .into()
}

#[no_mangle]
pub fn trade_fixed_in(
    src_acc: Val,
    asset_in: Val,
    amount_in: Val,
    asset_out: Val,
    min_amount_out: Val,
) -> Val {
    _trade_fixed_in(
        src_acc.try_into().or_abort(),
        asset_in.try_into().or_abort(),
        amount_in.try_into().or_abort(),
        asset_out.try_into().or_abort(),
        min_amount_out.try_into().or_abort(),
    )
    .try_into()
    .or_abort()
}

#[no_mangle]
pub fn trade_fixed_out(
    src_acc_id: Val,
    asset_in: Val,
    max_amount_in: Val,
    asset_out: Val,
    amount_out: Val,
) -> Val {
    _trade_fixed_out(
        src_acc_id.try_into().or_abort(),
        asset_in.try_into().or_abort(),
        max_amount_in.try_into().or_abort(),
        asset_out.try_into().or_abort(),
        amount_out.try_into().or_abort(),
    )
    .try_into()
    .or_abort()
}

fn _init(acc: Object, asset_p: Object, asset_a: Object, asset_b: Object) -> bool {
    Config {
        acc,
        asset_p,
        asset_a,
        asset_b,
    }
    .save();
    State {
        asset_p_circulating: 0,
    }
    .save();
    true
}

fn mul_sqrt(a: i64, b: i64) -> i64 {
    assert!(a >= 0);
    assert!(b >= 0);
    let a = a;
    let b = b;
    let mul = a as i128 * b as i128;
    let max = if a > b { a } else { b };
    let mut last = 0;
    for n in 0..=max {
        let nn = n as i128 * n as i128;
        if nn > mul {
            break;
        }
        last = n;
    }
    last
}

fn _deposit(src_acc: Object, amount_a: i64, amount_b: i64) -> i64 {
    if amount_a <= 0 || amount_b <= 0 {
        panic!("amounts must be greater than zero")
    }

    let config = Config::load();
    let state = State::load();

    let reserve_a: i64 = sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(
        config.acc,
        config.asset_a,
    ))
    .try_into()
    .or_abort();
    let reserve_b: i64 = sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(
        config.acc,
        config.asset_b,
    ))
    .try_into()
    .or_abort();

    let amount_pool: i64 = match state.asset_p_circulating {
        0 => {
            // TODO: Use BigNum instead of f64.sqrt().
            // let u: u64 = (BigNum::from(amount_a as u64) * BigNum::from(amount_b as u64))
            //     .sqrt()
            //     .try_into()
            //     .or_abort();
            // u as i64
            mul_sqrt(amount_a, amount_b)
        }
        _ => {
            let amount_pool_a = match reserve_a {
                0 => 0,
                _ => state.asset_p_circulating * amount_a / reserve_a,
            };
            let amount_pool_b = match reserve_b {
                0 => 0,
                _ => state.asset_p_circulating * amount_b / reserve_b,
            };
            if reserve_a > 0 && reserve_b > 0 {
                amount_pool_a.min(amount_pool_b)
            } else if reserve_a > 0 {
                amount_pool_a
            } else if reserve_b > 0 {
                amount_pool_b
            } else {
                unreachable!()
            }
        }
    };

    let state = State {
        asset_p_circulating: state.asset_p_circulating + amount_pool,
    };
    state.save();

    sdk::ledger::pay(
        src_acc.into(),
        config.acc.into(),
        config.asset_a,
        amount_a.try_into().or_abort(),
    );
    sdk::ledger::pay(
        src_acc.into(),
        config.acc.into(),
        config.asset_b,
        amount_b.try_into().or_abort(),
    );
    sdk::ledger::pay(
        config.acc.into(),
        src_acc.into(),
        config.asset_p,
        amount_pool.try_into().or_abort(),
    );

    amount_pool
}

fn _withdraw(src_acc: Object, amount_pool: i64) -> bool {
    if amount_pool <= 0 {
        panic!("amount must be greater than zero")
    }

    let config = Config::load();
    let state = State::load();

    if state.asset_p_circulating == 0 {
        panic!("none of pool asset issued")
    }

    let reserve_a: i64 = sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(
        config.acc,
        config.asset_a,
    ))
    .try_into()
    .or_abort();
    let reserve_b: i64 = sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(
        config.acc,
        config.asset_b,
    ))
    .try_into()
    .or_abort();

    let amount_a = amount_pool * reserve_a / state.asset_p_circulating;
    let amount_b = amount_pool * reserve_b / state.asset_p_circulating;

    sdk::ledger::pay(
        src_acc.into(),
        config.acc.into(),
        config.asset_p,
        amount_pool.try_into().or_abort(),
    );
    sdk::ledger::pay(
        config.acc.into(),
        src_acc.into(),
        config.asset_a,
        amount_a.try_into().or_abort(),
    );
    sdk::ledger::pay(
        config.acc.into(),
        src_acc.into(),
        config.asset_b,
        amount_b.try_into().or_abort(),
    );

    // let res: Vec<i64> = Vec::new();
    // res.push(amount_a);
    // res.push(amount_b);
    // res.into()
    true
}

fn _trade_fixed_in(
    src_acc: Object,
    asset_in: Object,
    amount_in: i64,
    asset_out: Object,
    min_amount_out: i64,
) -> i64 {
    if amount_in <= 0 {
        panic!("amount in must be greater than zero")
    }
    if min_amount_out <= 0 {
        panic!("min amount out must be zero or greater")
    }

    let config = Config::load();

    if !((asset_in == config.asset_a && asset_out == config.asset_b)
        || (asset_in == config.asset_b && asset_out == config.asset_a))
    {
        panic!("assets do not match pool")
    }

    let reserve_in: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(config.acc, asset_in))
            .try_into()
            .or_abort();
    let reserve_out: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(config.acc, asset_out))
            .try_into()
            .or_abort();

    // Calculate amount out to preserve current price.
    //   (x+a)*(y-b)=x*y
    //   b = (a*y)/(x+a)
    // TODO: Fees.
    let amount_out = (amount_in * reserve_out) / (reserve_in + amount_in);
    if amount_out < min_amount_out {
        panic!("min amount not met")
    }

    // TODO: Change pay to accept more specific types and native types.
    // TODO: Handle return values and errors from pay?
    sdk::ledger::pay(
        src_acc.into(),
        config.acc.into(),
        asset_in,
        amount_in.try_into().or_abort(),
    );
    sdk::ledger::pay(
        config.acc.into(),
        src_acc.into(),
        asset_out,
        amount_out.try_into().or_abort(),
    );
    amount_out
}

fn _trade_fixed_out(
    src_acc: Object,
    asset_in: Object,
    max_amount_in: i64,
    asset_out: Object,
    amount_out: i64,
) -> i64 {
    if amount_out == 0 {
        panic!("amount in must not be zero")
    }

    let config = Config::load();

    if !((asset_in == config.asset_a && asset_out == config.asset_b)
        || (asset_in == config.asset_b && asset_out == config.asset_a))
    {
        panic!("assets do not match pool")
    }

    let reserve_in: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(config.acc, asset_in))
            .try_into()
            .or_abort();
    let reserve_out: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(config.acc, asset_out))
            .try_into()
            .or_abort();

    // Calculate amount out to preserve current price.
    //   (x+a)*(y-b)=x*y
    //   a = (b*x)/(y-b)
    // TODO: Fees.
    let amount_in = (amount_out * reserve_in) / (reserve_out + amount_out);
    if amount_in > max_amount_in {
        panic!("max amount exceeded")
    }

    // TODO: Change pay to accept more specific types and native types.
    // TODO: Handle return values and errors from pay?
    sdk::ledger::pay(src_acc, config.acc, asset_in, amount_in.try_into().or_abort());
    sdk::ledger::pay(config.acc, src_acc, asset_out, amount_out.try_into().or_abort());
    amount_in
}

#[cfg(test)]
mod test {
    use crate::_trade_fixed_in;

    use super::{_deposit, _init, _withdraw};
    use alloc::string::ToString;
    use sdk::testing::mem::{AccountID, Asset, MemHost, MemLedgerKey, MemLedgerVal, MemObj};
    use sdk::testing::{swap_mock_host, with_mock_host};
    use stellar_contract_sdk as sdk;
    extern crate alloc;
    extern crate std;
    use sdk::Object;
    use std::boxed::Box;

    #[test]
    fn test() {
        let host = Box::new(MemHost::new());
        let og_host = swap_mock_host(host);

        let addr_p = AccountID("GP".as_bytes().to_vec());
        let addr_a = AccountID("GA".as_bytes().to_vec());
        let addr_b = AccountID("GB".as_bytes().to_vec());
        let addr_u1 = AccountID("GU1".as_bytes().to_vec());
        let addr_u2 = AccountID("GU2".as_bytes().to_vec());

        let asset_p = Asset::Credit {
            code: "P".to_string(),
            issuer: addr_p.clone(),
        };
        let asset_a = Asset::Credit {
            code: "A".to_string(),
            issuer: addr_a.clone(),
        };
        let asset_b = Asset::Credit {
            code: "B".to_string(),
            issuer: addr_b.clone(),
        };

        let asset_p_key = MemLedgerVal::Asset(asset_p.clone());
        let asset_a_key = MemLedgerVal::Asset(asset_a.clone());
        let asset_b_key = MemLedgerVal::Asset(asset_b.clone());

        let acc_p_key = MemLedgerKey::Account(addr_p.clone());
        let acc_p_tl_a_key = MemLedgerKey::TrustLine {
            account: addr_p.clone(),
            asset: asset_a.clone(),
        };
        let acc_p_tl_b_key = MemLedgerKey::TrustLine {
            account: addr_p.clone(),
            asset: asset_b.clone(),
        };

        let acc_u1_key = MemLedgerKey::Account(addr_u1.clone());
        let acc_u1_tl_a_key = MemLedgerKey::TrustLine {
            account: addr_u1.clone(),
            asset: asset_a.clone(),
        };
        let acc_u1_tl_b_key = MemLedgerKey::TrustLine {
            account: addr_u1.clone(),
            asset: asset_b.clone(),
        };
        let acc_u1_tl_p_key = MemLedgerKey::TrustLine {
            account: addr_u1.clone(),
            asset: asset_p.clone(),
        };

        let acc_u2_key = MemLedgerKey::Account(addr_u2.clone());
        let acc_u2_tl_a_key = MemLedgerKey::TrustLine {
            account: addr_u2.clone(),
            asset: asset_a.clone(),
        };
        let acc_u2_tl_b_key = MemLedgerKey::TrustLine {
            account: addr_u2.clone(),
            asset: asset_b.clone(),
        };
        let acc_u2_tl_p_key = MemLedgerKey::TrustLine {
            account: addr_u2.clone(),
            asset: asset_p.clone(),
        };

        with_mock_host(|h: &mut MemHost| {
            h.put_ledger_value(acc_p_key.clone(), MemLedgerVal::Account(0));
            h.put_ledger_value(acc_p_tl_a_key.clone(), MemLedgerVal::TrustLine(0));
            h.put_ledger_value(acc_p_tl_b_key.clone(), MemLedgerVal::TrustLine(0));

            h.put_ledger_value(acc_u1_key.clone(), MemLedgerVal::Account(0));
            h.put_ledger_value(acc_u1_tl_a_key.clone(), MemLedgerVal::TrustLine(100_000));
            h.put_ledger_value(acc_u1_tl_b_key.clone(), MemLedgerVal::TrustLine(100_000));
            h.put_ledger_value(acc_u1_tl_p_key.clone(), MemLedgerVal::TrustLine(0));

            h.put_ledger_value(acc_u2_key.clone(), MemLedgerVal::Account(0));
            h.put_ledger_value(acc_u2_tl_a_key.clone(), MemLedgerVal::TrustLine(100_000));
            h.put_ledger_value(acc_u2_tl_b_key.clone(), MemLedgerVal::TrustLine(100_000));
            h.put_ledger_value(acc_u2_tl_p_key.clone(), MemLedgerVal::TrustLine(0));
        });

        let acc_p_obj: Object =
            with_mock_host(|h: &mut MemHost| h.put_obj(MemObj::LedgerKey(acc_p_key.clone())));
        let acc_u1_obj: Object =
            with_mock_host(|h: &mut MemHost| h.put_obj(MemObj::LedgerKey(acc_u1_key.clone())));
        let acc_u2_obj: Object =
            with_mock_host(|h: &mut MemHost| h.put_obj(MemObj::LedgerKey(acc_u2_key.clone())));
        let asset_p_obj: Object =
            with_mock_host(|h: &mut MemHost| h.put_obj(MemObj::LedgerVal(asset_p_key.clone())));
        let asset_a_obj: Object =
            with_mock_host(|h: &mut MemHost| h.put_obj(MemObj::LedgerVal(asset_a_key.clone())));
        let asset_b_obj: Object =
            with_mock_host(|h: &mut MemHost| h.put_obj(MemObj::LedgerVal(asset_b_key.clone())));

        assert_eq!(
            _init(acc_p_obj, asset_p_obj, asset_a_obj, asset_b_obj),
            true
        );

        assert_eq!(_deposit(acc_u1_obj, 100_000, 10_000), 31622);
        with_mock_host(|h: &mut MemHost| {
            assert_eq!(
                h.get_ledger_value(acc_p_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(100_000))
            );
            assert_eq!(
                h.get_ledger_value(acc_p_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(10_000))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_p_key.clone()),
                Some(MemLedgerVal::TrustLine(31622))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(0))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(90_000))
            );
        });

        assert_eq!(
            _trade_fixed_in(acc_u2_obj, asset_a_obj, 100, asset_b_obj, 1),
            9
        );
        with_mock_host(|h: &mut MemHost| {
            assert_eq!(
                h.get_ledger_value(acc_p_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(100_100))
            );
            assert_eq!(
                h.get_ledger_value(acc_p_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(9_991))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_p_key.clone()),
                Some(MemLedgerVal::TrustLine(31622))
            );
            assert_eq!(
                h.get_ledger_value(acc_u2_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(99_900))
            );
            assert_eq!(
                h.get_ledger_value(acc_u2_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(100_009))
            );
        });

        assert_eq!(_deposit(acc_u2_obj, 100, 1), 3);
        with_mock_host(|h: &mut MemHost| {
            assert_eq!(
                h.get_ledger_value(acc_p_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(100_200))
            );
            assert_eq!(
                h.get_ledger_value(acc_p_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(9_992))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_p_key.clone()),
                Some(MemLedgerVal::TrustLine(31622))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(0))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(90_000))
            );
            assert_eq!(
                h.get_ledger_value(acc_u2_tl_p_key.clone()),
                Some(MemLedgerVal::TrustLine(3))
            );
            assert_eq!(
                h.get_ledger_value(acc_u2_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(99_800))
            );
            assert_eq!(
                h.get_ledger_value(acc_u2_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(100_008))
            );
        });

        assert_eq!(_withdraw(acc_u1_obj, 31622), true);
        with_mock_host(|h: &mut MemHost| {
            assert_eq!(
                h.get_ledger_value(acc_p_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(10))
            );
            assert_eq!(
                h.get_ledger_value(acc_p_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(1))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_p_key.clone()),
                Some(MemLedgerVal::TrustLine(0))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(100_190))
            );
            assert_eq!(
                h.get_ledger_value(acc_u1_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(99_991))
            );
            assert_eq!(
                h.get_ledger_value(acc_u2_tl_p_key.clone()),
                Some(MemLedgerVal::TrustLine(3))
            );
            assert_eq!(
                h.get_ledger_value(acc_u2_tl_a_key.clone()),
                Some(MemLedgerVal::TrustLine(99_800))
            );
            assert_eq!(
                h.get_ledger_value(acc_u2_tl_b_key.clone()),
                Some(MemLedgerVal::TrustLine(100_008))
            );
        });

        swap_mock_host(og_host);
    }
}
