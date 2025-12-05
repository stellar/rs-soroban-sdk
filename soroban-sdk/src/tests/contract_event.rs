use crate::{
    self as soroban_sdk, contract, contractevent, contracttype, map, symbol_short,
    testutils::{Address as _, ContractEvent, Events as _},
    Address, Env, Event, IntoVal, Map, String, Symbol, Val, Vec,
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

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("hello"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
        map![
            &env,
            (
                symbol_short!("value"),
                <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("hello"), &env),
            ),
        ]
        .into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("hello"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (
            symbol_short!("topic1"),
            symbol_short!("topic2"),
            symbol_short!("hi"),
        )
            .into_val(&env),
        map![
            &env,
            (
                symbol_short!("value"),
                <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("hello"), &env),
            ),
        ]
        .into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("hello"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("hi"),).into_val(&env),
        map![
            &env,
            (
                symbol_short!("value"),
                <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("hello"), &env),
            ),
        ]
        .into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        value: symbol_short!("hello"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        ().into_val(&env),
        map![
            &env,
            (
                symbol_short!("value"),
                <_ as IntoVal<Env, Val>>::into_val(&symbol_short!("hello"), &env),
            ),
        ]
        .into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
}

#[test]
fn test_no_topics_no_data() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());

    #[contractevent(topics = [])]
    pub struct MyEvent {}

    let event = MyEvent {};
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        ().into_val(&env),
        Map::<Symbol, Val>::new(&env).into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("yo"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
        symbol_short!("yo").into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: symbol_short!("hi"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
        ().into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("yo"),
        value2: 2,
        value3: Vec::new(&env),
        value4: String::from_str(&env, "asdf"),
        value5: MyType1 { value: 1 },
        value6: MyType2(2),
        value7: MyType3::Value(3),
        value8: MyType4::Value,
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
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
            .into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: symbol_short!("hi"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
        Vec::<Val>::new(&env).into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: symbol_short!("hi"),
        value: symbol_short!("yo"),
        value2: 2,
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
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
        .into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: symbol_short!("hi"),
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
        Map::<Symbol, Val>::new(&env).into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
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

    let event = MyEvent {
        name: &symbol_short!("hi"),
        value: &symbol_short!("yo"),
        value2: 2,
    };
    env.as_contract(&id, || {
        event.publish(&env);
    });

    let expected_event = ContractEvent::new(
        &env,
        id.clone(),
        (symbol_short!("my_event"), symbol_short!("hi")).into_val(&env),
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
        .into_val(&env),
    );
    assert_eq!(
        env.events().contract_events(),
        std::vec![expected_event.clone()],
    );
    assert_eq!(event.to_contract_event(&env, &id), expected_event);
}

#[test]
fn test_event_comparison_contract_id() {
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

    assert_ne!(
        env.events().contract_events()[0],
        event.to_contract_event(&env, &id_2)
    );
    assert_eq!(
        env.events().contract_events()[0],
        event.to_contract_event(&env, &id)
    );
}

#[test]
fn test_event_comparison_topics() {
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

    assert_ne!(
        env.events().contract_events()[0],
        event_2.to_contract_event(&env, &id)
    );
    assert_eq!(
        env.events().contract_events()[0],
        event.to_contract_event(&env, &id)
    );
}

#[test]
fn test_event_comparison_data_small() {
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

    assert_ne!(
        env.events().contract_events()[0],
        event_2.to_contract_event(&env, &id)
    );
    assert_eq!(
        env.events().contract_events()[0],
        event.to_contract_event(&env, &id)
    );
}

#[test]
fn test_event_comparison_data_small_and_host() {
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

    assert_ne!(
        env.events().contract_events()[0],
        event_2.to_contract_event(&env, &id)
    );
    assert_eq!(
        env.events().contract_events()[0],
        event.to_contract_event(&env, &id)
    );
}

#[test]
fn test_event_comparison_data_host() {
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

    assert_ne!(
        env.events().contract_events()[0],
        event_2.to_contract_event(&env, &id)
    );
    assert_eq!(
        env.events().contract_events()[0],
        event.to_contract_event(&env, &id)
    );
}

#[test]
fn test_events_for_diff_contracts() {
    let env = Env::default();

    #[contract]
    pub struct Contract;
    let id = env.register(Contract, ());
    let id_2 = env.register(Contract, ());

    #[contractevent]
    pub struct MyEvent {
        #[topic]
        name: Symbol,
        amount: i128,
    }

    let pub_event_1 = MyEvent {
        name: symbol_short!("hello"),
        amount: 42,
    };
    let pub_event_2 = MyEvent {
        name: symbol_short!("world"),
        amount: 0,
    };
    let pub_event_3 = MyEvent {
        name: symbol_short!("farewell"),
        amount: -1,
    };

    env.as_contract(&id, || {
        pub_event_1.publish(&env);
        env.as_contract(&id_2, || {
            pub_event_2.publish(&env);
        });
        pub_event_3.publish(&env);
    });

    assert_eq!(
        env.events().contract_events_for(&id),
        std::vec![
            pub_event_1.to_contract_event(&env, &id),
            pub_event_3.to_contract_event(&env, &id),
        ],
    );
    assert_eq!(
        env.events().contract_events_for(&id_2),
        std::vec![pub_event_2.to_contract_event(&env, &id_2),],
    );
}
