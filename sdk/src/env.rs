use core::fmt::Debug;

#[cfg(target_family = "wasm")]
pub mod internal {
    pub use stellar_contract_env_guest::*;
    pub type EnvImpl = Guest;
}

#[cfg(not(target_family = "wasm"))]
pub mod internal {
    pub use stellar_contract_env_host::*;
    pub type EnvImpl = Host;
}

pub use internal::meta;
pub use internal::xdr;
pub use internal::BitSet;
pub use internal::ConversionError;
pub use internal::EnvBase;
pub use internal::IntoEnvVal;
pub use internal::IntoVal;
pub use internal::Object;
pub use internal::RawVal;
pub use internal::Status;
pub use internal::Symbol;
pub use internal::TaggedVal;
pub use internal::TryFromVal;
pub use internal::Val;

pub type EnvVal = internal::EnvVal<Env, RawVal>;
pub type EnvObj = internal::EnvVal<Env, Object>;

pub trait IntoTryFromVal: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> {}
impl<C> IntoTryFromVal for C where C: IntoVal<Env, RawVal> + TryFromVal<Env, RawVal> {}

use crate::binary::{ArrayBinary, Binary};

#[derive(Clone, Default)]
pub struct Env {
    env_impl: internal::EnvImpl,
}

impl Env {
    pub fn with_impl(env_impl: internal::EnvImpl) -> Env {
        Env { env_impl }
    }

    // TODO: Implement methods on Env that are intended for use by contract
    // developers and that otherwise don't belong into an object like Vec, Map,
    // BigInt, etc. If there is any host fn we expect a developer to use, it
    // should be plumbed through this type with this type doing all RawVal
    // conversion.

    pub fn get_invoking_contract(&self) -> ArrayBinary<32> {
        let rv = internal::Env::get_invoking_contract(self).to_raw();
        let bin = Binary::try_from_val(self, rv).unwrap();
        bin.try_into().unwrap()
    }

    pub fn has_contract_data<K: IntoVal<Env, RawVal>>(&self, key: K) -> bool {
        let rv = internal::Env::has_contract_data(self, key.into_val(self));
        rv.try_into().unwrap()
    }

    pub fn get_contract_data<K: IntoVal<Env, RawVal>, V: IntoTryFromVal>(&self, key: K) -> V
    where
        V::Error: Debug,
    {
        let rv = internal::Env::get_contract_data(self, key.into_val(self));
        V::try_from_val(self, rv).unwrap()
    }

    pub fn put_contract_data<K: IntoVal<Env, RawVal>, V: IntoTryFromVal>(&self, key: K, val: V) {
        internal::Env::put_contract_data(self, key.into_val(self), val.into_val(self));
    }

    pub fn del_contract_data<K: IntoVal<Env, RawVal>>(&self, key: K) {
        internal::Env::del_contract_data(self, key.into_val(self));
    }

    pub fn serialize_to_binary<V: IntoTryFromVal>(&self, val: V) -> Binary {
        let val_obj: Object = val.into_val(self).try_into().unwrap();
        let bin_obj = internal::Env::serialize_to_binary(self, val_obj.to_raw());
        bin_obj.in_env(self).try_into().unwrap()
    }

    pub fn deserialize_from_binary<V: IntoTryFromVal>(&self, bin: Binary) -> V
    where
        V::Error: Debug,
    {
        let bin_obj: Object = RawVal::from(bin).try_into().unwrap();
        let val_obj = internal::Env::deserialize_from_binary(self, bin_obj);
        V::try_from_val(self, val_obj).unwrap()
    }

    pub fn compute_hash_sha256(&self, msg: Binary) -> Binary {
        let bin_obj = internal::Env::compute_hash_sha256(self, msg.into_val(self));
        bin_obj.in_env(self).try_into().unwrap()
    }

    pub fn verify_sig_ed25519(&self, sig: Binary, pk: Binary, msg: Binary) {
        let sig_obj: Object = RawVal::from(sig).try_into().unwrap();
        let pk_obj: Object = RawVal::from(pk).try_into().unwrap();
        let msg_obj: Object = RawVal::from(msg).try_into().unwrap();
        internal::Env::verify_sig_ed25519(self, msg_obj, pk_obj, sig_obj)
            .try_into()
            .unwrap()
    }

    pub fn account_get_low_threshold(&self, acc: Binary) -> u32 {
        let acc_obj: Object = RawVal::from(acc).try_into().unwrap();
        internal::Env::account_get_low_threshold(self, acc_obj)
            .try_into()
            .unwrap()
    }

    pub fn account_get_medium_threshold(&self, acc: Binary) -> u32 {
        let acc_obj: Object = RawVal::from(acc).try_into().unwrap();
        internal::Env::account_get_medium_threshold(self, acc_obj)
            .try_into()
            .unwrap()
    }

    pub fn account_get_high_threshold(&self, acc: Binary) -> u32 {
        let acc_obj: Object = RawVal::from(acc).try_into().unwrap();
        internal::Env::account_get_high_threshold(self, acc_obj)
            .try_into()
            .unwrap()
    }

    pub fn account_get_signer_weight(&self, acc: Binary, signer: Binary) -> u32 {
        let acc_obj: Object = RawVal::from(acc).try_into().unwrap();
        let signer_obj: Object = RawVal::from(signer).try_into().unwrap();
        internal::Env::account_get_signer_weight(self, acc_obj, signer_obj)
            .try_into()
            .unwrap()
    }

    pub fn create_contract_from_contract(&self, contract: Binary, salt: Binary) {
        let contract_obj: Object = RawVal::from(contract).try_into().unwrap();
        let salt_obj: Object = RawVal::from(salt).try_into().unwrap();
        internal::Env::create_contract_from_contract(self, contract_obj, salt_obj);
    }

    pub fn binary_new_from_linear_memory(&self, ptr: u32, len: u32) -> Binary {
        let bin_obj = internal::Env::binary_new_from_linear_memory(self, ptr.into(), len.into());
        bin_obj.in_env(self).try_into().unwrap()
    }

    pub fn binary_copy_to_linear_memory(&self, bin: Binary, b_pos: u32, lm_pos: u32, len: u32) {
        let bin_obj: Object = RawVal::from(bin).try_into().unwrap();
        internal::Env::binary_copy_to_linear_memory(
            self,
            bin_obj,
            b_pos.into(),
            lm_pos.into(),
            len.into(),
        );
    }

    pub fn binary_copy_from_linear_memory(
        &self,
        bin: Binary,
        b_pos: u32,
        lm_pos: u32,
        len: u32,
    ) -> Binary {
        let bin_obj: Object = RawVal::from(bin).try_into().unwrap();
        let new_obj = internal::Env::binary_copy_from_linear_memory(
            self,
            bin_obj,
            b_pos.into(),
            lm_pos.into(),
            len.into(),
        );
        new_obj.in_env(self).try_into().unwrap()
    }
}

#[cfg(feature = "testutils")]
use crate::TestContract;
#[cfg(feature = "testutils")]
use std::rc::Rc;
#[cfg(feature = "testutils")]
impl Env {
    pub fn with_empty_recording_storage() -> Env {
        struct EmptySnapshotSource();

        impl internal::storage::SnapshotSource for EmptySnapshotSource {
            fn get(
                &self,
                _key: &xdr::LedgerKey,
            ) -> Result<xdr::LedgerEntry, stellar_contract_env_host::HostError> {
                Err(internal::HostError::General("not found"))
            }

            fn has(
                &self,
                _key: &xdr::LedgerKey,
            ) -> Result<bool, stellar_contract_env_host::HostError> {
                Ok(false)
            }
        }

        let rf = Rc::new(EmptySnapshotSource());
        let storage = internal::storage::Storage::with_recording_footprint(rf);
        Env {
            env_impl: internal::EnvImpl::with_storage(storage),
        }
    }

    pub fn register_contract(self, contract_id: RawVal, contract: TestContract) {
        let id_obj: Object = contract_id.try_into().unwrap();
        self.env_impl
            .register_test_contract(id_obj, Rc::new(contract))
            .unwrap();
    }

    pub fn invoke_contract(&mut self, hf: xdr::HostFunction, args: xdr::ScVec) -> xdr::ScVal {
        self.env_impl.invoke_function(hf, args).unwrap()
    }
}

impl internal::EnvBase for Env {
    fn as_mut_any(&mut self) -> &mut dyn core::any::Any {
        self
    }

    fn check_same_env(&self, other: &Self) {
        self.env_impl.check_same_env(&other.env_impl);
    }

    fn deep_clone(&self) -> Self {
        Env {
            env_impl: self.env_impl.deep_clone(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
/// X-macro use: impl Env for SDK's Env
///////////////////////////////////////////////////////////////////////////////

// This is a helper macro used only by impl_env_for_sdk below. It consumes a
// token-tree of the form:
//
//  {fn $fn_id:ident $args:tt -> $ret:ty}
//
// and produces the the corresponding method definition to be used in the
// SDK's Env implementation of the Env (calling through to the corresponding
// guest or host implementation).
macro_rules! sdk_function_helper {
    {$mod_id:ident, fn $fn_id:ident($($arg:ident:$type:ty),*) -> $ret:ty}
    =>
    {
        fn $fn_id(&self, $($arg:$type),*) -> $ret {
            self.env_impl.$fn_id($($arg),*)
        }
    };
}

// This is a callback macro that pattern-matches the token-tree passed by the
// x-macro (call_macro_with_all_host_functions) and produces a suite of
// forwarding-method definitions, which it places in the body of the declaration
// of the implementation of Env for the SDK's Env.
macro_rules! impl_env_for_sdk {
    {
        $(
            // This outer pattern matches a single 'mod' block of the token-tree
            // passed from the x-macro to this macro. It is embedded in a `$()*`
            // pattern-repetition matcher so that it will match all provided
            // 'mod' blocks provided.
            $(#[$mod_attr:meta])*
            mod $mod_id:ident $mod_str:literal
            {
                $(
                    // This inner pattern matches a single function description
                    // inside a 'mod' block in the token-tree passed from the
                    // x-macro to this macro. It is embedded in a `$()*`
                    // pattern-repetition matcher so that it will match all such
                    // descriptions.
                    $(#[$fn_attr:meta])*
                    { $fn_str:literal, fn $fn_id:ident $args:tt -> $ret:ty }
                )*
            }
        )*
    }

    =>  // The part of the macro above this line is a matcher; below is its expansion.

    {
        // This macro expands to a single item: the implementation of Env for
        // the SDK's Env struct used by client contract code running in a WASM VM.
        impl internal::Env for Env
        {
            $(
                $(
                    // This invokes the guest_function_helper! macro above
                    // passing only the relevant parts of the declaration
                    // matched by the inner pattern above. It is embedded in two
                    // nested `$()*` pattern-repetition expanders that
                    // correspond to the pattern-repetition matchers in the
                    // match section, but we ignore the structure of the 'mod'
                    // block repetition-level from the outer pattern in the
                    // expansion, flattening all functions from all 'mod' blocks
                    // into the implementation of Env for Guest.
                    sdk_function_helper!{$mod_id, fn $fn_id $args -> $ret}
                )*
            )*
        }
    };
}

// Here we invoke the x-macro passing generate_env_trait as its callback macro.
internal::call_macro_with_all_host_functions! { impl_env_for_sdk }
