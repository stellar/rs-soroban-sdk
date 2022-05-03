use core::cell::RefCell;

use stellar_contract_host::HostContext;

thread_local! {
    pub static HOST: RefCell<Box<HostContext>> = RefCell::new(Box::new(HostContext::default()));
}

pub(crate) mod context {
    use super::HOST;
    use stellar_contract_host::{Host, Object, Val};
    pub(crate) unsafe fn log_value(v: Val) -> Val {
        HOST.with(|h| h.borrow_mut().log_value(v))
    }
    pub(crate) unsafe fn get_last_operation_result() -> Object {
        HOST.with(|h| h.borrow_mut().get_last_operation_result())
    }
}

pub(crate) mod u64 {
    use super::HOST;
    use stellar_contract_host::{Host, Object};
    pub(crate) unsafe fn from_u64(u: u64) -> Object {
        HOST.with(|h| h.borrow_mut().obj_from_u64(u))
    }

    pub(crate) unsafe fn to_u64(u: Object) -> u64 {
        HOST.with(|h| h.borrow_mut().obj_to_u64(u))
    }
}

pub(crate) mod i64 {
    use super::HOST;
    use stellar_contract_host::{Host, Object};
    pub(crate) unsafe fn from_i64(i: i64) -> Object {
        HOST.with(|h| h.borrow_mut().obj_from_i64(i))
    }

    pub(crate) unsafe fn to_i64(i: Object) -> i64 {
        HOST.with(|h| h.borrow_mut().obj_to_i64(i))
    }
}

pub(crate) mod map {
    use super::HOST;
    use stellar_contract_host::{Host, Object, Val};
    pub(crate) unsafe fn new() -> Object {
        HOST.with(|h| h.borrow_mut().map_new())
    }
    pub(crate) unsafe fn put(m: Object, k: Val, v: Val) -> Object {
        HOST.with(|h| h.borrow_mut().map_put(m, k, v))
    }

    pub(crate) unsafe fn get(m: Object, k: Val) -> Val {
        HOST.with(|h| h.borrow_mut().map_get(m, k))
    }

    pub(crate) unsafe fn del(m: Object, k: Val) -> Object {
        HOST.with(|h| h.borrow_mut().map_del(m, k))
    }

    pub(crate) unsafe fn len(m: Object) -> Val {
        HOST.with(|h| h.borrow_mut().map_len(m))
    }

    pub(crate) unsafe fn keys(m: Object) -> Object {
        HOST.with(|h| h.borrow_mut().map_keys(m))
    }

    pub(crate) unsafe fn has(m: Object, k: Val) -> Val {
        HOST.with(|h| h.borrow_mut().map_has(m, k))
    }
}

pub(crate) mod vec {
    use super::HOST;
    use stellar_contract_host::{Host, Object, Val};
    pub(crate) unsafe fn new() -> Object {
        HOST.with(|h| h.borrow_mut().vec_new())
    }

    pub(crate) unsafe fn put(v: Object, i: Val, x: Val) -> Object {
        HOST.with(|h| h.borrow_mut().vec_put(v, i, x))
    }

    pub(crate) unsafe fn get(v: Object, i: Val) -> Val {
        HOST.with(|h| h.borrow_mut().vec_get(v, i))
    }

    pub(crate) unsafe fn del(v: Object, i: Val) -> Object {
        HOST.with(|h| h.borrow_mut().vec_del(v, i))
    }

    pub(crate) unsafe fn len(v: Object) -> Val {
        HOST.with(|h| h.borrow_mut().vec_len(v))
    }

    pub(crate) unsafe fn push(v: Object, x: Val) -> Object {
        HOST.with(|h| h.borrow_mut().vec_push(v, x))
    }

    pub(crate) unsafe fn pop(v: Object) -> Object {
        HOST.with(|h| h.borrow_mut().vec_pop(v))
    }

    pub(crate) unsafe fn take(v: Object, n: Val) -> Object {
        HOST.with(|h| h.borrow_mut().vec_take(v, n))
    }

    pub(crate) unsafe fn drop(v: Object, n: Val) -> Object {
        HOST.with(|h| h.borrow_mut().vec_drop(v, n))
    }

    pub(crate) unsafe fn front(v: Object) -> Val {
        HOST.with(|h| h.borrow_mut().vec_front(v))
    }

    pub(crate) unsafe fn back(v: Object) -> Val {
        HOST.with(|h| h.borrow_mut().vec_back(v))
    }

    pub(crate) unsafe fn insert(v: Object, i: Val, x: Val) -> Object {
        HOST.with(|h| h.borrow_mut().vec_insert(v, i, x))
    }

    pub(crate) unsafe fn append(v1: Object, v2: Object) -> Object {
        HOST.with(|h| h.borrow_mut().vec_append(v1, v2))
    }
}

pub(crate) mod ledger {
    use super::HOST;
    use stellar_contract_host::{Host, Object, Val};
    pub(crate) unsafe fn get_current_ledger_num() -> Val {
        todo!()
    }

    // NB: this returns a raw/unboxed u64, not a Val union.

    pub(crate) unsafe fn get_current_ledger_close_time() -> u64 {
        todo!()
    }

    // NB: returns a Status; details can be fetched with
    // get_last_operation_result.

    pub(crate) unsafe fn pay(src: Object, dst: Object, asset: Object, amount: Val) -> Val {
        HOST.with(|h| h.borrow_mut().pay(src, dst, asset, amount))
    }

    pub(crate) unsafe fn account_balance(acc: Object) -> Val {
        HOST.with(|h| h.borrow_mut().account_balance(acc))
    }

    pub(crate) unsafe fn account_trust_line(acc: Object, asset: Object) -> Object {
        HOST.with(|h| h.borrow_mut().account_trust_line(acc, asset))
    }

    pub(crate) unsafe fn trust_line_balance(trust_line: Object) -> Val {
        HOST.with(|h| h.borrow_mut().trust_line_balance(trust_line))
    }

    pub(crate) unsafe fn put_contract_data(key: Val, val: Val) -> Val {
        HOST.with(|h| h.borrow_mut().put_contract_data(key, val))
    }

    pub(crate) unsafe fn has_contract_data(key: Val) -> Val {
        HOST.with(|h| h.borrow_mut().has_contract_data(key))
    }

    pub(crate) unsafe fn get_contract_data(key: Val) -> Val {
        HOST.with(|h| h.borrow_mut().get_contract_data(key))
    }

    pub(crate) unsafe fn del_contract_data(key: Val) -> Val {
        todo!()
    }
}

pub(crate) mod call {
    use crate::Val;
    pub(crate) unsafe fn call0(contract: Val, func: Val) -> Val {
        todo!()
    }

    pub(crate) unsafe fn call1(contract: Val, func: Val, a: Val) -> Val {
        todo!()
    }

    pub(crate) unsafe fn call2(contract: Val, func: Val, a: Val, b: Val) -> Val {
        todo!()
    }

    pub(crate) unsafe fn call3(contract: Val, func: Val, a: Val, b: Val, c: Val) -> Val {
        todo!()
    }

    pub(crate) unsafe fn call4(contract: Val, func: Val, a: Val, b: Val, c: Val, d: Val) -> Val {
        todo!()
    }
}

pub(crate) mod bignum {
    use stellar_contract_host::{Object, Val};
    pub(crate) unsafe fn from_u64(x: u64) -> Object {
        todo!()
    }

    pub(crate) unsafe fn from_i64(x: i64) -> Object {
        todo!()
    }

    pub(crate) unsafe fn add(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn sub(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn mul(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn div(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn rem(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn and(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn or(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn xor(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn shl(lhs: Object, rhs: u64) -> Object {
        todo!()
    }

    pub(crate) unsafe fn shr(lhs: Object, rhs: u64) -> Object {
        todo!()
    }

    pub(crate) unsafe fn cmp(lhs: Object, rhs: Object) -> Val {
        todo!()
    }

    pub(crate) unsafe fn is_zero(x: Object) -> Val {
        todo!()
    }

    pub(crate) unsafe fn neg(x: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn not(x: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn gcd(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn lcm(lhs: Object, rhs: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn pow(lhs: Object, rhs: u64) -> Object {
        todo!()
    }

    pub(crate) unsafe fn pow_mod(p: Object, q: Object, m: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn sqrt(x: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn bits(x: Object) -> u64 {
        todo!()
    }

    pub(crate) unsafe fn to_u64(x: Object) -> u64 {
        todo!()
    }

    pub(crate) unsafe fn to_i64(x: Object) -> i64 {
        todo!()
    }
}
