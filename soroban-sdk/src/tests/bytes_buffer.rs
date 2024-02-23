use crate::{Bytes, Env};

#[test]
fn test_bytes_buffer() {
    let env = Env::default();

    let bytes = Bytes::from_slice(&env, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    assert_eq!(
        &[1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        bytes.to_buffer::<1024>().as_slice(),
    );
}

#[test]
#[should_panic(expected = "range end index 10 out of range for slice of length 9")]
fn test_bytes_buffer_panic() {
    let env = Env::default();

    let bytes = Bytes::from_slice(&env, &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    let _ = bytes.to_buffer::<9>();
}
