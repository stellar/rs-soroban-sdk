use crate::{self as soroban_sdk};
use crate::{bytes, vec, Env, Val, Vec};
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
        assert_eq!(e.prng().u64_in_range(0..=9), 5);
    });

    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);
    e.host().set_base_prng_seed([2; 32]).unwrap();
    e.as_contract(&id, || {
        e.prng().seed(bytes!(
            &e,
            0x0000000000000000000000000000000000000000000000000000000000000001
        ));
        assert_eq!(e.prng().u64_in_range(0..=9), 5);
    });
}

#[test]
fn test_prng_shuffle() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let v = vec![&e, 1, 2, 3];
        assert_eq!(e.prng().shuffle(v), vec![&e, 3, 2, 1].to_vals());
    });

    e.as_contract(&id, || {
        let v = Vec::<i64>::new(&e);
        assert_eq!(e.prng().shuffle(v), Vec::<Val>::new(&e).to_vals());
    });
}

#[test]
fn test_vec_shuffle() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        let v = vec![&e, 1, 2, 3];
        let s = v.shuffle();
        assert_eq!(s, vec![&e, 3, 2, 1]);
        assert_eq!(v, vec![&e, 1, 2, 3]);
    });

    e.as_contract(&id, || {
        let v = Vec::<i64>::new(&e);
        let s = v.shuffle();
        assert_eq!(s, vec![&e]);
        assert_eq!(v, vec![&e]);
    });
}

#[test]
fn test_prng_u64_in_range() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        assert_eq!(e.prng().u64_in_range(..), 15905370036469238889);
        assert_eq!(e.prng().u64_in_range(u64::MAX..), u64::MAX);
        assert_eq!(
            e.prng().u64_in_range(u64::MAX - 1..u64::MAX),
            18446744073709551614
        );
        assert_eq!(e.prng().u64_in_range(u64::MAX..=u64::MAX), u64::MAX);
        assert_eq!(e.prng().u64_in_range(0..1), 0);
        assert_eq!(e.prng().u64_in_range(0..=0), 0);
        assert_eq!(e.prng().u64_in_range(..=0), 0);
    });
}

#[test]
#[should_panic(expected = "low > high")]
fn test_prng_u64_in_range_panic_on_empty_range() {
    let e = Env::default();
    let id = e.register_contract(None, TestPrngContract);

    e.as_contract(&id, || {
        e.prng().u64_in_range(u64::MAX..u64::MAX);
    });
}
