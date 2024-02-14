#![cfg(feature = "alloc")]

use crate::{Bytes, Env};

#[test]
fn test_bytes_alloc_vec() {
    let env = Env::default();

    let bytes = Bytes::from_slice(&env, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    assert_eq!(
        &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        bytes.to_alloc_vec().as_slice()
    );
}
