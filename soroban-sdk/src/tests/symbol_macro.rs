use crate::{Env, Symbol};

#[test]
fn test_symbol_macro_short() {
    let env = Env::default();
    
    // Test short symbol (≤9 chars) - should use compile-time optimization
    let short_sym = crate::symbol!(&env, "short");
    let expected = crate::symbol_short!("short");
    assert_eq!(short_sym.to_val(), expected.to_val());
}

#[test]
fn test_symbol_macro_long() {
    let env = Env::default();
    
    // Test long symbol (>9 chars) - should use runtime host function
    let long_sym = crate::symbol!(&env, "this_is_a_long_symbol");
    let expected = Symbol::new(&env, "this_is_a_long_symbol");
    assert_eq!(long_sym.to_val(), expected.to_val());
}

#[test]
fn test_symbol_macro_edge_cases() {
    let env = Env::default();
    
    // Test exactly 9 characters (still short)
    let nine_chars = crate::symbol!(&env, "exactly_9");
    let expected_nine = crate::symbol_short!("exactly_9");
    assert_eq!(nine_chars.to_val(), expected_nine.to_val());
    
    // Test exactly 10 characters (becomes long)
    let ten_chars = crate::symbol!(&env, "exactly_10");
    let expected_ten = Symbol::new(&env, "exactly_10");
    assert_eq!(ten_chars.to_val(), expected_ten.to_val());
}

#[test]
fn test_symbol_macro_valid_chars() {
    let env = Env::default();
    
    // Test all valid character types
    let mixed = crate::symbol!(&env, "aB3_test");
    let expected = Symbol::new(&env, "aB3_test");
    assert_eq!(mixed.to_val(), expected.to_val());
}

#[test]
fn test_symbol_macro_max_length() {
    let env = Env::default();
    
    // Test maximum length (32 chars)
    let max_len = crate::symbol!(&env, "this_symbol_is_exactly_32_chars!");
    let expected = Symbol::new(&env, "this_symbol_is_exactly_32_chars!");
    assert_eq!(max_len.to_val(), expected.to_val());
}