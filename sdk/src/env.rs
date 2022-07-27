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

    #[doc(hidden)]
    impl<F, T> TryConvert<F, T> for super::Env
    where
        EnvImpl: TryConvert<F, T>,
    {
        type Error = <EnvImpl as TryConvert<F, T>>::Error;
        fn convert(&self, f: F) -> Result<T, Self::Error> {
            self.env_impl.convert(f)
        }
    }
}

pub use internal::meta;
pub use internal::xdr;
pub use internal::BitSet;
pub use internal::ConversionError;
pub use internal::EnvBase;
pub use internal::IntoVal;
pub use internal::Object;
pub use internal::RawVal;
pub use internal::RawValConvertible;
pub use internal::Status;
pub use internal::Symbol;
pub use internal::TryFromVal;
pub use internal::TryIntoVal;
pub use internal::Val;

pub type EnvType<V> = internal::EnvVal<Env, V>;
pub type EnvVal = internal::EnvVal<Env, RawVal>;
pub type EnvObj = internal::EnvVal<Env, Object>;

use crate::binary::{Binary, FixedBinary};

#[derive(Clone)]
pub struct Env {
    env_impl: internal::EnvImpl,
}

impl Default for Env {
    #[cfg(not(feature = "testutils"))]
    fn default() -> Self {
        Self {
            env_impl: Default::default(),
        }
    }

    #[cfg(feature = "testutils")]
    fn default() -> Self {
        Self::with_empty_recording_storage()
    }
}

impl Env {
    pub fn invoke_contract<T: TryFromVal<Env, RawVal>>(
        &self,
        contract_id: Binary,
        func: Symbol,
        args: crate::vec::Vec<EnvVal>,
    ) -> T {
        let rv = internal::Env::call(
            self,
            RawVal::from(contract_id).try_into().unwrap(),
            func,
            args.to_object(),
        );
        T::try_from_val(&self, rv).map_err(|_| ()).unwrap()
    }

    pub fn get_current_contract(&self) -> FixedBinary<32> {
        internal::Env::get_current_contract(self)
            .in_env(self)
            .try_into()
            .unwrap()
    }

    pub fn get_invoking_contract(&self) -> FixedBinary<32> {
        let rv = internal::Env::get_invoking_contract(self).to_raw();
        let bin = Binary::try_from_val(self, rv).unwrap();
        bin.try_into().unwrap()
    }

    pub fn has_contract_data<K>(&self, key: K) -> bool
    where
        K: IntoVal<Env, RawVal>,
    {
        let rv = internal::Env::has_contract_data(self, key.into_val(self));
        rv.try_into().unwrap()
    }

    pub fn get_contract_data<K, V>(&self, key: K) -> V
    where
        V::Error: Debug,
        K: IntoVal<Env, RawVal>,
        V: TryFromVal<Env, RawVal>,
    {
        let rv = internal::Env::get_contract_data(self, key.into_val(self));
        V::try_from_val(self, rv).unwrap()
    }

    pub fn put_contract_data<K, V>(&self, key: K, val: V)
    where
        K: IntoVal<Env, RawVal>,
        V: IntoVal<Env, RawVal>,
    {
        internal::Env::put_contract_data(self, key.into_val(self), val.into_val(self));
    }

    pub fn del_contract_data<K>(&self, key: K)
    where
        K: IntoVal<Env, RawVal>,
    {
        internal::Env::del_contract_data(self, key.into_val(self));
    }

    pub fn compute_hash_sha256(&self, msg: Binary) -> Binary {
        let bin_obj = internal::Env::compute_hash_sha256(self, msg.into());
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

    #[doc(hidden)]
    pub fn create_contract_from_contract(&self, contract: Binary, salt: Binary) -> FixedBinary<32> {
        let contract_obj: Object = RawVal::from(contract).try_into().unwrap();
        let salt_obj: Object = RawVal::from(salt).try_into().unwrap();
        let id_obj = internal::Env::create_contract_from_contract(self, contract_obj, salt_obj);
        id_obj.in_env(self).try_into().unwrap()
    }

    #[doc(hidden)]
    pub fn binary_new_from_linear_memory(&self, ptr: u32, len: u32) -> Binary {
        let bin_obj = internal::Env::binary_new_from_linear_memory(self, ptr.into(), len.into());
        bin_obj.in_env(self).try_into().unwrap()
    }

    #[doc(hidden)]
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

    #[doc(hidden)]
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

    #[doc(hidden)]
    pub fn log_value<V: IntoVal<Env, RawVal>>(&self, v: V) {
        internal::Env::log_value(self, v.into_val(self));
    }
}

#[cfg(feature = "testutils")]
use crate::testutils::ContractFunctionSet;
#[cfg(feature = "testutils")]
use std::rc::Rc;
#[cfg(feature = "testutils")]
#[cfg_attr(feature = "docs", doc(cfg(feature = "testutils")))]
impl Env {
    fn with_empty_recording_storage() -> Env {
        struct EmptySnapshotSource();

        impl internal::storage::SnapshotSource for EmptySnapshotSource {
            fn get(
                &self,
                _key: &xdr::LedgerKey,
            ) -> Result<xdr::LedgerEntry, stellar_contract_env_host::HostError> {
                use xdr::{ScHostStorageErrorCode, ScStatus};
                let status: internal::Status =
                    ScStatus::HostStorageError(ScHostStorageErrorCode::UnknownError).into();
                Err(status.into())
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

    pub fn register_contract<T: ContractFunctionSet + 'static>(
        &self,
        contract_id: Binary,
        contract: T,
    ) {
        struct InternalContractFunctionSet<T: ContractFunctionSet>(pub(crate) T);
        impl<T: ContractFunctionSet> internal::ContractFunctionSet for InternalContractFunctionSet<T> {
            fn call(
                &self,
                func: &Symbol,
                env_impl: &internal::EnvImpl,
                args: &[RawVal],
            ) -> Option<RawVal> {
                self.0.call(func, Env::with_impl(env_impl.clone()), args)
            }
        }

        let id_obj: Object = RawVal::from(contract_id).try_into().unwrap();
        self.env_impl
            .register_test_contract(id_obj, Rc::new(InternalContractFunctionSet(contract)))
            .unwrap();
    }

    #[doc(hidden)]
    pub fn invoke_contract_external_raw(
        &self,
        hf: xdr::HostFunction,
        args: xdr::ScVec,
    ) -> RawVal {
        self.env_impl.invoke_function_raw(hf, args).unwrap()
    }

    #[doc(hidden)]
    pub fn invoke_contract_external(
        &self,
        hf: xdr::HostFunction,
        args: xdr::ScVec,
    ) -> xdr::ScVal {
        self.env_impl.invoke_function(hf, args).unwrap()
    }

    #[cfg(not(target_family = "wasm"))]
    fn clone_self_and_catch_panic<F, T>(&self, f: F) -> (Env, std::thread::Result<T>)
    where
        F: FnOnce(Env) -> T,
    {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| ()));
        let deep_clone = self.deep_clone();
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(deep_clone.clone())));
        std::panic::set_hook(hook);
        (deep_clone, res)
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn assert_panic_with_string<F, T: Debug>(&self, s: &str, f: F)
    where
        F: FnOnce(Env) -> T,
    {
        match self.clone_self_and_catch_panic(f) {
            (_, Ok(v)) => panic!("inner function expected to panic, but returned {:?}", v),
            (_, Err(e)) => match e.downcast_ref::<String>() {
                None => match e.downcast_ref::<&str>() {
                    Some(ps) => assert_eq!(*ps, s),
                    None => panic!(
                        "inner function panicked with unknown type when \"{}\" expected",
                        s
                    ),
                },
                Some(ps) => assert_eq!(*ps, s),
            },
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn assert_panic_with_status<F, T: Debug>(&self, status: Status, f: F)
    where
        F: FnOnce(Env) -> T,
    {
        use stellar_contract_env_host::events::{DebugArg, HostEvent};

        match self.clone_self_and_catch_panic(f) {
            (_, Ok(v)) => panic!("inner function expected to panic, but returned {:?}", v),
            (clone, Err(e)) => {
                // Allow if there was a panic literally _carrying_ the status requested.
                if let Some(st) = e.downcast_ref::<Status>() {
                    assert_eq!(*st, status);
                    return;
                }
                // Allow if the last debug log entry contains the status of requested.
                if let Some(HostEvent::Debug(dbg)) = clone.env_impl.get_events().0.last() {
                    for arg in dbg.args.iter() {
                        if let DebugArg::Val(v) = arg {
                            if let Ok(st) = TryInto::<Status>::try_into(*v) {
                                if st == status {
                                    return;
                                }
                            }
                        }
                    }
                }

                // Otherwise we're going to fail but we'll try to produce a useful diagnostic if
                // the panic was a string, which many are.
                if let Some(s) = e.downcast_ref::<String>() {
                    panic!(
                        "inner function panicked with \"{}\" when status {:?} expected",
                        s, status
                    );
                }
                if let Some(s) = e.downcast_ref::<&str>() {
                    panic!(
                        "inner function panicked with \"{}\" when status {:?} expected",
                        s, status
                    );
                }
                panic!(
                    "inner function panicked with unknown type when status {:?} expected",
                    status
                );
            }
        }
    }
}

#[doc(hidden)]
impl Env {
    pub fn with_impl(env_impl: internal::EnvImpl) -> Env {
        Env { env_impl }
    }
}

#[doc(hidden)]
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

    fn binary_copy_from_slice(&self, _: Object, _: RawVal, _: &[u8]) -> Object {
        unimplemented!()
    }

    fn binary_copy_to_slice(&self, _: Object, _: RawVal, _: &mut [u8]) {
        unimplemented!()
    }

    fn binary_new_from_slice(&self, _: &[u8]) -> Object {
        unimplemented!()
    }

    fn log_static_fmt_val(&self, _: &'static str, _: RawVal) {
        unimplemented!()
    }

    fn log_static_fmt_static_str(&self, _: &'static str, _: &'static str) {
        unimplemented!()
    }

    fn log_static_fmt_val_static_str(&self, _: &'static str, _: RawVal, _: &'static str) {
        unimplemented!()
    }

    fn log_static_fmt_general(&self, _: &'static str, _: &[RawVal], _: &[&'static str]) {
        unimplemented!()
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
        #[doc(hidden)]
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
