use crate::{
    self as soroban_sdk, contract, contractevent, map, symbol_short, testutils::Events as _, vec,
    Env, IntoVal, Map, Symbol, Val, Vec,
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

    #[contractevent(data_format = "vec")]
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
                (symbol_short!("yo"), 2u32).into_val(&env)
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
