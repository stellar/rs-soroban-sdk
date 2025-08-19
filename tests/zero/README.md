# test_zero

The test_zero test vector exists to build a contract that imports
the SDK but makes no material use of any of its logic / code.

The test vector when built should still result in a valid Soroban
contract that contains env-meta and sdk meta. This can be tested by
using the stellar-cli's `stellar contract info env-meta` command to
inspect the wasm file.

The `Makefile` in this repository checks all built test vectors for
env-meta and sdk meta as part of the `build` target.

This is a bit of an edge case to have a test vector for.
Historically there have been cases where the SDK was seen to be
imported but env-meta was not written. This test vector exists to
prove that the case we have been able to replicate is addressed in
this version of the SDK.
