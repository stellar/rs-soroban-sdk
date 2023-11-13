mod fn_ {
    use crate as soroban_sdk;
    use soroban_sdk::{contract, contractimpl, Env};
    use stellar_xdr::curr as stellar_xdr;
    use stellar_xdr::{Limits, ReadXdr, ScSpecEntry, ScSpecFunctionV0};

    #[contract]
    pub struct Contract;

    #[contractimpl]
    impl Contract {
        /// Add adds
        // TODO: Implement this.
        /// things together.
        pub fn add() {}
    }

    #[test]
    fn test_functional() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);
        client.add();
    }

    #[test]
    fn test_spec() {
        let entry = ScSpecEntry::from_xdr(__SPEC_XDR_FN_ADD, Limits::none()).unwrap();
        let expect = ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
            doc: "Add adds\nthings together.".try_into().unwrap(),
            name: "add".try_into().unwrap(),
            inputs: vec![].try_into().unwrap(),
            outputs: vec![].try_into().unwrap(),
        });
        assert_eq!(entry, expect);
    }
}

mod struct_ {
    use crate as soroban_sdk;
    use soroban_sdk::contracttype;
    use stellar_xdr::curr as stellar_xdr;
    use stellar_xdr::{Limits, ReadXdr, ScSpecEntry, ScSpecUdtStructFieldV0, ScSpecUdtStructV0};

    /// S holds a and
    // TODO: Implement.
    #[contracttype]
    /// b.
    pub struct S {
        /// a is
        /// a
        a: u64,
        /// b is b
        b: u64,
    }

    #[test]
    fn test_spec() {
        let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_S, Limits::none()).unwrap();
        let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "S holds a and\nb.".try_into().unwrap(),
            name: "S".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            fields: [
                ScSpecUdtStructFieldV0 {
                    doc: "a is\na".try_into().unwrap(),
                    name: "a".try_into().unwrap(),
                    type_: stellar_xdr::ScSpecTypeDef::U64,
                },
                ScSpecUdtStructFieldV0 {
                    doc: "b is b".try_into().unwrap(),
                    name: "b".try_into().unwrap(),
                    type_: stellar_xdr::ScSpecTypeDef::U64,
                },
            ]
            .try_into()
            .unwrap(),
        });
        assert_eq!(entry, expect);
    }
}

mod struct_tuple {
    use crate as soroban_sdk;
    use soroban_sdk::contracttype;
    use stellar_xdr::curr as stellar_xdr;
    use stellar_xdr::{Limits, ReadXdr, ScSpecEntry, ScSpecUdtStructFieldV0, ScSpecUdtStructV0};

    /// S holds two u64s.
    #[contracttype]
    pub struct S(
        /// first
        /// line
        u64,
        /// second
        u64,
    );

    #[test]
    fn test_spec() {
        let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_S, Limits::none()).unwrap();
        let expect = ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "S holds two u64s.".try_into().unwrap(),
            name: "S".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            fields: [
                ScSpecUdtStructFieldV0 {
                    doc: "first\nline".try_into().unwrap(),
                    name: "0".try_into().unwrap(),
                    type_: stellar_xdr::ScSpecTypeDef::U64,
                },
                ScSpecUdtStructFieldV0 {
                    doc: "second".try_into().unwrap(),
                    name: "1".try_into().unwrap(),
                    type_: stellar_xdr::ScSpecTypeDef::U64,
                },
            ]
            .try_into()
            .unwrap(),
        });
        assert_eq!(entry, expect);
    }
}

mod enum_ {
    use crate as soroban_sdk;
    use soroban_sdk::contracttype;
    use stellar_xdr::curr as stellar_xdr;
    use stellar_xdr::{
        Limits, ReadXdr, ScSpecEntry, ScSpecUdtUnionCaseTupleV0, ScSpecUdtUnionCaseV0,
        ScSpecUdtUnionCaseVoidV0, ScSpecUdtUnionV0,
    };

    /// E has variants A and B.
    #[contracttype]
    #[derive(Copy, Clone)]
    pub enum E {
        /// A is
        /// a.
        A,
        /// B is
        /// b.
        B(u64, u64),
    }

    #[test]
    fn test_spec() {
        let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_E, Limits::none()).unwrap();
        let expect = ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
            doc: "E has variants A and B.".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: "E".try_into().unwrap(),
            cases: [
                ScSpecUdtUnionCaseV0::VoidV0(ScSpecUdtUnionCaseVoidV0 {
                    doc: "A is\na.".try_into().unwrap(),
                    name: "A".try_into().unwrap(),
                }),
                ScSpecUdtUnionCaseV0::TupleV0(ScSpecUdtUnionCaseTupleV0 {
                    doc: "B is\nb.".try_into().unwrap(),
                    name: "B".try_into().unwrap(),
                    type_: [
                        // TODO: Add docs for tuple values in union cases.
                        stellar_xdr::ScSpecTypeDef::U64,
                        stellar_xdr::ScSpecTypeDef::U64,
                    ]
                    .try_into()
                    .unwrap(),
                }),
            ]
            .try_into()
            .unwrap(),
        });
        assert_eq!(entry, expect);
    }
}

mod enum_int {
    use crate as soroban_sdk;
    use soroban_sdk::contracttype;
    use stellar_xdr::curr as stellar_xdr;
    use stellar_xdr::{Limits, ReadXdr, ScSpecEntry, ScSpecUdtEnumCaseV0, ScSpecUdtEnumV0};

    /// E has variants A and B.
    #[contracttype]
    #[derive(Copy, Clone)]
    pub enum E {
        /// A is
        /// a.
        A = 1,
        /// B is b.
        B = 2,
    }

    #[test]
    fn test_spec() {
        let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_E, Limits::none()).unwrap();
        let expect = ScSpecEntry::UdtEnumV0(ScSpecUdtEnumV0 {
            doc: "E has variants A and B.".try_into().unwrap(),
            name: "E".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            cases: [
                ScSpecUdtEnumCaseV0 {
                    doc: "A is\na.".try_into().unwrap(),
                    name: "A".try_into().unwrap(),
                    value: 1,
                },
                ScSpecUdtEnumCaseV0 {
                    doc: "B is b.".try_into().unwrap(),
                    name: "B".try_into().unwrap(),
                    value: 2,
                },
            ]
            .try_into()
            .unwrap(),
        });
        assert_eq!(entry, expect);
    }
}

mod enum_error_int {
    use crate as soroban_sdk;
    use soroban_sdk::contracterror;
    use stellar_xdr::curr as stellar_xdr;
    use stellar_xdr::{
        Limits, ReadXdr, ScSpecEntry, ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
    };

    /// E has variants A and B.
    #[contracterror]
    #[derive(Copy, Clone)]
    pub enum E {
        /// A is
        /// a.
        A = 1,
        /// B is b.
        B = 2,
    }

    #[test]
    fn test_spec() {
        let entry = ScSpecEntry::from_xdr(__SPEC_XDR_TYPE_E, Limits::none()).unwrap();
        let expect = ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
            doc: "E has variants A and B.".try_into().unwrap(),
            name: "E".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            cases: [
                ScSpecUdtErrorEnumCaseV0 {
                    doc: "A is\na.".try_into().unwrap(),
                    name: "A".try_into().unwrap(),
                    value: 1,
                },
                ScSpecUdtErrorEnumCaseV0 {
                    doc: "B is b.".try_into().unwrap(),
                    name: "B".try_into().unwrap(),
                    value: 2,
                },
            ]
            .try_into()
            .unwrap(),
        });
        assert_eq!(entry, expect);
    }
}
