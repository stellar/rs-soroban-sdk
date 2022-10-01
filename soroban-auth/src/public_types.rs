use soroban_sdk::{contracttype, AccountId, BytesN, Env, Invoker, Vec};

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
                Invoker::Account(a) => Identifier::Account(a),
                Invoker::Contract(c) => Identifier::Contract(c),
            },
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
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype(lib = "soroban_auth")]
pub enum Identifier {
    Contract(BytesN<32>),
    Ed25519(BytesN<32>),
    Account(AccountId),
}
