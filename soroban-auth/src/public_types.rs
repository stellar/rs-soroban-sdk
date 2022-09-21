use soroban_sdk::{contracttype, Bytes, BytesN, Env, RawVal, Symbol, Vec};

/// An Ed25519 signature contains a single signature for the
/// [`SignaturePayload`].
#[derive(Clone)]
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
#[derive(Clone)]
#[contracttype(lib = "soroban_auth")]
pub struct AccountSignatures {
    pub account_id: BytesN<32>,
    pub signatures: Vec<Ed25519Signature>,
}

/// Signature contains a signature of a [`SignaturePayload`] that can be
/// verified by [`verify`](crate::verify).
#[derive(Clone)]
#[contracttype(lib = "soroban_auth")]
pub enum Signature {
    Contract,
    Ed25519(Ed25519Signature),
    Account(AccountSignatures),
}

impl Signature {
    /// Returns the identifier that this signatures claims to authenticate.
    pub fn identifier(&self, env: &Env) -> Identifier {
        match self {
            Signature::Contract => Identifier::Contract(env.get_invoking_contract()),
            Signature::Ed25519(e) => Identifier::Ed25519(e.public_key.clone()),
            Signature::Account(a) => Identifier::Account(a.account_id.clone()),
        }
    }

    #[doc(hidden)]
    #[deprecated(note = "use Signature::identifier(...)")]
    pub fn get_identifier(&self, env: &Env) -> Identifier {
        self.identifier(env)
    }
}

/// Identifier is an identifier for a authenticating party. Each [`Signature`]
/// has a corresponding identifier.
#[derive(Clone, Eq, PartialEq)]
#[contracttype(lib = "soroban_auth")]
pub enum Identifier {
    Contract(BytesN<32>),
    Ed25519(BytesN<32>),
    Account(BytesN<32>),
}

/// Signature payload v0 contains the data that must be signed to authenticate
/// the [`Identifier`] within when invoking a contract.
///
/// The data contained within includes a domain separator formed from the fields
/// below. The domain separator constrains where the signature is valid. It is
/// only valid for the invocation of a specific function, of a specific
/// contract, on a specific network.
///
/// - `network`
///
///    The network passphrase for the network that the invocation is to occur.
///
/// - `contract`
///
///   The contract ID for the function being invoked.
///
/// - `function`
///
///   The symbol for the function being invoked.
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
#[derive(Clone)]
#[contracttype(lib = "soroban_auth")]
pub struct SignaturePayloadV0 {
    pub network: Bytes,
    pub contract: BytesN<32>,
    pub function: Symbol,
    pub args: Vec<RawVal>,
}

/// Signature payload contains the data that must be signed to authenticate the
/// [`Identifier`] within when invoking a contract.
#[derive(Clone)]
#[contracttype(lib = "soroban_auth")]
pub enum SignaturePayload {
    V0(SignaturePayloadV0),
}
