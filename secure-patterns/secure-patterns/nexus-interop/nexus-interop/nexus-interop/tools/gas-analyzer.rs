use soroban_sdk::{Env, Symbol};

pub fn log_gas_usage(e: &Env, tag: Symbol) {
    let cpu = e.cost_estimate().cpu_insns;
    let mem = e.cost_estimate().mem_bytes;
    e.events().publish(tag, (cpu, mem));
}
