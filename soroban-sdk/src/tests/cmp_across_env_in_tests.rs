use crate::{self as soroban_sdk};
use soroban_sdk::{vec, Address, Bytes, BytesN, Env, Map, String, Vec};

#[test]
fn test_address() {
    let e1 = Env::default();
    let e2 = Env::default();

    // Two addresses with the same value should be comparable and equal even though they
    // belong to different environments.
    let a1 = Address::from_str(
        &e1,
        "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE",
    );
    let a2 = Address::from_str(
        &e2,
        "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA",
    );
    assert_ne!(a1, a2);

    // Two addresses with different values should be comparable and not equal even though they
    // belong to different environments.
    let a1 = Address::from_str(
        &e1,
        "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE",
    );
    let a2 = Address::from_str(
        &e2,
        "CA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQGAXE",
    );
    assert_eq!(a1, a2);
}

#[test]
fn test_string() {
    let e1 = Env::default();
    let e2 = Env::default();

    // Two strings with different values should be comparable and not equal even though they
    // belong to different environments.
    let s1 = String::from_str(&e1, "hello");
    let s2 = String::from_str(&e2, "world");
    assert_ne!(s1, s2);

    // Two strings with the same value should be comparable and equal even though they
    // belong to different environments.
    let s1 = String::from_str(&e1, "hello");
    let s2 = String::from_str(&e2, "hello");
    assert_eq!(s1, s2);
}

#[test]
fn test_bytes() {
    let e1 = Env::default();
    let e2 = Env::default();

    // Two bytes with different values should be comparable and not equal even though they
    // belong to different environments.
    let b1 = Bytes::from_slice(&e1, &[1, 2, 3]);
    let b2 = Bytes::from_slice(&e2, &[4, 5, 6]);
    assert_ne!(b1, b2);

    // Two bytes with the same value should be comparable and equal even though they
    // belong to different environments.
    let b1 = Bytes::from_slice(&e1, &[1, 2, 3]);
    let b2 = Bytes::from_slice(&e2, &[1, 2, 3]);
    assert_eq!(b1, b2);
}

#[test]
fn test_bytesn() {
    let e1 = Env::default();
    let e2 = Env::default();

    // Two bytesn with different values should be comparable and not equal even though they
    // belong to different environments.
    let b1 = BytesN::from_array(&e1, &[1, 2, 3]);
    let b2 = BytesN::from_array(&e2, &[4, 5, 6]);
    assert_ne!(b1, b2);

    // Two bytesn with the same value should be comparable and equal even though they
    // belong to different environments.
    let b1 = BytesN::from_array(&e1, &[1, 2, 3]);
    let b2 = BytesN::from_array(&e2, &[1, 2, 3]);
    assert_eq!(b1, b2);
}

#[test]
fn test_vec() {
    let e1 = Env::default();
    let e2 = Env::default();

    // Two vecs with different values should be comparable and not equal even though they
    // belong to different environments.
    let v1: Vec<i32> = vec![&e1, 1, 2, 3];
    let v2: Vec<i32> = vec![&e2, 4, 5, 6];
    assert_ne!(v1, v2);

    // Two vecs with the same value should be comparable and equal even though they
    // belong to different environments.
    let v1: Vec<i32> = vec![&e1, 1, 2, 3];
    let v2: Vec<i32> = vec![&e2, 1, 2, 3];
    assert_eq!(v1, v2);
}

#[test]
fn test_map() {
    let e1 = Env::default();
    let e2 = Env::default();

    // Two maps with different values should be comparable and not equal even though they
    // belong to different environments.
    let mut m1: Map<String, i32> = Map::new(&e1);
    m1.set(String::from_str(&e1, "a"), 1);
    m1.set(String::from_str(&e1, "b"), 2);

    let mut m2: Map<String, i32> = Map::new(&e2);
    m2.set(String::from_str(&e2, "c"), 3);
    m2.set(String::from_str(&e2, "d"), 4);

    assert_ne!(m1, m2);

    // Two maps with the same value should be comparable and equal even though they
    // belong to different environments.
    let mut m1: Map<String, i32> = Map::new(&e1);
    m1.set(String::from_str(&e1, "a"), 1);
    m1.set(String::from_str(&e1, "b"), 2);

    let mut m2: Map<String, i32> = Map::new(&e2);
    m2.set(String::from_str(&e2, "a"), 1);
    m2.set(String::from_str(&e2, "b"), 2);

    assert_eq!(m1, m2);
}
