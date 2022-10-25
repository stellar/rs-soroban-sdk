use soroban_sdk::{
    accounts::AccountId, contracttype, Address, Bytes, BytesN, Env, RawVal, Symbol, Vec,
};

/// An Ed25519 signature contains a single signature for the
/// [`SignaturePayload`].
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype(lib = "soroban_auth")]
pub struct Ed25519Signature {
    pub public_key: BytesN<32>,
    pub signature: BytesN<64>,
}

/// Account signatures contains signatures for an account for a
/// [`SignaturePayload`].
///
/// Multiple signatures may be present within if the
/// account has multiple signers.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype(lib = "soroban_auth")]
pub struct AccountSignatures {
    pub account_id: AccountId,
    pub signatures: Vec<Ed25519Signature>,
}

/// Signature contains a signature of a [`SignaturePayload`] that can be
/// verified by [`verify`](crate::verify).
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype(lib = "soroban_auth")]
pub enum Signature {
    Invoker,
    Ed25519(Ed25519Signature),
    Account(AccountSignatures),
}

impl Signature {
    /// Returns the identifier that this signatures claims to authenticate.
    pub fn identifier(&self, env: &Env) -> Identifier {
        match self {
            Signature::Invoker => match env.invoker() {
                Address::Account(a) => Identifier::Account(a),
                Address::Contract(c) => Identifier::Contract(c),
            },
            Signature::Ed25519(e) => Identifier::Ed25519(e.public_key.clone()),
            Signature::Account(a) => Identifier::Account(a.account_id.clone()),
        }
    }
}

/// Identifier is an identifier for a authenticating party. Each [`Signature`]
/// has a corresponding identifier.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype(lib = "soroban_auth")]
pub enum Identifier {
    Contract(BytesN<32>),
    Ed25519(BytesN<32>),
    Account(AccountId),
}

impl From<AccountId> for Identifier {
    fn from(v: AccountId) -> Self {
        Identifier::Account(v)
    }
}

impl From<&AccountId> for Identifier {
    fn from(v: &AccountId) -> Self {
        Identifier::Account(v.clone())
    }
}

impl From<Address> for Identifier {
    fn from(addr: Address) -> Self {
        match addr {
            Address::Account(a) => Identifier::Account(a),
            Address::Contract(c) => Identifier::Contract(c),
        }
    }
}

impl From<&Address> for Identifier {
    fn from(addr: &Address) -> Self {
        Identifier::from(addr.clone())
    }
}

/// Signature payload v0 contains the data that must be signed to authenticate
/// the [`Identifier`] within when invoking a contract.
///
/// The data contained within includes a domain separator formed from the fields
/// below. The domain separator constrains where the signature is valid. It is
/// only valid for the invocation in the context of a contract defined name, of
/// a specific contract, on a specific network.
///
/// - `network`
///
///    The network passphrase for the network that the invocation is to occur.
///
/// - `contract`
///
///   The contract ID for the name being invoked.
///
/// - `name`
///
///   The name of the signing domain. Could be the name of the function to be
///   invoked, or some other name the contract has defined.
///
/// The data contained also includes all the arguments that are to be included
/// with the invocation. The arguments constrain what inputs may be provided to
/// the function. The signature over them ensures that the signer is approving
/// these inputs to accompany their authentication.
///
/// Applications using the signature payload must take care to only sign
/// argument lists for contracts by first constructing the [`SignaturePayload`]
/// and signing the whole payload only. Applications should never trust a
/// signature payload without either inspecting its entire contents, or building
/// it themselves.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype(lib = "soroban_auth")]
pub struct SignaturePayloadV0 {
    pub network: Bytes,
    pub contract: BytesN<32>,
    pub name: Symbol,
    pub args: Vec<RawVal>,
}

/// Signature payload contains the data that must be signed to authenticate the
/// [`Identifier`] within when invoking a contract.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype(lib = "soroban_auth")]
pub enum SignaturePayload {
    V0(SignaturePayloadV0),
}
