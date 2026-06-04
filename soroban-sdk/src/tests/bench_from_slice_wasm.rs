//! WASM compute-budget bench for [`soroban_sdk::Vec::from_slice`].
//!
//! Measures real guest WASM execution cost (the host budget's CPU instructions
//! and memory bytes) for `Vec::from_slice`, in the same spirit as the
//! `BytesN -> [u8; N]` bench that motivated PR #1888.
//!
//! `Vec::from_slice` is reported alongside:
//! - its `baseline_*` twin (dispatch + array setup only), and
//! - `Vec::from_array`, which builds the same Vec with a single bulk host call.
//!
//! Subtracting the baseline isolates the cost of the construction; the gap
//! between `from_slice` and `from_array` is the inefficiency.
//!
//! Disabled by default (`#[ignore]`) because they should run under `--release`
//! and only produce meaningful output with `--nocapture`.
//!
//! Run with:
//!   make build-test-wasms
//!   cargo test --release --package soroban-sdk --lib --features testutils \
//!     -- tests::bench_from_slice_wasm --ignored --nocapture

use crate::Env;

mod bench {
    use crate as soroban_sdk;
    soroban_sdk::contractimport!(
        file = "../target/wasm32v1-none/release/test_bench_from_slice.wasm"
    );
}

fn report(label: &str, env: &Env) {
    let cpu = env.cost_estimate().budget().cpu_instruction_cost();
    let mem = env.cost_estimate().budget().memory_bytes_cost();
    println!("BENCH {label} cpu={cpu} mem={mem}");
}

macro_rules! bench_call {
    ($bench:ident, $fn_:ident) => {
        #[test]
        #[ignore]
        fn $bench() {
            let env = Env::default();
            let id = env.register(bench::WASM, ());
            let client = bench::Client::new(&env, &id);
            env.cost_estimate().budget().reset_unlimited();
            let _ = client.$fn_();
            report(stringify!($bench), &env);
        }
    };
}

bench_call!(bench_baseline_32, baseline_32);
bench_call!(bench_baseline_96, baseline_96);
bench_call!(bench_baseline_192, baseline_192);

bench_call!(bench_vec_from_slice_32, vec_from_slice_32);
bench_call!(bench_vec_from_slice_96, vec_from_slice_96);
bench_call!(bench_vec_from_slice_192, vec_from_slice_192);

bench_call!(bench_vec_from_array_32, vec_from_array_32);
bench_call!(bench_vec_from_array_96, vec_from_array_96);
bench_call!(bench_vec_from_array_192, vec_from_array_192);
