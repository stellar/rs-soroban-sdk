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

pub use internal::xdr;
pub use internal::BitSet;
pub use internal::EnvBase;
pub use internal::IntoEnvVal;
pub use internal::IntoVal;
use internal::Object;
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

#[derive(Clone, Default)]
pub struct Env {
    env_impl: internal::EnvImpl,
}

impl Env {
    // TODO: Implement methods on Env that are intended for use by contract
    // developers and that otherwise don't belong into an object like Vec, Map,
    // BigInt, etc. If there is any host fn we expect a developer to use, it
    // should be plumbed through this type with this type doing all RawVal
    // conversion.

    pub fn put_contract_data<K: IntoTryFromVal, V: IntoTryFromVal>(&self, key: K, val: V) {
        internal::Env::put_contract_data(self, key.into_val(self), val.into_val(self));
    }

    pub fn del_contract_data<K: IntoTryFromVal>(&self, key: K) {
        internal::Env::del_contract_data(self, key.into_val(self));
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
