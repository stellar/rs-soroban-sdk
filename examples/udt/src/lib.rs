#![no_std]
use stellar_contract_sdk::{Env, EnvVal, IntoEnvVal, IntoVal, RawVal, TryFromVal};

pub struct Udt {
    pub a: i64,
    pub b: i64,
}

impl TryFrom<EnvVal> for Udt {
    type Error = ();

    fn try_from(ev: EnvVal) -> Result<Self, Self::Error> {
        let (a, b): (i64, i64) = ev.try_into()?;
        Ok(Udt { a, b })
    }
}

impl IntoEnvVal<Env, RawVal> for Udt {
    fn into_env_val(self, env: &Env) -> EnvVal {
        (self.a, self.b).into_env_val(env)
    }
}

#[no_mangle]
pub fn add(e: Env, udt: RawVal) -> RawVal {
    let udt: Udt = Udt::try_from_val(&e, udt).unwrap();

    let c = udt.a + udt.b;

    return c.into_val(&e);
}

#[cfg(test)]
mod test {
    use super::{add, Udt};
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add() {
        let e = Env::default();
        let udt = Udt { a: 10, b: 12 }.into_val(&e);
        let z = add(e.clone(), udt);
        let z = i64::try_from_val(&e, z).unwrap();
        assert!(z == 22);
    }
}
