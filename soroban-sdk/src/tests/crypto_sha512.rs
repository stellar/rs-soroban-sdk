use crate::{bytes, bytesn, Bytes, BytesN, Env};

#[test]
fn test_sha512() {
    let env = Env::default();

    let input = bytes!(&env, 0x01);
    let expect = bytesn!(
        &env,
        0x7b54b66836c1fbdd13d2441d9e1434dc62ca677fb68f5fe66a464baadecdbd00576f8d6b5ac3bcc80844b7d50b1cc6603444bbe7cfcf8fc0aa1ee3c636d9e339
    );
    let hash: BytesN<64> = env.crypto().sha512(&input).into();
    assert_eq!(hash, expect);
}

#[test]
fn test_sha512_empty() {
    let env = Env::default();

    let input = bytes!(&env);
    let expect = bytesn!(
        &env,
        0xcf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e
    );
    let hash: BytesN<64> = env.crypto().sha512(&input).into();
    assert_eq!(hash, expect);
}

#[test]
fn test_sha512_string() {
    let env = Env::default();

    let input = Bytes::from_slice(&env, "test vector for soroban".as_bytes());
    let expect = bytesn!(
        &env,
        0x6d4e7ca8020ae8d9da82c3f64b7bf78ebab725e7b9a65861ff301eac292dd240e775f33734e4a9c70a242456ac4e19c2ab43a1f4e4f102c4e5a8ad2dc7297341
    );
    let hash: BytesN<64> = env.crypto().sha512(&input).into();
    assert_eq!(hash, expect);
}
