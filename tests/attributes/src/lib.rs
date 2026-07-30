#![no_std]

use soroban_sdk::{contract, contractevent, contractimpl, contracttrait, contracttype, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttributeType {
    pub value: u32,
}

#[contractevent]
pub struct AttributeEvent {
    #[topic]
    topic: u32,
    value: u32,
}

#[contract]
pub struct Contract;

#[contracttrait]
pub trait AttributeTrait {
    fn trait_override() -> u32 {
        1
    }

    fn trait_default() -> u32 {
        2
    }

    fn trait_default_stacked_cfg() -> u32 {
        5
    }
}

#[contractimpl]
impl Contract {
    pub fn always(value: AttributeType) -> u32 {
        value.value
    }

    #[cfg(all())]
    pub fn cfg_included(value: u32) -> u32 {
        value
    }

    #[cfg(any())]
    pub fn cfg_excluded(value: u32) -> u32 {
        value
    }

    pub fn publish(env: Env, topic: u32, value: u32) {
        AttributeEvent { topic, value }.publish(&env);
    }
}

#[contractimpl(contracttrait)]
impl AttributeTrait for Contract {
    #[cfg(all())]
    fn trait_override() -> u32 {
        3
    }

    #[cfg(any())]
    fn trait_default() -> u32 {
        4
    }

    #[cfg(all())]
    #[cfg(any())]
    fn trait_default_stacked_cfg() -> u32 {
        6
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use soroban_sdk::{testutils::Events, Env};

    use super::*;

    fn attribute_value() -> AttributeType {
        AttributeType { value: 1 }
    }

    #[test]
    fn test_cfg_items() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        assert_eq!(client.always(&attribute_value()), 1);

        client.publish(&1, &2);
        assert_eq!(env.events().all().events().len(), 1);
    }

    #[test]
    fn test_cfg_methods() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        assert_eq!(client.cfg_included(&3), 3);
    }

    #[test]
    fn test_contracttrait_cfg_override_and_default() {
        let env = Env::default();
        let contract_id = env.register(Contract, ());
        let client = ContractClient::new(&env, &contract_id);

        assert_eq!(client.trait_override(), 3);
        assert_eq!(client.trait_default(), 2);
        assert_eq!(client.trait_default_stacked_cfg(), 5);
    }

    #[test]
    fn test_specs_include_active_attribute_items() {
        use soroban_sdk::xdr::{
            Limits, ReadXdr, ScSpecEntry, ScSpecEventParamLocationV0, ScSpecFunctionV0,
            ScSpecTypeDef,
        };
        use std::collections::HashSet;

        let type_entry = ScSpecEntry::from_xdr(AttributeType::spec_xdr(), Limits::none()).unwrap();
        let ScSpecEntry::UdtStructV0(type_spec) = type_entry else {
            panic!("expected struct spec");
        };
        assert_eq!(type_spec.fields[0].name.to_utf8_string_lossy(), "value");
        assert_eq!(type_spec.fields[0].type_, ScSpecTypeDef::U32);

        let event_entry =
            ScSpecEntry::from_xdr(AttributeEvent::spec_xdr(), Limits::none()).unwrap();
        let ScSpecEntry::EventV0(event_spec) = event_entry else {
            panic!("expected event spec");
        };
        let event_params = event_spec
            .params
            .iter()
            .map(|p| {
                (
                    p.name.to_utf8_string_lossy(),
                    p.type_.clone(),
                    p.location.clone(),
                )
            })
            .collect::<std::vec::Vec<_>>();
        assert_eq!(
            event_params,
            [
                (
                    "topic".into(),
                    ScSpecTypeDef::U32,
                    ScSpecEventParamLocationV0::TopicList
                ),
                (
                    "value".into(),
                    ScSpecTypeDef::U32,
                    ScSpecEventParamLocationV0::Data
                ),
            ]
        );

        let method_entry =
            ScSpecEntry::from_xdr(Contract::spec_xdr_cfg_included(), Limits::none()).unwrap();
        let ScSpecEntry::FunctionV0(ScSpecFunctionV0 { name, inputs, .. }) = method_entry else {
            panic!("expected function spec");
        };
        assert_eq!(name.to_utf8_string_lossy(), "cfg_included");
        assert_eq!(inputs[0].type_, ScSpecTypeDef::U32);

        let wasm = include_bytes!("../../../target/wasm32v1-none/release/test_attributes.wasm");
        let fn_names: HashSet<std::string::String> = soroban_spec::read::from_wasm(wasm)
            .unwrap()
            .iter()
            .filter_map(|e| {
                if let ScSpecEntry::FunctionV0(f) = e {
                    Some(f.name.to_utf8_string_lossy())
                } else {
                    None
                }
            })
            .collect();

        assert!(fn_names.contains("always"));
        assert!(fn_names.contains("cfg_included"));
        assert!(fn_names.contains("publish"));
        assert!(fn_names.contains("trait_override"));
        assert!(fn_names.contains("trait_default"));
        assert!(fn_names.contains("trait_default_stacked_cfg"));
        assert!(!fn_names.contains("cfg_excluded"));
    }
}
