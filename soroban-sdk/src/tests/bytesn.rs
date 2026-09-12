use crate::{Bytes, BytesN, ConversionError, Env};

#[test]
#[cfg(target_pointer_width = "64")]
fn test_bytesn_try_from_bytes_length_exceeds_u32() {
    let env = Env::default();

    // A Bytes of length 0.
    let bytes = Bytes::new(&env);

    // Choose a length that truncates to the same length.
    const N: usize = u32::MAX as usize + 1;
    assert_eq!(N as u32, bytes.len());

    // Try to convert the bytes into the BytesN with the length that will truncate.
    let result = BytesN::<N>::try_from(bytes);

    // Reject the conversion even though the truncated length matches the bytes.
    assert_eq!(result, Err(ConversionError));
}

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

#[test]
fn test_bytesn_into_array() {
    let env = Env::default();
    let b: BytesN<4> = BytesN::from_array(&env, &[1, 2, 3, 4]);
    let a: [u8; 4] = b.into();
    assert_eq!(a, [1, 2, 3, 4]);
}

#[test]
fn test_bytesn_ref_into_array() {
    let env = Env::default();
    let b: BytesN<4> = BytesN::from_array(&env, &[1, 2, 3, 4]);
    let a: [u8; 4] = (&b).into();
    assert_eq!(a, [1, 2, 3, 4]);
}

#[test]
fn test_bytesn_into_array_zero_length() {
    let env = Env::default();
    let b: BytesN<0> = BytesN::from_array(&env, &[]);
    // The conversion of an empty BytesN just needs to not panic.
    let _: [u8; 0] = b.into();
}
