use crate::{
    self as soroban_sdk, contract, contractevent, contracttype, map, symbol_short,
    testutils::{Address as _, Events as _},
    vec, Address, Env, Event, IntoVal, Map, String, Symbol, Val, Vec,
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

#[test]
fn test_event_matches_contract_id() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());
    let id_2 = env.register(Contract, ());

    #[contractevent]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        value: Symbol,
    }

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("hello"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let published = env.events().all().get_unchecked(0);
    assert!(!event.matches(&env, &id_2, &published));
    assert!(event.matches(&env, &id, &published));
}

#[test]
fn test_event_matches_topics() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        #[topic]
        value: Symbol,
    }

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("hello"),
    };
    let event_2 = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("world"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let published = env.events().all().get_unchecked(0);
    assert!(!event_2.matches(&env, &id, &published));
    assert!(event.matches(&env, &id, &published));
}

#[test]
fn test_event_matches_data_host() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        value: Address,
    }

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: Address::generate(&env),
    };
    let event_2 = MyEvent {
        name: symbol_short!("hi"),
        value: Address::generate(&env),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let published = env.events().all().get_unchecked(0);
    assert!(!event_2.matches(&env, &id, &published));
    assert!(event.matches(&env, &id, &published));
}

#[test]
fn test_event_matches_data_host_and_small() {
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

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: Symbol::new(&env, "i_am_too_long_for_symbol_short"),
    };
    let event_2 = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("hello"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let published = env.events().all().get_unchecked(0);
    assert!(!event_2.matches(&env, &id, &published));
    assert!(event.matches(&env, &id, &published));
}

#[test]
fn test_event_matches_data_small() {
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

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("world"),
    };
    let event_2 = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("hello"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let published = env.events().all().get_unchecked(0);
    assert!(!event_2.matches(&env, &id, &published));
    assert!(event.matches(&env, &id, &published));
}

#[test]
fn test_event_contains_no_events() {
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

    let event = MyEvent {
        name: symbol_short!("hello"),
        value: symbol_short!("world"),
    };

    assert!(!env.events().contains(&id, &event));
}

#[test]
fn test_event_contains() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent]
    pub struct MyEvent1 {
        #[topic]
        name: Symbol,
        #[topic]
        address: Address,
        value: Symbol,
        amount: i128,
    }

    #[contractevent]
    pub struct MyEvent2 {
        #[topic]
        name: Symbol,
        #[topic]
        address: Address,
        value: Symbol,
        amount: i128,
    }

    let addr_1 = Address::generate(&env);
    let addr_2 = Address::generate(&env);
    let pub_event_1 = MyEvent1 {
        name: symbol_short!("hello"),
        address: addr_1.clone(),
        value: symbol_short!("world"),
        amount: 42,
    };
    let not_pub_event_1 = MyEvent1 {
        name: symbol_short!("hello"),
        address: addr_2.clone(),
        value: symbol_short!("world"),
        amount: 42,
    };
    let pub_event_2 = MyEvent2 {
        name: symbol_short!("goodbye"),
        address: addr_1.clone(),
        value: symbol_short!("world"),
        amount: 0,
    };
    let not_pub_event_2 = MyEvent2 {
        name: symbol_short!("goodbye"),
        address: addr_1.clone(),
        value: symbol_short!("world"),
        amount: 1,
    };
    let pub_event_2_like_not_1 = MyEvent2 {
        name: symbol_short!("hello"),
        address: addr_2.clone(),
        value: symbol_short!("world"),
        amount: 42,
    };

    env.as_contract(&id, || {
        pub_event_1.publish(&env);
        pub_event_2.publish(&env);
        pub_event_2_like_not_1.publish(&env);
    });

    assert!(env.events().contains(&id, &pub_event_1));
    assert!(!env.events().contains(&id, &not_pub_event_1));
    assert!(env.events().contains(&id, &pub_event_2));
    assert!(!env.events().contains(&id, &not_pub_event_2));
    assert!(env.events().contains(&id, &pub_event_2_like_not_1));
}
