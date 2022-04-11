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
fn default_fn() {}

#[sdkmacros::contractfn]
fn default_fn_one(_a: i32) {}

#[sdkmacros::contractfn]
fn default_fn_two(_a: i32, _b: i32) {}

#[cfg(test)]
mod test {
    use super::{
        __cf_default_fn, __cf_default_fn_one, __cf_default_fn_two, __cf_typed_fn,
        __cf_typed_fn_one, __cf_typed_fn_two, default_fn, default_fn_one, default_fn_two, typed_fn,
        typed_fn_one, typed_fn_two,
    };
    use sdk::Val;
    use stellar_contract_sdk as sdk;

    #[test]
    fn test_typed_fn() {
        assert_eq!(typed_fn(), 1);
        assert_eq!(__cf_typed_fn(), Val::from_i32(1));
    }

    #[test]
    fn test_typed_fn_one() {
        assert_eq!(typed_fn_one(2), 2);
        assert_eq!(__cf_typed_fn_one(Val::from_i32(2)), Val::from_i32(2));
    }

    #[test]
    fn test_typed_fn_two() {
        assert_eq!(typed_fn_two(2, 4), 6);
        assert_eq!(
            __cf_typed_fn_two(Val::from_i32(2), Val::from_i32(4)),
            Val::from_i32(6)
        );
    }

    #[test]
    fn test_default_fn() {
        assert_eq!(default_fn(), ());
        assert_eq!(__cf_default_fn(), Val::from_void());
    }

    #[test]
    fn test_default_fn_one() {
        assert_eq!(default_fn_one(2), ());
        assert_eq!(__cf_default_fn_one(Val::from_i32(2)), Val::from_void());
    }

    #[test]
    fn test_default_fn_two() {
        assert_eq!(default_fn_two(2, 4), ());
        assert_eq!(
            __cf_default_fn_two(Val::from_i32(2), Val::from_i32(4)),
            Val::from_void()
        );
    }
}
