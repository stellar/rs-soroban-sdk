// This module implements the host interface using a fake in-memory thread-local
// structure. It exists for running unit tests in contracts, when they do not
// even want to compile to wasm and run against the sandbox. It is not an exact
// copy of either the sandbox or the real implementation in stellar-core, but it
// has semantics that are as close as possible.

use crate::{Object, Status, Val};

use core::fmt::Debug;

impl Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Val")
            .field("body", &self.get_body())
            .field("tag", &self.get_tag())
            .finish()
    }
}

impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (ty, code) = self.decompose_status();
        f.debug_struct("Status")
            .field("type", &ty)
            .field("code", &code)
            .finish()
    }
}

pub trait MockHost {
    fn as_mut_any(&mut self) -> &mut dyn any::Any;

    fn log_value(&mut self, v: Val) -> Val;
    fn get_last_operation_result(&mut self) -> Object;

    fn u64_from_u64(&mut self, u: u64) -> Object;
    fn u64_to_u64(&mut self, u: Object) -> u64;
    fn i64_from_i64(&mut self, i: i64) -> Object;
    fn i64_to_i64(&mut self, i: Object) -> i64;

    fn map_new(&mut self) -> Object;
    fn map_put(&mut self, m: Object, k: Val, v: Val) -> Object;
    fn map_get(&mut self, m: Object, k: Val) -> Val;
    fn map_del(&mut self, m: Object, k: Val) -> Object;
    fn map_len(&mut self, m: Object) -> Val;
    fn map_keys(&mut self, m: Object) -> Object;
    fn map_has(&mut self, m: Object, k: Val) -> Val;

    fn vec_new(&mut self) -> Object;
    fn vec_put(&mut self, v: Object, i: Val, x: Val) -> Object;
    fn vec_get(&mut self, v: Object, i: Val) -> Val;
    fn vec_del(&mut self, v: Object, i: Val) -> Object;
    fn vec_len(&mut self, v: Object) -> Val;
    fn vec_push(&mut self, v: Object, x: Val) -> Object;
    fn vec_pop(&mut self, v: Object) -> Object;
    fn vec_take(&mut self, v: Object, n: Val) -> Object;
    fn vec_drop(&mut self, v: Object, n: Val) -> Object;
    fn vec_front(&mut self, v: Object) -> Val;
    fn vec_back(&mut self, v: Object) -> Val;
    fn vec_insert(&mut self, v: Object, i: Val, n: Val) -> Object;
    fn vec_append(&mut self, v1: Object, v2: Object) -> Object;

    fn pay(&mut self, src: Object, dst: Object, asset: Object, amount: Val) -> Val;
    fn account_balance(&mut self, acc: Object) -> Val;
    fn account_trust_line(&mut self, acc: Object, asset: Object) -> Object;
    fn trust_line_balance(&mut self, tl: Object) -> Val;
    fn get_contract_data(&mut self, k: Val) -> Val;
    fn put_contract_data(&mut self, k: Val, v: Val) -> Val;
    fn has_contract_data(&mut self, k: Val) -> Val;
    //...
}

use core::cell::RefCell;
use std::any;
thread_local! {
    // The default mock host is a memory-backed one, but the user can replace it if they like.
    pub static MOCK_HOST: RefCell<Box<dyn MockHost>> = RefCell::new(Box::new(super::mem::MemHost::new()));
}

// Replaces the current mock host with `mock`, returning the previous mock.
// This will panic if the MOCK_HOST is currently borrowed when called.
pub fn swap_mock_host(mut mock: Box<dyn MockHost>) -> Box<dyn MockHost> {
    MOCK_HOST.with(|h| core::mem::swap(&mut *h.borrow_mut(), &mut mock));
    mock
}

pub fn with_mock_host<H: MockHost + 'static, F: FnOnce(&mut H)>(f: F) {
    MOCK_HOST.with(|h| {
        f(h.borrow_mut()
            .as_mut_any()
            .downcast_mut::<H>()
            .expect("wrong MockHost type"))
    })
}

pub(crate) mod context {
    use super::MOCK_HOST;
    use crate::{Object, Val};
    pub(crate) unsafe fn log_value(v: Val) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().log_value(v))
    }
    pub(crate) unsafe fn get_last_operation_result() -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().get_last_operation_result())
    }
}

pub(crate) mod u64 {
    use super::MOCK_HOST;
    use crate::Object;
    pub(crate) unsafe fn from_u64(u: u64) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().u64_from_u64(u))
    }

    pub(crate) unsafe fn to_u64(u: Object) -> u64 {
        MOCK_HOST.with(|h| h.borrow_mut().u64_to_u64(u))
    }
}

pub(crate) mod i64 {
    use super::MOCK_HOST;
    use crate::Object;
    pub(crate) unsafe fn from_i64(i: i64) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().i64_from_i64(i))
    }

    pub(crate) unsafe fn to_i64(i: Object) -> i64 {
        MOCK_HOST.with(|h| h.borrow_mut().i64_to_i64(i))
    }
}

pub(crate) mod map {
    use super::MOCK_HOST;
    use crate::{Object, Val};
    pub(crate) unsafe fn new() -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().map_new())
    }
    pub(crate) unsafe fn put(m: Object, k: Val, v: Val) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().map_put(m, k, v))
    }

    pub(crate) unsafe fn get(m: Object, k: Val) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().map_get(m, k))
    }

    pub(crate) unsafe fn del(m: Object, k: Val) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().map_del(m, k))
    }

    pub(crate) unsafe fn len(m: Object) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().map_len(m))
    }

    pub(crate) unsafe fn keys(m: Object) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().map_keys(m))
    }

    pub(crate) unsafe fn has(m: Object, k: Val) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().map_has(m, k))
    }
}

pub(crate) mod vec {
    use super::MOCK_HOST;
    use crate::{Object, Val};
    pub(crate) unsafe fn new() -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_new())
    }

    pub(crate) unsafe fn put(v: Object, i: Val, x: Val) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_put(v, i, x))
    }

    pub(crate) unsafe fn get(v: Object, i: Val) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().vec_get(v, i))
    }

    pub(crate) unsafe fn del(v: Object, i: Val) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_del(v, i))
    }

    pub(crate) unsafe fn len(v: Object) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().vec_len(v))
    }

    pub(crate) unsafe fn push(v: Object, x: Val) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_push(v, x))
    }

    pub(crate) unsafe fn pop(v: Object) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_pop(v))
    }

    pub(crate) unsafe fn take(v: Object, n: Val) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_take(v, n))
    }

    pub(crate) unsafe fn drop(v: Object, n: Val) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_drop(v, n))
    }

    pub(crate) unsafe fn front(v: Object) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().vec_front(v))
    }

    pub(crate) unsafe fn back(v: Object) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().vec_back(v))
    }

    pub(crate) unsafe fn insert(v: Object, i: Val, x: Val) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_insert(v, i, x))
    }

    pub(crate) unsafe fn append(v1: Object, v2: Object) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().vec_append(v1, v2))
    }
}

pub(crate) mod ledger {
    use super::MOCK_HOST;
    use crate::{Object, Val};
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
        MOCK_HOST.with(|h| h.borrow_mut().pay(src, dst, asset, amount))
    }

    pub(crate) unsafe fn account_balance(acc: Object) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().account_balance(acc))
    }

    pub(crate) unsafe fn account_trust_line(acc: Object, asset: Object) -> Object {
        MOCK_HOST.with(|h| h.borrow_mut().account_trust_line(acc, asset))
    }

    pub(crate) unsafe fn trust_line_balance(trust_line: Object) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().trust_line_balance(trust_line))
    }

    pub(crate) unsafe fn put_contract_data(key: Val, val: Val) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().put_contract_data(key, val))
    }

    pub(crate) unsafe fn has_contract_data(key: Val) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().has_contract_data(key))
    }

    pub(crate) unsafe fn get_contract_data(key: Val) -> Val {
        MOCK_HOST.with(|h| h.borrow_mut().get_contract_data(key))
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
    use crate::{Object, Val};
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
