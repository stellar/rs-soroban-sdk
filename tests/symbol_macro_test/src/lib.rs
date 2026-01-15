#![no_std]

mod example_contract;

#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Symbol, symbol, symbol_short};

    #[test]
    fn test_symbol_macro_short() {
        let env = Env::default();
        
        // Test short symbol (≤9 chars) - should use compile-time optimization
        let short_sym = symbol!(&env, "short");
        let expected = symbol_short!("short");
        
        // These should be equivalent - compare the symbols directly
        assert_eq!(short_sym, expected);
    }

    #[test]
    fn test_symbol_macro_long() {
        let env = Env::default();
        
        // Test long symbol (>9 chars) - should use runtime host function  
        let long_sym = symbol!(&env, "this_is_a_long_symbol");
        let expected = Symbol::new(&env, "this_is_a_long_symbol");
        
        // These should be equivalent - compare the symbols directly
        assert_eq!(long_sym, expected);
    }

    #[test]
    fn test_symbol_macro_edge_cases() {
        let env = Env::default();
        
        // Test exactly 9 characters (still short)
        let nine_chars = symbol!(&env, "exactly_9");
        let expected_nine = symbol_short!("exactly_9");
        assert_eq!(nine_chars, expected_nine);
        
        // Test exactly 10 characters (becomes long)
        let ten_chars = symbol!(&env, "exactly_10");
        let expected_ten = Symbol::new(&env, "exactly_10");
        assert_eq!(ten_chars, expected_ten);
    }

    #[test]
    fn test_symbol_macro_valid_chars() {
        let env = Env::default();
        
        // Test all valid character types
        let mixed = symbol!(&env, "aB3_test");
        let expected = Symbol::new(&env, "aB3_test");
        assert_eq!(mixed, expected);
    }

    #[test]
    fn test_symbol_macro_compile_time_validation() {
        let env = Env::default();
        
        // These should all compile without errors as they use valid symbols
        let _valid1 = symbol!(&env, "valid");
        let _valid2 = symbol!(&env, "Valid123");
        let _valid3 = symbol!(&env, "valid_underscore");
        let _valid4 = symbol!(&env, "a");
        let _valid5 = symbol!(&env, "abcdefghi"); // exactly 9 chars
        let _valid6 = symbol!(&env, "abcdefghij"); // exactly 10 chars
    }
}