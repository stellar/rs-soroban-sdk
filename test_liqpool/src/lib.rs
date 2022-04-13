#![no_std]
use sdk::{Object, OrAbort, Symbol, Val};
use stellar_contract_sdk as sdk;

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

const DATA_KEY_ACCOUNT: Val = Val::from_symbol(Symbol::from_str("accid"));
const DATA_KEY_ASSET_POOL: Val = Val::from_symbol(Symbol::from_str("assetpool"));
const DATA_KEY_ASSET_POOL_CIRCULATING: Val =
    Val::from_symbol(Symbol::from_str("assetpoolcirculating")); // TODO: This symbol seems too long, why does creating it not fail?
const DATA_KEY_ASSET_A: Val = Val::from_symbol(Symbol::from_str("asseta"));
const DATA_KEY_ASSET_B: Val = Val::from_symbol(Symbol::from_str("assetb"));

fn _init(acc: Object, asset_p: Object, asset_a: Object, asset_b: Object) -> bool {
    sdk::ledger::put_contract_data(DATA_KEY_ACCOUNT, acc.into());
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL, asset_p.into());
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING, Val::from_u63(0));
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_A, asset_a.into());
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_B, asset_b.into());
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

    let acc: Object = sdk::ledger::get_contract_data(DATA_KEY_ACCOUNT)
        .try_into()
        .or_abort();
    let asset_pool: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL)
        .try_into()
        .or_abort();
    let asset_a: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A)
        .try_into()
        .or_abort();
    let asset_b: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B)
        .try_into()
        .or_abort();

    let asset_pool_circulating: i64 =
        sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING)
            .try_into()
            .or_abort();
    let reserve_a: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(acc, asset_a))
            .try_into()
            .or_abort();
    let reserve_b: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(acc, asset_b))
            .try_into()
            .or_abort();

    let amount_pool: i64 = match asset_pool_circulating {
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
                _ => asset_pool_circulating * amount_a / reserve_a,
            };
            let amount_pool_b = match reserve_b {
                0 => 0,
                _ => asset_pool_circulating * amount_b / reserve_b,
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

    sdk::ledger::put_contract_data(
        DATA_KEY_ASSET_POOL_CIRCULATING,
        (asset_pool_circulating + amount_pool).try_into().or_abort(),
    );

    sdk::ledger::pay(
        src_acc.into(),
        acc.into(),
        asset_a,
        amount_a.try_into().or_abort(),
    );
    sdk::ledger::pay(
        src_acc.into(),
        acc.into(),
        asset_b,
        amount_b.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc.into(),
        src_acc.into(),
        asset_pool,
        amount_pool.try_into().or_abort(),
    );

    amount_pool
}

fn _withdraw(src_acc: Object, amount_pool: i64) -> bool {
    if amount_pool <= 0 {
        panic!("amount must be greater than zero")
    }

    let acc: Object = sdk::ledger::get_contract_data(DATA_KEY_ACCOUNT)
        .try_into()
        .or_abort();
    let asset_pool: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL)
        .try_into()
        .or_abort();
    let asset_a: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A)
        .try_into()
        .or_abort();
    let asset_b: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B)
        .try_into()
        .or_abort();

    let asset_pool_circulating: i64 =
        sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING)
            .try_into()
            .or_abort();
    if asset_pool_circulating == 0 {
        panic!("none of pool asset issued")
    }
    let reserve_a: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(acc, asset_a))
            .try_into()
            .or_abort();
    let reserve_b: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(acc, asset_b))
            .try_into()
            .or_abort();

    let amount_a = amount_pool * reserve_a / asset_pool_circulating;
    let amount_b = amount_pool * reserve_b / asset_pool_circulating;

    sdk::ledger::pay(
        src_acc.into(),
        acc.into(),
        asset_pool,
        amount_pool.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc.into(),
        src_acc.into(),
        asset_a,
        amount_a.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc.into(),
        src_acc.into(),
        asset_b,
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

    let acc: Object = sdk::ledger::get_contract_data(DATA_KEY_ACCOUNT)
        .try_into()
        .or_abort();
    let asset_a: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A)
        .try_into()
        .or_abort();
    let asset_b: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B)
        .try_into()
        .or_abort();

    if !((asset_in == asset_a && asset_out == asset_b)
        || (asset_in == asset_b && asset_out == asset_a))
    {
        panic!("assets do not match pool")
    }

    let reserve_in: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(acc, asset_in))
            .try_into()
            .or_abort();
    let reserve_out: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(acc, asset_out))
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
        acc.into(),
        asset_in,
        amount_in.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc.into(),
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

    let acc: Object = sdk::ledger::get_contract_data(DATA_KEY_ACCOUNT)
        .try_into()
        .or_abort();
    let asset_a: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A)
        .try_into()
        .or_abort();
    let asset_b: Object = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B)
        .try_into()
        .or_abort();

    if !((asset_in == asset_a && asset_out == asset_b)
        || (asset_in == asset_b && asset_out == asset_a))
    {
        panic!("assets do not match pool")
    }

    let reserve_in: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(acc, asset_in))
            .try_into()
            .or_abort();
    let reserve_out: i64 =
        sdk::ledger::trust_line_balance(sdk::ledger::account_trust_line(acc, asset_out))
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
    sdk::ledger::pay(src_acc, acc, asset_in, amount_in.try_into().or_abort());
    sdk::ledger::pay(acc, src_acc, asset_out, amount_out.try_into().or_abort());
    amount_in
}

#[cfg(test)]
mod test {
    use crate::_trade_fixed_in;

    use super::{_deposit, _init, _withdraw};
    use alloc::string::ToString;
    use sdk::testing::mem::{Address, Asset, MemHost, MemLedgerKey, MemLedgerVal, MemObj};
    use sdk::testing::swap_mock_host;
    use stellar_contract_sdk as sdk;
    extern crate alloc;
    extern crate std;
    use std::boxed::Box;

    #[test]
    fn test() {
        let mut host = Box::new(MemHost::new());

        let addr_p = Address("GP".as_bytes().to_vec());
        let addr_a = Address("GA".as_bytes().to_vec());
        let addr_b = Address("GB".as_bytes().to_vec());
        let addr_u1 = Address("GU1".as_bytes().to_vec());
        let addr_u2 = Address("GU2".as_bytes().to_vec());

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
        host.put_ledger_value(acc_p_key.clone(), MemLedgerVal::Account(0));
        host.put_ledger_value(acc_p_tl_a_key.clone(), MemLedgerVal::TrustLine(0));
        host.put_ledger_value(acc_p_tl_b_key.clone(), MemLedgerVal::TrustLine(0));

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
        host.put_ledger_value(acc_u1_key.clone(), MemLedgerVal::Account(0));
        host.put_ledger_value(acc_u1_tl_a_key.clone(), MemLedgerVal::TrustLine(100_000));
        host.put_ledger_value(acc_u1_tl_b_key.clone(), MemLedgerVal::TrustLine(100_000));
        host.put_ledger_value(acc_u1_tl_p_key, MemLedgerVal::TrustLine(0));

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
        host.put_ledger_value(acc_u2_key.clone(), MemLedgerVal::Account(0));
        host.put_ledger_value(acc_u2_tl_a_key, MemLedgerVal::TrustLine(100_000));
        host.put_ledger_value(acc_u2_tl_b_key, MemLedgerVal::TrustLine(100_000));
        host.put_ledger_value(acc_u2_tl_p_key, MemLedgerVal::TrustLine(0));

        let acc_p_obj = host.put_obj(MemObj::LedgerKey(acc_p_key.clone()));
        let acc_u1_obj = host.put_obj(MemObj::LedgerKey(acc_u1_key.clone()));
        let acc_u2_obj = host.put_obj(MemObj::LedgerKey(acc_u2_key.clone()));
        let asset_p_obj = host.put_obj(MemObj::LedgerVal(asset_p_key.clone()));
        let asset_a_obj = host.put_obj(MemObj::LedgerVal(asset_a_key.clone()));
        let asset_b_obj = host.put_obj(MemObj::LedgerVal(asset_b_key.clone()));

        swap_mock_host(host);

        assert_eq!(
            _init(acc_p_obj, asset_p_obj, asset_a_obj, asset_b_obj),
            true
        );
        assert_eq!(_deposit(acc_u1_obj, 100_000, 10_000), 31622);
        assert_eq!(
            _trade_fixed_in(acc_u2_obj, asset_a_obj, 100, asset_b_obj, 1),
            9
        );
        assert_eq!(_withdraw(acc_u1_obj, 31622), true);
    }
}
