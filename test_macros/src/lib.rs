#![no_std]
use sdk::{OrAbort, Val};
use stellar_contract_sdk as sdk;
use stellar_contract_sdk_macros as sdkmacros;

#[sdkmacros::contractfn]
fn typed_fn() -> i32 {
    return 1;
}

#[sdkmacros::contractfn]
fn typed_fn_one(a: i32) -> i32 {
    return a;
}

#[sdkmacros::contractfn]
fn typed_fn_two(a: i32, b: i32) -> i32 {
    return a + b;
}

#[sdkmacros::contractfn]
fn typed_fn_val(v: Val, b: i32) -> Val {
    return Val::from_i32(v.as_i32() + b);
}

#[sdkmacros::contractfn]
fn typed_fn_try(a: i64) -> i64 {
    return a;
}

#[sdkmacros::contractfn]
fn default_fn() {}

#[sdkmacros::contractfn]
fn default_fn_one(_a: i32) {}

#[sdkmacros::contractfn]
fn default_fn_two(_a: i32, _b: i32) {}

#[sdkmacros::contractfn]
fn default_fn_val(_a: Val, _b: i32) {}

#[cfg(test)]
mod test {
    use super::{
        contractfn_default_fn, contractfn_default_fn_one, contractfn_default_fn_two,
        contractfn_default_fn_val, contractfn_typed_fn, contractfn_typed_fn_one,
        contractfn_typed_fn_try, contractfn_typed_fn_two, contractfn_typed_fn_val, default_fn,
        default_fn_one, default_fn_two, default_fn_val, typed_fn, typed_fn_one, typed_fn_try,
        typed_fn_two, typed_fn_val,
    };
    use sdk::Val;
    use stellar_contract_sdk as sdk;

    #[test]
    fn test_typed_fn() {
        assert_eq!(typed_fn(), 1);
        assert_eq!(contractfn_typed_fn(), Val::from_i32(1));
    }

    #[test]
    fn test_typed_fn_one() {
        assert_eq!(typed_fn_one(2), 2);
        assert_eq!(contractfn_typed_fn_one(Val::from_i32(2)), Val::from_i32(2));
    }

    #[test]
    fn test_typed_fn_two() {
        assert_eq!(typed_fn_two(2, 4), 6);
        assert_eq!(
            contractfn_typed_fn_two(Val::from_i32(2), Val::from_i32(4)),
            Val::from_i32(6)
        );
    }

    #[test]
    fn test_typed_fn_val() {
        assert_eq!(typed_fn_val(Val::from_i32(2), 4), Val::from_i32(6));
        assert_eq!(
            contractfn_typed_fn_val(Val::from_i32(2), Val::from_i32(4)),
            Val::from_i32(6)
        );
    }

    #[test]
    fn test_typed_fn_try() {
        assert_eq!(typed_fn_try(2), 2);
        assert_eq!(contractfn_typed_fn_try(Val::from_u63(2)), Val::from_u63(2));
    }

    #[test]
    fn test_default_fn() {
        assert_eq!(default_fn(), ());
        assert_eq!(contractfn_default_fn(), Val::from_void());
    }

    #[test]
    fn test_default_fn_one() {
        assert_eq!(default_fn_one(2), ());
        assert_eq!(
            contractfn_default_fn_one(Val::from_i32(2)),
            Val::from_void()
        );
    }

    #[test]
    fn test_default_fn_two() {
        assert_eq!(default_fn_two(2, 4), ());
        assert_eq!(
            contractfn_default_fn_two(Val::from_i32(2), Val::from_i32(4)),
            Val::from_void()
        );
    }

    #[test]
    fn test_default_fn_val() {
        assert_eq!(default_fn_val(Val::from_i32(2), 4), ());
        assert_eq!(
            contractfn_default_fn_val(Val::from_i32(2), Val::from_i32(4)),
            Val::from_void()
        );
    }
}
