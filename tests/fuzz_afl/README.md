# test_fuzz_afl

A minimal Soroban contract, plus a fuzz test for it that runs under
[AFL++] via the [afl.rs] crate (the `cargo-afl` tooling).

If you have never fuzzed anything before, this README is for you: follow it
top to bottom and you will watch a fuzzer find a bug in the contract in
`src/lib.rs`.

[AFL++]: https://aflplus.plus
[afl.rs]: https://github.com/rust-fuzz/afl.rs

## What is fuzzing?

A fuzzer runs your code over and over with generated input, watching which
branches the code takes, and mutating the input it feeds in so as to reach
branches it has not reached yet. When an input makes the code panic, the
fuzzer saves that input to disk so you can replay it. It is a way of finding
the inputs you did not think to write a test for.

Two fuzzers are commonly used with Rust, and the SDK supports both:

- **libFuzzer**, driven by `cargo-fuzz`. See the `tests/fuzz` crate next
  door for the same example written for it. It requires a nightly compiler.
- **AFL++**, driven by `cargo-afl`, which is what this crate uses. It works
  on stable Rust, runs each input in a separate forked process (so a crash
  cannot corrupt the fuzzer itself), and has a terminal UI that shows what
  the campaign is doing.

Neither is better; they explore differently, and fuzzing a contract with
both finds more than fuzzing it with either.

## What is in this crate

```
tests/fuzz_afl
├── Cargo.toml              the contract crate, a normal member of the workspace
├── src/lib.rs              the contract being fuzzed
└── fuzz                    the fuzz crate: a separate crate, and its own workspace
    ├── Cargo.toml
    ├── in/seed             the starting input the fuzzer mutates ("seed corpus")
    └── src/fuzz_target_1.rs   the fuzz test itself
```

The contract has one function that panics on some inputs and not others:

```rust
pub fn run(a: U256, b: U256) {
    if a < b {
        panic!("unexpected")
    }
}
```

The fuzz test in `fuzz/src/fuzz_target_1.rs` turns the fuzzer's raw bytes
into two `U256` values and calls `run` with them. The panic is the bug we
expect the fuzzer to find.

The fuzz crate is deliberately *not* part of the repository's Cargo
workspace — it declares its own `[workspace]` — because it has to be built
with special compiler flags that you do not want applied to everything else.

## One-time setup

You need a Rust toolchain ([rustup]), and on Linux the packages `build-essential`,
`clang`, and `llvm` (on macOS, Xcode command line tools and `brew install llvm`).
`cargo-afl` compiles AFL++ from source when you install it, which is why a C
compiler is needed:

```console
cargo install cargo-afl --locked
```

Then let AFL++ tune the machine for fuzzing. This asks for your password,
because it writes kernel settings that stop crash handlers from stealing
crashes from the fuzzer:

```console
cargo afl system-config
```

If you would rather not run that, AFL++ will tell you at startup exactly
which setting it is unhappy about.

[rustup]: https://rustup.rs

## Build the fuzz test

All the commands below are run from the `fuzz` directory:

```console
cd tests/fuzz_afl/fuzz
```

Build with `cargo afl build` rather than `cargo build`. It is a normal
`cargo build` with the AFL++ instrumentation added, which is how the fuzzer
sees which branches an input reached:

```console
cargo afl build
```

That produces an instrumented binary at `target/debug/fuzz_target_1`.

Note that this is a debug build. Fuzz in debug, at least at first: debug
builds keep integer overflow checks and `debug_assert!`s turned on, and
those catch bugs that a release build would silently allow. If you later
want the extra speed, `cargo afl build --release` works too, and the binary
lands in `target/release/fuzz_target_1`.

## Run the fuzzer

```console
cargo afl fuzz -i in -o out target/debug/fuzz_target_1
```

- `-i in` is the directory of starting inputs. Mutating an existing input is
  much more effective than starting from nothing, so a fuzzer always wants
  at least one seed. Ours is a file of 64 zero characters, which decodes to
  `a == b` and so does not panic.
- `-o out` is where AFL++ writes everything it learns: the inputs it decides
  are interesting, the crashes it finds, and its statistics. It is
  `.gitignore`d.

A full-screen status display takes over the terminal. The two numbers to
watch when you are starting out:

- **exec speed** — how many inputs per second are being tried. Expect
  hundreds to a few thousand per second here; a Soroban `Env` is not cheap
  to create.
- **saved crashes** — how many distinct crashing inputs have been found. For
  this contract it should tick up from 0 within seconds.

Press `Ctrl-C` to stop. You can resume a campaign later by pointing at the
same output directory with `-i-` instead of `-i in`.

Two messages that look alarming but are normal here. AFL++ warns about
"instability detected during calibration", and reports a **stability** figure
below 100%: it means the same input did not take exactly the same path
through the code twice. The Soroban host keeps some state for the lifetime of
the process, and AFL++ reuses one process for many inputs, so a little
instability is expected. It makes the fuzzer somewhat less efficient at
finding new paths; it does not make the crashes it finds any less real. The
other is the low **coverage** percentage: it is measured over all the code
linked into the binary, most of which is the host environment, so a few
percent is normal.

## Reproduce a crash

Crashing inputs are saved as files in `out/default/crashes/`:

```console
ls out/default/crashes
```

Ignore the `README.txt` there; the `id:000000,...` files are the inputs. The
fuzz binary reads an input on stdin when it is not being driven by AFL++, so
you can replay one directly and see the panic and its backtrace:

```console
RUST_BACKTRACE=1 ./target/debug/fuzz_target_1 < out/default/crashes/id:000000*
```

For this contract the output looks like this:

```
thread 'main' panicked at .../soroban-env-host-28.0.2/src/host.rs:892:9:
HostError: Error(WasmVm, InvalidAction)

Event log (newest first):
   0: [Diagnostic Event] topics:[error, Error(WasmVm, InvalidAction)], data:"escalating error to panic"
   1: [Diagnostic Event] topics:[error, Error(WasmVm, InvalidAction)], data:["contract call failed", run, [2179615797408...8145 76, 2179615797408...227248]]
   ...
```

Note that the panic you see is the host escalating the contract's failure,
not the contract's own `panic!("unexpected")`; a contract panic is opaque to
the caller, so the message is a generic `HostError`. What tells you what
happened is the event log: it names the function that failed, `run`, and the
arguments it was called with — and there you can see that the fuzzer found a
pair where `a < b`. The crash file is a reproducer you can keep, and the
backtrace below the event log points at the line of the fuzz test that made
the call.

Two more commands are worth knowing once you have a crash:

- `cargo afl tmin -i <crash-file> -o <smaller-file> target/debug/fuzz_target_1`
  shrinks one crashing input down to the smallest input that still crashes,
  which usually makes it much easier to understand.
- `cargo afl cmin -i out/default/queue -o corpus target/debug/fuzz_target_1`
  reduces the directory of interesting inputs down to a small set with the
  same coverage, a good corpus to keep and reuse as `-i` next time.

## Writing a fuzz test for your own contract

The fuzzer only knows how to produce bytes, and Soroban types such as
`U256`, `Vec`, `Map`, or `Address` cannot be built from bytes alone — they
live inside an `Env`. The SDK bridges that gap with the
[`SorobanArbitrary`] trait: every contract type has an associated
`Prototype` type that *can* be built from bytes, and that converts into the
real type with `into_val(&env)`. So a fuzz test is:

1. Declare a struct of prototypes and derive `Arbitrary` on it, which is
   what lets the fuzzer generate it:

   ```rust
   #[derive(Arbitrary, Debug)]
   struct Input {
       a: <U256 as SorobanArbitrary>::Prototype,
       b: <U256 as SorobanArbitrary>::Prototype,
   }
   ```

2. Pass it to `fuzz!`, convert the prototypes into contract types, register
   the contract, and call it:

   ```rust
   fn main() {
       fuzz!(|input: Input| {
           let env = Env::default();
           let a: U256 = input.a.into_val(&env);
           let b: U256 = input.b.into_val(&env);
           let contract_id = env.register(Contract, ());
           let client = ContractClient::new(&env, &contract_id);
           let _ = client.run(&a, &b);
       });
   }
   ```

Things to keep in mind:

- Create the `Env` **inside** `fuzz!`, not outside. AFL++ reuses the process
  for many inputs ("persistent mode"), so state created outside the closure
  would leak from one input into the next and make crashes hard to
  reproduce.
- Contract types of your own, those with `#[contracttype]`, get a
  `Prototype` automatically, but only when the `soroban-sdk/testutils`
  feature is on. That means your contract crate needs its own `testutils`
  feature that enables `soroban-sdk/testutils`, and the fuzz crate must turn
  it on.
- A panicking contract call is what the fuzzer detects, so call `client.foo()`
  and let it panic. If you use `client.try_foo()` you get a `Result` back and
  a failing call is no longer a crash — useful when you want to check
  something about the error instead, with `assert!`/`panic!` of your own.
- The fuzz crate depends on `arbitrary` directly, because that is where the
  `Arbitrary` derive comes from and `fuzz!` expects it to be nameable. Keep
  its version matching the one `soroban-sdk` uses.

[`SorobanArbitrary`]: https://docs.rs/soroban-sdk/latest/soroban_sdk/testutils/arbitrary/trait.SorobanArbitrary.html

## Note for maintainers of this repository

`make build-fuzz-afl` builds this fuzz test, and requires `cargo-afl` to be
installed as above. Unlike the `cargo-fuzz` example in `tests/fuzz`, it is
not built in CI, because building it means building AFL++ from source.
