// This module implements the host interface using a fake in-memory thread-local
// structure. It exists for running unit tests in contracts, when they do not
// even want to compile to wasm and run against the sandbox. It is not an exact
// copy of either the sandbox or the real implementation in stellar-core, but it
// has semantics that are as close as possible.

#![allow(unused_variables)]

pub mod fs;
pub mod mem;

use crate::{Object, Val};

pub trait MockHost {
    fn log_value(&mut self, v: Val) -> Val;
    fn get_last_operation_result(&mut self) -> Object;

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

    fn get_contract_data(&mut self, k: Val) -> Val;
    fn put_contract_data(&mut self, k: Val, v: Val) -> Val;
    fn has_contract_data(&mut self, k: Val) -> Val;
    //...
}

use core::cell::RefCell;
thread_local! {
    // The default mock host is a memory-backed one, but the user can replace it if they like.
    pub static MOCK_HOST: RefCell<Box<dyn MockHost>> = RefCell::new(Box::new(mem::MemHost::new()));
}

// Replaces the current mock host with `mock`, returning the previous mock.
// This will panic if the MOCK_HOST is currently borrowed when called.
pub fn swap_mock_host(mut mock: Box<dyn MockHost>) -> Box<dyn MockHost> {
    MOCK_HOST.with(|h| core::mem::swap(&mut *h.borrow_mut(), &mut mock));
    mock
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
    use crate::Val;
    pub(crate) unsafe fn get_current_ledger_num() -> Val {
        todo!()
    }

    // NB: this returns a raw/unboxed u64, not a Val union.

    pub(crate) unsafe fn get_current_ledger_close_time() -> u64 {
        todo!()
    }

    // NB: returns a Status; details can be fetched with
    // get_last_operation_result.

    pub(crate) unsafe fn pay(src: Val, dst: Val, asset: Val, amount: Val) -> Val {
        todo!()
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
}
