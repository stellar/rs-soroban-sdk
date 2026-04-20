use crate::{self as soroban_sdk, Event};
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, testutils::Events as _, Env,
};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecEventDataFormat, ScSpecEventParamLocationV0,
    ScSpecEventParamV0, ScSpecEventV0, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
    ScSpecTypeResult, ScSpecTypeUdt, ScSpecUdtEnumCaseV0, ScSpecUdtEnumV0,
    ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0, ScSpecUdtStructFieldV0, ScSpecUdtStructV0,
    ScSymbol,
};

#[contract]
pub struct Contract;

#[contracterror]
#[derive(Debug, Eq, PartialEq)]
pub enum TestError {
    r#Error = 1,
}

#[contractevent]
pub struct TestEvent {
    #[topic]
    pub r#type: u32,
    pub r#fn: u32,
}

#[contracttype]
#[derive(Debug)]
pub struct TestType {
    pub r#type: u32,
    // `rank` is intentionally non-raw and starts with `r` so that its sort
    // position differs between the raw form (`r#type`,`r#value`,`raw`) and
    // unraw form (`raw`,`type`,`value`) of the other fields.
    pub rank: u32,
    pub r#value: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub enum TestEnum {
    r#Enum = 0,
    TestError = 1,
}

#[contractimpl]
impl Contract {
    pub fn r#type(env: Env, a: TestEnum) -> Result<TestType, TestError> {
        TestEvent {
            r#type: a.clone() as u32,
            r#fn: 1,
        }
        .publish(&env);

        match a {
            TestEnum::r#Enum => Ok(TestType {
                rank: 7,
                r#type: a as u32,
                r#value: 42,
            }),
            TestEnum::TestError => Err(TestError::r#Error),
        }
    }
}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let res = client.r#type(&TestEnum::r#Enum);
    assert_eq!(res.rank, 7);
    assert_eq!(res.r#type, 0);
    assert_eq!(res.value, 42);

    assert_eq!(
        env.events().all(),
        std::vec![TestEvent { r#type: 0, r#fn: 1 }.to_xdr(&env, &contract_id)],
    );

    let err = client.try_type(&TestEnum::TestError);
    assert_eq!(err.unwrap_err().unwrap(), TestError::r#Error);
}

#[test]
fn test_spec() {
    // `r#type` function must emit as "type", with inputs and outputs unraw'd.
    let fn_entry = ScSpecEntry::from_xdr(Contract::spec_xdr_type(), Limits::none()).unwrap();
    assert_eq!(
        fn_entry,
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "type".try_into().unwrap(),
            inputs: [ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "a".try_into().unwrap(),
                type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "TestEnum".try_into().unwrap(),
                }),
            }]
            .try_into()
            .unwrap(),
            outputs: [ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "TestType".try_into().unwrap(),
                })),
                error_type: Box::new(ScSpecTypeDef::Udt(ScSpecTypeUdt {
                    name: "TestError".try_into().unwrap(),
                })),
            }))]
            .try_into()
            .unwrap(),
        }),
    );

    // `TestType` struct fields `r#type` and `r#value` must emit as "type" and
    // "value" and appear sorted by the unraw'd name.
    let type_entry = ScSpecEntry::from_xdr(TestType::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        type_entry,
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "TestType".try_into().unwrap(),
            fields: [
                ScSpecUdtStructFieldV0 {
                    doc: "".try_into().unwrap(),
                    name: "rank".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                },
                ScSpecUdtStructFieldV0 {
                    doc: "".try_into().unwrap(),
                    name: "type".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                },
                ScSpecUdtStructFieldV0 {
                    doc: "".try_into().unwrap(),
                    name: "value".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                },
            ]
            .try_into()
            .unwrap(),
        }),
    );

    // `TestEnum` variant `r#Enum` must emit as "Enum"; the non-raw `TestError`
    // variant is unchanged.
    let enum_entry = ScSpecEntry::from_xdr(TestEnum::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        enum_entry,
        ScSpecEntry::UdtEnumV0(ScSpecUdtEnumV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "TestEnum".try_into().unwrap(),
            cases: [
                ScSpecUdtEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "Enum".try_into().unwrap(),
                    value: 0,
                },
                ScSpecUdtEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "TestError".try_into().unwrap(),
                    value: 1,
                },
            ]
            .try_into()
            .unwrap(),
        }),
    );

    // `TestError` variant `r#Error` must emit as "Error".
    let error_entry = ScSpecEntry::from_xdr(TestError::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        error_entry,
        ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "TestError".try_into().unwrap(),
            cases: [ScSpecUdtErrorEnumCaseV0 {
                doc: "".try_into().unwrap(),
                name: "Error".try_into().unwrap(),
                value: 1,
            }]
            .try_into()
            .unwrap(),
        }),
    );

    // `TestEvent` must emit its prefix topic as snake_case of the unraw'd
    // struct name ("test_event"), and its fields `r#type` and `r#fn` as "type"
    // and "fn" in declaration order.
    let event_entry = ScSpecEntry::from_xdr(TestEvent::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        event_entry,
        ScSpecEntry::EventV0(ScSpecEventV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: ScSymbol("TestEvent".try_into().unwrap()),
            prefix_topics: [ScSymbol("test_event".try_into().unwrap())]
                .try_into()
                .unwrap(),
            params: [
                ScSpecEventParamV0 {
                    doc: "".try_into().unwrap(),
                    name: "type".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                    location: ScSpecEventParamLocationV0::TopicList,
                },
                ScSpecEventParamV0 {
                    doc: "".try_into().unwrap(),
                    name: "fn".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                    location: ScSpecEventParamLocationV0::Data,
                },
            ]
            .try_into()
            .unwrap(),
            data_format: ScSpecEventDataFormat::Map,
        }),
    );
}
