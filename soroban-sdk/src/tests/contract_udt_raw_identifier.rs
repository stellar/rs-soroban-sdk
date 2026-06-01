use crate::{self as soroban_sdk, Event};
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttrait, contracttype,
    testutils::Events as _, Env,
};
use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{
    Limits, ReadXdr, ScSpecEntry, ScSpecEventDataFormat, ScSpecEventParamLocationV0,
    ScSpecEventParamV0, ScSpecEventV0, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef,
    ScSpecTypeResult, ScSpecTypeUdt, ScSpecUdtEnumCaseV0, ScSpecUdtEnumV0,
    ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0, ScSpecUdtStructFieldV0, ScSpecUdtStructV0,
    ScSpecUdtUnionCaseTupleV0, ScSpecUdtUnionCaseV0, ScSpecUdtUnionV0, ScSymbol,
};

#[contract]
pub struct r#Contract;

#[contracterror]
#[derive(Debug, Eq, PartialEq)]
pub enum r#TestError {
    r#Error = 1,
}

#[contractevent]
pub struct r#TestEvent {
    #[topic]
    pub r#type: u32,
    pub r#fn: u32,
}

#[contracttype]
#[derive(Debug)]
pub struct r#TestType {
    pub r#type: u32,
    // `rank` is intentionally non-raw and starts with `r` so that its sort
    // position differs between the raw form (`r#type`,`r#value`,`raw`) and
    // unraw form (`raw`,`type`,`value`) of the other fields.
    pub rank: u32,
    pub r#value: u32,
}

#[contracttype]
#[derive(Clone)]
pub enum r#TestEnum {
    r#Enum = 0,
    TestError = 1,
}

#[contracttype]
#[derive(Clone)]
pub enum r#TestTupleEnum {
    r#Tuple(u32),
    TestTuple(i32),
}

#[contracttrait]
pub trait r#RawTrait {
    fn r#method(env: &Env) -> u32 {
        let _ = env;
        42
    }
}

#[contracttype]
#[allow(non_camel_case_types)]
pub struct r#type {
    pub value: u32,
}

#[contracttype]
pub struct r#TupleStruct(u32, u32);

#[contractimpl]
impl r#Contract {
    pub fn r#type(env: Env, r#fn: TestEnum) -> Result<TestType, TestError> {
        TestEvent {
            r#type: r#fn.clone() as u32,
            r#fn: 1,
        }
        .publish(&env);

        match r#fn {
            TestEnum::r#Enum => Ok(TestType {
                rank: 7,
                r#type: r#fn as u32,
                r#value: 42,
            }),
            TestEnum::TestError => Err(TestError::r#Error),
        }
    }
}

#[contractimpl(contracttrait)]
impl r#RawTrait for r#Contract {}

#[test]
fn test_functional() {
    let env = Env::default();
    let contract_id = env.register(r#Contract, ());
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

    assert_eq!(client.method(), 42);
}

#[test]
fn test_spec_contract() {
    let fn_entry = ScSpecEntry::from_xdr(r#Contract::spec_xdr_type(), Limits::none()).unwrap();
    assert_eq!(
        fn_entry,
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "type".try_into().unwrap(),
            inputs: [ScSpecFunctionInputV0 {
                doc: "".try_into().unwrap(),
                name: "fn".try_into().unwrap(),
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

    let trait_fn_entry =
        ScSpecEntry::from_xdr(r#Contract::spec_xdr_method(), Limits::none()).unwrap();
    assert_eq!(
        trait_fn_entry,
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "method".try_into().unwrap(),
            inputs: [].try_into().unwrap(),
            outputs: [ScSpecTypeDef::U32].try_into().unwrap(),
        }),
    );

    let trait_method_spec =
        ScSpecEntry::from_xdr(RawTraitSpec::spec_xdr_method(), Limits::none()).unwrap();
    assert_eq!(
        trait_method_spec,
        ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: "".try_into().unwrap(),
            name: "method".try_into().unwrap(),
            inputs: [].try_into().unwrap(),
            outputs: [ScSpecTypeDef::U32].try_into().unwrap(),
        }),
    );
}

#[test]
fn test_spec_struct() {
    // `r#TestType` with raw fields `r#type` and `r#value` must emit with its
    // name unraw'd, fields unraw'd, and sorted by the unraw'd field name.
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

    let keyword_struct_entry = ScSpecEntry::from_xdr(r#type::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        keyword_struct_entry,
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "type".try_into().unwrap(),
            fields: [ScSpecUdtStructFieldV0 {
                doc: "".try_into().unwrap(),
                name: "value".try_into().unwrap(),
                type_: ScSpecTypeDef::U32,
            }]
            .try_into()
            .unwrap(),
        }),
    );

    let tuple_struct_entry =
        ScSpecEntry::from_xdr(r#TupleStruct::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        tuple_struct_entry,
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "TupleStruct".try_into().unwrap(),
            fields: [
                ScSpecUdtStructFieldV0 {
                    doc: "".try_into().unwrap(),
                    name: "0".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                },
                ScSpecUdtStructFieldV0 {
                    doc: "".try_into().unwrap(),
                    name: "1".try_into().unwrap(),
                    type_: ScSpecTypeDef::U32,
                },
            ]
            .try_into()
            .unwrap(),
        }),
    );
}

#[test]
fn test_spec_enum() {
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

    let tuple_enum_entry =
        ScSpecEntry::from_xdr(TestTupleEnum::spec_xdr(), Limits::none()).unwrap();
    assert_eq!(
        tuple_enum_entry,
        ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "TestTupleEnum".try_into().unwrap(),
            cases: [
                ScSpecUdtUnionCaseV0::TupleV0(ScSpecUdtUnionCaseTupleV0 {
                    doc: "".try_into().unwrap(),
                    name: "Tuple".try_into().unwrap(),
                    type_: [ScSpecTypeDef::U32].try_into().unwrap(),
                }),
                ScSpecUdtUnionCaseV0::TupleV0(ScSpecUdtUnionCaseTupleV0 {
                    doc: "".try_into().unwrap(),
                    name: "TestTuple".try_into().unwrap(),
                    type_: [ScSpecTypeDef::I32].try_into().unwrap(),
                }),
            ]
            .try_into()
            .unwrap(),
        }),
    );
}

#[test]
fn test_spec_event() {
    // `r#TestEvent` must emit its prefix topic as snake_case of the unraw'd
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
