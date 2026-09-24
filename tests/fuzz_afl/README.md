# test_fuzz_afl

A contract, and a fuzz test for it that runs under [AFL++] via [afl.rs].

Install `cargo-afl`, which builds AFL++ from source and so needs a C compiler:

```console
cargo install cargo-afl --locked
cargo afl system-config
```

Then build and run the fuzz test:

```console
cd fuzz
cargo afl build
cargo afl fuzz -i in -o out target/debug/fuzz_target_1
```

Crashing inputs are written to `out/default/crashes/`, and can be replayed by
feeding one back in on stdin:

```console
./target/debug/fuzz_target_1 < out/default/crashes/id:000000*
```

[AFL++]: https://aflplus.plus
[afl.rs]: https://github.com/rust-fuzz/afl.rs
