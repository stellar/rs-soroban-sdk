use crate::{BytesN, Env};

#[test]
fn test_bytesn_is_empty_zero_length() {
    let env = Env::default();
    let b: BytesN<0> = BytesN::from_array(&env, &[]);
    assert_eq!(b.len(), 0);
    assert!(b.is_empty());
}

#[test]
fn test_bytesn_is_empty_nonzero_length() {
    let env = Env::default();
    let b: BytesN<4> = BytesN::from_array(&env, &[1, 2, 3, 4]);
    assert_eq!(b.len(), 4);
    assert!(!b.is_empty());
}
