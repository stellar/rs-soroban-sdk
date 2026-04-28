---
name: spec-shaking
description: Spec shaking v2 in soroban-sdk — what it is, why it exists, and the four reachability designs we've explored (direct calls, function-pointer keep-alive via `keep_reachable`, a static data graph, and root-only pruning with a removable sidecar graph). For each design, covers what it does, what it costs in the emitted WASM, and what a post-build tool can strip. Use when changing `soroban-sdk/src/spec_shaking.rs`, `soroban-sdk-macros/src/shaking.rs`, the `TryFromValForContractFn` / `IntoValForContractFn` boundary wrappers, or the `soroban-spec::shaking` / `soroban-spec::strip` post-build helpers.
---

# Spec shaking v2 in soroban-sdk

## What spec shaking v2 is

A two-part mechanism that lets a contract WASM ship only the
`contractspecv0` entries actually reachable from a contract boundary:

1. **In-WASM**: the SDK emits 14-byte markers (`SpEcV1` magic + first
   8 bytes of `SHA256(spec_entry_xdr)`) for roots whose reachability
   cannot be inferred from `contractspecv0`. In solutions 1–3 this was
   every reachable user type; in solution 4 this is only event publish
   roots.
2. **Post-build**: `soroban-spec::shaking::find_all` scans the data
  section for surviving `SpEcV1…` patterns; `shaking::filter_with_graph`
  decides which entries are reachable; `soroban_spec::strip::shake_contract_spec`
  rewrites `contractspecv0` and removes the private sidecar graph.

The contract metav0 key `rssdk_spec_shaking = "2"` declares that v2
markers are present; the post-build tool checks this before stripping.

Solutions 1–3 encode type reachability in generated marker code or
data. Solution 4 instead treats `contractspecv0` as the type graph,
uses markers only for roots that cannot be inferred from the spec, and
uses a removable private sidecar graph so duplicate UDT names can be
resolved by exact spec-entry identity. The current CLI-facing
post-build entry point is `soroban_spec::strip::shake_contract_spec`,
which packages marker scanning, graph-aware filtering, `contractspecv0`
rewriting, and sidecar removal behind one narrow API.

Boundary sites that root reachability:

1. Contract function input parameters (`TryFromValForContractFn`) —
   marker-rooted in solutions 1–3, spec-rooted in solution 4.
2. Contract function return values (`IntoValForContractFn`) —
   marker-rooted in solutions 1–3, spec-rooted in solution 4.
3. Event publishing (`#[contractevent]::publish`) — marker-rooted in
  all reachability designs.

## Why we need it

`#[contracttype]`, `#[contractevent]`, and `#[contracterror]` macros
emit an `ScSpecEntry` for every annotated type into `contractspecv0`.
The linker doesn't inspect that custom section, so unused-type entries
ship as-is. Contracts that declare many helper types but use few at the
boundary ship significant on-chain bloat.

The whole problem is reachability *signaling*: there's no first-class
WASM "this byte is needed" annotation, so the SDK has to wire
type-level reachability into rustc/LLVM/wasm-ld's dead-code-elimination
machinery, and the post-build tool has to read the result back out.
Every design below is a different way to do that wiring.

---

# Solution 1 — direct calls (`T::spec_shaking_marker()`)

## What it is

Every type implements `SpecShakingMarker::spec_shaking_marker()`. The
fn body recursively calls each field type's `spec_shaking_marker()`,
then volatile-reads its own 14-byte `MARKER` static. The fn is
`#[inline(always)]`, so the chain collapses into the caller. Boundary
wrappers call `U::spec_shaking_marker()` on each arg/return type.

```rust
impl SpecShakingMarker for MyType {
    #[inline(always)]
    fn spec_shaking_marker() {
        <Field1 as SpecShakingMarker>::spec_shaking_marker();
        <Field2 as SpecShakingMarker>::spec_shaking_marker();
        #[cfg(target_family = "wasm")]
        {
            static MARKER: [u8; 14] = *b"SpEcV1\xXX\xXX\xXX\xXX\xXX\xXX\xXX\xXX";
            let _ = unsafe { core::ptr::read_volatile(MARKER.as_ptr()) };
        }
    }
}
```

The `read_volatile` is what LLVM can't optimise away — it forces the
caller body to reference `MARKER`, which forces the 14 bytes into the
data section at link time. Recursive calls into field markers
propagate reachability through the type graph: any reachable field's
marker fn is alive, so its `MARKER` survives too.

## Impact on the output WASM

After full inlining, each transitively-reachable type contributes a
three-instruction marker load at the boundary call site:

```
i32.const 0
i32.load8_u offset=<MARKER_addr>
drop
```

That's a real one-byte load from the data section per reachable type,
**every time the boundary is invoked at runtime**. For a boundary
touching N reachable types, that's N data-section loads on every
contract function call.

Per-type data cost: 14 bytes for the `MARKER` static. No standalone
marker fn is emitted (everything inlined); no indirect function table
surface.

## Post-build cleanup possible

Limited.

- **`contractspecv0` filtering**: scan for surviving `SpEcV1…` bytes
  and drop unmatched entries. ✅ Always safe.
- **Zeroing `MARKER` bytes**: ❌ Not safe. Wrapper bodies contain
  inlined `i32.load8_u offset=<MARKER_addr>` instructions that
  reference these bytes at runtime. Zeroing would make them read
  garbage; removing them outright would shift later addresses and
  break the loads.
- **Removing the load instructions themselves**: would require
  pattern-matching on every wrapper body and rewriting it. Possible
  but a separate, much bigger project.

## Why it's not enough on its own

A direct-call chain `S::marker → Vec<S>::marker → S::marker → …` would
inline forever and stack-overflow at runtime. Direct call only works
for types where Rust's type system already prevents recursion at
compile time (`Option<S>` in `S`, tuple fields, embedded UDT fields).
Heap-indirected recursion (`Vec<S>` in `S`, `Map<S, X>` in `S`) is
type-system-legal in Rust and needs another mechanism. That's where
solutions 2 and 3 come in.

---

# Solution 2 — function pointers (`keep_reachable`)

## What it is

Instead of *calling* the inner type's marker fn, take its *address*:

```rust
fn keep_reachable(f: fn()) {
    let _ = unsafe { core::ptr::read_volatile(&f) };
}

impl<T: SpecShakingMarker> SpecShakingMarker for Vec<T> {
    #[inline(always)]
    fn spec_shaking_marker() {
        keep_reachable(T::spec_shaking_marker);
    }
}
```

The volatile-read is on the fn pointer in a stack slot — never
invoked. Address-taking forces the referenced function to be emitted
as a standalone `() -> ()` and added to the indirect function table,
which keeps its body (and therefore its `MARKER` bytes) alive through
DCE. The inlining chain terminates at the address-take, breaking the
cycle.

## Impact on the output WASM

Three pieces per `keep_reachable` edge:

1. **At each caller site (after inlining)**, a 6-instruction sequence:
   ```
   local.get <base>
   i32.const <fn_idx>
   i32.store offset=<slot>     ;; store fn ptr to stack slot
   local.get <base>
   i32.load offset=<slot>      ;; volatile-load it back
   drop
   ```
   Runs every time the boundary is invoked. Nothing downstream
   interprets `<fn_idx>` as a function reference — it's just a number
   moved through a stack slot.

2. **A standalone marker fn** for the referenced type, emitted into
   the code section with its body intact. The body contains the
   `read_volatile(MARKER.as_ptr())` that keeps the type's `MARKER`
   bytes alive at link time. The body is **never called at runtime**
   in the `keep_reachable` path.

3. **An entry in the element section** (indirect function table)
   pointing at that standalone fn.

Typical overhead per `keep_reachable` edge: ~10–40 bytes per
standalone fn body + 1 byte element-table entry + 1 byte
function-section entry + 14 bytes of `MARKER` in data + 6 runtime
instructions per caller site (vs ~3 for direct call).

## Post-build cleanup possible

Substantially better than solution 1, especially in the
"all-`keep_reachable`" variant where every type uses this mechanism.

- **`contractspecv0` filtering**: ✅ Always safe.
- **Standalone marker fns** referenced only via the element table
  (not exported, not directly called): ✅ Safe to delete. The four
  conservative filters in `soroban-spec/src/strip.rs::identify_marker_fns`
  encode this: signature `() -> ()`, in element-set, not exported,
  not directly called.
- **Element-table entries** pointing at deleted marker fns: ✅ Safe to
  clear. The slot becomes `ref.null`; nothing `call_indirect`s it.
- **`MARKER` bytes**: ✅ Safe to zero, **iff** every path to them was
  through `keep_reachable` (no inlined direct-call MARKER load
  remaining). In the mixed direct + `keep_reachable` design, wrapper
  bodies contain inlined direct-call MARKER loads, so the bytes stay.
  In the all-`keep_reachable` variant, every load lives inside a
  standalone fn body that the stripper just deleted, so the bytes
  are unreferenced and can be zeroed.
- **The `i32.const <fn_idx>` immediates** left behind in caller bodies
  pointing at deleted fns: harmless. They're consumed by the
  `store`/`volatile-load`/`drop` sequence, never by `call_indirect`.

If you commit to the post-build stripper, the all-`keep_reachable`
variant is the cleanest version of this solution: the stripper is
section surgery (remove fns, clear element entries, zero `MARKER`
bytes, rewrite `contractspecv0`), and the entire `MARKER` data is
recoverable. It costs ~+230 bytes pre-strip on `test_spec_shaking_v2`
vs the mixed design but saves ~2.5 kB post-strip.

---

# Solution 3 — static data graph

## What it is

Each type exposes a per-type `static` of a struct holding its 14-byte
marker plus references to child types' markers. Reachability flows
through the linker's data-section relocation tracking: when one
struct's bytes contain a pointer to another static, keeping the first
alive transitively keeps the second alive. Boundary wrappers root the
graph with one volatile read per fn-call site.

We've explored two flavours.

### Flavour A: hybrid (current `refactor-shaking-marker-static-tuple` branch)

`spec_shaking_marker()` survives — but its body is only one volatile
read of a per-type packed struct, instead of recursive calls + marker
load. Each UDT also gets a `static __SPEC_SHAKING_MARKER_<UDT>` of
that struct, holding the marker bytes and an array of `&'static [u8]`
refs to each field's `SPEC_SHAKING_MARKER_REF`:

```rust
#[repr(packed)]
pub struct __SpecShakingMarkerOfMyType {
    pub marker: [u8; 14],
    pub fields: [&'static [u8]; 2],
}
pub static __SPEC_SHAKING_MARKER_MYTYPE: __SpecShakingMarkerOfMyType =
    __SpecShakingMarkerOfMyType {
        marker: *b"SpEcV1\xXX\xXX\xXX\xXX\xXX\xXX\xXX\xXX",
        fields: [
            <Field1 as SpecShakingMarker>::SPEC_SHAKING_MARKER_REF,
            <Field2 as SpecShakingMarker>::SPEC_SHAKING_MARKER_REF,
        ],
    };
impl SpecShakingMarker for MyType {
    const SPEC_SHAKING_MARKER_REF: &'static [u8] =
        &__SPEC_SHAKING_MARKER_MYTYPE.marker;
    #[inline(always)]
    fn spec_shaking_marker() {
        #[cfg(target_family = "wasm")]
        {
            let _ = unsafe { core::ptr::read_volatile(
                &__SPEC_SHAKING_MARKER_MYTYPE as *const _ as *const u8,
            ) };
        }
    }
}
```

The fn still recursively calls `spec_shaking_marker()` on each field
type, because the `SPEC_SHAKING_MARKER_REF` const is single-slot:
multi-arg containers (`Map<K, V>`, `Result<T, E>`, tuples ≥ 2) can't
expose all children through one `&'static [u8]`, so the fn-call
chain handles those. The hybrid design is essentially solution 1's
fn structure with the per-type marker propagation upgraded to a
static graph for single-slot containers.

### Flavour B: pure (explored on `full-fn-ptr` branch)

No `fn spec_shaking_marker()` at all. Every type — primitive,
container, UDT — exposes:

```rust
const MARKER_NODE: *const MarkerNode;

pub struct MarkerNode {
    pub marker: [u8; 14],
    pub children: &'static [*const MarkerNode],
}
```

UDTs emit a named module-scope `static` plus `const MARKER_NODE = &raw const STATIC`. Multi-child containers emit a promoted
`&MarkerNode { … } as *const MarkerNode`. Single-child containers
forward (`T::MARKER_NODE`). Boundary wrappers do
`read_volatile(&T::MARKER_NODE)`.

Raw pointers (not `&'static`) are required to sidestep rustc's CTFE
cycle detector — `&'static MarkerNode` triggers an "evaluating
initializer of static" cycle error on recursive types, while
`*const MarkerNode` (via `&raw const STATIC` or `&expr as *const _`)
doesn't walk into the target.

## Impact on the output WASM

Per UDT in flavour A: 14 bytes marker + `N * 8` bytes for the
`[&'static [u8]; N]` slice headers (on wasm32, each `&[u8]` is
ptr+len = 8 bytes). Plus a small fn body containing one volatile read.

Per UDT in flavour B: 14 bytes marker + 2 padding + 8 slice header +
`N * 4` bytes for child pointer array = 24 + 4N bytes. No fn body at
all.

Both flavours: no indirect-function-table surface; no standalone
marker fns. Per-boundary-call cost is **constant** in instruction
count regardless of reachability depth (one ~6-instruction
`store/load/drop` of a pointer immediate), versus solution 1's
O(N) data-section loads or solution 2's O(edges) `keep_reachable`
sequences.

Pre-strip overhead on `test_spec_shaking_v2`:
- Mixed direct + `keep_reachable` (solutions 1 + 2): 11,925
- All-`keep_reachable` (solution 2 throughout): 12,155
- Static graph flavour B: 13,816

So solution 3 pays ~+1.7 kB pre-strip vs all-`keep_reachable`. That
cost is recoverable post-strip.

## Post-build cleanup possible

The most strippable of the three designs.

- **`contractspecv0` filtering**: ✅ Always safe.
- **`MARKER` bytes** (the 14-byte `SpEcV1…` span inside each node):
  ✅ Safe to zero. Wrapper bodies only do
  `read_volatile(&T::MARKER_NODE)` — they read pointer *values* off
  the stack and drop them; the `MarkerNode` bytes are never
  dereferenced at runtime.
- **The full `MarkerNode` struct extent** (24 bytes including marker,
  padding, slice header) and the **children slice backing**
  (N * 4 bytes of child pointers): ✅ Safe to zero. Same reasoning —
  nothing reads through them. The stripper reaches them by BFS-ing
  the graph from each `SpEcV1` seed: at each node, parse its slice
  pointer/length, resolve the address back to a `(data_segment,
  offset)`, zero the slice backing, follow each child pointer to the
  next node. Container-wrapper nodes (whose own marker bytes are
  all-zero, invisible to a bare byte scan) are reached this way via
  pointers from UDT nodes that the scan finds.
- **Recovering the bytes from the WASM file**: zeroing alone doesn't
  shrink the binary — zero bytes still serialize. To actually
  recover bytes, split each active data segment around any zero run
  ≥ ~16 bytes. Each surviving piece is re-emitted at its original
  absolute memory address, so live code's `i32.const <addr>`
  references still resolve. Threshold 16 covers the per-segment
  header overhead (mode + offset expr + length LEB128, ~5–10 bytes);
  below it, splitting costs more than it saves.

The stripper is **pure data-section work** — no fn deletion, no
element-table surgery, no instruction rewriting. Implemented in
`soroban-spec/src/strip.rs::strip_spec_shaking` (on the static-graph
branch).

Post-strip on `test_spec_shaking_v2` with the full graph-walk +
segment-compaction stripper: **~9,000 bytes**. Lands ~300 bytes
under all-`keep_reachable`'s ~9,324 and ~1,400 bytes under the v1
baseline of 10,424 (v1 ships every declared spec whether used or not;
v2 drops the unused entries *and* removes its own metadata after
scanning).

## Trade-offs and risks

vs. solution 1: handles cycles natively (no inlining recursion).
Larger pre-strip; far smaller post-strip.

vs. solution 2: no indirect-function-table surface; stripper is
simpler; per-boundary runtime cost is O(1) instead of O(edges).
Pre-strip larger by ~1.7 kB on `test_spec_shaking_v2`; post-strip
~300 bytes smaller after segment compaction.

**Risk specific to solution 3**: depends on rustc CTFE not walking
into `*const STATIC` targets. Currently it doesn't; a future version
conceivably could tighten this. Solutions 1 and 2 rely only on
`read_volatile(&fn_ptr)` preserving fn-pointer addresses, which is
fundamental and not going to change. If `*const STATIC` CTFE behaviour
ever breaks the static-graph design, falling back to all-`keep_reachable`
recovers a working build.

The `unsafe impl Sync for MarkerNode` (flavour B) — needed because
raw pointers aren't `Sync` by default — is a small footgun for
maintainers, sound only because the pointers are never dereferenced
at runtime.

---

# Solution 4 — root-only pruning with removable sidecar graph

## What it is

This is the current design. It stops trying to encode the full UDT
graph in runtime marker code. The SDK still emits every candidate
`ScSpecEntry` into `contractspecv0`, but reachability is computed by
the post-build tool from contract boundary roots plus a removable
private sidecar graph.

The only in-WASM markers are for roots that cannot be inferred from
`contractspecv0`. Today that means event publish sites: a generated
`#[contractevent]::publish` method volatile-reads a 14-byte marker for
that event entry. Function roots do not need markers because exported
contract functions are already explicit `FunctionV0` entries in the
spec.

For exact UDT reachability, macros also emit graph records into a
private custom section named `contractspecv0.rssdk.graphv0`. The public
`contractspecv0` format stays unchanged. Each graph record is keyed by
the full 32-byte `SHA256(spec_entry_xdr)` ID of the spec entry and lists
the exact child spec IDs referenced by that entry.

The fixed graph record header is:

```text
8 bytes  magic: "SpGrV0\0\0"
1 byte   kind: 0 function, 1 event, 2 UDT
7 bytes  reserved
32 bytes spec_id: SHA256(spec_entry_xdr)
4 bytes  ref_count_le
N*32     referenced spec IDs
```

Macros emit these records while they still know the original Rust type
paths. UDT macros also implement the hidden SDK trait:

```rust
pub trait SpecTypeId {
  const SPEC_TYPE_ID: [u8; 32];
}
```

Function records point at exact input/output UDT IDs. Event records
point at exact event param UDT IDs. UDT records point at exact field or
union-case UDT IDs. Containers such as `Option`, `Result`, `Vec`, `Map`,
tuples, and references are recursively traversed by the macro helper.

Post-build filtering works like this:

1. Scan the WASM data section for surviving `SpEcV1…` event markers.
2. Read the sidecar with `soroban_spec::shaking::find_graph`.
3. Keep every `FunctionV0` entry and enqueue its graph refs.
4. Keep only marked `EventV0` entries and enqueue their graph refs.
5. BFS exact spec IDs through UDT graph records.
6. Keep matching entries and collapse exact duplicate XDR entries.
7. Rewrite `contractspecv0` and remove `contractspecv0.rssdk.graphv0`.

If the sidecar graph is absent, `filter_with_graph` falls back to the
older name-based traversal over `ScSpecTypeDef` references. In that
fallback only, duplicate UDT names are handled conservatively: if a
referenced name matches multiple distinct UDT entries, keep all matching
entries. With the sidecar present, traversal is by full spec-entry hash,
so distinct same-name UDTs can be disambiguated.

## Impact on the output WASM

Function boundaries have **zero spec-shaking runtime marker code**.
There is no `SpecShakingMarker` trait, no recursive type marker impl,
no `keep_reachable`, no marker function body, and no static graph node
that executes at runtime for function input/output types.

Each event type whose generated `publish` method is reachable contributes
one marker block at the publish site:

```rust
#[cfg(target_family = "wasm")]
{
    static MARKER: [u8; 14] = *b"SpEcV1\xXX\xXX\xXX\xXX\xXX\xXX\xXX\xXX";
    let _ = unsafe { core::ptr::read_volatile(MARKER.as_ptr()) };
}
```

That is the same small data-section marker and one-byte volatile load
used by the other designs, but only for published events. UDT
reachability is paid for in the post-build graph walk rather than at
contract runtime.

The pre-strip WASM can still contain unused `contractspecv0` entries,
because macros keep emitting candidate specs. It also contains one
removable graph record per emitted spec entry. That sidecar increases
the pre-filter binary, but it is private build metadata and fully
removed from the final output.

## Post-build cleanup possible

Simple and precise.

- **`contractspecv0` filtering**: ✅ Keep functions, marked events,
  and exact graph-reachable UDTs; drop everything else.
- **Sidecar graph removal**: ✅ Drop every
  `contractspecv0.rssdk.graphv0` custom section after rewriting the
  public spec.
- **Marker bytes**: event marker bytes can be zeroed or compacted only
  if the corresponding inlined volatile loads are also removed or made
  irrelevant by a broader stripper. The basic filter does not need to
  mutate marker bytes at all.
- **Functions / element table / runtime marker graph data**: no cleanup
  needed, because this design does not emit marker fns, element-table
  roots, or runtime marker graph nodes.

The cleanup logic is intentionally less powerful than solution 3's full
data-section compaction, but it has much less surface area: scan event
markers, parse `contractspecv0`, read the removable sidecar graph, run
exact graph traversal, rewrite `contractspecv0`, and drop the sidecar.

## Trade-offs and risks

Best fit when the priority is minimal runtime impact plus exact
post-build pruning.

vs. solutions 1 and 2: removes marker calls from function boundaries
and avoids recursive marker code entirely. There is no direct-call
cycle problem and no indirect-function-table surface.

vs. solution 3: emits less reachability metadata and needs no custom
data graph cleanup. The post-strip binary may retain event marker bytes
unless a separate data-section cleanup pass removes them, but those
bytes are small and event-only.

The sidecar is compile-time and post-build metadata only; it does not
add contract runtime code beyond event markers. It does increase the
pre-filter WASM by one static graph record per emitted spec entry, but
those bytes are fully removable.

The graph is only as complete as macro emission. If a type appears in a
function/event/UDT boundary but does not implement `SpecTypeId`, the
macro must either treat it as an SDK built-in or compilation fails when
`experimental_spec_shaking_v2` is enabled. Keep the macro helper's
built-in list aligned with `map_type` and SDK boundary-only types such
as `auth::Context`.

This design also assumes function specs are the correct public API
roots. That matches the contract interface model: functions are always
kept because they define the callable API, while events need markers
because a declared event type may never be published by reachable code.

---

# Cross-cutting practical notes

- **Markers only exist when the macro's `spec_shaking_v2_enabled()`
  is true.** That requires both the `experimental_spec_shaking_v2`
  Cargo feature and the `SOROBAN_SDK_BUILD_SYSTEM_SUPPORTS_SPEC_SHAKING_V2=1`
  env var at build time. `soroban-sdk/build.rs` sets a `spec_shaking_v2`
  cfg from both signals and gates the entire `spec_shaking` module on it.
- **Cargo feature unification leaks `experimental_spec_shaking_v2`
  into sibling test crates** when running `cargo test` across multiple
  contracts. For a clean v1-baseline measurement, build the v1 contract
  in isolation (`cargo build -p test_spec_shaking_v1`) or use the
  Makefile's per-crate sequence in `build-test-wasms`.
- **`tests-expanded/` checks in macro-expanded sources.** Update via
  `make expand-tests`; inspect those to see exactly what each macro
  emits in the current variant.
- **Test surface**: `test_spec_shaking_v1` is the no-marker baseline,
  `test_spec_shaking_v2` exercises the full marker surface across
  containers/refs/events/recursion, `test_udt` exercises genuine
  recursion through `Vec`/`Map`.
- **Lifetime erasure in the macro** (solution 3 flavours): module-scope
  statics can't see lifetime params declared on the impl (`<'a>`,
  etc.). The macro substitutes `'static` for every lifetime in field
  types via `syn::visit_mut`. Sound because `&'a T: SpecShakingMarker`
  forwards to `T`, so the marker value is lifetime-independent.

# Key locations

| Role | Path |
|---|---|
| Trait + container impls | `soroban-sdk/src/spec_shaking.rs` |
| Macro-generated marker emit | `soroban-sdk-macros/src/shaking.rs` |
| Boundary-wrapper call sites | `soroban-sdk/src/{try_from_val_for_contract_fn,into_val_for_contract_fn}.rs` |
| Event-publish call site | `soroban-sdk-macros/src/derive_event.rs` |
| Feature gate (cfg) | `soroban-sdk/build.rs`, `soroban-sdk-macros/src/lib.rs` → `spec_shaking_v2_enabled` |
| Post-build scan + filter | `soroban-spec/src/shaking.rs` |
| Post-build wasm stripping | `soroban-spec/src/strip.rs` |
| Test contracts | `tests/spec_shaking_v1/`, `tests/spec_shaking_v2/`, `tests/udt/` |
