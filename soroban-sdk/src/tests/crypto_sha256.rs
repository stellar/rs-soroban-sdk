use crate::{bytes, bytesn, Env};

#[test]
fn test_sha256() {
    let env = Env::default();

    let input = bytes!(&env, 0x01);
    let expect = bytesn!(
        &env,
        0x4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a
    );
    assert_eq!(env.crypto().sha256(&input), expect);
}
