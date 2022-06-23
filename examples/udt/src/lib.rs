#![no_std]
use stellar_contract_sdk::{contractfn, Env, EnvVal, IntoEnvVal, RawVal};

pub struct Udt {
    pub a: i64,
    pub b: i64,
}

// TODO: These trait implementations will be hidden behind a macro, and probably
// be implemented using a Map rather than a tuple to provide for better data
// migration.

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

#[contractfn]
pub fn add(udt: Udt) -> i64 {
    udt.a + udt.b
}

#[cfg(test)]
mod test {
    use super::{Udt, __add};
    use stellar_contract_sdk::{Env, IntoVal, TryFromVal};

    #[test]
    fn test_add() {
        let e = Env::default();
        let udt = Udt { a: 10, b: 12 }.into_val(&e);
        let z = __add(e.clone(), udt);
        let z = i64::try_from_val(&e, z).unwrap();
        assert!(z == 22);
    }
}
