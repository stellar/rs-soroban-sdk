# Soroban Rust SDK v28

Version 28 of the Soroban Rust SDK is not released yet. Three of the changes it
carries are worth knowing about ahead of time, because two of them change how a
contract's data and events are encoded, and the third changes how contracts are
built.

Spec shaking, previously an experimental feature, is now the only behaviour.
`#[contracttype]` structs tolerate missing and additional fields when unpacking,
built on the sparse host functions added in
[CAP-86](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0086.md).
And `#[contractevent]` events with a map data format omit fields that hold no
value when publishing.

## Spec shaking is always on

A contract's spec is the `contractspecv0` custom section in its Wasm binary. It
holds an entry for every function, type, error, and event the contract defines.
Entries for types that never appear at the contract boundary carry no
information a caller can use, and cost bytes in the binary.

Before v28 the SDK decided what to include from Rust visibility plus an `export`
argument: `pub` types and all events got an entry, non-`pub` types did not, and
`export` overrode either way. That put the decision in the wrong place. Whether
a type belongs in the spec is a property of the contract's interface, not of the
module it happens to be declared in.

Spec shaking inverts it. The SDK emits an entry for everything, and the build
system strips the entries the contract does not use. The mechanism is a 14-byte
marker in the Wasm data section for each type and event: a `SpEcV1` magic prefix
followed by eight bytes of the SHA-256 of the entry's XDR. Markers are placed
inside code that only runs when the type is actually used at the boundary —
deserializing a parameter, serializing a return value, a `Result` conversion, a
`panic_with_error!`, an event `publish()`. A type's marker function calls the
marker functions of its field types, so nesting propagates, and so do `Vec<T>`,
`Map<K, V>`, `Option<T>`, and `Result<T, E>`. The Rust compiler's dead code
elimination removes the markers for types nothing reaches. Post-build tooling
scans for the markers that survived and strips every spec entry without one.

This was the `experimental_spec_shaking_v2` feature in v27. It has been enabled
automatically for every contract built with OpenZeppelin's `stellar-contracts`
v0.7.0 or newer since April 2026, so a large share of contracts already build
with the v28 behaviour. In v28 the feature flag is gone and the behaviour is
unconditional.

Two consequences follow.

`contractimport!` produces spec entries in the importing contract, where
previously it produced none. An importing contract's spec is now self-contained:
it carries the definitions of every type at its own boundary, wherever those
types were originally declared, and nothing else. Importing a large interface no
longer means callers have to go read the imported contract's spec to find the
types, and does not drag in entries for the parts of that interface the contract
does not touch.

The `export` argument on `contracttype`, `contracterror`, and `contractevent` is
removed. It was deprecated in v27, and is now a compile error pointing at the
argument. There is nothing left for it to do: `export = false` on a type that is
never reachable is what shaking already does, and `export = true` on a reachable
type is redundant. Drop the argument.

```rust
#[contracttype] // 👈 reachability determines the final spec
pub struct InternalState {
    pub counter: u32,
}
```

Because the final strip happens after the compiler runs, contracts must be built
with `stellar contract build` from `stellar-cli` v25.2.0 or newer. Building for
Wasm with anything else now fails:

```text
error: soroban-sdk requires stellar-cli v25.2.0+ to build a contract
```

The check only fires for Wasm targets. Native builds and unit tests are
unaffected.

## Contract data can be migrated

A `#[contracttype]` struct is represented on the ledger as a map keyed by field
names. Before v28 that map had to match the struct exactly. Every field of the
struct had to be present as a key, and no other key could be present. Anything
else was an error.

That made stored data effectively frozen. Adding a field meant every value
written by the previous version failed to unpack, and a contract that reads its
own data trapped rather than returned. Removing a field did the same. The only
way through was to migrate every stored value before deploying the new code,
which for a contract with data spread across many keys is not a thing that can
be done atomically.

CAP-86 added host functions that unpack maps sparsely, described in
[Migration-friendly contract data](https://stellar.org/blog/developers/introducing-adapter-protocol-28-on-stellar)
in the Protocol 28 announcement. v28 of the SDK uses them for `contracttype`
unpacking, and two differences that were errors are now tolerated.

**A field absent from the map unpacks as void.** Void unpacks into an `Option`
as `None`, so a field added to a struct as an `Option` reads back as `None` from
data stored before the field existed. A field of any other type errors, unless
its type accepts void, as the unit type and `Val` do.

```rust
#[contracttype]
pub struct State {
    pub count: u32,
    pub label: Option<u32>, // 👈 added after `count` was already stored
}

// A map holding only `count`, as stored before `label` existed.
let val = map![&env, (symbol_short!("count"), 5u32)].to_val();

assert_eq!(
    State::try_from_val(&env, &val),
    Ok(State { count: 5, label: None }),
);
```

**A key that is not a field of the struct is ignored.** A field removed from a
struct is discarded when reading data stored while the field still existed.

The migration strategy that falls out of this is: add fields as `Option`, and
remove fields by deleting them. Neither requires touching stored data first. A
field whose type does not accept void still needs the data migrated, because
there is no value to unpack it from.

Two caveats.

Discarded keys are not remembered. If a value is unpacked and then packed and
written back, the stored data loses every field the struct doing the write did
not have. Where more than one version of a struct reads the same stored data,
only the writer's fields survive.

Unpacking is no longer sufficient on its own to determine that a value is of the
expected type. A struct where every field is an `Option` unpacks from any map,
including an empty one and one written for an unrelated type, producing a value
with every field `None`. Where types need to be distinguished, include a field
that is not an `Option`, or store the values under distinct keys.

Packing is unchanged. Every field of the struct is written, including `Option`
fields that are `None`, which are written as void. Omitting them on write would
break cross-contract APIs whenever only one side of a call had upgraded. The
tolerance is on the read side, where the version skew actually is.

The change applies to structs with named fields, and to every unpack of one —
not only values read from storage. A contract function taking such a struct as
an argument accepts a map that omits its `Option` fields and carries keys the
struct does not define. There is no way to ask for the strict unpacking of
earlier versions. Tuple structs and enums are represented as vecs, not maps, and
are unaffected, as are `contractevent` and `contracterror` types.

## Events publish sparsely

An event declared with `data_format = "map"`, the default, publishes its data
fields as a map keyed by field names. Every field used to be written, including
fields whose value is void — an `Option` that is `None` among them. That is a
key and a void value carrying no information, in every event the field is unset.

In v28 a void field is omitted from the published map.

```rust
#[contractevent]
pub struct Transfer {
    #[topic]
    to: Address,
    to_muxed_id: Option<u64>,
    amount: i128,
}
```

A transfer to a muxed destination publishes `to_muxed_id` and `amount`. A
transfer to a plain address publishes only `amount`.

The saving in bytes is the smaller half of this. The larger half is that one
event type can now carry a field that is only sometimes meaningful, without
paying for it in every event. The SEP-41 `transfer` event is the case in point:
a single `Transfer` with an optional `to_muxed_id`, rather than two event types,
one with the muxed id and one without. Consumers see the field when it means
something and no trace of it when it does not.

This applies only to the top level of an event's data map. Packing everywhere
else is unchanged: a `contracttype` struct writes all of its fields, whether it
is stored, passed to a function, or nested inside the value of an event field.

An event that must keep publishing every field opts out:

```rust
#[contractevent(sparse = false)]
pub struct Transfer {
    #[topic]
    to: Address,
    to_muxed_id: Option<u64>,
    amount: i128,
}
```

The opt out is for the edge cases where an absent field and a `None` field need
to be visibly different to a consumer. Prefer the default. `sparse` applies only
to the map data format, and is a compile error on `single-value` and `vec`.

## Upgrading

Publishing events needs no code changes. Nor does most unpacking. What does need
attention:

- Build with `stellar contract build` from `stellar-cli` v25.2.0 or newer.
- Remove `experimental_spec_shaking_v2` from the `soroban-sdk` dependency's
  feature list, and remove every `export` argument.
- Review consumers that read an event's data map, because a field they expect
  present with a void value is now absent.
- Review code that relied on unpacking failing to detect a mismatch.
- Update tests asserting on the data map of an event with a void field. Rather
  than asserting field by field, compare against the event's `to_xdr` form,
  which is built the same way as the published event and stays correct as the
  packing changes:

  ```rust
  assert_eq!(env.events().all(), [event.to_xdr(&env, &id)]);
  ```

The full set of breaking changes, with runnable examples, is in the SDK's
`_migrating` module.

## Taken together

The three changes pull in the same direction: the contract's boundary decides
what leaves the contract, and nothing else does. Spec shaking means the spec
describes the interface rather than the source layout. Sparse unpacking means
stored data can gain and lose fields while the contract keeps reading it. Sparse
publishing means an event describes what happened rather than the shape of the
struct that recorded it.
