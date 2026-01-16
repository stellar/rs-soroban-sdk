use crate::{
    self as soroban_sdk, contract, contractevent, contracttype, map, symbol_short,
    testutils::Events as _, vec, Env, IntoVal, Map, String, Symbol, Val, Vec,
};

#[test]
fn test_defaults() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        value: Symbol,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
            value: symbol_short!("hello"),
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
                // Expect this event body.
                map![
                    &env,
                    (
                        symbol_short!("value"),
                        <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("hello"), &env),
                    ),
                ]
                .into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_prefix_topics() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(topics = ["topic1", "topic2"])]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        value: Symbol,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
            value: symbol_short!("hello"),
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (
                    symbol_short!("topic1"),
                    symbol_short!("topic2"),
                    symbol_short!("hi")
                )
                    .into_val(&env),
                // Expect this event body.
                map![
                    &env,
                    (
                        symbol_short!("value"),
                        <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("hello"), &env),
                    ),
                ]
                .into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_no_prefix_topics() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(topics = [])]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        value: Symbol,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
            value: symbol_short!("hello"),
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("hi"),).into_val(&env),
                // Expect this event body.
                map![
                    &env,
                    (
                        symbol_short!("value"),
                        <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("hello"), &env),
                    ),
                ]
                .into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_no_topics() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(topics = [])]
    pub struct MyEvent {
        value: Symbol,
    }

    env.as_contract(&id, || {
        MyEvent {
            value: symbol_short!("hello"),
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                ().into_val(&env),
                // Expect this event body.
                map![
                    &env,
                    (
                        symbol_short!("value"),
                        <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("hello"), &env),
                    ),
                ]
                .into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_no_topics_no_data() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(topics = [])]
    pub struct MyEvent {}

    env.as_contract(&id, || {
        MyEvent {}.publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                ().into_val(&env),
                // Expect this event body.
                Map::<Symbol, Val>::new(&env).into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_data_single_value() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(data_format = "single-value")]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        value: Symbol,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
            value: symbol_short!("yo"),
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
                // Expect this event body.
                symbol_short!("yo").into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_data_single_value_no_data() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(data_format = "single-value")]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
                // Expect this event body.
                ().into_val(&env),
            ),
        ],
    );
}

#[test]
fn test_data_vec() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contracttype]
    pub struct MyType1 {
        value: u32,
    }

    #[contracttype]
    pub struct MyType2(u32);

    #[contracttype]
    pub enum MyType3 {
        Value(u32),
    }

    #[contracttype]
    pub enum MyType4 {
        Value = 4,
    }

    #[contractevent(data_format = "vec")]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        value: Symbol,
        value2: u32,
        value3: Vec<u32>,
        value4: String,
        value5: MyType1,
        value6: MyType2,
        value7: MyType3,
        value8: MyType4,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
            value: symbol_short!("yo"),
            value2: 2,
            value3: Vec::new(&env),
            value4: String::from_str(&env, "asdf"),
            value5: MyType1 { value: 1 },
            value6: MyType2(2),
            value7: MyType3::Value(3),
            value8: MyType4::Value,
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
                // Expect this event body.
                (
                    symbol_short!("yo"),
                    2u32,
                    Vec::<u32>::new(&env),
                    String::from_str(&env, "asdf"),
                    MyType1 { value: 1 },
                    MyType2(2),
                    MyType3::Value(3),
                    MyType4::Value,
                )
                    .into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_data_vec_no_data() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(data_format = "vec")]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
                // Expect this event body.
                Vec::<Val>::new(&env).into_val(&env),
            ),
        ],
    );
}

#[test]
fn test_data_vec_preserves_field_order() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    // Fields are named so that alphabetical order differs from declaration order
    #[contractevent(data_format = "vec")]
    pub struct Deposit {
        #[topic]
        addr: Symbol,
        time: u64,
        amount: i128,
    }

    let event = Deposit {
        addr: symbol_short!("user"),
        time: 1000u64,
        amount: 500i128,
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let data: Val = (1000u64, 500i128).into_val(&env);
    let expected_event = xdr::ContractEvent {
        ext: xdr::ExtensionPoint::V0,
        type_: xdr::ContractEventType::Contract,
        contract_id: Some(id.contract_id()),
        body: xdr::ContractEventBody::V0(xdr::ContractEventV0 {
            topics: vec![&env, symbol_short!("deposit"), symbol_short!("user")].into(),
            data: xdr::ScVal::try_from_val(&env, &data).unwrap(),
        }),
    };
    assert_eq!(env.events().all(), std::vec![expected_event.clone()],);
    assert_eq!(event.to_xdr(&env, &id), expected_event);
}

#[test]
fn test_data_map() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(data_format = "map")]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        value: Symbol,
        value2: u32,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
            value: symbol_short!("yo"),
            value2: 2,
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
                // Expect this event body.
                map![
                    &env,
                    (
                        symbol_short!("value"),
                        <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("yo"), &env),
                    ),
                    (
                        symbol_short!("value2"),
                        <_ as IntoVal<Env, Val>>::into_val(&2u32, &env),
                    ),
                ]
                .into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_data_map_no_data() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(data_format = "map")]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: symbol_short!("hi"),
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
                // Expect this event body.
                Map::<Symbol, Val>::new(&env).into_val(&env)
            ),
        ],
    );
}

#[test]
fn test_data_map_sorts_fields() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(data_format = "map")]
    pub struct Deposit {
        #[topic]
        addr: Symbol,
        time: u64,
        amount: i128,
    }

    let event = Deposit {
        addr: symbol_short!("user"),
        time: 1000u64,
        amount: 500i128,
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let data: Val = map![
        &env,
        (
            symbol_short!("amount"),
            <_ as IntoVal<Env, Val>>::into_val(&500i128, &env),
        ),
        (
            symbol_short!("time"),
            <_ as IntoVal<Env, Val>>::into_val(&1000u64, &env),
        ),
    ]
    .into_val(&env);
    let expected_event = xdr::ContractEvent {
        ext: xdr::ExtensionPoint::V0,
        type_: xdr::ContractEventType::Contract,
        contract_id: Some(id.contract_id()),
        body: xdr::ContractEventBody::V0(xdr::ContractEventV0 {
            topics: vec![&env, symbol_short!("deposit"), symbol_short!("user")].into(),
            data: xdr::ScVal::try_from_val(&env, &data).unwrap(),
        }),
    };
    assert_eq!(env.events().all(), std::vec![expected_event.clone()],);
    assert_eq!(event.to_xdr(&env, &id), expected_event);
}

#[test]
fn test_ref_fields() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent]
    pub struct MyEvent<'a> {
        #[topic]
        name: &'a Symbol,
        value: &'a Symbol,
        value2: u32,
    }

    env.as_contract(&id, || {
        MyEvent {
            name: &symbol_short!("hi"),
            value: &symbol_short!("yo"),
            value2: 2,
        }
        .publish(&env);
    });

    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                id,
                // Expect these event topics.
                (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
                // Expect this event body.
                map![
                    &env,
                    (
                        symbol_short!("value"),
                        <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("yo"), &env),
                    ),
                    (
                        symbol_short!("value2"),
                        <_ as IntoVal<Env, Val>>::into_val(&2u32, &env),
                    ),
                ]
                .into_val(&env)
            ),
        ],
    );
}
