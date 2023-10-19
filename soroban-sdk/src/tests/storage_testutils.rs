use crate::{
    self as soroban_sdk,
    testutils::storage::{Instance as _, Persistent as _, Temporary as _},
    Map, Val,
};
use soroban_sdk::{contract, Env};

#[contract]
pub struct Contract;

#[test]
fn all() {
    let e = Env::default();
    let id = e.register_contract(None, Contract);

    e.as_contract(&id, || {
        e.storage().instance().set(&1, &2);
        e.storage().instance().set(&1, &true);
        e.storage().instance().set(&2, &3);
        e.storage().persistent().set(&10, &20);
        e.storage().persistent().set(&10, &false);
        e.storage().persistent().set(&20, &30);
        e.storage().temporary().set(&100, &200);
        e.storage().temporary().set(&100, &());
        e.storage().temporary().set(&200, &300);
    });

    e.as_contract(&id, || {
        assert_eq!(
            e.storage().instance().all(),
            Map::<Val, Val>::from_array(&e, [(1.into(), true.into()), (2.into(), 3.into())])
        );
        assert_eq!(
            e.storage().persistent().all(),
            Map::<Val, Val>::from_array(&e, [(10.into(), false.into()), (20.into(), 30.into())])
        );
        assert_eq!(
            e.storage().temporary().all(),
            Map::<Val, Val>::from_array(&e, [(100.into(), ().into()), (200.into(), 300.into())])
        );
    });
}
