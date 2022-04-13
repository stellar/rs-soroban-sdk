#![no_std]
use sdk::{BigNum, Object, OrAbort, Symbol, Val};
use stellar_contract_sdk as sdk;

// This contract is a WIP port of:
// https://github.com/leighmcculloch/sjc-liqpool

const DATA_KEY_ACC_ID: Val = Val::from_symbol(Symbol::from_str("accid"));
const DATA_KEY_ASSET_POOL: Val = Val::from_symbol(Symbol::from_str("assetpool"));
const DATA_KEY_ASSET_POOL_CIRCULATING: Val =
    Val::from_symbol(Symbol::from_str("assetpoolcirculating")); // TODO: This symbol seems too long, why does creating it not fail?
const DATA_KEY_ASSET_A: Val = Val::from_symbol(Symbol::from_str("asseta"));
const DATA_KEY_ASSET_B: Val = Val::from_symbol(Symbol::from_str("assetb"));

// TODO: Define types for AccountId, and Asset.
fn _init(acc_id: Object, asset_pool: Object, asset_a: Object, asset_b: Object) -> bool {
    // TODO: Wrap the config data values into a type.
    sdk::ledger::put_contract_data(DATA_KEY_ACC_ID, acc_id.into());
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL, asset_pool.into());
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING, Val::from_u63(0));
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_A, asset_a.into());
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_B, asset_b.into());
    true
}

fn _deposit(src_acc_id: Object, amount_a: i64, amount_b: i64) -> i64 {
    if amount_a <= 0 || amount_b <= 0 {
        panic!("amounts must be greater than zero")
    }

    let acc_id: Object = sdk::ledger::get_contract_data(DATA_KEY_ACC_ID)
        .try_into()
        .or_abort();
    let asset_pool = sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL);
    let asset_a = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A);
    let asset_b = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B);

    let asset_pool_circulating: i64 =
        sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING)
            .try_into()
            .or_abort();
    let reserve_a: i64 = sdk::ledger::account_balance(acc_id.into(), asset_a)
        .try_into()
        .or_abort();
    let reserve_b: i64 = sdk::ledger::account_balance(acc_id.into(), asset_b)
        .try_into()
        .or_abort();

    let amount_pool: i64 = match asset_pool_circulating {
        0 => {
            // TODO: Use BigNum instead of f64.sqrt().
            let u: u64 = (BigNum::from(amount_a as u64) * BigNum::from(amount_b as u64))
                .sqrt()
                .try_into()
                .or_abort();
            u as i64
            //(amount_a as f64 * amount_b as f64).sqrt() as i64
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

    // TODO: Change pay to accept more specific types and native types.
    // TODO: Handle return values and errors from pay?
    sdk::ledger::pay(
        src_acc_id.into(),
        acc_id.into(),
        asset_a,
        amount_a.try_into().or_abort(),
    );
    sdk::ledger::pay(
        src_acc_id.into(),
        acc_id.into(),
        asset_b,
        amount_b.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc_id.into(),
        src_acc_id.into(),
        asset_pool,
        amount_pool.try_into().or_abort(),
    );
    amount_pool
}

fn _withdraw(src_acc_id: Object, amount_pool: i64) -> bool {
    if amount_pool <= 0 {
        panic!("amount must be greater than zero")
    }

    let acc_id: Object = sdk::ledger::get_contract_data(DATA_KEY_ACC_ID)
        .try_into()
        .or_abort();
    let asset_pool = sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL);
    let asset_a = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A);
    let asset_b = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B);

    let asset_pool_circulating: i64 =
        sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING)
            .try_into()
            .or_abort();
    if asset_pool_circulating == 0 {
        panic!("none of pool asset issued")
    }
    let reserve_a: i64 = sdk::ledger::account_balance(acc_id.into(), asset_a)
        .try_into()
        .or_abort();
    let reserve_b: i64 = sdk::ledger::account_balance(acc_id.into(), asset_b)
        .try_into()
        .or_abort();

    let amount_a = amount_pool * reserve_a / asset_pool_circulating;
    let amount_b = amount_pool * reserve_b / asset_pool_circulating;

    sdk::ledger::pay(
        src_acc_id.into(),
        acc_id.into(),
        asset_pool,
        amount_pool.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc_id.into(),
        src_acc_id.into(),
        asset_a,
        amount_a.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc_id.into(),
        src_acc_id.into(),
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
    src_acc_id: Object,
    asset_in: Val,
    amount_in: i64,
    asset_out: Val,
    min_amount_out: i64,
) -> i64 {
    if amount_in <= 0 {
        panic!("amount in must be greater than zero")
    }
    if min_amount_out <= 0 {
        panic!("min amount out must be zero or greater")
    }

    let acc_id = sdk::ledger::get_contract_data(DATA_KEY_ACC_ID);
    let asset_a = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A);
    let asset_b = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B);

    if !((asset_in == asset_a && asset_out == asset_b)
        || (asset_in == asset_b && asset_out == asset_a))
    {
        panic!("assets do not match pool")
    }

    let reserve_in: i64 = sdk::ledger::account_balance(acc_id, asset_in)
        .try_into()
        .or_abort();
    let reserve_out: i64 = sdk::ledger::account_balance(acc_id, asset_out)
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
        src_acc_id.into(),
        acc_id.into(),
        asset_in,
        amount_in.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc_id.into(),
        src_acc_id.into(),
        asset_out,
        amount_out.try_into().or_abort(),
    );
    amount_out
}

fn _trade_fixed_out(
    src_acc_id: Object,
    asset_in: Val,
    max_amount_in: i64,
    asset_out: Val,
    amount_out: i64,
) -> i64 {
    if amount_out == 0 {
        panic!("amount in must not be zero")
    }

    let acc_id: Object = sdk::ledger::get_contract_data(DATA_KEY_ACC_ID)
        .try_into()
        .or_abort();
    let asset_a = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A);
    let asset_b = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B);

    if !((asset_in == asset_a && asset_out == asset_b)
        || (asset_in == asset_b && asset_out == asset_a))
    {
        panic!("assets do not match pool")
    }

    let reserve_in: i64 = sdk::ledger::account_balance(acc_id.into(), asset_in)
        .try_into()
        .or_abort();
    let reserve_out: i64 = sdk::ledger::account_balance(acc_id.into(), asset_out)
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
    sdk::ledger::pay(
        src_acc_id.into(),
        acc_id.into(),
        asset_in,
        amount_in.try_into().or_abort(),
    );
    sdk::ledger::pay(
        acc_id.into(),
        src_acc_id.into(),
        asset_out,
        amount_out.try_into().or_abort(),
    );
    amount_in
}

#[cfg(test)]
mod test {
    use super::{
        _deposit, _init, DATA_KEY_ACC_ID, DATA_KEY_ASSET_A, DATA_KEY_ASSET_B, DATA_KEY_ASSET_POOL,
        DATA_KEY_ASSET_POOL_CIRCULATING,
    };
    use alloc::string::ToString;
    use sdk::testing::mem::{Address, MemHost, MemLedgerKey, MemLedgerVal, MemObj};
    use sdk::testing::swap_mock_host;
    use sdk::Val;
    use stellar_contract_sdk as sdk;
    extern crate alloc;
    extern crate std;
    use std::boxed::Box;

    #[test]
    fn test_init() {
        let mut host = MemHost::new();
        let acc_id = host.put_obj(MemObj::LedgerKey(MemLedgerKey::Account(Address(
            "GP".as_bytes().to_vec(),
        ))));
        let asset_pool = host.put_obj(MemObj::LedgerKey(MemLedgerKey::Asset((
            "P".to_string(),
            Address("GP".as_bytes().to_vec()),
        ))));
        let asset_a = host.put_obj(MemObj::LedgerKey(MemLedgerKey::Asset((
            "A".to_string(),
            Address("GA".as_bytes().to_vec()),
        ))));
        let asset_b = host.put_obj(MemObj::LedgerKey(MemLedgerKey::Asset((
            "B".to_string(),
            Address("GB".as_bytes().to_vec()),
        ))));
        let og_host = swap_mock_host(Box::new(host));

        assert_eq!(_init(acc_id, asset_pool, asset_a, asset_b), true);
        assert_eq!(
            sdk::ledger::get_contract_data(DATA_KEY_ACC_ID),
            acc_id.into(),
        );
        assert_eq!(
            sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL),
            asset_pool.into(),
        );
        assert_eq!(
            sdk::ledger::get_contract_data(DATA_KEY_ASSET_A),
            asset_a.into()
        );
        assert_eq!(
            sdk::ledger::get_contract_data(DATA_KEY_ASSET_B),
            asset_b.into()
        );
        assert_eq!(
            sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING),
            Val::from_u63(0),
        );

        swap_mock_host(og_host);
    }

    #[test]
    fn test_deposit() {
        let mut host = MemHost::new();

        let addr_p = Address("GP".as_bytes().to_vec());
        let addr_a = Address("GA".as_bytes().to_vec());
        let addr_b = Address("GB".as_bytes().to_vec());
        let addr_u1 = Address("GU1".as_bytes().to_vec());
        let addr_u2 = Address("GU2".as_bytes().to_vec());

        let acc_p_key = MemLedgerKey::Account(addr_p.clone());
        host.put_ledger_value(acc_p_key, MemLedgerVal::Account(0));
        let acc_p_tl_a_key = MemLedgerKey::TrustLine {
            account: addr_p.clone(),
            code: "A".to_string(),
            issuer: addr_a.clone(),
        };
        host.put_ledger_value(acc_p_tl_a_key, MemLedgerVal::TrustLine(0));
        let acc_p_tl_b_key = MemLedgerKey::TrustLine {
            account: addr_p.clone(),
            code: "B".to_string(),
            issuer: addr_b.clone(),
        };
        host.put_ledger_value(acc_p_tl_b_key, MemLedgerVal::TrustLine(0));

        let acc_u1_key = MemLedgerKey::Account(addr_u1.clone());
        host.put_ledger_value(acc_u1_key, MemLedgerVal::Account(0));
        let acc_u1_tl_a_key = MemLedgerKey::TrustLine {
            account: addr_u1.clone(),
            code: "A".to_string(),
            issuer: addr_a.clone(),
        };
        host.put_ledger_value(acc_u1_tl_a_key, MemLedgerVal::TrustLine(0));
        let acc_u1_tl_b_key = MemLedgerKey::TrustLine {
            account: addr_u1.clone(),
            code: "B".to_string(),
            issuer: addr_b.clone(),
        };
        host.put_ledger_value(acc_u1_tl_b_key, MemLedgerVal::TrustLine(0));

        let acc_id_obj = host.put_obj(MemObj::LedgerKey(acc_p_key));
        let asset_p_obj = host.put_obj(MemObj::LedgerKey(asset_p));
        let asset_a_obj = host.put_obj(MemObj::LedgerKey(asset_a));
        let asset_b_obj = host.put_obj(MemObj::LedgerKey(asset_b));
        let og_host = swap_mock_host(Box::new(host));

        assert_eq!(
            _init(acc_id_obj, asset_p_obj, asset_a_obj, asset_b_obj),
            true
        );
        assert_eq!(_deposit(acc_id_obj, 1000, 100), 30);

        swap_mock_host(og_host);
    }
}

#[no_mangle]
pub fn init(acc_id: Val, asset_pool: Val, asset_a: Val, asset_b: Val) -> Val {
    _init(
        acc_id.try_into().or_abort(),
        asset_pool.try_into().or_abort(),
        asset_a.try_into().or_abort(),
        asset_b.try_into().or_abort(),
    )
    .into()
}

#[no_mangle]
pub fn deposit(src_acc_id: Val, amount_a: Val, amount_b: Val) -> Val {
    _deposit(
        src_acc_id.try_into().or_abort(),
        amount_a.try_into().or_abort(),
        amount_b.try_into().or_abort(),
    )
    .try_into()
    .or_abort()
}

#[no_mangle]
pub fn withdraw(src_acc_id: Val, amount_pool: Val) -> Val {
    _withdraw(
        src_acc_id.try_into().or_abort(),
        amount_pool.try_into().or_abort(),
    )
    .into()
}

#[no_mangle]
pub fn trade_fixed_in(
    src_acc_id: Val,
    asset_in: Val,
    amount_in: Val,
    asset_out: Val,
    min_amount_out: Val,
) -> Val {
    _trade_fixed_in(
        src_acc_id.try_into().or_abort(),
        asset_in,
        amount_in.try_into().or_abort(),
        asset_out,
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
        asset_in,
        max_amount_in.try_into().or_abort(),
        asset_out,
        amount_out.try_into().or_abort(),
    )
    .try_into()
    .or_abort()
}
