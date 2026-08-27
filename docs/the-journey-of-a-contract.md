# From `fn` to on-chain

Every Rust developer knows what a `fn` is. Every Stellar developer knows what a deployed contract is — an address on the network with functions anyone can call.

The path between them is one command and a hash. Run `stellar contract build`, then `stellar contract deploy`, and your function is on a global network. What happens in the middle is invisible, and it's more interesting than it looks: cargo, rustc, LLVM, the linker, the soroban-sdk's macros, the stellar-cli, and finally the Soroban environment itself each take a turn, and each one is solving a problem you'd never guess was there. Why doesn't the build just run `cargo build`? How does code sharing work in libraries like OpenZeppelin? Why does the compiler target a WebAssembly spec from 2019? How does a type learn its own name? How does the network decide your contract is safe to run?

Let's follow one contract the whole way. By the end, the interesting part won't be how much machinery sits in that gap — it'll be how much of it is machinery Rust already had, and how little of it leaks into the contract you write.

## The source

A contract starts as ordinary Rust. The Soroban Rust SDK's attribute macros mark which parts of it form the contract:

```rust
#[contract]
pub struct Token;

#[contracttype]
pub struct Allowance {
    pub amount: i128,
    pub live_until_ledger: u32,
}

#[contractevent]
pub struct Approve {
    #[topic]
    pub from: Address,
    #[topic]
    pub spender: Address,
    pub amount: i128,
}

#[contracterror]
pub enum Error {
    InsufficientBalance = 1,
}

#[contractimpl]
impl Token {
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        // ...
        Approve { from, spender, amount }.publish(&env);
    }
}
```

What has to come out the other end is one Wasm file that carries the compiled code and, alongside it, everything the network and its tools need to use that code: a machine-readable description of the contract's interface (its spec, defined in [SEP-48](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0048.md)), metadata naming the version of the Soroban environment the contract was built against, and a record of the toolchain that built it. Everything below is about how those get produced, and why each one is produced where it is.

## Building with cargo rustc, not cargo build

The build starts with one command:

```console
stellar contract build
```

One gate runs before anything else: the CLI refuses to build unless the profile sets `overflow-checks = true` in `Cargo.toml`. Integer overflow that silently wraps is not a bug class a contract should be allowed to ship with.

Underneath, the stellar-cli does not run `cargo build`. It runs `cargo rustc`, and the difference is the first problem the pipeline solves.

A contract crate declares two crate types in its `Cargo.toml`:

```toml
[lib]
crate-type = ["lib", "cdylib"]
```

The `cdylib` is the contract — a self-contained dynamic library that becomes the Wasm file. The `lib` (an rlib) exists so that the crate can be imported like any other Rust library: by its own tests, by fuzz targets, by other contracts that reuse its types. Both are needed, but they don't coexist well. When `cargo build` compiles a library target, it compiles it once and emits every declared crate type from that single compilation. But a compilation that must emit an rlib cannot have link-time optimization applied: an rlib has to remain linkable into future compilations, while LTO is only valid for final artifacts like executables and cdylibs. So with both crate types declared, cargo quietly disables the `lto = true` that contract profiles set, and the contract ships larger and slower than it should.

`cargo rustc` has an escape hatch that `cargo build` doesn't: a `--crate-type` flag that overrides the manifest's list for this one invocation. The CLI uses it to build the contract as *only* a cdylib:

```console
cargo rustc --manifest-path=Cargo.toml --crate-type=cdylib --target=wasm32v1-none --release
```

The rlib is still there in the manifest for tests and consumers; it just isn't built here. With a single final crate type, LTO applies, and the whole dependency graph — the SDK included — is optimized as one unit.

The CLI also sets two environment variables on the build. One is a handshake with the SDK that we'll come back to when the spec gets trimmed; the other carries a rustc flag through a deliberately chosen channel:

```console
CARGO_BUILD_RUSTFLAGS=--remap-path-prefix=/home/amy/.cargo/registry/src= \
SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2=1 \
    cargo rustc --crate-type=cdylib --target=wasm32v1-none --release
```

`--remap-path-prefix` strips the absolute path of the local cargo registry out of the build. Without it, every panic message that names a source file in a dependency bakes a string like `/home/amy/.cargo/registry/src/index.crates.io-6f17d22bba15001f/...` into the Wasm's data section — which both leaks the builder's home directory and, more importantly, makes the same source produce different bytes on different machines. Reproducible builds are how anyone can verify that deployed bytecode matches published source, so the paths have to go.

The flag travels via `CARGO_BUILD_RUSTFLAGS` rather than as an argument to `cargo rustc`, because arguments to `cargo rustc` reach only the top crate being compiled while the paths being remapped live in the *dependencies*. And it travels there rather than via `RUSTFLAGS` because `CARGO_BUILD_RUSTFLAGS` has the lowest precedence of all the ways cargo accepts rustc flags: it can never stomp on flags the developer set through a stronger channel. The CLI merges its flag into any `CARGO_BUILD_RUSTFLAGS` the developer already exported, and if a higher-precedence source like `RUSTFLAGS` is set — which would make cargo ignore this one entirely — the CLI skips the remapping and warns that the build may not be reproducible.

## A target that doesn't move

The `--target=wasm32v1-none` in that command line has its own story, and it starts with the target it replaced.

For years, Rust's WebAssembly compilation target for non-browser embedders was `wasm32-unknown-unknown`. Its defaults track whatever LLVM enables, and LLVM's defaults track the expanding WebAssembly standard. In September 2024 the Rust project [announced](https://blog.rust-lang.org/2024/09/24/webassembly-targets-change-in-default-target-features/) that the upgrade to LLVM 19 — shipping in Rust 1.82 that October — would turn on the `reference-types` and `multivalue` proposals by default. The reference-types change was not cosmetic: it changes the binary encoding of `call_indirect`, an instruction almost every Rust program emits, from a fixed `0x00` byte to a 5-byte variable-length encoding. Nearly every module produced by the new compiler became undecodable by engines that implement WebAssembly 1.0.

Soroban's virtual machine is deliberately one of those engines. Determinism across every validator is the reason: the environment accepts the WebAssembly 1.0 feature set plus a small, explicitly chosen list of post-MVP features (the MVP being the minimum-viable-product feature set WebAssembly launched with) that Rust and Clang had long emitted: sign-extension, mutable globals, and bulk memory operations. Everything else it rejects — floats included. Its configuration of wasmi, the WebAssembly interpreter the environment embeds, turns each feature off by name so a toolchain upgrade can never opt the network into new semantics:

```rust
config
    .wasm_bulk_memory(true)
    .wasm_mutable_global(true)
    .wasm_sign_extension(true)
    .wasm_saturating_float_to_int(false)
    .wasm_multi_value(false)
    .wasm_reference_types(false)
    .wasm_tail_call(false)
    .wasm_extended_const(false)
    .floats(false)
```

So contracts built with Rust 1.82 were rejected by the network, and the only workaround was pinning Rust 1.81 or building the standard library from source on nightly with `-Ctarget-cpu=mvp`. The deeper problem was that `wasm32-unknown-unknown` is a target whose meaning drifts: it serves the audience that wants ever more of WebAssembly available, and blockchains are the opposite audience — embedders that need the output frozen to a spec.

Graydon Hoare, who created Rust and now works on Soroban at the Stellar Development Foundation, fixed this upstream rather than downstream. Within days of the September announcement — before 1.82 had even reached stable — he filed [a proposal with the Rust compiler team](https://github.com/rust-lang/compiler-team/issues/791) for a minimal-features Wasm target and [implemented it](https://github.com/rust-lang/rust/pull/131487), and it stabilized as a Tier 2 target in Rust 1.84 in January 2025: [`wasm32v1-none`](https://doc.rust-lang.org/rustc/platform-support/wasm32v1-none.html).

The target's definition is its promise. It pins LLVM's `target-cpu` to `mvp`, disabling every post-MVP proposal, and re-enables exactly one: mutable globals — the one proposal that was folded into the W3C WebAssembly Core 1.0 Recommendation adopted in December 2019. It also ships no standard library at all, only `core` and `alloc`, because a target that imports nothing from its host has no OS facilities for `std` to wrap. The `v1` in the name is the point: what this target emits is WebAssembly 1.0 today, and still WebAssembly 1.0 after the next ten LLVM upgrades. Anyone who needs stable ground for Wasm development — not just Soroban, but any embedder with a fixed feature set — now has a Rust target to stand on.

`stellar contract build` selects it automatically: Rust 1.84 and later build for `wasm32v1-none`; the 1.82–1.83 window that can only produce rejected modules is refused outright.

## Sharing code with traits

Contracts have a lot in common with each other. Plenty of them want to be pausable, to have an owner, to be upgradeable, to behave like a token. On other chains that shared behavior arrives through inheritance or copy-paste. Rust already has the right tool for it — a trait with default method bodies — but there's a gap between a Rust trait and a contract: a contract's functions are Wasm *exports*, and a default method body sitting in a library crate isn't an export of your contract.

`#[contracttrait]` closes that gap. A library defines an interface as an ordinary trait, with real working bodies as default methods:

```rust
#[contracttrait]
pub trait Pausable: RequireAuthForPause {
    fn is_paused(env: &Env) -> bool {
        env.storage().instance().has(&"paused")
    }

    fn pause(env: &Env) {
        Self::require_auth_for_pause(env);
        env.storage().instance().set(&"paused", &true);
    }

    fn unpause(env: &Env) {
        Self::require_auth_for_pause(env);
        env.storage().instance().remove(&"paused");
    }
}
```

A contract then picks the whole interface up with an impl block that can be completely empty:

```rust
#[contractimpl(contracttrait)]
impl Pausable for MyContract {}
```

That contract now exports `is_paused`, `pause`, and `unpause`, and a `PausableClient` exists for calling them on any contract that implements the trait. Overriding is just Rust: write the function in the impl block and it wins; the ones you leave out keep their defaults. The SDK ships its own interfaces this way — `TokenInterface` and `StellarAssetInterface` are both `#[contracttrait]`s — and it's how ecosystem libraries like OpenZeppelin's `stellar-contracts` hand contracts working implementations rather than just signatures to fill in.

Making that work runs into the proc-macro problem again, from a new direction. The two pieces of knowledge needed to decide "export this default function" live in two different expansions. `#[contracttrait]` expands over the trait, which is where the default bodies are — but at that moment nobody has implemented the trait yet. `#[contractimpl]` expands over the impl block, which is where we learn which functions the contract overrode — but a proc macro is handed one item and cannot look anything else up, so it can't see the trait to know what defaults exist. The two halves never meet, and in the normal case they aren't even in the same crate.

The SDK bridges them with a third macro whose only job is to carry data. When `#[contracttrait]` expands, one of the things it generates is a `macro_rules!` macro named after the trait itself, so that importing `Pausable` brings it along. Inside that generated macro are the signatures of every default function in the trait, captured at trait-definition time and stringified so they survive as macro arguments:

```rust
macro_rules! __contractimpl_for_pausable {
    (/* ... */, $impl_fns:expr, /* ... */) => {
        soroban_sdk::contractimpl_trait_default_fns_not_overridden!(
            trait_default_fns = ["fn is_paused(_)", "fn pause(_)", "fn unpause(_)"],
            impl_fns = $impl_fns,
            // ...
        );
    };
}
pub use __contractimpl_for_pausable as Pausable;
```

Now `#[contractimpl(contracttrait)]`, expanding at the impl site, does the one thing it can: it emits a call to that macro, passing the list of functions the impl block actually defined — `Pausable!(MyContract, ["is_paused"], ...)`. The declarative macro holds the trait's half; the call supplies the impl's half; and it forwards both to a final proc macro that subtracts one list from the other. For every default the contract didn't override, that macro generates exactly what `#[contractimpl]` generates for a hand-written function: the exported Wasm function, its spec entry, a method on the client, a variant on the args enum, and the registration the test environment needs.

Note where those exports come from. Not the library — a `#[contracttrait]` doesn't even emit spec entries where it's defined, because the crate defining it usually isn't a contract. The export and the spec entry for an inherited default are generated by an expansion inside the *implementing* crate, so a contract's Wasm describes precisely the interfaces that contract implements and nothing the library merely offered.

Three macros — two procedural, one generated — arranged so that information captured during one expansion survives into another. None of it surfaces in the contract. The author writes `impl Pausable for MyContract {}` and gets three more contract functions.

## A spec computed by the compiler

While rustc compiles the contract's functions, it is also computing the contract's spec. This is the part of the journey where the work moved from the macros into the compiler itself.

The spec is a sequence of entries — one per function, user-defined type, event, and error — encoded in XDR, the binary encoding Stellar uses for all of its data structures, and embedded in a Wasm custom section named `contractspecv0`. It is how everything downstream understands the contract without seeing its source: the CLI uses it for typed invocation and client generation, indexers use it to decode events, wallets use it to render what a transaction will do.

The SDK's macros used to build those XDR bytes themselves: the proc macro serialized the entry while it ran and pasted the result into the generated code as an opaque byte-string literal. That works until you ask the spec to contain something a proc macro cannot know. Proc macros see tokens — the raw text of the item they're expanding — and nothing else. They don't know what module the type sits in. They can't follow a field's type to its definition. Given the tokens `struct Approval { flag: Flag }`, a macro has no way to learn which of two `Flag` types in the program this one is, or what its full path is. Two crates, or two modules of one crate, can each define a `Flag`, and a spec that names both simply `Flag` cannot say which is which.

So the macros stopped computing the spec and started *describing* it. What `#[contracttype]` and `#[contractimpl]` emit now is a const expression — a value the compiler evaluates at compile time — built from borrowed views over static data:

```rust
impl Token {
    const __SPEC_XDR_VIEW_approve: ScSpecEntryView<'static> =
        ScSpecEntryView::FunctionV0(ScSpecFunctionV0View {
            name: ScSymbolView(StringMView::new(b"approve")),
            inputs: VecMView::new(&[
                ScSpecFunctionInputV0View {
                    name: StringMView::new(b"amount"),
                    type_: ScSpecTypeDefView::I128,
                },
                // ...
            ]),
            // ...
        });

    pub const fn spec_xdr_approve() -> [u8; Token::__SPEC_XDR_VIEW_approve.const_xdr_len()] {
        Token::__SPEC_XDR_VIEW_approve.const_to_xdr()
    }
}

#[link_section = "contractspecv0"]
pub static __SPEC_XDR_FN_APPROVE: [u8; Token::__SPEC_XDR_VIEW_approve.const_xdr_len()] =
    Token::spec_xdr_approve();
```

The XDR encoding happens inside `const_to_xdr()` while the contract compiles — even the length of the byte array is a const expression. Moving the encoding changed nothing about the bytes: on its own, this change produced `contractspecv0` sections byte-identical to what the macros made. But the spec stopped being an opaque blob. It's a value, readable in `cargo expand`, referenceable by other generated code.

That last property is the point, because const evaluation can do the thing tokens can't: resolve relationships between types. At const-evaluation time, a type can know its own fully qualified path. Each spec-bearing type gains one small const fn:

```rust
impl Allowance {
    pub const fn spec_type_name() -> &'static str {
        ::core::concat!(::core::module_path!(), "::", "Allowance")
    }
}
```

`module_path!()` is a compiler builtin that expands to the path of the module it appears in — information only the compiler has, which is exactly why the macro emits the call instead of a name. And every *reference* to a type routes through the same function: when another type or a function signature uses `Allowance`, its spec view says `StringMView::new_str(<Allowance>::spec_type_name())` — Rust's ordinary type resolution finds the right `Allowance`, and const evaluation asks it for its name. The definition entry and every reference to it are now guaranteed to agree, because they are the same expression.

The result in the built spec is that types carry names like `token::Allowance` and `soroban_sdk::auth::Context` rather than `Allowance` and `Context`. Names are unique, aliases resolve to the type they name, and — as the next two sections show — the relationships between entries become something a tool can walk.

The common instinct is to do all of this work in the proc macro, because the proc macro is where the code generation happens. But a proc macro is the wrong altitude: it runs before the program means anything. Leaning on const expressions hands the semantic work to the phase of compilation that actually has the semantics.

## Markers in the data section

The spec now describes the contract precisely, but it describes too much of it. A contract that depends on a library of types — a token library, say, with a dozen `#[contracttype]` definitions — gets a spec entry for every one of them, whether or not it uses them. Spec entries live in a custom section, and custom sections are invisible to the linker's dead-code elimination: the linker strips unused *functions* and unused *data*, but it emits `contractspecv0` wholesale. Left alone, the spec would grow with a contract's dependency tree rather than with its interface.

The entries reachable from the contract's functions don't need any extra machinery — a function entry names its parameter and return types by their now-unambiguous qualified names, so those references can be followed. But two kinds of entries are referenced by nothing. An event is published, not passed; no function signature mentions it. An error enum appears in function specs only as the built-in `Error` type, never by name. A tool that only followed references from functions would conclude every event and error is unused.

For exactly those entries — the ones no function references — the SDK plants a marker where the linker *can* see it: in the Wasm's data section. A marker is 14 bytes — the magic `SpEcV1` followed by the first 8 bytes of the SHA-256 of the entry's XDR — emitted inside the code path that uses the entry. An event's marker sits inside its generated `publish()`; an error's inside the panic path that raises it:

```rust
fn spec_shaking_marker() {
    // Marker in data section. Post-build tools can scan for "SpEcV1"
    // patterns and match against specs in contractspecv0.
    static MARKER: [u8; 14] = soroban_sdk::spec_marker(&Approve::spec_xdr());
    // Volatile read prevents DCE of this function and keeps MARKER
    // in the data section.
    let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
}
```

The volatile read is the whole trick. An ordinary read of a static the program never uses would be optimized away, and the static with it. A volatile read is one the compiler must assume has side effects, so it cannot be elided — and a static that is read cannot be stripped. Reading a single byte is enough: one volatile read of the static keeps the entire 14-byte array alive. The marker's fate is now tied to the code around it: if the contract calls `Approve { .. }.publish()`, the marker rides along into the binary; if no reachable code ever publishes the event, dead-code elimination deletes the function, the static, and the marker together. The linker ends up computing liveness for spec entries it cannot see, by proxy.

Note what the marker hashes: the same const-encoded bytes that fill the spec section, hashed by a const-fn SHA-256 at compile time. Marker and entry are derived from one expression, so they cannot drift apart — which matters, because a marker that hashed differently from its entry would silently declare the entry unused.

## Shaking the spec

Back in the stellar-cli, cargo has finished and a Wasm file exists — but the build isn't done. The CLI now post-processes the binary it was handed.

First it appends its own metadata into the `contractmetav0` section alongside what the SDK wrote there (the Rust and SDK versions used, and a flag telling the CLI the spec needs shaking): a `cliver` entry recording the CLI's exact version, plus any `--meta key=value` pairs the developer passed. Reproducibility again — verifying a build requires knowing every tool that touched it.

Then it shakes the spec — the same idea as tree-shaking in a JavaScript bundler: keep what the interface reaches, drop the rest. The CLI parses `contractspecv0` and reduces it to the contract's public surface:

- **Functions are always kept.** They are the interface.
- **Types are kept if the interface reaches them.** Starting from every function's parameter and return types, the CLI follows the qualified-name references from entry to entry — a struct's field types, a union's case payloads — and retains the transitive closure. This walk is only trustworthy because of the work above: fully qualified names make every reference unambiguous.
- **Events and errors are kept if their marker survived.** The CLI scans the data section for `SpEcV1` patterns and matches the hashes against the entries. A marker present means dead-code elimination could not rule out the publishing or panicking code; a marker absent means it proved that code unreachable and deleted it.

Everything else is dropped, duplicate entries are deduplicated, and the `contractspecv0` section is rewritten in place. What remains is exactly the spec of what the contract can actually do: its functions, the types those functions speak, and the events and errors it can emit. Finally the CLI runs `wasm-opt` over the binary — Binaryen's aggressive size optimization, configured to the same feature set the network accepts — and prints the resulting file's SHA-256, which is about to become the contract's identity.

## The environment version handshake

One more thing was stamped into the Wasm back at compile time, and nothing in the build pipeline touches it, because its intended reader is the network itself. The SDK embeds a custom section named `contractenvmetav0`, containing the version of the Soroban environment interface the contract was built against:

```rust
#[link_section = "contractenvmetav0"]
static __ENV_META_XDR: [u8; env::internal::meta::XDR.len()] = env::internal::meta::XDR;
```

The payload is twelve bytes of XDR: a union discriminant, then the ledger protocol version the SDK's environment interface corresponds to, then a pre-release number that is zero for released versions. A contract built against SDK v23 carries protocol 23 in this section forever, because that is the interface that SDK implements. This is the contract declaring, in a form the network can verify, which contract–host interface it expects — which host functions exist, with which semantics. At deployment, the environment uses it to decide whether the contract is safe to admit at all.

## Deployment

Deploying happens in two steps: the Wasm is uploaded to the network, then a contract instance is created that references it. Upload is where the environment — the same code every validator runs — takes custody of the artifact, and it does not take it on faith.

The environment hashes the bytes; the SHA-256 is the code's identifier from here on. Then it instantiates a throwaway VM from the upload, on an isolated engine, purely as a gate — in the words of the code, to check "that the wasm is basically not garbage." That parse enforces everything this post has been building toward: the module must validate; it must contain no floats, no start function, and nothing beyond the feature set the configuration above allows — a set that `wasm32v1-none` output satisfies by construction; and it must carry a `contractenvmetav0` section the host agrees with.

That last check is the version handshake completing. The rule is ordered compatibility for protocols and exact identity for pre-releases: a contract whose embedded protocol version is **older or equal** to the network's current protocol is accepted — contracts built years ago keep deploying and keep running, because each protocol's environment maintains the semantics of the interfaces before it. A contract claiming a **newer** protocol than the network is running is rejected. It might seem harmless to accept one early — it would go live when the network upgrades — but the environment is conservative for a reason: the semantics a contract baked in against a "future" protocol might differ from the semantics the network finally adopts.

And a pre-release version must match the host exactly; every pre-release is incompatible with every other, so nothing built against an unreleased interface can slip onto a released network.

Once the contract is admitted, the environment does one more pass over the Wasm — enforcing a few final structural limits, but mostly measuring it. It counts what the module is made of: instructions, functions, globals, types, imports, exports, table entries, element segments, data segments, and data segment bytes. Those ten numbers are stored in the ledger alongside the code, in the contract code entry's `ContractCodeCostInputs`. They are the module's cost profile, computed once at upload so that every future parse and instantiation can be charged precisely for what this module actually contains rather than coarsely for its byte length.

That up-front work — the module validated once, its costs measured and stored — is what makes execution cheap. Validators keep a module cache: every live contract, parsed once into a shared engine, so that invoking a contract skips parsing entirely and pays only instantiation — priced by those stored cost inputs. The expensive, careful work happens once, at the ledger's edge; the hot path reuses it.

Creating the contract instance is then small: a ledger entry binding a freshly derived contract address to the code hash, and a call to the contract's constructor if it has one. Many instances can share one uploaded Wasm. The journey ends as two ledger entries — the measured, validated code keyed by its hash, and the instance that gives it an address — plus the small companion entries that track how long each lives.

## Leaning on the language

Every stage of this pipeline exists to move work to the tool that can do it best, as early as it can be done — and at nearly every stage, that tool is something Rust already had.

The CLI drives `cargo rustc` directly because only that invocation can isolate the cdylib and keep LTO alive across the whole dependency graph. The build targets `wasm32v1-none` because a contract needs a compilation target anchored to a spec rather than to a toolchain's momentum — a fix Stellar's own Graydon Hoare contributed upstream to Rust for every Wasm embedder. Shared interfaces are traits with default methods, because that is how Rust shares behavior; the macros only carry the information across expansions so those defaults become real exports. The spec is computed by const evaluation because the compiler is the only tool that knows what types *are*: macros see only tokens, but a const expression can ask a type for its fully qualified name. And volatile reads let dead-code elimination — the pass already deciding what code ships — also decide which events and errors the contract can really emit.

The last stretch belongs to the network. The CLI shakes the spec down to the public interface because everything downstream deserves a description of what the contract does, not of what it depends on. The environment validates the version handshake and measures the module at upload, storing the results in the ledger, so the network admits only what it can run deterministically — and then runs it fast.

The payoff is what *isn't* in a contract. No manifest of exported entry points, no hand-written interface description, no versioning shim, no build script tuning the optimizer. A Soroban contract is a Rust library: structs, enums, traits, impls, functions. The pipeline's whole job is to take that ordinary Rust and hand the network something it can verify, price, and honor for years — while the code stays about the contract.
