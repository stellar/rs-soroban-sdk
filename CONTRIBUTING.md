# Contributing

Contributions are welcome to the soroban-sdk. Please discuss issues to be solved
and potential solutions on issues ahead of opening a pull request.

## Development Environment Setup

Install rustup:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install rust stable:
```
rustup install stable
rustup +stable target add wasm32-unknown-unknown
rustup +stable component add rust-src
```

Install rust nightly:
```
rustup install nightly
rustup +nightly target add wasm32-unknown-unknown
rustup +nightly component add rust-src
```

Install cargo tools:
```
cargo install --locked cargo-hack
cargo install --locked cargo-nextest
cargo install --locked cargo-watch
```

Install binaryen (for `wasm-opt`):
```
brew install binaryen
```

## Command Cheatsheet

See the `Makefile` for all the common commands you might need.

Fmt code with:
```
make fmt
```

Open docs locally:
```
make doc
```

Build:
```
make build
```

Run tests:
```
make test
```

Build and test on changes:
```
make watch
```
