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
    fn log_value(&self, v: Val) -> Val;
    fn get_last_operation_result(&self) -> Object;
    // ...
}

use std::cell::RefCell;
thread_local! {
    // The default mock host is a memory-backed one, but the user can replace it if they like.
    pub static MOCK_HOST: RefCell<Box<dyn MockHost>> = RefCell::new(Box::new(mem::MemHost::new()));
}

// This will panic if the MOCK_HOST is currently borrowed when called.
pub fn install_mock(mock: Box<dyn MockHost>) {
    MOCK_HOST.with(|h| *h.borrow_mut() = mock)
}

pub(crate) mod context {
    use super::MOCK_HOST;
    use crate::{Object, Val};
    pub(crate) unsafe fn log_value(v: Val) -> Val {
        MOCK_HOST.with(|h| h.borrow().log_value(v))
    }
    pub(crate) unsafe fn get_last_operation_result() -> Object {
        MOCK_HOST.with(|h| h.borrow().get_last_operation_result())
    }
    pub(crate) unsafe fn new() -> Object {
        todo!()
    }
}
pub(crate) mod map {
    use crate::{Object, Val};
    pub(crate) unsafe fn new() -> Object {
        todo!()
    }
    pub(crate) unsafe fn put(m: Object, k: Val, v: Val) -> Object {
        todo!()
    }

    pub(crate) unsafe fn get(m: Object, k: Val) -> Val {
        todo!()
    }

    pub(crate) unsafe fn del(m: Object, k: Val) -> Object {
        todo!()
    }

    pub(crate) unsafe fn len(m: Object) -> Val {
        todo!()
    }

    pub(crate) unsafe fn keys(m: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn has(m: Object, k: Val) -> Val {
        todo!()
    }
}

pub(crate) mod vec {
    use crate::{Object, Val};
    pub(crate) unsafe fn new() -> Object {
        todo!()
    }

    pub(crate) unsafe fn put(v: Object, i: Val, x: Val) -> Object {
        todo!()
    }

    pub(crate) unsafe fn get(v: Object, i: Val) -> Val {
        todo!()
    }

    pub(crate) unsafe fn del(v: Object, i: Val) -> Object {
        todo!()
    }

    pub(crate) unsafe fn len(v: Object) -> Val {
        todo!()
    }

    pub(crate) unsafe fn push(v: Object, x: Val) -> Object {
        todo!()
    }

    pub(crate) unsafe fn pop(v: Object) -> Object {
        todo!()
    }

    pub(crate) unsafe fn take(v: Object, n: Val) -> Object {
        todo!()
    }

    pub(crate) unsafe fn drop(v: Object, n: Val) -> Object {
        todo!()
    }

    pub(crate) unsafe fn front(v: Object) -> Val {
        todo!()
    }

    pub(crate) unsafe fn back(v: Object) -> Val {
        todo!()
    }

    pub(crate) unsafe fn insert(v: Object, i: Val, x: Val) -> Object {
        todo!()
    }

    pub(crate) unsafe fn append(v1: Object, v2: Object) -> Object {
        todo!()
    }
}

pub(crate) mod ledger {
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
        todo!()
    }

    pub(crate) unsafe fn has_contract_data(key: Val) -> Val {
        todo!()
    }

    pub(crate) unsafe fn get_contract_data(key: Val) -> Val {
        todo!()
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
