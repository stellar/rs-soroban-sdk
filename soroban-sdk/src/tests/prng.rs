use crate::{self as soroban_sdk, Bytes, BytesN};
use crate::{bytes, vec, Env, Vec};
use soroban_sdk::contract;

#[contract]
pub struct TestPrngContract;

#[test]
fn test_prng_seed() {
    let e = Env::default();
    e.host().set_base_prng_seed([0; 32]).unwrap();
    let id = e.register_contract(None, TestPrngContract);
    e.as_contract(&id, || {
        e.prng().seed(bytes!(
            &e,
            0x0000000000000000000000000000000000000000000000000000000000000001
        ));
        assert_eq!(e.prng().gen_range::<u64>(0..=9), 5);
    });

    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);
    e.host().set_base_prng_seed([2; 32]).unwrap();
    e.as_contract(&id, || {
        e.prng().seed(bytes!(
            &e,
            0x0000000000000000000000000000000000000000000000000000000000000001
        ));
        assert_eq!(e.prng().gen_range::<u64>(0..=9), 5);
    });
}

#[test]
fn test_prng_shuffle() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let v = vec![&e, 1, 2, 3];
        assert_eq!(v.to_shuffled(), vec![&e, 3, 2, 1]);
    });

    e.as_contract(&id, || {
        let v = Vec::<i64>::new(&e);
        assert_eq!(v.to_shuffled(), Vec::new(&e));
    });
}

#[test]
fn test_vec_shuffle() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let v = vec![&e, 1, 2, 3];
        let s = v.to_shuffled();
        assert_eq!(s, vec![&e, 3, 2, 1]);
        assert_eq!(v, vec![&e, 1, 2, 3]);
    });

    e.as_contract(&id, || {
        let v = Vec::<i64>::new(&e);
        let s = v.to_shuffled();
        assert_eq!(s, vec![&e]);
        assert_eq!(v, vec![&e]);
    });
}

#[test]
fn test_prng_fill_u64() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let mut v: u64 = 0;
        e.prng().fill(&mut v);
        assert_eq!(v, 15905370036469238889);
        e.prng().fill(&mut v);
        assert_eq!(v, 9820564573332354559);
    });
}

#[test]
fn test_prng_gen_u64() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        assert_eq!(e.prng().gen::<u64>(), 15905370036469238889);
        assert_eq!(e.prng().gen::<u64>(), 9820564573332354559);
    });
}

#[test]
fn test_prng_gen_range_u64() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        assert_eq!(e.prng().gen_range::<u64>(..), 15905370036469238889);
        assert_eq!(e.prng().gen_range::<u64>(u64::MAX..), u64::MAX);
        assert_eq!(
            e.prng().gen_range::<u64>(u64::MAX - 1..u64::MAX),
            18446744073709551614
        );
        assert_eq!(e.prng().gen_range::<u64>(u64::MAX..=u64::MAX), u64::MAX);
        assert_eq!(e.prng().gen_range::<u64>(0..1), 0);
        assert_eq!(e.prng().gen_range::<u64>(0..=0), 0);
        assert_eq!(e.prng().gen_range::<u64>(..=0), 0);
    });
}

#[test]
#[should_panic(expected = "Error(Value, InvalidInput)")]
fn test_prng_gen_range_u64_panic_on_invalid_range() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        e.prng().gen_range::<u64>(u64::MAX..u64::MAX);
    });
}

#[test]
fn test_prng_fill_bytes() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let mut v = Bytes::from_array(&e, &[0u8; 32]);
        e.prng().fill(&mut v);
        assert_eq!(
            v,
            Bytes::from_array(
                &e,
                &[
                    105, 12, 228, 36, 199, 57, 187, 220, 255, 181, 66, 167, 114, 167, 73, 136, 126,
                    229, 99, 124, 156, 9, 231, 42, 211, 148, 110, 234, 189, 179, 224, 119
                ]
            )
        );
        e.prng().fill(&mut v);
        assert_eq!(
            v,
            Bytes::from_array(
                &e,
                &[
                    12, 120, 166, 125, 4, 130, 72, 67, 232, 216, 155, 171, 240, 65, 91, 25, 149,
                    135, 147, 217, 131, 98, 2, 123, 78, 144, 194, 14, 36, 113, 79, 193
                ]
            )
        );
    });
}

#[test]
fn test_prng_gen_len_bytes() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        assert_eq!(
            e.prng().gen_len::<Bytes>(32),
            Bytes::from_array(
                &e,
                &[
                    105, 12, 228, 36, 199, 57, 187, 220, 255, 181, 66, 167, 114, 167, 73, 136, 126,
                    229, 99, 124, 156, 9, 231, 42, 211, 148, 110, 234, 189, 179, 224, 119
                ]
            )
        );
        assert_eq!(
            e.prng().gen_len::<Bytes>(32),
            Bytes::from_array(
                &e,
                &[
                    12, 120, 166, 125, 4, 130, 72, 67, 232, 216, 155, 171, 240, 65, 91, 25, 149,
                    135, 147, 217, 131, 98, 2, 123, 78, 144, 194, 14, 36, 113, 79, 193
                ]
            )
        );
    });
}

#[test]
fn test_prng_fill_bytesn() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let mut v = BytesN::from_array(&e, &[0u8; 32]);
        e.prng().fill(&mut v);
        assert_eq!(
            v,
            BytesN::from_array(
                &e,
                &[
                    105, 12, 228, 36, 199, 57, 187, 220, 255, 181, 66, 167, 114, 167, 73, 136, 126,
                    229, 99, 124, 156, 9, 231, 42, 211, 148, 110, 234, 189, 179, 224, 119
                ]
            )
        );
        e.prng().fill(&mut v);
        assert_eq!(
            v,
            BytesN::from_array(
                &e,
                &[
                    12, 120, 166, 125, 4, 130, 72, 67, 232, 216, 155, 171, 240, 65, 91, 25, 149,
                    135, 147, 217, 131, 98, 2, 123, 78, 144, 194, 14, 36, 113, 79, 193
                ]
            )
        );
    });
}

#[test]
fn test_prng_gen_bytesn() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        assert_eq!(
            e.prng().gen::<BytesN<32>>(),
            BytesN::from_array(
                &e,
                &[
                    105, 12, 228, 36, 199, 57, 187, 220, 255, 181, 66, 167, 114, 167, 73, 136, 126,
                    229, 99, 124, 156, 9, 231, 42, 211, 148, 110, 234, 189, 179, 224, 119
                ]
            )
        );
        assert_eq!(
            e.prng().gen::<BytesN<32>>(),
            BytesN::from_array(
                &e,
                &[
                    12, 120, 166, 125, 4, 130, 72, 67, 232, 216, 155, 171, 240, 65, 91, 25, 149,
                    135, 147, 217, 131, 98, 2, 123, 78, 144, 194, 14, 36, 113, 79, 193
                ]
            )
        );
    });
}

#[test]
fn test_prng_fill_slice() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let mut buf = [0u8; 32];
        let v: &mut [u8] = &mut buf;
        e.prng().fill(v);
        assert_eq!(
            v,
            [
                105, 12, 228, 36, 199, 57, 187, 220, 255, 181, 66, 167, 114, 167, 73, 136, 126,
                229, 99, 124, 156, 9, 231, 42, 211, 148, 110, 234, 189, 179, 224, 119
            ]
        );
        e.prng().fill(v);
        assert_eq!(
            v,
            [
                12, 120, 166, 125, 4, 130, 72, 67, 232, 216, 155, 171, 240, 65, 91, 25, 149, 135,
                147, 217, 131, 98, 2, 123, 78, 144, 194, 14, 36, 113, 79, 193
            ]
        );
    });
}

#[test]
fn test_prng_fill_array() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let mut v = [0u8; 32];
        e.prng().fill(&mut v);
        assert_eq!(
            v,
            [
                105, 12, 228, 36, 199, 57, 187, 220, 255, 181, 66, 167, 114, 167, 73, 136, 126,
                229, 99, 124, 156, 9, 231, 42, 211, 148, 110, 234, 189, 179, 224, 119
            ]
        );
        e.prng().fill(&mut v);
        assert_eq!(
            v,
            [
                12, 120, 166, 125, 4, 130, 72, 67, 232, 216, 155, 171, 240, 65, 91, 25, 149, 135,
                147, 217, 131, 98, 2, 123, 78, 144, 194, 14, 36, 113, 79, 193
            ]
        );
    });
}

#[test]
fn test_prng_gen_array() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        assert_eq!(
            e.prng().gen::<[u8; 32]>(),
            [
                105, 12, 228, 36, 199, 57, 187, 220, 255, 181, 66, 167, 114, 167, 73, 136, 126,
                229, 99, 124, 156, 9, 231, 42, 211, 148, 110, 234, 189, 179, 224, 119
            ]
        );
        assert_eq!(
            e.prng().gen::<[u8; 32]>(),
            [
                12, 120, 166, 125, 4, 130, 72, 67, 232, 216, 155, 171, 240, 65, 91, 25, 149, 135,
                147, 217, 131, 98, 2, 123, 78, 144, 194, 14, 36, 113, 79, 193
            ]
        );
    });
}
