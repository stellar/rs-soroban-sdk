use soroban_sdk::{contract, contractimpl, contracttrait, Env};

#[contracttrait]
pub trait DefaultCfg {
    #[cfg(any())]
    fn hidden(env: Env) -> u32 {
        let _ = env;
        7
    }
}

#[contracttrait]
pub trait DefaultCfgAttr {
    #[cfg_attr(any(), allow(dead_code))]
    fn hidden(env: Env) -> u32 {
        let _ = env;
        7
    }
}

#[contracttrait]
pub trait OverrideCfgAttr {
    fn hidden(env: Env) -> u32 {
        let _ = env;
        7
    }
}

#[contract]
pub struct C;

#[contractimpl(contracttrait)]
impl OverrideCfgAttr for C {
    #[cfg_attr(any(), allow(dead_code))]
    fn hidden(env: Env) -> u32 {
        let _ = env;
        8
    }
}

// Implementing a trait whose `#[contracttrait]` expansion failed should surface
// only the trait's own diagnostic, not a `cannot find macro`/`cannot find trait`
// cascade from the dropped trait declaration.
#[contract]
pub struct D;

#[contractimpl(contracttrait)]
impl DefaultCfgAttr for D {
    fn hidden(env: Env) -> u32 {
        let _ = env;
        9
    }
}

fn main() {}
