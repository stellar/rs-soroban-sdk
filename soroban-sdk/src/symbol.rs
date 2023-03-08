use core::{cmp::Ordering, convert::Infallible, fmt::Debug};

use super::{
    env::internal::{Env as _, EnvBase as _, Symbol as SymbolVal, SymbolSmall},
    ConversionError, Env, RawVal, TryFromVal, TryIntoVal,
};

#[cfg(not(target_family = "wasm"))]
use super::env::SymbolStr;

#[cfg(not(target_family = "wasm"))]
use crate::env::internal::xdr::{ScSymbol, ScVal};
use crate::{
    env::MaybeEnv,
    unwrap::{UnwrapInfallible, UnwrapOptimized},
};

/// Symbol is a short string with a limited character set.
///
/// Valid characters are `a-zA-Z0-9_` and maximum length is 32 characters.
///
/// Symbols are used for the for symbolic identifiers, such as function
/// names and user-defined structure field/enum variant names. That's why
/// these idenfiers have limited length.
///
/// While Symbols up to 30 characters long are allowed, Symbols that are 9
/// characters long or shorter are more efficient at runtime and also can be
/// computed at compile time.
#[derive(Clone)]
pub struct Symbol {
    env: MaybeEnv,
    val: SymbolVal,
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        #[cfg(target_family = "wasm")]
        write!(f, "Symbol(..)")?;
        #[cfg(not(target_family = "wasm"))]
        write!(f, "Symbol({})", self.to_string())?;
        Ok(())
    }
}

impl Eq for Symbol {}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_raw = self.val.to_raw();
        let other_raw = other.val.to_raw();

        match (
            SymbolSmall::try_from(self_raw),
            SymbolSmall::try_from(other_raw),
        ) {
            // Compare small symbols.
            (Ok(self_sym), Ok(other_sym)) => self_sym.cmp(&other_sym),
            // The object-to-small symbol comparisons are handled by `obj_cmp`,
            // so it's safe to handle all the other cases using it.
            _ => {
                let env: Option<Env> =
                    match (self.env.clone().try_into(), other.env.clone().try_into()) {
                        (Err(_), Err(_)) => None,
                        (Err(_), Ok(e)) => Some(e),
                        (Ok(e), Err(_)) => Some(e),
                        (Ok(e1), Ok(e2)) => {
                            e1.check_same_env(&e2);
                            Some(e1)
                        }
                    };
                if let Some(env) = env {
                    let v = env.obj_cmp(self_raw, other_raw).unwrap_infallible();
                    v.cmp(&0)
                } else {
                    panic!("symbol object is missing the env reference");
                }
            }
        }
    }
}

impl TryFromVal<Env, SymbolVal> for Symbol {
    type Error = Infallible;

    fn try_from_val(env: &Env, val: &SymbolVal) -> Result<Self, Self::Error> {
        Ok(unsafe { Symbol::unchecked_new(env.clone(), *val) })
    }
}

impl TryFromVal<Env, RawVal> for Symbol {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &RawVal) -> Result<Self, Self::Error> {
        Ok(SymbolVal::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl TryFromVal<Env, Symbol> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &Symbol) -> Result<Self, Self::Error> {
        Ok(v.to_raw())
    }
}

impl TryFromVal<Env, &Symbol> for RawVal {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &&Symbol) -> Result<Self, Self::Error> {
        Ok(v.to_raw())
    }
}

impl TryFromVal<Env, &str> for Symbol {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &&str) -> Result<Self, Self::Error> {
        Ok(SymbolVal::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<&Symbol> for ScVal {
    type Error = ConversionError;
    fn try_from(v: &Symbol) -> Result<Self, Self::Error> {
        if let Ok(ss) = SymbolSmall::try_from(v.val) {
            ScVal::try_from(ss)
        } else {
            let e: Env = v.env.try_into()?;
            ScVal::try_from_val(&e, &v.to_raw())
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFrom<Symbol> for ScVal {
    type Error = ConversionError;
    fn try_from(v: Symbol) -> Result<Self, Self::Error> {
        (&v).try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, Symbol> for ScVal {
    type Error = ConversionError;
    fn try_from_val(_e: &Env, v: &Symbol) -> Result<Self, Self::Error> {
        v.try_into()
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScVal> for Symbol {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScVal) -> Result<Self, Self::Error> {
        Ok(
            SymbolVal::try_from_val(env, &RawVal::try_from_val(env, val)?)?
                .try_into_val(env)
                .unwrap_infallible(),
        )
    }
}

#[cfg(not(target_family = "wasm"))]
impl TryFromVal<Env, ScSymbol> for Symbol {
    type Error = ConversionError;
    fn try_from_val(env: &Env, val: &ScSymbol) -> Result<Self, Self::Error> {
        Ok(SymbolVal::try_from_val(env, val)?
            .try_into_val(env)
            .unwrap_infallible())
    }
}

impl Symbol {
    /// Creates a new Symbol given a string with valid characters.
    ///
    /// Valid characters are `a-zA-Z0-9_` and maximum string length is 32
    /// characters.
    ///
    /// Use `Symbol::try_from_val(env, s)`/`s.try_into_val(env)` in case if
    /// failures need to be handled gracefully.
    ///
    /// ### Panics
    ///
    /// When the input string is not representable by Symbol.
    pub fn new(env: &Env, s: &str) -> Self {
        Self {
            env: env.clone().into(),
            val: s.try_into_val(env).unwrap_optimized(),
        }
    }

    /// Creates a new Symbol given a short string with valid characters.
    ///
    /// Valid characters are `a-zA-Z0-9_` and maximum length is 9 characters.
    ///
    /// The conversion happens at compile time.
    ///
    /// ### Panics
    ///
    /// When the input string is not representable by Symbol.
    pub const fn short(s: &str) -> Self {
        if let Ok(sym) = SymbolSmall::try_from_str(s) {
            Symbol {
                env: MaybeEnv::none(),
                val: SymbolVal::from_small(sym),
            }
        } else {
            panic!("short symbols are limited to 9 characters");
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn unchecked_new(env: Env, val: SymbolVal) -> Self {
        Self {
            env: env.into(),
            val,
        }
    }

    pub fn as_raw(&self) -> &RawVal {
        self.val.as_raw()
    }

    pub fn to_raw(&self) -> RawVal {
        self.val.to_raw()
    }

    pub fn to_val(&self) -> SymbolVal {
        self.val
    }
}

#[cfg(not(target_family = "wasm"))]
extern crate std;
#[cfg(not(target_family = "wasm"))]
impl ToString for Symbol {
    fn to_string(&self) -> String {
        if let Ok(s) = SymbolSmall::try_from(self.val) {
            s.to_string()
        } else {
            let e: Env = self.env.try_into().unwrap_optimized();
            SymbolStr::try_from_val(&e, &self.val)
                .unwrap_optimized()
                .to_string()
        }
    }
}
