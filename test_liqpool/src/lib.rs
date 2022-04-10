#![no_std]
use sdk::{OrAbort, Symbol, Val};
use stellar_contract_sdk as sdk;
use stellar_contract_sdk_macros as sdkmacros;

const DATA_KEY_ACC_ID: Val = Val::from_symbol(Symbol::from_str("accid"));
const DATA_KEY_ASSET_POOL: Val = Val::from_symbol(Symbol::from_str("assetpool"));
const DATA_KEY_ASSET_POOL_CIRCULATING: Val =
    Val::from_symbol(Symbol::from_str("assetpoolcirculating")); // TODO: This symbol seems too long, why does creating it not fail?
const DATA_KEY_ASSET_A: Val = Val::from_symbol(Symbol::from_str("asseta"));
const DATA_KEY_ASSET_B: Val = Val::from_symbol(Symbol::from_str("assetb"));

// TODO: Define types for AccountId, and Asset.
#[sdkmacros::contractfn]
pub fn init(acc_id: Val, asset_pool: Val, asset_a: Val, asset_b: Val) -> Val {
    // TODO: Wrap the config data values into a type.
    sdk::ledger::put_contract_data(DATA_KEY_ACC_ID, acc_id);
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL, asset_pool);
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING, Val::from_u63(0));
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_A, asset_a);
    sdk::ledger::put_contract_data(DATA_KEY_ASSET_B, asset_b);
    Val::from_bool(true)
}

#[sdkmacros::contractfn]
pub fn deposit(src_acc_id: Val, amount_a: i64, amount_b: i64) -> Val {
    if amount_a == 0 || amount_b == 0 {
        panic!("amounts must not be zero")
    }

    let acc_id = sdk::ledger::get_contract_data(DATA_KEY_ACC_ID);
    let asset_pool = sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL);
    let asset_a = sdk::ledger::get_contract_data(DATA_KEY_ASSET_A);
    let asset_b = sdk::ledger::get_contract_data(DATA_KEY_ASSET_B);

    let asset_pool_circulating: i64 =
        sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING)
            .try_into()
            .or_abort();
    let reserve_a: i64 = sdk::ledger::account_balance(acc_id, asset_a)
        .try_into()
        .or_abort();
    let reserve_b: i64 = sdk::ledger::account_balance(acc_id, asset_b)
        .try_into()
        .or_abort();

    let amount_pool: i64 = match asset_pool_circulating {
        0 => {
            // TODO: Use BigNum instead of f64.sqrt().
            (amount_a as f64 * amount_b as f64).sqrt() as i64
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
        amount_pool.try_into().or_abort(),
    );

    // TODO: Change pay to accept more specific types and native types.
    // TODO: Handle return values from pay?
    sdk::ledger::pay(src_acc_id, acc_id, asset_a, amount_a.try_into().or_abort());
    sdk::ledger::pay(src_acc_id, acc_id, asset_b, amount_b.try_into().or_abort());
    sdk::ledger::pay(
        acc_id,
        src_acc_id,
        asset_pool,
        amount_pool.try_into().or_abort(),
    );
    Val::from_bool(true)
}

#[sdkmacros::contractfn]
pub fn withdraw(_src_acc_id: Val, _pool_amount: Val) -> Val {
    todo!()
}

#[sdkmacros::contractfn]
pub fn trade_fixed_in(
    _src_acc_id: Val,
    _asset_in: Val,
    _amount_in: Val,
    _asset_out: Val,
    _min_amount_out: Val,
) -> Val {
    todo!()
}

#[sdkmacros::contractfn]
pub fn trade_fixed_out(
    _src_acc_id: Val,
    _asset_in: Val,
    _max_amount_in: Val,
    _asset_out: Val,
    _amount_out: Val,
) -> Val {
    todo!()
}

#[cfg(test)]
mod test {
    use super::{
        deposit, init, DATA_KEY_ACC_ID, DATA_KEY_ASSET_A, DATA_KEY_ASSET_B, DATA_KEY_ASSET_POOL,
        DATA_KEY_ASSET_POOL_CIRCULATING,
    };
    use sdk::{Symbol, Val};
    use stellar_contract_sdk as sdk;

    #[test]
    fn test_init() {
        // TODO: Figure out how to create AccountIds and Assets.
        let acc_id = Val::from_symbol(Symbol::from_str(&"accP"));
        let pool_asset = Val::from_symbol(Symbol::from_str(&"assetP"));
        let asset_a = Val::from_symbol(Symbol::from_str(&"assetA"));
        let asset_b = Val::from_symbol(Symbol::from_str(&"assetB"));
        assert_eq!(
            init(acc_id, pool_asset, asset_a, asset_b),
            Val::from_bool(true)
        );
        assert_eq!(acc_id, sdk::ledger::get_contract_data(DATA_KEY_ACC_ID));
        assert_eq!(
            pool_asset,
            sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL)
        );
        assert_eq!(asset_a, sdk::ledger::get_contract_data(DATA_KEY_ASSET_A));
        assert_eq!(asset_b, sdk::ledger::get_contract_data(DATA_KEY_ASSET_B));
        assert_eq!(
            Val::from_u63(0),
            sdk::ledger::get_contract_data(DATA_KEY_ASSET_POOL_CIRCULATING)
        );
    }

    #[test]
    fn test_deposit() {
        // TODO: Figure out how to create AccountIds and Assets.
        let acc_id = Val::from_symbol(Symbol::from_str(&"accP"));
        let pool_asset = Val::from_symbol(Symbol::from_str(&"assetP"));
        let asset_a = Val::from_symbol(Symbol::from_str(&"assetA"));
        let asset_b = Val::from_symbol(Symbol::from_str(&"assetB"));
        assert_eq!(
            init(acc_id, pool_asset, asset_a, asset_b),
            Val::from_bool(true)
        );
    }
}
