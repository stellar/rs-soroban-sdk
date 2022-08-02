use crate::{env::internal::Env as _, Binary, Env, IntoVal, RawVal, TryFromVal};

pub trait Serialize {
    fn serialize(self, env: &Env) -> Binary;
}

pub trait Deserialize: Sized {
    type Error;
    fn deserialize(env: &Env, b: &Binary) -> Result<Self, Self::Error>;
}

impl<T> Serialize for T
where
    T: IntoVal<Env, RawVal>,
{
    fn serialize(self, env: &Env) -> Binary {
        let val: RawVal = self.into_val(env);
        let bin = env.serialize_to_binary(val);
        unsafe { Binary::unchecked_new(bin.in_env(env)) }
    }
}

impl<T> Deserialize for T
where
    T: TryFromVal<Env, RawVal>,
{
    type Error = T::Error;

    fn deserialize(env: &Env, b: &Binary) -> Result<Self, Self::Error> {
        let t = env.deserialize_from_binary(b.into());
        T::try_from_val(env, t)
    }
}
