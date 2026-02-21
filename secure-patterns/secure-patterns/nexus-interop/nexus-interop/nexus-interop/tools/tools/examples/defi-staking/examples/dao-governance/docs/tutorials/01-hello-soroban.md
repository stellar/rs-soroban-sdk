# Hello Soroban 👋

Contoh kontrak sederhana:

```rust
#[contract]
pub struct Hello;

#[contractimpl]
impl Hello {
    pub fn hi(env: Env) -> Symbol {
        Symbol::short("hello")
    }
}

cargo build --target wasm32-unknown-unknown --release

---

## 🤖 `.github/workflows/ci.yml`

```yaml
name: Soroban Nexus CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown
      - name: Build
        run: cargo build --all --release
      - name: Test
        run: cargo test
