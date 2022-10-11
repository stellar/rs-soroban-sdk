#![cfg(test)]

mod use_token_contract;

mod test_spec_xdr {
    use crate::Token;

    #[test]
    fn test_spec_xdr() {
        // it shouldn't panic
        let _ = Token::spec_xdr();
    }
}
