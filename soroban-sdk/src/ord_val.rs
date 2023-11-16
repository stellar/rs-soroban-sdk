use crate::{
    env::internal::Env as _, unwrap::UnwrapInfallible, ConversionError, Env, TryFromVal, Val,
};
use core::{cmp::Ordering, fmt::Debug, ops::Deref};

#[derive(Clone)]
pub struct OrdVal {
    env: Env,
    val: Val,
}

impl Deref for OrdVal {
    type Target = Val;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl Debug for OrdVal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.val.fmt(f)
    }
}

impl Eq for OrdVal {}

impl PartialEq for OrdVal {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for OrdVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for OrdVal {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let v = self.env.obj_cmp(self.val, other.val).unwrap_infallible();
        v.cmp(&0)
    }
}

impl From<OrdVal> for Val {
    fn from(v: OrdVal) -> Val {
        v.to_val()
    }
}

impl TryFromVal<Env, Val> for OrdVal {
    type Error = ConversionError;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        Ok(OrdVal::new(env, val.clone()))
    }
}

impl OrdVal {
    #[inline(always)]
    pub fn new(env: &Env, val: Val) -> Self {
        Self {
            env: env.clone(),
            val,
        }
    }

    #[inline(always)]
    pub fn env(&self) -> &Env {
        &self.env
    }

    pub fn as_val(&self) -> &Val {
        &self.val
    }

    pub fn to_val(&self) -> Val {
        self.val
    }
}
