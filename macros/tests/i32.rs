use stellar_contract_macros::contractfn;

// TODO: Test spec generation.

#[contractfn]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_i32() {
    panic!()
}

#[test]
fn test_add() {
    let e = Env::default();
    let x = 10i32.into_val(&e);
    let y = 12i32.into_val(&e);
    let z = __add(e.clone(), x, y);
    let z = i32::try_from_val(&e, z).unwrap();
    assert!(z == 22);
}
