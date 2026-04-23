---
name: spec-shaking
description: How soroban-sdk's spec shaking v2 works end-to-end — the `SpecShakingMarker` trait, direct calls vs `keep_reachable`, what ends up in the WASM for each, and which parts a post-build tool can safely strip. Use when changing `soroban-sdk/src/spec_shaking.rs`, `soroban-sdk-macros/src/shaking.rs`, the `TryFromValForContractFn` / `IntoValForContractFn` boundary wrappers, or the `soroban-spec::shaking` / `soroban-spec::strip` post-build helpers.
---

# Spec shaking v2 in soroban-sdk

## The problem

Rust's `#[contracttype]`/`#[contractevent]`/`#[contracterror]` macros emit an
`ScSpecEntry` for every annotated type into the WASM's `contractspecv0`
custom section. Many contracts declare types that are never used at a contract
boundary (function params, return values, published events), and shipping
those unused specs is pure on-chain bloat.

Spec shaking v2 solves this by letting a post-build tool (stellar-cli) strip
entries from `contractspecv0` for types the contract never uses at a boundary.
The SDK has to give the tool a way to distinguish used from unused types,
despite the fact that spec entries themselves are just static bytes in a
custom section that the linker never inspects.

## The mechanism, in one sentence

For each SDK-generated type, emit a **14-byte marker** in the data section
(`SpEcV1` + first 8 bytes of `SHA256(spec_entry_xdr)`), and wire up a
function-call graph such that the marker survives `rustc`/LLVM dead-code
elimination **iff** the type is reached from a contract-fn boundary.

The post-build tool then:
1. Scans the data section for `SpEcV1…` patterns.
2. For each `ScSpecEntry` in `contractspecv0`, hashes the entry and checks
   whether the corresponding marker is present.
3. Drops entries whose marker is absent.

See `soroban-spec/src/shaking.rs` — `find_all`, `generate_marker_for_entry`,
`filter` are the primitives. The spec-shaking meta key is `rssdk_spec_shaking`
in `contractmetav0`; a value of `"2"` declares that v2 markers are present.

## How `T::spec_shaking_marker()` keeps a marker alive

Each `#[contracttype]`/`#[contractevent]`/`#[contracterror]` macro emits:

```rust
impl SpecShakingMarker for MyType {
    #[doc(hidden)]
    #[inline(always)]
    fn spec_shaking_marker() {
        <Field1 as SpecShakingMarker>::spec_shaking_marker();
        <Field2 as SpecShakingMarker>::spec_shaking_marker();
        // ...
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 14] = *b"SpEcV1\xXX\xXX\xXX\xXX\xXX\xXX\xXX\xXX";
            let _ = unsafe { ::core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
```

Two mechanisms combine:

- **The `read_volatile(MARKER.as_ptr())` call** is what LLVM can't optimise
  away — it forces the function body to reference the `MARKER` static, which
  in turn forces the 14 bytes into the data section at link time.
- **The recursive calls to field markers** propagate reachability through the
  type graph. If any field's type is used, its marker function is alive, so
  its `MARKER` survives too.

At the contract boundary, `TryFromValForContractFn` /
`IntoValForContractFn` (see `soroban-sdk/src/{try_from_val_for_contract_fn,
into_val_for_contract_fn}.rs`) call `U::spec_shaking_marker()` on the arg
and return types. That call is the root of the reachability chain.

### What a direct call adds to the WASM

Because `spec_shaking_marker` is `#[inline(always)]`, calling
`T::spec_shaking_marker()` directly from another marker body gets **inlined**:
the whole chain collapses into the caller. Intermediate marker functions
get DCE'd if nothing else references them by address.

At the inlined call site, each transitively-reachable type contributes a
three-instruction MARKER load at runtime:

```
i32.const 0
i32.load8_u offset=<MARKER_ADDR>
drop
```

The volatile byte-load is only there to preserve the reference for LLVM.
Runtime execution loads and discards the byte — cheap, but it runs on every
contract-fn invocation.

**Trade-offs of direct call:**
- Smaller for single-use: one inlined copy, no standalone function.
- Inlines the full chain into the caller body.
- Cannot be used for types that can recurse through heap indirection — the
  inlining would recurse forever and stack-overflow at runtime.

## How `keep_reachable` works

```rust
fn keep_reachable(f: fn()) {
    let _ = unsafe { core::ptr::read_volatile(&f) };
}
```

Takes a function pointer and volatile-reads the **pointer itself** (from a
stack slot). The function is never called. The address-taking forces the
referenced function to be emitted as a standalone `() -> ()` function and
added to the indirect function table.

### What `keep_reachable` adds to the WASM

Three separate pieces:

1. **At the caller site (after inlining)**, a 6-instruction sequence:
   ```
   local.get <base>
   i32.const <fn_idx>
   i32.store offset=<slot>   ;; store fn ptr to stack
   local.get <base>
   i32.load offset=<slot>    ;; volatile-load it back
   drop
   ```
   This runs at runtime every time the caller runs. Nothing downstream
   interprets `<fn_idx>` as a function reference — it's just a number
   moved onto the stack and read back.

2. **A standalone function** for the referenced `T::spec_shaking_marker`,
   emitted into the code section with its body intact. The body is
   **never called at runtime** in the `keep_reachable` path; its only
   purpose is to contain the `read_volatile(MARKER.as_ptr())` that keeps
   `MARKER` alive at link time.

3. **An entry in the element section** (indirect function table) pointing
   at that standalone function.

All three are what makes `keep_reachable` more expensive than a direct call
in both code size and per-boundary runtime cost. Typical overhead:
~10–40 bytes per standalone function + 1 byte element-table entry + 1 byte
function-section entry + 14 bytes of `MARKER` in data + 3 extra runtime
instructions per caller site (~6 vs ~3 for direct).

### When `keep_reachable` is required

When a type can recurse through heap indirection:

```rust
struct S { v: Vec<S> }             // Rust allows this (finite size via heap)
```

A direct-call chain `S::marker → Vec<S>::marker → S::marker → …` would
stack-overflow at runtime. The cycle must be broken. `keep_reachable` at
the container level does exactly that: the reference is by address, so
the inlining chain terminates there.

Rust rejects the same shape for inline-embedded containers
(`struct S { o: Option<S> }`, `struct S { t: (S, u32) }`, or any
UDT-with-direct-field recursion) as infinite-size types. So for
`Option`/`Result`/tuples/UDT fields, direct call is **provably safe** —
no cycle possible at the type-system level.

### Current split in `soroban-sdk/src/spec_shaking.rs`

- `Vec<T>`, `Map<K,V>`, `&T`, `&mut T` — use `keep_reachable`
  (cycles are possible through heap indirection).
- `Option<T>`, `Result<T,E>`, tuples (1..=13), UDT macro-generated fields
  — use direct call (no cycle possible).
- Primitive and built-in SDK types — no-op default impl.

The split is empirically optimal for pre-strip size: broadening
`keep_reachable` to `Option`/`Result`/tuples grew `test_spec_shaking_v2` by
+230 bytes and made `test_udt` −47 bytes (because udt has genuine recursion
that already forces standalones to exist). Pushing it all the way into the
UDT macros grew v2 by +1,054 bytes and udt by +248 bytes — uniformly
worse.

## Post-build stripping

After link time, the marker infrastructure has served its purpose: the
`MARKER` bytes were scanned by the post-build tool to decide which
`contractspecv0` entries to keep. The marker functions, element entries,
and `MARKER` bytes are all strippable.

### What's always safe to strip

**Standalone marker functions** whose only reference is the element table
(no direct callers, not exported). The `i32.const <fn_idx>` immediates left
behind in `keep_reachable` inline sequences are harmless: they're numeric
constants consumed by `store`/`volatile-load`/`drop`, never by
`call_indirect`. The table slot going `ref.null` is fine for the same
reason.

**Element-table entries** pointing at the above.

**`contractspecv0` entries** whose marker wasn't seen in the scan. This is
the main size win on contracts that declare many types but use few at
boundaries.

### What needs a live code-rewrite to strip

**The `SpEcV1…` bytes themselves in the data section.** This is only safe
if no live code still contains an inlined `i32.load8_u offset=<MARKER_addr>`
pointing at them. That's the case **only when `keep_reachable` is used for
everything** — in the current mixed design, contract-fn wrappers contain
inlined MARKER loads (the direct-call path), and stripping the bytes would
make those loads read garbage or trap.

The stripper in `soroban-spec/src/strip.rs` zeroes the `SpEcV1…` bytes in
place rather than removing them, so segment offsets stay stable. True byte
reduction would require splitting data segments around each marker range
(documented non-goal).

**The inlined `keep_reachable` store/load/drop sequences in wrappers.** These
still execute at runtime even though they reference a removed standalone.
Removing them requires pattern-matching on
`i32.const <removed_idx>; i32.store; i32.load; drop` and rewriting function
bodies. A separate, bigger project — not done by the current stripper.

### The design implication

If the project commits to a post-build stripper, **all-`keep_reachable`**
becomes the cleaner design choice than the current mixed split:

| | Current (mixed) | All-`keep_reachable` |
|---|---|---|
| v2 pre-strip (bytes) | 11,925 | 12,155 (+230) |
| v2 post-strip (bytes) | ~11,875 | ~9,324 (−2,551) |
| MARKERs strippable without rewriting wrapper bodies? | no (wrappers have inlined MARKER loads) | yes |
| Stripper complexity | cannot remove MARKER data without instruction rewriting | pure section surgery |

The ~230-byte pre-strip penalty is paid back many times over post-strip,
and the stripper stays simple. The tradeoff is worth discussing if on-chain
size is the primary metric; keep the current split if pre-strip size of
dev/test builds also matters.

## Practical rules of thumb

- **Never broaden `keep_reachable` without measuring.** Pre-strip it almost
  always costs bytes and per-boundary runtime instructions.
- **Never narrow `keep_reachable` past Vec/Map/refs.** Those are the places
  where Rust permits a compile-time-legal cycle; direct call there would
  be unsafe.
- **Markers only exist when the macro's `spec_shaking_v2_enabled()` is true.**
  That needs both the `experimental_spec_shaking_v2` Cargo feature **and**
  the `SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2=1` env var at
  build time. `soroban-sdk/build.rs` sets a `spec_shaking_v2` cfg from both
  signals and gates the entire `spec_shaking` module on it.
- **Measure with the `test_spec_shaking_v1`, `test_spec_shaking_v2`, and
  `test_udt` contracts.** `v1` is the feature-off baseline (no markers);
  `v2` exercises the full marker surface across `Option`/`Result`/tuples/
  events/refs; `udt` has genuine recursion through `Vec`/`Map` that forces
  standalones to exist.
- **The `tests-expanded/` directory** has macro-expanded sources checked in;
  these update via `make expand-tests` and are the fastest way to see what
  the macros emit without rebuilding WASMs.

## Key locations

| Role | Path |
|---|---|
| Trait, container impls, `keep_reachable` | `soroban-sdk/src/spec_shaking.rs` |
| Macro-generated marker impl | `soroban-sdk-macros/src/shaking.rs` |
| Boundary-wrapper marker call sites | `soroban-sdk/src/{try_from_val_for_contract_fn,into_val_for_contract_fn}.rs` |
| Feature gate (cfg) | `soroban-sdk/build.rs`, `soroban-sdk-macros/src/lib.rs` → `spec_shaking_v2_enabled` |
| Post-build scan + filter | `soroban-spec/src/shaking.rs` |
| Post-build wasm stripping (all-`keep_reachable` variant) | `soroban-spec/src/strip.rs` |
| Test contracts | `tests/spec_shaking_v1/`, `tests/spec_shaking_v2/`, `tests/udt/` |

## Alternative design: static `MarkerNode` graph (no fns, no `keep_reachable`)

An alternative reachability mechanism replaces both the fn-based
`spec_shaking_marker()` direct-call chain *and* `keep_reachable(fn)` with
a pure static-pointer graph. Every type exposes one associated const
pointing at a `MarkerNode` in the data section; the boundary wrapper
does a single volatile read of that pointer; wasm-ld's section-GC walks
the `children` relocations the rest of the way. No function pointers,
no indirect-function-table surface, no DCE asymmetry between
direct-call and `keep_reachable` paths.

### The trait and the graph

```rust
#[repr(C)]
pub struct MarkerNode {
    pub marker: [u8; 14],                           // SpEcV1 + hash (or zeros)
    pub children: &'static [*const MarkerNode],
}
unsafe impl Sync for MarkerNode {}                  // raw pointers aren't Sync by default

pub static EMPTY_MARKER_NODE: MarkerNode = MarkerNode {
    marker: [0u8; 14],
    children: &[],
};

pub trait SpecShakingMarker {
    const MARKER_NODE: *const MarkerNode;
}
```

Every impl has the same shape: one associated const returning a
`*const MarkerNode`. No `fn spec_shaking_marker()`, no `keep_reachable`.

### Impl shapes

| Category | RHS of `MARKER_NODE` | Where the node lives |
|---|---|---|
| Primitives (`u32`, `bool`, `Address`, `Symbol`, …) | `&raw const EMPTY_MARKER_NODE` | Shared single static in soroban-sdk |
| Single-child containers (`Vec<T>`, `Option<T>`, `&T`, `&mut T`, `(T,)`) | `T::MARKER_NODE` (direct forward) | No new node |
| Multi-child containers (`Map<K,V>`, `Result<T,E>`, tuples 2..=13) | `&MarkerNode { marker: [0;14], children: &[…] } as *const _` | Promoted anonymous static per monomorphization |
| UDTs (macro-emitted) | `&raw const __SOROBAN_SDK_SPEC_MARKER_NODE_<hash>` | Named module-scope static per UDT |

UDT macros emit one module-scope static plus a tiny impl:

```rust
#[doc(hidden)]
static __SOROBAN_SDK_SPEC_MARKER_NODE_5803F674C7D00122: MarkerNode = MarkerNode {
    marker: *b"SpEcV1X\x03\xf6t\xc7\xd0\x01\"",
    children: &[
        <u32 as SpecShakingMarker>::MARKER_NODE,
        <UsedNestedInStruct as SpecShakingMarker>::MARKER_NODE,
    ],
};
impl SpecShakingMarker for UsedParamStruct {
    const MARKER_NODE: *const MarkerNode =
        &raw const __SOROBAN_SDK_SPEC_MARKER_NODE_5803F674C7D00122;
}
```

### Why raw pointers, not `&'static MarkerNode`

The graph needs to allow cycles (`struct S { v: Vec<S> }` →
`S_NODE → Vec<S>_wrapper → S_NODE`). Rust allows static-to-static
cycles through `&` at the value level, but a trait-associated
`const MARKER_NODE: &'static MarkerNode` triggers rustc's
const-evaluator cycle detector the moment a concrete monomorphization
closes the loop:

```
note: ...which requires const-evaluating + checking
       `<Vec<S> as SpecShakingMarker>::MARKER_NODE`
note: ...which again requires evaluating initializer of static
       `__SOROBAN_SDK_SPEC_MARKER_NODE_…`, completing the cycle
```

CTFE on a const expression of type `&'static T` walks into the
referenced allocation's initializer. The cycle is unavoidable with
associated-const references because the container's promoted static and
the UDT's named static both have initializers that reference each
other's values.

Raw pointers dodge this. `&raw const STATIC` and `&expr as *const T`
produce `*const T` at const time as opaque address values — CTFE does
not walk into the target. Runtime behaviour is identical; addresses
resolve at link time the same way. The `unsafe impl Sync` is needed
because raw pointers aren't `Sync` by default; it's safe because the
pointers are never dereferenced at runtime.

### How the boundary wrapper roots the graph

`TryFromValForContractFn` and `IntoValForContractFn` each contain one
volatile read per boundary:

```rust
#[cfg(target_family = "wasm")]
{
    let _ = unsafe { core::ptr::read_volatile(&U::MARKER_NODE) };
}
```

`U::MARKER_NODE` is a compile-time-known `*const MarkerNode` value.
`&U::MARKER_NODE` puts it in a local stack slot; `read_volatile` reads
the pointer value back. The volatile read is what LLVM can't optimise
away — it forces the pointer immediate to be emitted in the caller's
body, which tells wasm-ld "the `MarkerNode` at this address is live."

`#[contractevent]` publishers emit the same call inline in the
generated `publish` method (see `soroban-sdk-macros/src/derive_event.rs`).

### Lifetime handling in the macro

Module-scope statics can't see lifetime params declared on the impl
(e.g. `<'a>` on `UsedEventWithRefs<'a>` — field types include
`&'a UsedRefTopicType`). The macro substitutes `'static` for every
lifetime in field types via `syn::visit_mut` before emitting the
static. Sound because `&'a T: SpecShakingMarker` forwards to `T`, so
the marker value is lifetime-independent.

### What ends up in the WASM, per piece

Assume wasm32. Addresses and pointers are 4 bytes.

**Per UDT.** One `static __SOROBAN_SDK_SPEC_MARKER_NODE_<hash>: MarkerNode`:
- 14 bytes `SpEcV1…` marker
- 2 bytes padding (4-byte alignment from slice field)
- 8 bytes slice header (ptr + len)
- N × 4 bytes slice backing (one `*const MarkerNode` per field)
- **= 24 + 4·N bytes per UDT**

**Per multi-child container monomorphization.** Same shape, with
`marker: [0; 14]`. 24 + 4·N bytes. wasm-ld's ICF can dedup identical
wrappers across call sites.

**Per single-child container monomorphization.** Zero bytes — forwards
`T::MARKER_NODE` directly.

**Per primitive.** Zero bytes per type; all primitives share
`EMPTY_MARKER_NODE` (24 bytes amortised once across the binary).

**Per boundary call site.** The `read_volatile(&T::MARKER_NODE)` inlines
to roughly six instructions, one stack slot, **constant regardless of
reachability depth**:

```
i32.const  <stack_base>
i32.const  <marker_node_addr>
i32.store  offset=<slot>        ;; store ptr to stack slot
i32.const  <stack_base>
i32.load   offset=<slot>        ;; volatile-load it back
drop
```

This is the main runtime win over the fn-based design, which compounded
an `i32.load8_u` per direct-call-reachable type plus a 6-instruction
`keep_reachable` round trip per cycle-breaking container edge.

### Post-build stripping in this variant

`soroban-spec/src/strip.rs` becomes dramatically simpler. No marker
functions exist, so no function deletion, no element-table surgery, no
call-site rewriting. The pass runs a graph walk over the data section
and a segment-compaction pass:

1. Refuse non-v2 wasms (checks `rssdk_spec_shaking = "2"`).
2. Scan data segments for `SpEcV1…` patterns. Collect (a) the marker
   set (for spec filtering) and (b) the `(data_id, offset)` of each
   occurrence — these are the UDT `MarkerNode`s (marker bytes live at
   offset 0..14 of the 24-byte struct on wasm32).
3. BFS the `MarkerNode` graph from those seeds: at each node, zero the
   full 24-byte struct; resolve its `children` slice pointer to
   `(data_id, offset)`; zero the `len * 4`-byte slice backing; follow
   each child pointer into the next node. Container-wrapper nodes
   (whose own marker is all-zero, invisible to the bare byte scan) are
   reached this way via the pointers from UDT nodes that found them.
4. Compact each active data segment by splitting it around any run of
   zero bytes ≥ 16 bytes. Zeroing alone doesn't reduce the emitted
   wasm — zero bytes still serialize — but because live code references
   nothing inside the zeroed ranges, splitting lets those bytes
   simply not be emitted. Each split produces new segments at the
   original absolute memory addresses so surrounding code still
   resolves. Threshold 16 covers the per-segment header cost (mode +
   offset expr + length LEB128, ~5–10 bytes); below it, splitting
   would cost more than it saves.
5. Rewrite `contractspecv0` via `shaking::filter` to drop entries whose
   marker wasn't seen.

Safe because runtime code never reads through the graph — boundary
wrappers read pointer *values* off the stack and drop them; `MarkerNode`
fields (including pointer children between nodes) are never
dereferenced at runtime. The split-around-zeros pass preserves every
live byte's memory address; only the zeroed ranges stop being
serialized.

### Measured impact (test contracts, release wasm32v1-none)

| Contract | v1 baseline | Static-graph pre-strip | Static-graph post-strip | Post-strip vs v1 |
|---|---:|---:|---:|---:|
| `test_spec_shaking_v2` | 10,424 | 13,816 | **9,000**  | **−1,424** |
| `test_udt`             | 5,028* | 5,405  | **4,742**  | **−286** |

\* `test_udt` has no non-boundary types; its v1-equivalent baseline is
the post-strip size.

The pre-strip cost (+3,392 bytes on v2) comes from the `MarkerNode`
graph plus one boundary read-volatile per contract-fn boundary. Every
byte of that overhead turns into savings post-strip: unused
`contractspecv0` entries drop out, MARKER bytes zero out, and then the
segment-compaction pass splits data segments around the zeroed node
extents so those bytes stop being emitted. The final binary is
**smaller than the v1 baseline by ~1.4 kB on `test_spec_shaking_v2`**
because v1 ships every declared spec whether used or not, while v2
drops the unused ones *and* removes its own metadata after scanning.
Compared to the prior "all-`keep_reachable`" variant (per SKILL
numbers ~9,324 post-strip on v2), this approach lands ~300 bytes
smaller post-strip while also removing the entire fn-pointer surface
and keeping the stripper to pure data-section edits.

### Trade-offs vs the current mixed direct-call / `keep_reachable` design

| | Current (mixed) | Static-graph |
|---|---|---|
| Reachability mechanism | fn-pointer + direct inlined fn call | static-pointer graph + one volatile read |
| Cycles through heap indirection | handled by `keep_reachable` | handled natively via static cycle |
| Per-boundary runtime cost | 3 insn per direct-reachable type + 6 insn per `keep_reachable` edge | 6 insn, constant |
| Indirect-function-table surface | yes (one entry per standalone marker fn) | none |
| Stripper needs fn/element surgery | yes (all-`keep_reachable` variant) | no — scan + graph-walk + segment split + spec rewrite |
| `SpEcV1…` bytes strippable | no in mixed; yes in all-`keep_reachable` | yes, always; full `MarkerNode` extents also removed via segment compaction |
| Pre-strip vs current mixed | baseline | +~1.9 kB on `test_spec_shaking_v2` |
| Post-strip vs current mixed (all-`keep_reachable`) | ~9,324 bytes | **~9,000 bytes on `test_spec_shaking_v2`** |

Post-strip the static-graph design comes in slightly smaller than
all-`keep_reachable` while also dropping the indirect-function-table
surface and simplifying the stripper to pure data-section edits.

### Additional rules of thumb for the static-graph variant

- **Keep the impl shape uniform.** Primitive, container, and UDT impls
  all expose `const MARKER_NODE: *const MarkerNode`. Don't mix in an
  occasional fn-based impl — the cycle story and strippability depend
  on every impl behaving the same.
- **Never use `&'static MarkerNode` in the trait or in `children`.**
  The CTFE cycle detector will reject any recursive monomorphization.
  `*const MarkerNode` (via `&raw const STATIC` or `&expr as *const _`)
  is what makes recursive types compile.
- **Single-child containers forward; multi-child containers wrap.**
  Forwarding is zero-cost and handles all single-child cycles
  automatically. A wrapper is needed only when the container carries
  multiple distinct children.
