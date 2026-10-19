use crate as soroban_sdk;
use soroban_sdk::{
    contract, contractimpl, crypto::Hash, xdr, xdr::ReadXdr as _, Bytes, BytesN, Env, Error,
};

#[test]
fn test_sha256() {
    let env = Env::default();

    let input = bytes!(&env, 0x01);
    let expect = bytesn!(
        &env,
        0x4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a
    );
    let hash: BytesN<32> = env.crypto().sha256(&input).into();
    assert_eq!(hash, expect);
}

#[contract]
pub struct Contract;

type Digest = Hash<32>;
type DigestResult = Result<Hash<32>, Error>;

#[contractimpl]
impl Contract {
    pub fn sha256(env: Env, data: Bytes) -> Hash<32> {
        env.crypto().sha256(&data)
    }

    pub fn sha256_aliased(env: Env, data: Bytes) -> Digest {
        env.crypto().sha256(&data)
    }

    pub fn sha256_result(env: Env, data: Bytes) -> Result<Hash<32>, Error> {
        Ok(env.crypto().sha256(&data))
    }

    pub fn sha256_option(env: Env, data: Bytes) -> Option<Hash<32>> {
        Some(env.crypto().sha256(&data))
    }

    pub fn sha256_result_aliased(env: Env, data: Bytes) -> DigestResult {
        Ok(env.crypto().sha256(&data))
    }

    pub fn sha256_tuple(env: Env, data: Bytes) -> (Hash<32>, u32) {
        (env.crypto().sha256(&data), data.len())
    }
}

#[test]
fn test_sha256_returned_from_contract_fn() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let input = bytes!(&env, 0x01);
    let expect = bytesn!(
        &env,
        0x4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a
    );
    // The client returns a Hash as a BytesN, because it can't guarantee the
    // bytes it receives came from a secure cryptographic hash function.
    let hash: BytesN<32> = client.sha256(&input);
    assert_eq!(hash, expect);
    let hash: BytesN<32> = client.try_sha256(&input).unwrap().unwrap();
    assert_eq!(hash, expect);
    let hash: BytesN<32> = client.sha256_aliased(&input);
    assert_eq!(hash, expect);
    let hash: BytesN<32> = client.sha256_result(&input);
    assert_eq!(hash, expect);
    let hash: BytesN<32> = client.try_sha256_result(&input).unwrap().unwrap();
    assert_eq!(hash, expect);
    let hash: Option<BytesN<32>> = client.sha256_option(&input);
    assert_eq!(hash, Some(expect.clone()));
    let hash: Option<BytesN<32>> = client.try_sha256_option(&input).unwrap().unwrap();
    assert_eq!(hash, Some(expect.clone()));
    // An aliased Result is split into its ok and error types like any Result.
    let hash: BytesN<32> = client.sha256_result_aliased(&input);
    assert_eq!(hash, expect);
    let hash: BytesN<32> = client.try_sha256_result_aliased(&input).unwrap().unwrap();
    assert_eq!(hash, expect);
    let hash: (BytesN<32>, u32) = client.sha256_tuple(&input);
    assert_eq!(hash, (expect, 1));
}

/// The output types of the function's spec entry.
fn spec_outputs(spec_xdr: &[u8]) -> std::vec::Vec<xdr::ScSpecTypeDef> {
    let entry = xdr::ScSpecEntry::from_xdr(spec_xdr, xdr::Limits::none()).unwrap();
    let xdr::ScSpecEntry::FunctionV0(f) = entry else {
        panic!("expected a function spec entry");
    };
    f.outputs.to_vec()
}

#[test]
fn test_sha256_returned_from_contract_fn_spec() {
    // A Hash is a BytesN in the spec, so a client generated from the spec
    // returns a BytesN.
    let bytes_n_32 = xdr::ScSpecTypeDef::BytesN(xdr::ScSpecTypeBytesN { n: 32 });
    assert_eq!(
        spec_outputs(&Contract::spec_xdr_sha256()),
        [bytes_n_32.clone()]
    );
    assert_eq!(
        spec_outputs(&Contract::spec_xdr_sha256_aliased()),
        [bytes_n_32.clone()]
    );
    assert_eq!(
        spec_outputs(&Contract::spec_xdr_sha256_result()),
        [xdr::ScSpecTypeDef::Result(Box::new(
            xdr::ScSpecTypeResult {
                ok_type: Box::new(bytes_n_32.clone()),
                error_type: Box::new(xdr::ScSpecTypeDef::Error),
            }
        ))]
    );
    assert_eq!(
        spec_outputs(&Contract::spec_xdr_sha256_option()),
        [xdr::ScSpecTypeDef::Option(Box::new(
            xdr::ScSpecTypeOption {
                value_type: Box::new(bytes_n_32.clone()),
            }
        ))]
    );
    assert_eq!(
        spec_outputs(&Contract::spec_xdr_sha256_result_aliased()),
        spec_outputs(&Contract::spec_xdr_sha256_result())
    );
    assert_eq!(
        spec_outputs(&Contract::spec_xdr_sha256_tuple()),
        [xdr::ScSpecTypeDef::Tuple(Box::new(xdr::ScSpecTypeTuple {
            value_types: [bytes_n_32, xdr::ScSpecTypeDef::U32].try_into().unwrap(),
        }))]
    );
}

// A default fn of a contracttrait, whose types are named at the impl site
// through the trait's associated types, returns a Hash as a BytesN from the
// client too.
mod hash_trait {
    use crate as soroban_sdk;
    use soroban_sdk::{contracttrait, crypto::Hash, Bytes, Env};

    #[contracttrait]
    pub trait HashTrait {
        fn sha256(env: Env, data: Bytes) -> Hash<32> {
            env.crypto().sha256(&data)
        }

        fn sha256_option(env: Env, data: Bytes) -> Option<Hash<32>> {
            Some(env.crypto().sha256(&data))
        }
    }
}

#[contract]
pub struct TraitContract;

#[contractimpl(contracttrait)]
impl hash_trait::HashTrait for TraitContract {}

#[test]
fn test_sha256_returned_from_contracttrait_default_fn() {
    let env = Env::default();
    let contract_id = env.register(TraitContract, ());
    let client = TraitContractClient::new(&env, &contract_id);

    let input = bytes!(&env, 0x01);
    let expect = bytesn!(
        &env,
        0x4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a
    );
    let hash: BytesN<32> = client.sha256(&input);
    assert_eq!(hash, expect);
    let hash: Option<BytesN<32>> = client.sha256_option(&input);
    assert_eq!(hash, Some(expect));
}
