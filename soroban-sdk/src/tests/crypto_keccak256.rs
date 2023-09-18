use crate::{bytesn, Env, IntoVal};

#[test]
fn test_keccak256() {
    let env = Env::default();

    let bytes = b"test vector for soroban".into_val(&env);
    let expect = bytesn!(
        &env,
        0x352fe2eaddf44eb02eb3eab1f8d6ff4ba426df4f1734b1e3f210d621ee8853d9
    );
    assert_eq!(env.crypto().keccak256(&bytes), expect);
}
