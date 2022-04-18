#![no_std]
use sdk::Val;
use stellar_contract_sdk as sdk;

#[no_mangle]
pub fn log(v: Val) -> Val {
    sdk::log_value(v);
    Val::from_void()
}

#[cfg(test)]
mod test {
    use super::log;
    use sdk::{Symbol, Val};
    use stellar_contract_sdk as sdk;

    #[test]
    fn test_log() {
        log(Val::from_i64(7));
        log(Val::from_i64(-7));
        log(Symbol::from_str("asdf").into());
    }
}
