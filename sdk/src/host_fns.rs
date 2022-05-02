// Most host functions have strong contractual guarantees from the host: they
// will either return the correctly-typed objects here, or they will trap. So we
// do not need to check return types, and we can even unsafely downcast Vals to
// specific subtypes if we know them.
//
// (Recall that there must be no way for the guest to corrupt the host even if
// the guest does receive unexpected objects -- at worst the host can corrupt or
// confuse the guest this way, but the guest can never defend itself against a
// malicious host anyways.)
//
// We do this mostly to minimize codesize, and also reduce the chance of users
// missing an error in a contract binding to another language that doesn't have
// Result<> types. Every error that can trap this way typically has a way to
// avoid it by checking some value first (eg. check a map to see if it contains
// a value before getting it).
//
// The exceptions are ledger-interaction calls and especially cross-contract
// calls: these we project into a Result<> because they are fairly failure-prone
// and impossible to guard against failure of, typically. We assume users might
// wish to contain these and, in general, that users won't be doing a _ton_ of
// them so it's ok that they are a little more expensive code-size-wise.

// General context-access functions live in the wasm 'x' module.
pub(crate) mod context {
    use crate::{Object, Val};
    #[link(wasm_import_module = "x")]
    extern "C" {

        // link names are chosen to be (a) unrepresentable as rust identifiers so
        // they cannot collide with exported user functions and (b) very short, so
        // they do not take up a lot of space in import tables. They consist of a $
        // symbol followed by a single character category identifier and a single
        // character function identifier, each drawn from the same 64-character
        // repertoire as symbol: [_0-9A-Za-z], in that order. If we ever need more
        // than 64 functions within a category we can just overflow to 2-character
        // function identifiers; if we ever need more than 64 categories, we can
        // pick a different prefix-char for the category-overflow space; both of
        // these possibilities seem unlikely at present, but either way they're
        // fairly straightforward.
        #[link_name = "$_"]
        pub(crate) fn log_value(v: Val) -> Val;

        // Fetches an OpResult object for inspection, in the rare case the user
        // wants more detail than is conveyed in a simple Status.
        #[link_name = "$0"]
        pub(crate) fn get_last_operation_result() -> Object;
    }
}

// U64 functions live in the wasm 'u' module
pub(crate) mod u64 {
    use crate::Object;
    #[link(wasm_import_module = "u")]
    extern "C" {
        #[link_name = "$_"]
        pub(crate) fn from_u64(x: u64) -> Object;
        #[link_name = "$0"]
        pub(crate) fn to_u64(x: Object) -> u64;
    }
}

// I64 functions live in the wasm 'i' module
pub(crate) mod i64 {
    use crate::Object;
    #[link(wasm_import_module = "i")]
    extern "C" {
        #[link_name = "$_"]
        pub(crate) fn from_i64(x: i64) -> Object;
        #[link_name = "$0"]
        pub(crate) fn to_i64(x: Object) -> i64;
    }
}

// Map functions live in the wasm 'm' module
pub(crate) mod map {
    use crate::{Object, Val};
    #[link(wasm_import_module = "m")]
    extern "C" {
        #[link_name = "$_"]
        pub(crate) fn new() -> Object;
        #[link_name = "$0"]
        pub(crate) fn put(m: Object, k: Val, v: Val) -> Object;
        #[link_name = "$1"]
        pub(crate) fn get(m: Object, k: Val) -> Val;
        #[link_name = "$2"]
        pub(crate) fn del(m: Object, k: Val) -> Object;
        #[link_name = "$3"]
        pub(crate) fn len(m: Object) -> Val;
        #[link_name = "$4"]
        pub(crate) fn keys(m: Object) -> Object;
        #[link_name = "$5"]
        pub(crate) fn has(m: Object, k: Val) -> Val;
    }
}

// Vec functions live in the wasm 'v' module
pub(crate) mod vec {
    use crate::{Object, Val};
    #[link(wasm_import_module = "v")]
    extern "C" {
        #[link_name = "$_"]
        pub(crate) fn new() -> Object;
        #[link_name = "$0"]
        pub(crate) fn put(v: Object, i: Val, x: Val) -> Object;
        #[link_name = "$1"]
        pub(crate) fn get(v: Object, i: Val) -> Val;
        #[link_name = "$2"]
        pub(crate) fn del(v: Object, i: Val) -> Object;
        #[link_name = "$3"]
        pub(crate) fn len(v: Object) -> Val;

        #[link_name = "$4"]
        pub(crate) fn push(v: Object, x: Val) -> Object;
        #[link_name = "$5"]
        pub(crate) fn pop(v: Object) -> Object;
        #[link_name = "$6"]
        pub(crate) fn take(v: Object, n: Val) -> Object;
        #[link_name = "$7"]
        pub(crate) fn drop(v: Object, n: Val) -> Object;
        #[link_name = "$8"]
        pub(crate) fn front(v: Object) -> Val;
        #[link_name = "$9"]
        pub(crate) fn back(v: Object) -> Val;
        #[link_name = "$A"]
        pub(crate) fn insert(v: Object, i: Val, x: Val) -> Object;
        #[link_name = "$B"]
        pub(crate) fn append(v1: Object, v2: Object) -> Object;
    }
}

// Ledger functions live in the wasm 'l' module
pub(crate) mod ledger {
    use crate::{Object, Val};
    #[link(wasm_import_module = "l")]
    extern "C" {
        #[link_name = "$_"]
        pub(crate) fn get_current_ledger_num() -> Val;

        // NB: this returns a raw/unboxed u64, not a Val union.
        #[link_name = "$0"]
        pub(crate) fn get_current_ledger_close_time() -> u64;

        // NB: returns a Status; details can be fetched with
        // get_last_operation_result.
        #[link_name = "$1"]
        pub(crate) fn pay(src: Object, dst: Object, asset: Object, amount: Val) -> Val;

        #[link_name = "$2"]
        pub(crate) fn put_contract_data(key: Val, val: Val) -> Val;
        #[link_name = "$3"]
        pub(crate) fn has_contract_data(key: Val) -> Val;
        #[link_name = "$4"]
        pub(crate) fn get_contract_data(key: Val) -> Val;
        #[link_name = "$5"]
        pub(crate) fn del_contract_data(key: Val) -> Val;

        #[link_name = "$6"]
        pub(crate) fn account_balance(acc: Object) -> Val;

        #[link_name = "$7"]
        pub(crate) fn account_trust_line(acc: Object, asset: Object) -> Object;

        #[link_name = "$8"]
        pub(crate) fn trust_line_balance(tl: Object) -> Val;
    }
}

// Cross-contract functions live in the wasm 'c' module
pub(crate) mod call {
    use crate::Val;
    #[link(wasm_import_module = "c")]
    extern "C" {
        // NB: returns callee-return-value-or-Status; details can be fetched with
        // get_last_operation_result.
        //
        // TODO: possibly revisit this since it adds ambiguity to whether callee
        // successfully returned a status, or call failed and failure _generated_ a
        // status. Possibly this distinction is too fussy to disambiguate.
        #[link_name = "$_"]
        pub(crate) fn call0(contract: Val, func: Val) -> Val;
        #[link_name = "$0"]
        pub(crate) fn call1(contract: Val, func: Val, a: Val) -> Val;
        #[link_name = "$1"]
        pub(crate) fn call2(contract: Val, func: Val, a: Val, b: Val) -> Val;
        #[link_name = "$2"]
        pub(crate) fn call3(contract: Val, func: Val, a: Val, b: Val, c: Val) -> Val;
        #[link_name = "$3"]
        pub(crate) fn call4(contract: Val, func: Val, a: Val, b: Val, c: Val, d: Val) -> Val;
    }
}

// BigNum functions live in the wasm 'b' module
pub(crate) mod bignum {
    use crate::{Object, Val};
    #[link(wasm_import_module = "b")]
    extern "C" {
        #[link_name = "$_"]
        pub(crate) fn from_u64(x: u64) -> Object;
        #[link_name = "$0"]
        pub(crate) fn add(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$1"]
        pub(crate) fn sub(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$2"]
        pub(crate) fn mul(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$3"]
        pub(crate) fn div(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$4"]
        pub(crate) fn rem(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$5"]
        pub(crate) fn and(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$6"]
        pub(crate) fn or(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$7"]
        pub(crate) fn xor(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$8"]
        pub(crate) fn shl(lhs: Object, rhs: u64) -> Object;
        #[link_name = "$9"]
        pub(crate) fn shr(lhs: Object, rhs: u64) -> Object;
        #[link_name = "$A"]
        pub(crate) fn cmp(lhs: Object, rhs: Object) -> Val;
        #[link_name = "$B"]
        pub(crate) fn is_zero(x: Object) -> Val;
        #[link_name = "$C"]
        pub(crate) fn neg(x: Object) -> Object;
        #[link_name = "$D"]
        pub(crate) fn not(x: Object) -> Object;
        #[link_name = "$E"]
        pub(crate) fn gcd(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$F"]
        pub(crate) fn lcm(lhs: Object, rhs: Object) -> Object;
        #[link_name = "$G"]
        pub(crate) fn pow(lhs: Object, rhs: u64) -> Object;
        #[link_name = "$H"]
        pub(crate) fn pow_mod(p: Object, q: Object, m: Object) -> Object;
        #[link_name = "$I"]
        pub(crate) fn sqrt(x: Object) -> Object;
        #[link_name = "$J"]
        pub(crate) fn bits(x: Object) -> u64;
        #[link_name = "$K"]
        pub(crate) fn to_u64(x: Object) -> u64;
        #[link_name = "$L"]
        pub(crate) fn to_i64(x: Object) -> i64;
        #[link_name = "$M"]
        pub(crate) fn from_i64(x: i64) -> Object;
    }
}
