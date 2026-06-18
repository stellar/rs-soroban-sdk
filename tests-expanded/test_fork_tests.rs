#![feature(prelude_import)]
#![no_std]
#[macro_use]
extern crate core;
#[prelude_import]
use core::prelude::rust_2021::*;
use soroban_sdk::contract;
pub struct Contract;
///ContractArgs is a type for building arg lists for functions defined in "Contract".
pub struct ContractArgs;
///ContractClient is a client for calling the contract defined in "Contract".
pub struct ContractClient<'a> {
    pub env: soroban_sdk::Env,
    pub address: soroban_sdk::Address,
    #[doc(hidden)]
    set_auths: Option<&'a [soroban_sdk::xdr::SorobanAuthorizationEntry]>,
    #[doc(hidden)]
    mock_auths: Option<&'a [soroban_sdk::testutils::MockAuth<'a>]>,
    #[doc(hidden)]
    mock_all_auths: bool,
    #[doc(hidden)]
    allow_non_root_auth: bool,
}
impl<'a> ContractClient<'a> {
    pub fn new(env: &soroban_sdk::Env, address: &soroban_sdk::Address) -> Self {
        Self {
            env: env.clone(),
            address: address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Set authorizations in the environment which will be consumed by
    /// contracts when they invoke `Address::require_auth` or
    /// `Address::require_auth_for_args` functions.
    ///
    /// Requires valid signatures for the authorization to be successful.
    /// To mock auth without requiring valid signatures, use `mock_auths`.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn set_auths(&self, auths: &'a [soroban_sdk::xdr::SorobanAuthorizationEntry]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: Some(auths),
            mock_auths: self.mock_auths.clone(),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock authorizations in the environment which will cause matching invokes
    /// of `Address::require_auth` and `Address::require_auth_for_args` to
    /// pass.
    ///
    /// See `soroban_sdk::Env::set_auths` for more details and examples.
    pub fn mock_auths(&self, mock_auths: &'a [soroban_sdk::testutils::MockAuth<'a>]) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: self.set_auths.clone(),
            mock_auths: Some(mock_auths),
            mock_all_auths: false,
            allow_non_root_auth: false,
        }
    }
    /// Mock all calls to the `Address::require_auth` and
    /// `Address::require_auth_for_args` functions in invoked contracts,
    /// having them succeed as if authorization was provided.
    ///
    /// See `soroban_sdk::Env::mock_all_auths` for more details and
    /// examples.
    pub fn mock_all_auths(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: false,
        }
    }
    /// A version of `mock_all_auths` that allows authorizations that
    /// are not present in the root invocation.
    ///
    /// Refer to `mock_all_auths` documentation for details and
    /// prefer using `mock_all_auths` unless non-root authorization is
    /// required.
    ///
    /// See `soroban_sdk::Env::mock_all_auths_allowing_non_root_auth`
    /// for more details and examples.
    pub fn mock_all_auths_allowing_non_root_auth(&self) -> Self {
        Self {
            env: self.env.clone(),
            address: self.address.clone(),
            set_auths: None,
            mock_auths: None,
            mock_all_auths: true,
            allow_non_root_auth: true,
        }
    }
}
mod __contract_fn_set_registry {
    use super::*;
    extern crate std;
    use std::collections::BTreeMap;
    use std::sync::Mutex;
    pub type F = soroban_sdk::testutils::ContractFunctionF;
    static FUNCS: Mutex<BTreeMap<&'static str, &'static F>> = Mutex::new(BTreeMap::new());
    pub fn register(name: &'static str, func: &'static F) {
        FUNCS.lock().unwrap().insert(name, func);
    }
    pub fn call(
        name: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        let fopt: Option<&'static F> = FUNCS.lock().unwrap().get(name).map(|f| f.clone());
        fopt.map(|f| f(env, args))
    }
}
impl soroban_sdk::testutils::ContractFunctionRegister for Contract {
    fn register(name: &'static str, func: &'static __contract_fn_set_registry::F) {
        __contract_fn_set_registry::register(name, func);
    }
}
#[doc(hidden)]
impl soroban_sdk::testutils::ContractFunctionSet for Contract {
    fn call(
        &self,
        func: &str,
        env: soroban_sdk::Env,
        args: &[soroban_sdk::Val],
    ) -> Option<soroban_sdk::Val> {
        __contract_fn_set_registry::call(func, env, args)
    }
}
mod example {
    extern crate std;
    use bytes_lit::bytes;
    use soroban_ledger_snapshot_source_tx::Network;
    use soroban_ledger_snapshot_source_tx::TxSnapshotSource;
    use soroban_sdk::{token::TokenClient, Address, Env};
    extern crate test;
    #[rustc_test_marker = "example::test"]
    #[doc(hidden)]
    pub const test: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("example::test"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "tests/fork/src/lib.rs",
            start_line: 16usize,
            start_col: 8usize,
            end_line: 16usize,
            end_col: 12usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::UnitTest,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(test()),
        ),
    };
    fn test() {
        const NATIVE_ADDRESS: &str = "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA";
        let source = TxSnapshotSource::new(
            Network::mainnet(None),
            61340000,
            Some([
                32u8, 26u8, 30u8, 156u8, 208u8, 212u8, 142u8, 213u8, 231u8, 250u8, 204u8, 118u8,
                66u8, 120u8, 90u8, 6u8, 33u8, 247u8, 90u8, 80u8, 194u8, 200u8, 252u8, 6u8, 209u8,
                233u8, 54u8, 168u8, 43u8, 100u8, 35u8, 18u8,
            ]),
        );
        let env = Env::from_ledger_snapshot(source);
        let contract = Address::from_str(&env, NATIVE_ADDRESS);
        let client = TokenClient::new(&env, &contract);
        let addr = Address::from_str(
            &env,
            "GCO45COWIBDZEGJ3DRGDGCCCJXK777F2S6D6HXQKXVB3EKQVCQU7B2WA",
        );
        let bal = client.balance(&addr);
        match (&bal, &827726773) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
}
mod mainnet {
    extern crate std;
    use soroban_ledger_snapshot_source_tx::Network;
    use soroban_ledger_snapshot_source_tx::TxSnapshotSource;
    use soroban_sdk::{token::TokenClient, Address, Env};
    #[rustfmt::skip]
    fn test() {
        match (&test_internal(61292151, None), &28640019212) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292152,
                Some(
                    hex_hash(
                        "2bfec2fe0d0b33fcb0ebb37f838d3b964b8aed50977afb0b7195fe49dd536ec9",
                    ),
                ),
            ),
            &28640019112,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292153,
                Some(
                    hex_hash(
                        "93028d85b1d5119df64ddb8f3130dc5411498193120f26ac65fef7609ec5b566",
                    ),
                ),
            ),
            &26640019125,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292155,
                Some(
                    hex_hash(
                        "2e777e004e90fe2f0023d0a2d1879ac62118eb4a9cb5e02cad4f7776a84923bb",
                    ),
                ),
            ),
            &24640019136,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292156,
                Some(
                    hex_hash(
                        "32879b7c7ea6a30584f36844f1d4cc3189d6c48839fc77bed2eb63487fae9ba3",
                    ),
                ),
            ),
            &24634416302,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292157,
                Some(
                    hex_hash(
                        "fccf410decdeb4204809bc632e74e9730e936b000e9b603dfd18912f3e62dd52",
                    ),
                ),
            ),
            &24134416305,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292159,
                Some(
                    hex_hash(
                        "a2c2e40030a56f6d016b34978a91ff8cc89d6e25adc8093c1061c59da90d0f4d",
                    ),
                ),
            ),
            &24128733912,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292161,
                Some(
                    hex_hash(
                        "1c94a019ca0a6b4a54d15ec171c8f902d54f305145454b54173b7b31d2bd6623",
                    ),
                ),
            ),
            &24121908843,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292163,
                Some(
                    hex_hash(
                        "fc9dbb4d7c71339267b857952e18f3a7cde1ba20c813dbca55e4849cd0f5f37d",
                    ),
                ),
            ),
            &24114311687,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292165,
                Some(
                    hex_hash(
                        "b0cc4d5fe54d9a59c47395243b648a8c6415c0aa0b1807a33337460413774fbb",
                    ),
                ),
            ),
            &24112182156,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292168,
                Some(
                    hex_hash(
                        "4df4a1257fcc05b631ae3b08d5118f8b6a7e297133e893a1f08b1bdc90efa463",
                    ),
                ),
            ),
            &24110475788,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292169,
                Some(
                    hex_hash(
                        "94191e94402dcf319d35b75f923461c0b382304c73edd12848bbd3bb63a48a34",
                    ),
                ),
            ),
            &24079646158,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292170,
                Some(
                    hex_hash(
                        "777a5c2cabd17ffda907dc44975c917a12e1f4e98946d58f055cef0c13a8c9ac",
                    ),
                ),
            ),
            &24072910038,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292171,
                Some(
                    hex_hash(
                        "8027e372b1c324bff22907ef81830a9e58f316a1be202b34d3154ea4397fea77",
                    ),
                ),
            ),
            &21572910052,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292171,
                Some(
                    hex_hash(
                        "b4eb6dd6afab43a91d2e7acec26980b04e1d47477890313b214e9b1563747f3f",
                    ),
                ),
            ),
            &22072910049,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292173,
                Some(
                    hex_hash(
                        "9dc6aab3420066ea89a8098bb62f844ec796fed44a26336546f29333f0420021",
                    ),
                ),
            ),
            &21571672284,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292173,
                Some(
                    hex_hash(
                        "20d8ee570fcc2064e5a6d3f1cf7cdace934707e47a43607ab564abb39417057f",
                    ),
                ),
            ),
            &21569563764,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292174,
                Some(
                    hex_hash(
                        "d9012d0e40eee4e66b88a9197448b189fa95e139de2f0c4677aa065ad5100231",
                    ),
                ),
            ),
            &21069563767,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292175,
                Some(
                    hex_hash(
                        "388a047b86ce015f74d82c881885eef51078325e11f0dc45cfbd3154e0489685",
                    ),
                ),
            ),
            &20713716699,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292175,
                Some(
                    hex_hash(
                        "27002c56aa2ad3c4964af019da56c59cc2c84693446ab0f56f01e7fb9dd644f6",
                    ),
                ),
            ),
            &21066642483,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292176,
                Some(
                    hex_hash(
                        "d22e7e7889053fe3de070c1610f40e113929090368e9df4e49e6743687a4323a",
                    ),
                ),
            ),
            &20712900251,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292177,
                Some(
                    hex_hash(
                        "1e94b58b93c7ed3f7f0c7e3cf044197216a103163ab1cc20cdd5a05ce6f14ff6",
                    ),
                ),
            ),
            &20673806969,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292177,
                Some(
                    hex_hash(
                        "e5ad20075b7aad00711252306a5098ad472f444310d3526e54b31ffe3a5fb88a",
                    ),
                ),
            ),
            &20677374810,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292178,
                Some(
                    hex_hash(
                        "a370c2092e77bfac40e2c1ade271fcbd6cebb9b8430b0c06419296fea7beb3e5",
                    ),
                ),
            ),
            &20668813924,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292179,
                Some(
                    hex_hash(
                        "aa7ccf318ee811240a6bdea16607dea0607baa1116561667de57aba3287c2cc7",
                    ),
                ),
            ),
            &20668456579,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292179,
                Some(
                    hex_hash(
                        "bc96dbe783474d71aeea99dfba7aadaca5e76e29855c30eb0900e9c192cb2e04",
                    ),
                ),
            ),
            &20667200123,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292180,
                Some(
                    hex_hash(
                        "de461160e3760ed2623282c14f884bc55b0db2e2608ed768cb1f189eca72b0f2",
                    ),
                ),
            ),
            &20667164303,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292181,
                Some(
                    hex_hash(
                        "2c3c62f80d006c0c76bf78b82bcd40d8b39606cfdb325739973af7eb0de4ad97",
                    ),
                ),
            ),
            &20447168649,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292181,
                Some(
                    hex_hash(
                        "86837220b0dcdd52cfc97d52953ba3c39fd1ab91134fa5ec2c555625af4fd776",
                    ),
                ),
            ),
            &20442139665,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292181,
                Some(
                    hex_hash(
                        "afc06a1b9051549b2a4593fa4472f05e00ed0a37297bf133e49dace3364312c9",
                    ),
                ),
            ),
            &20667160725,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292181,
                Some(
                    hex_hash(
                        "13517f66e83a8270e1ffe333a9eacb73768988ce7e24649fee1b6ea150674245",
                    ),
                ),
            ),
            &20416854956,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292182,
                Some(
                    hex_hash(
                        "8b7fef92629d12c4aac3b6a206e53c0f29d0536908353861ac4383ddab01a61c",
                    ),
                ),
            ),
            &20416854592,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292182,
                Some(
                    hex_hash(
                        "d8db89f42639b5bf788ab6ac6b7d2b6a4dedc76149ffdbcc417ea13bc7c8a963",
                    ),
                ),
            ),
            &20416854592,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292183,
                Some(
                    hex_hash(
                        "dbca3f43511ff2c894e392c9e42e27d356604ca49e8f51ff10b8380e4a2c0863",
                    ),
                ),
            ),
            &20414752362,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292184,
                Some(
                    hex_hash(
                        "8cd425ad71c81f5f4b5b68ccfc4a48cc0b8badff1daf30b93281365dce0d94a1",
                    ),
                ),
            ),
            &20414752362,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292184,
                Some(
                    hex_hash(
                        "edb2139c974d22c73d2fced26922ded276aebd75a42488849d798cdbc32bf415",
                    ),
                ),
            ),
            &19172679311,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292185,
                Some(
                    hex_hash(
                        "54be9cf57633cf04c85c0a558888e78e0ca3856cde3199f8708497e383346bb1",
                    ),
                ),
            ),
            &19172679311,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292186,
                Some(
                    hex_hash(
                        "93d6ed4b9d82b35bc89b0666dc8e4bacf1c589074c45dec9c31af892f6eaa721",
                    ),
                ),
            ),
            &19163487111,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292187,
                Some(
                    hex_hash(
                        "59898ca58428a19a2f2cc6a8ab7def4ec72c2ed16ffb44dd118598630b9e9b3d",
                    ),
                ),
            ),
            &19163487111,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292188,
                Some(
                    hex_hash(
                        "9170bb7e95b0e378c28c03662ec1cdf4529dcfcc53dd7a89f2bd951130a909c3",
                    ),
                ),
            ),
            &19154858911,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292189,
                Some(
                    hex_hash(
                        "b9d3b96a4dcebb1493867fea89f4f6baf392cdf9598f489531d70880745d9a6e",
                    ),
                ),
            ),
            &19152544114,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292189,
                Some(
                    hex_hash(
                        "5167b538dde5c47bbae712d4b47186e11936dcee00fe11249369fecea779b908",
                    ),
                ),
            ),
            &19154858911,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292191,
                Some(
                    hex_hash(
                        "d478ec95a68e1af912ed842d73a659715c2a24dff29d561a763802790be5c54b",
                    ),
                ),
            ),
            &19152544114,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292197,
                Some(
                    hex_hash(
                        "1f4efe8404e6b421676d462852883d7ea5e602ecb9080daab38eb70779ef3a64",
                    ),
                ),
            ),
            &18710779602,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292204,
                Some(
                    hex_hash(
                        "7e6190c9e46620a0859dcad113cdb1f8d5b98e9f35e999070970482d524406c1",
                    ),
                ),
            ),
            &18665585699,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292208,
                Some(
                    hex_hash(
                        "1fba8adbb5255de150df3b195619d4a25ea2ccee465d417b8ab84d98adcff4a9",
                    ),
                ),
            ),
            &18665085382,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292212,
                Some(
                    hex_hash(
                        "25974cdeffe0d1917935114bd1826dc51517ddf8f94d458c920d548e50950722",
                    ),
                ),
            ),
            &18663714838,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292214,
                Some(
                    hex_hash(
                        "d2fbad2f8418706aadab192660ee04c1bf16c5300014a8219b97c12d0a3456ff",
                    ),
                ),
            ),
            &18662250331,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292214,
                Some(
                    hex_hash(
                        "8e1c5b5a8d65b1a57312e77939e311705cb5e95d9b340dce9a1cf605dbd1ed8b",
                    ),
                ),
            ),
            &18661077898,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292220,
                Some(
                    hex_hash(
                        "45a06ce004fc7e8aa80e7be5de59cae65dc599841e65b0c2f8f2e034fcf8f8f3",
                    ),
                ),
            ),
            &18660765200,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292220,
                Some(
                    hex_hash(
                        "12a46b59c62fab061ae9b37be73c6f3658d109d826425b6c7e1d27562eca71c4",
                    ),
                ),
            ),
            &18649217174,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292222,
                Some(
                    hex_hash(
                        "8d631a78cf6d41adc653dd55af4490ddb1f807747fb09efb106070cbd4122496",
                    ),
                ),
            ),
            &18648151686,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292225,
                Some(
                    hex_hash(
                        "7748d2f83eb1eb6088ff81aafc6caab287aefeda0b833656b09041e6fb2e6288",
                    ),
                ),
            ),
            &18645351502,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292227,
                Some(
                    hex_hash(
                        "e44ddb4445e82481d7a00545b61e6a5bc345694de84048f6c11f40d192c86708",
                    ),
                ),
            ),
            &18642134811,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        match (
            &test_internal(
                61292232,
                Some(
                    hex_hash(
                        "221e786fd872913af3f23c809b39c022cf3404d90faed1f898342f689574a09b",
                    ),
                ),
            ),
            &18640441226,
        ) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
    fn hex_hash(s: &str) -> [u8; 32] {
        hex::decode(s).unwrap().try_into().unwrap()
    }
    #[rustfmt::skip]
    fn test_internal(ledger: u32, tx: Option<[u8; 32]>) -> i128 {
        use std::time::Instant;
        let t0 = Instant::now();
        let s = TxSnapshotSource::new(Network::mainnet(None), ledger, tx);
        let e = Env::from_ledger_snapshot(s);
        let contract = Address::from_str(
            &e,
            "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA",
        );
        let client = TokenClient::new(&e, &contract);
        let addr = Address::from_str(
            &e,
            "GBLKX4UPDM7CC4UUG2FXBLOCXOTQ6ARHOQYVL4RD6A4AQVB6TPTLIUYN",
        );
        let res = client.balance(&addr);
        let t1 = Instant::now();
        {
            ::std::io::_print(
                format_args!(
                    "\u{1b}[32m{2} {0} bal = {3} (t={1:?})\u{1b}[0m\n",
                    tx.map(|t| hex::encode(t)).unwrap_or_default(),
                    t1 - t0,
                    ledger,
                    res,
                ),
            );
        };
        res
    }
    #[rustfmt::skip]
    mod tests {
        use super::test;
        extern crate test;
        #[rustc_test_marker = "mainnet::tests::test_1"]
        #[doc(hidden)]
        pub const test_1: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("mainnet::tests::test_1"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "tests/fork/src/lib.rs",
                start_line: 142usize,
                start_col: 20usize,
                end_line: 142usize,
                end_col: 26usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::UnitTest,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(test_1()),
            ),
        };
        fn test_1() {
            test()
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test, &test_1])
}
