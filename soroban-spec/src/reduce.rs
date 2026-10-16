//! Reduce the user-defined type and event names in a contract spec from their
//! fully qualified form to simple names.
//!
//! [`reduce`] is a const fn over the XDR encoding of the spec that needs
//! neither std nor an allocator, so that it can reduce a spec at compile time,
//! as the token and Stellar Asset contract spec crates do for the specs they
//! publish, as well as at runtime, as tools do for a contract's spec.

use crate::names::{GENERATED_NAMES, RESERVED_NAMES};

/// The most bytes a spec type or event name can hold
/// (`SC_SPEC_TYPE_NAME_LIMIT`), which bounds the names the numbering may
/// produce.
const NAME_LIMIT: usize = 1024;

/// The most type and event names a spec can define for it to be reduced.
const MAX_NAMES: usize = 1024;

/// The deepest types can nest within a spec entry for it to be reduced.
const DEPTH_LIMIT: u32 = 500;

/// A spec that cannot be reduced.
///
/// The names are byte strings as they appear in the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<'a> {
    /// The spec defines the same fully qualified user-defined type or event
    /// name more than once, with or without a leading `::`, which never
    /// happens, as a qualified name is the path of the one Rust type that
    /// defines it. Simple names are not checked, as specs from before
    /// qualified names can define the same simple name more than once.
    DuplicateName(&'a [u8]),
    /// The spec defines or refers to a name whose last segment is empty, such
    /// as `mycrate::` or an empty name, which no Rust type has.
    InvalidName(&'a [u8]),
    /// The spec refers to a fully qualified name that it does not define. A
    /// spec defines every type it refers to, and a reference that reduced to
    /// its last segment could not be told apart from another type that shares
    /// it, so it would bind to the wrong type, or to none, in the generated
    /// code. Simple names are not checked, as they are not reduced.
    UndefinedName(&'a [u8]),
    /// The spec defines more names than can be reduced.
    TooManyNames,
    /// The spec is not a valid XDR encoding of spec entries, or nests types
    /// too deeply.
    InvalidXdr,
    /// The output is not the length of the reduced spec.
    OutputLength,
}

impl core::fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fn name(f: &mut core::fmt::Formatter<'_>, name: &[u8]) -> core::fmt::Result {
            for chunk in name.utf8_chunks() {
                f.write_str(chunk.valid())?;
                if !chunk.invalid().is_empty() {
                    f.write_str("\u{FFFD}")?;
                }
            }
            Ok(())
        }
        match self {
            Error::DuplicateName(n) => {
                f.write_str("contract spec defines the name `")?;
                name(f, n)?;
                f.write_str("` more than once")
            }
            Error::InvalidName(n) => {
                f.write_str("contract spec contains the name `")?;
                name(f, n)?;
                f.write_str("`, whose last segment is empty")
            }
            Error::UndefinedName(n) => {
                f.write_str("contract spec refers to the name `")?;
                name(f, n)?;
                f.write_str("`, which it does not define")
            }
            Error::TooManyNames => f.write_str("contract spec defines too many names to reduce"),
            Error::InvalidXdr => f.write_str("contract spec is not valid spec entry XDR"),
            Error::OutputLength => {
                f.write_str("output is not the length of the reduced contract spec")
            }
        }
    }
}

impl core::error::Error for Error<'_> {}

/// Reduces every user-defined type and event name in the spec from its fully
/// qualified form (`mycrate::mymod::MyType`) to its simple name (`MyType`),
/// rewriting every reference to a type to follow the type to its new name.
///
/// The spec is the XDR encoding of its entries, split across any number of
/// slices, each holding zero or more whole entries. With no output, returns
/// the length of the reduced spec. With an output, which must be exactly that
/// length, writes the XDR encoding of the reduced entries into it, in the
/// order of the spec, and returns the length.
///
/// ```
/// # const SPEC: &[&[u8]] = &[];
/// const LEN: usize = match soroban_spec::reduce::reduce(SPEC, None) {
///     Ok(len) => len,
///     Err(_) => panic!("spec cannot be reduced"),
/// };
/// const REDUCED: [u8; LEN] = {
///     let mut out = [0u8; LEN];
///     if soroban_spec::reduce::reduce(SPEC, Some(&mut out)).is_err() {
///         panic!("spec cannot be reduced");
///     }
///     out
/// };
/// ```
///
/// Two types whose names share a last segment stay distinct: the one whose
/// full name sorts first keeps the simple name, and the rest are numbered
/// (`MyType2`, `MyType3`, …) in the order of their full names, stepping over
/// names claimed by other types. The names do not depend on the order the
/// spec defines the types in. A spec whose type names are already simple,
/// and are not the names of SDK types, comes back unchanged.
///
/// A reference by a fully qualified name to a type the spec does not define
/// is an error. A reference by a simple name is left as it is, whether or not
/// the spec defines it.
///
/// Names are compared without a leading `::`, so a path written from the
/// crate root, as the SDK writes names (`::mycrate::MyType`), and the same
/// path without it, as other tools may write names (`mycrate::MyType`), name
/// the same type: a reference in either form follows the type to its name,
/// numbered or not, and defining both forms defines the name twice.
///
/// Names are treated as byte strings throughout and preserved exactly; the
/// spec does not guarantee valid UTF-8. When a numbered name would exceed the
/// spec's name limit, the base is trimmed from its end to make room for the
/// number, at a character boundary if the base is valid UTF-8.
///
/// A simple name, one without `::`, is not reduced: it keeps its name, and
/// claims it ahead of the qualified names, so specs from before qualified
/// names, which can define the same simple name more than once, come back
/// unchanged.
///
/// The names of SDK types, listed in [`RESERVED_NAMES`], are taken before any
/// type claims a name, as code generated for a type with one of them would be
/// rejected or mistaken for the SDK type. So are the names of the items
/// generated alongside the types, listed in [`GENERATED_NAMES`], as a type
/// with one of them would collide with the generated item. A type with one of
/// them is numbered (`Address2`, `Client2`, …) like any other collision,
/// including a simple name, which older specs can define, for example as an
/// event name.
///
/// # Errors
///
/// - If the spec defines or refers to a name whose last segment is empty.
/// - If the spec defines the same qualified name more than once.
/// - If the spec refers to a qualified name that it does not define.
/// - If the spec defines more than 1024 names.
/// - If the spec is not a valid encoding of spec entries.
/// - If the output is not the length of the reduced spec.
pub const fn reduce<'a>(spec: &[&'a [u8]], out: Option<&mut [u8]>) -> Result<usize, Error<'a>> {
    let mut empty = [0u8; 0];
    let (out, mode) = match out {
        Some(out) => (out, Mode::Write),
        None => (&mut empty as &mut [u8], Mode::Measure),
    };
    let mut ctx = Ctx::new(out);

    ctx.mode = Mode::Collect;
    walk_spec(&mut ctx, spec);
    if ctx.error.is_none() {
        validate(&mut ctx);
    }
    if ctx.error.is_none() {
        assign(&mut ctx);
    }
    if ctx.error.is_none() {
        ctx.mode = mode;
        ctx.at = 0;
        walk_spec(&mut ctx, spec);
    }

    if let Some(e) = ctx.error {
        return Err(e);
    }
    if let Mode::Write = mode {
        if ctx.at != ctx.out.len() {
            return Err(Error::OutputLength);
        }
    }
    Ok(ctx.at)
}

#[derive(Clone, Copy)]
enum Mode {
    /// Record the names the spec defines.
    Collect,
    /// Count the bytes of the reduced spec.
    Measure,
    /// Write the reduced spec.
    Write,
}

/// A reduced name: a part of a name in the spec, followed by a number if it
/// is numbered.
#[derive(Clone, Copy)]
struct Name<'a> {
    base: &'a [u8],
    /// The number, or zero if the name is not numbered.
    n: u32,
}

impl<'a> Name<'a> {
    const EMPTY: Name<'static> = Name { base: &[], n: 0 };

    const fn new(base: &'a [u8]) -> Self {
        Name { base, n: 0 }
    }

    /// The decimal digits of the number, right aligned, and how many there
    /// are.
    const fn digits(&self) -> ([u8; 10], usize) {
        let mut digits = [0u8; 10];
        let mut count = 0;
        let mut v = self.n;
        while v != 0 {
            digits[digits.len() - 1 - count] = b'0' + (v % 10) as u8;
            count += 1;
            v /= 10;
        }
        (digits, count)
    }

    const fn len(&self) -> usize {
        self.base.len() + self.digits().1
    }

    /// The byte at the index of the name.
    const fn byte(&self, i: usize) -> u8 {
        if i < self.base.len() {
            self.base[i]
        } else {
            let (digits, count) = self.digits();
            digits[digits.len() - count + i - self.base.len()]
        }
    }

    const fn eq(&self, other: &Name) -> bool {
        let len = self.len();
        if len != other.len() {
            return false;
        }
        let mut i = 0;
        while i < len {
            if self.byte(i) != other.byte(i) {
                return false;
            }
            i += 1;
        }
        true
    }
}

struct Ctx<'a, 'o> {
    mode: Mode,
    /// The first reason the spec cannot be reduced.
    error: Option<Error<'a>>,
    /// The first reference whose last segment is empty, reported after the
    /// definitions are checked.
    invalid_ref: Option<&'a [u8]>,
    /// How deeply the type being walked is nested.
    depth: u32,
    /// The names the spec defines, in definition order.
    defined: [&'a [u8]; MAX_NAMES],
    count: usize,
    /// What each defined name reduces to.
    assigned: [Name<'a>; MAX_NAMES],
    out: &'o mut [u8],
    at: usize,
}

impl<'a, 'o> Ctx<'a, 'o> {
    const fn new(out: &'o mut [u8]) -> Self {
        Ctx {
            mode: Mode::Collect,
            error: None,
            invalid_ref: None,
            depth: 0,
            defined: [&[]; MAX_NAMES],
            count: 0,
            assigned: [Name::EMPTY; MAX_NAMES],
            out,
            at: 0,
        }
    }

    const fn fail(&mut self, e: Error<'a>) {
        if self.error.is_none() {
            self.error = Some(e);
        }
    }

    const fn ok(&self) -> bool {
        self.error.is_none()
    }
}

// Name helpers.

const fn eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Whether `a` sorts before `b`, as byte slices compare.
const fn less(a: &[u8], b: &[u8]) -> bool {
    let mut i = 0;
    while i < a.len() && i < b.len() {
        if a[i] != b[i] {
            return a[i] < b[i];
        }
        i += 1;
    }
    a.len() < b.len()
}

/// The index of the last `::` in the name.
const fn last_sep(name: &[u8]) -> Option<usize> {
    let mut i = name.len();
    while i >= 2 {
        i -= 1;
        if name[i - 1] == b':' && name[i] == b':' {
            return Some(i - 1);
        }
    }
    None
}

/// Whether the name is qualified by a path, and so is reduced.
const fn is_qualified(name: &[u8]) -> bool {
    last_sep(name).is_some()
}

/// The name without a leading `::`, which names are compared by, so that a
/// path from the crate root, such as `::mycrate::MyType` as the SDK writes
/// it, and the same path without it, such as `mycrate::MyType` as other tools
/// may write it, name the same type.
const fn unrooted(name: &[u8]) -> &[u8] {
    if name.len() >= 2 && name[0] == b':' && name[1] == b':' {
        name.split_at(2).1
    } else {
        name
    }
}

/// The last `::`-separated segment of a fully qualified type name.
pub const fn last_segment(name: &[u8]) -> &[u8] {
    match last_sep(name) {
        Some(i) => name.split_at(i + 2).1,
        None => name,
    }
}

/// Whether the name is the name of an SDK type or of an item generated
/// alongside the types.
const fn is_reserved(name: &[u8]) -> bool {
    let mut i = 0;
    while i < RESERVED_NAMES.len() {
        if eq(RESERVED_NAMES[i].as_bytes(), name) {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < GENERATED_NAMES.len() {
        if eq(GENERATED_NAMES[i].as_bytes(), name) {
            return true;
        }
        i += 1;
    }
    false
}

/// The base followed by the number, keeping as much of the base as leaves room
/// for the number within the limit, trimming bytes off its end when the base is
/// too long to hold the number too. A base that is valid UTF-8 is trimmed at a
/// character boundary, so that the name stays valid UTF-8. The limit always
/// exceeds the number's length, so a fitting name always exists.
const fn numbered_name(base: &[u8], n: u32) -> Name<'_> {
    let digits = Name { base: &[], n }.len();
    let mut keep = base.len();
    if keep > NAME_LIMIT - digits {
        keep = NAME_LIMIT - digits;
    }
    if core::str::from_utf8(base).is_ok() {
        while keep > 0 && keep < base.len() && base[keep] & 0xC0 == 0x80 {
            keep -= 1;
        }
    }
    Name {
        base: base.split_at(keep).0,
        n,
    }
}

/// Checks that the defined names can be reduced. References are checked for
/// an empty last segment as they are collected, and for being defined as they
/// are resolved.
///
/// The spec can come from an untrusted wasm, so every name is checked before
/// any is reduced, rather than an invalid name reducing to an empty one that
/// only fails when something later uses it.
const fn validate(ctx: &mut Ctx) {
    let mut i = 0;
    while i < ctx.count {
        if last_segment(ctx.defined[i]).is_empty() {
            ctx.fail(Error::InvalidName(ctx.defined[i]));
            return;
        }
        i += 1;
    }
    if let Some(name) = ctx.invalid_ref {
        ctx.fail(Error::InvalidName(name));
        return;
    }
    // Simple names can be defined more than once, by specs from before
    // qualified names, but a qualified name cannot, including one that is a
    // simple name once its leading `::` is removed, such as `::MyType` beside
    // `MyType`.
    let mut i = 0;
    while i < ctx.count {
        let d = ctx.defined[i];
        if is_qualified(d) {
            let mut j = 0;
            while j < ctx.count {
                let e = ctx.defined[j];
                let same = if is_qualified(e) {
                    j < i && eq(unrooted(e), unrooted(d))
                } else {
                    eq(e, unrooted(d))
                };
                if same {
                    ctx.fail(Error::DuplicateName(d));
                    return;
                }
                j += 1;
            }
        }
        i += 1;
    }
}

/// Resolves what each defined name reduces to.
///
/// The names of SDK types and generated items are taken before any type
/// claims a name. Simple names keep their names, unless reserved, so they
/// claim them next. Then the qualified names claim their last segments in the
/// order of their full names, rather than the order they are defined in, as
/// the compiler does not guarantee the order it writes spec entries in. The
/// first to claim a last segment keeps it, so a type only ever loses its own
/// name to one that sorts before it, never to a number handed to a type that
/// collided with something else. The names that collided are numbered in the
/// order of their full names.
const fn assign(ctx: &mut Ctx) {
    // Whether each defined name collided and so is numbered.
    let mut colliding = [false; MAX_NAMES];
    let mut i = 0;
    while i < ctx.count {
        let d = ctx.defined[i];
        colliding[i] = if is_qualified(d) {
            let base = last_segment(d);
            let mut lost = is_reserved(base) || is_kept_simple_name(ctx, base);
            let mut j = 0;
            while !lost && j < ctx.count {
                let e = ctx.defined[j];
                lost =
                    is_qualified(e) && less(unrooted(e), unrooted(d)) && eq(last_segment(e), base);
                j += 1;
            }
            lost
        } else {
            is_reserved(d)
        };
        ctx.assigned[i] = Name::new(last_segment(d));
        i += 1;
    }

    // The names numbered so far, which later numbers step over.
    let mut numbered = [Name::EMPTY; MAX_NAMES];
    let mut numbered_count = 0;
    let mut prev: Option<&[u8]> = None;
    loop {
        // The colliding name that sorts next. A simple name can be defined
        // more than once, and each definition is numbered once, the same.
        let mut next: Option<usize> = None;
        let mut i = 0;
        while i < ctx.count {
            let u = unrooted(ctx.defined[i]);
            let after_prev = match prev {
                Some(p) => less(p, u),
                None => true,
            };
            let before_next = match next {
                Some(n) => less(u, unrooted(ctx.defined[n])),
                None => true,
            };
            if colliding[i] && after_prev && before_next {
                next = Some(i);
            }
            i += 1;
        }
        let Some(m) = next else { break };
        let u = unrooted(ctx.defined[m]);
        let base = last_segment(ctx.defined[m]);
        let mut n = 2u32;
        let name = loop {
            let candidate = numbered_name(base, n);
            if !is_taken(ctx, &colliding, &numbered, numbered_count, &candidate) {
                break candidate;
            }
            n += 1;
        };
        numbered[numbered_count] = name;
        numbered_count += 1;
        let mut i = 0;
        while i < ctx.count {
            if colliding[i] && eq(unrooted(ctx.defined[i]), u) {
                ctx.assigned[i] = name;
            }
            i += 1;
        }
        prev = Some(u);
    }
}

/// Whether the spec defines the name as a simple name that is not reserved,
/// which keeps its name.
const fn is_kept_simple_name(ctx: &Ctx, name: &[u8]) -> bool {
    let mut i = 0;
    while i < ctx.count {
        let d = ctx.defined[i];
        if !is_qualified(d) && !is_reserved(d) && eq(d, name) {
            return true;
        }
        i += 1;
    }
    false
}

/// Whether a numbered name is already taken: by a reserved name, a simple
/// name, the last segment a qualified name kept, or an earlier number.
const fn is_taken(
    ctx: &Ctx,
    colliding: &[bool; MAX_NAMES],
    numbered: &[Name; MAX_NAMES],
    numbered_count: usize,
    name: &Name,
) -> bool {
    let mut i = 0;
    while i < RESERVED_NAMES.len() {
        if name.eq(&Name::new(RESERVED_NAMES[i].as_bytes())) {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < GENERATED_NAMES.len() {
        if name.eq(&Name::new(GENERATED_NAMES[i].as_bytes())) {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < ctx.count {
        let d = ctx.defined[i];
        let kept = if is_qualified(d) {
            !colliding[i] && name.eq(&Name::new(last_segment(d)))
        } else {
            !is_reserved(d) && name.eq(&Name::new(d))
        };
        if kept {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < numbered_count {
        if numbered[i].eq(name) {
            return true;
        }
        i += 1;
    }
    false
}

/// What a defined or referenced name reduces to.
const fn resolve<'a>(ctx: &mut Ctx<'a, '_>, name: &'a [u8]) -> Name<'a> {
    let mut i = 0;
    while i < ctx.count {
        if eq(unrooted(ctx.defined[i]), unrooted(name)) {
            return ctx.assigned[i];
        }
        i += 1;
    }
    if is_qualified(name) {
        ctx.fail(Error::UndefinedName(name));
    }
    Name::new(name)
}

// XDR walking. Each fn takes the offset of a value in `b` and returns the
// offset after it, copying the value to the output with its names rewritten.
// On invalid XDR the walk records the error and returns the end of `b`, and
// the walk stops at the next check.

const fn read_u32(ctx: &mut Ctx, b: &[u8], at: usize) -> u32 {
    if at > b.len() || b.len() - at < 4 {
        ctx.fail(Error::InvalidXdr);
        return 0;
    }
    u32::from_be_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

const fn padded(len: usize) -> usize {
    len.div_ceil(4) * 4
}

const fn copy(ctx: &mut Ctx, b: &[u8], at: usize, n: usize) -> usize {
    if at > b.len() || b.len() - at < n {
        ctx.fail(Error::InvalidXdr);
        return b.len();
    }
    if let Mode::Write = ctx.mode {
        if ctx.at > ctx.out.len() || ctx.out.len() - ctx.at < n {
            ctx.fail(Error::OutputLength);
            return b.len();
        }
        let mut i = 0;
        while i < n {
            ctx.out[ctx.at + i] = b[at + i];
            i += 1;
        }
    }
    ctx.at += n;
    at + n
}

/// The string at the offset, and the offset after it.
const fn string<'a>(ctx: &mut Ctx<'a, '_>, b: &'a [u8], at: usize) -> (&'a [u8], usize) {
    let len = read_u32(ctx, b, at) as usize;
    if !ctx.ok() || b.len() - (at + 4) < len {
        ctx.fail(Error::InvalidXdr);
        return (&[], b.len());
    }
    let s = b.split_at(at + 4).1.split_at(len).0;
    let next = at + 4 + padded(len);
    if next > b.len() {
        ctx.fail(Error::InvalidXdr);
        return (&[], b.len());
    }
    (s, next)
}

const fn copy_u32(ctx: &mut Ctx, b: &[u8], at: usize) -> usize {
    copy(ctx, b, at, 4)
}

const fn copy_string<'a>(ctx: &mut Ctx<'a, '_>, b: &'a [u8], at: usize) -> usize {
    let (s, next) = string(ctx, b, at);
    if !ctx.ok() {
        return next;
    }
    copy(ctx, b, at, 4 + padded(s.len()))
}

/// Copies the length of an array, returning it and the offset after it.
const fn copy_len(ctx: &mut Ctx, b: &[u8], at: usize) -> (u32, usize) {
    let n = read_u32(ctx, b, at);
    (n, copy_u32(ctx, b, at))
}

/// Writes a name as an XDR string.
const fn emit(ctx: &mut Ctx, name: &Name) {
    let len = name.len();
    copy(ctx, &(len as u32).to_be_bytes(), 0, 4);
    copy(ctx, name.base, 0, name.base.len());
    let (digits, count) = name.digits();
    copy(ctx, &digits, digits.len() - count, count);
    copy(ctx, &[0u8; 3], 0, padded(len) - len);
}

/// A name that an entry defines.
const fn def_name<'a>(ctx: &mut Ctx<'a, '_>, b: &'a [u8], at: usize) -> usize {
    let (name, next) = string(ctx, b, at);
    if !ctx.ok() {
        return next;
    }
    if let Mode::Collect = ctx.mode {
        if ctx.count == MAX_NAMES {
            ctx.fail(Error::TooManyNames);
            return b.len();
        }
        ctx.defined[ctx.count] = name;
        ctx.count += 1;
    } else {
        let name = resolve(ctx, name);
        emit(ctx, &name);
    }
    next
}

/// A name that a type refers to.
const fn ref_name<'a>(ctx: &mut Ctx<'a, '_>, b: &'a [u8], at: usize) -> usize {
    let (name, next) = string(ctx, b, at);
    if !ctx.ok() {
        return next;
    }
    if let Mode::Collect = ctx.mode {
        if ctx.invalid_ref.is_none() && last_segment(name).is_empty() {
            ctx.invalid_ref = Some(name);
        }
    } else {
        let name = resolve(ctx, name);
        emit(ctx, &name);
    }
    next
}

const fn walk_spec<'a>(ctx: &mut Ctx<'a, '_>, spec: &[&'a [u8]]) {
    let mut i = 0;
    while i < spec.len() && ctx.ok() {
        let b = spec[i];
        let mut at = 0;
        while at < b.len() && ctx.ok() {
            at = walk_entry(ctx, b, at);
        }
        i += 1;
    }
}

/// An `SCSpecEntry`.
const fn walk_entry<'a>(ctx: &mut Ctx<'a, '_>, b: &'a [u8], at: usize) -> usize {
    let kind = read_u32(ctx, b, at);
    let mut at = copy_u32(ctx, b, at);
    match kind {
        // SC_SPEC_ENTRY_FUNCTION_V0: doc, name, inputs (doc, name, type),
        // outputs.
        0 => {
            at = copy_string(ctx, b, at);
            at = copy_string(ctx, b, at);
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n && ctx.ok() {
                at = copy_string(ctx, b, at);
                at = copy_string(ctx, b, at);
                at = walk_type(ctx, b, at);
                i += 1;
            }
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n && ctx.ok() {
                at = walk_type(ctx, b, at);
                i += 1;
            }
        }
        // SC_SPEC_ENTRY_UDT_STRUCT_V0: doc, lib, name, fields (doc, name,
        // type).
        1 => {
            at = copy_string(ctx, b, at);
            at = copy_string(ctx, b, at);
            at = def_name(ctx, b, at);
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n && ctx.ok() {
                at = copy_string(ctx, b, at);
                at = copy_string(ctx, b, at);
                at = walk_type(ctx, b, at);
                i += 1;
            }
        }
        // SC_SPEC_ENTRY_UDT_UNION_V0: doc, lib, name, cases.
        2 => {
            at = copy_string(ctx, b, at);
            at = copy_string(ctx, b, at);
            at = def_name(ctx, b, at);
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n && ctx.ok() {
                let case = read_u32(ctx, b, at);
                at = copy_u32(ctx, b, at);
                // Void and tuple cases: doc, name.
                at = copy_string(ctx, b, at);
                at = copy_string(ctx, b, at);
                match case {
                    0 => {}
                    // Tuple cases: types.
                    1 => {
                        let (n, next) = copy_len(ctx, b, at);
                        at = next;
                        let mut j = 0;
                        while j < n && ctx.ok() {
                            at = walk_type(ctx, b, at);
                            j += 1;
                        }
                    }
                    _ => ctx.fail(Error::InvalidXdr),
                }
                i += 1;
            }
        }
        // SC_SPEC_ENTRY_UDT_ENUM_V0 and SC_SPEC_ENTRY_UDT_ERROR_ENUM_V0: doc,
        // lib, name, cases (doc, name, value).
        3 | 4 => {
            at = copy_string(ctx, b, at);
            at = copy_string(ctx, b, at);
            at = def_name(ctx, b, at);
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n && ctx.ok() {
                at = copy_string(ctx, b, at);
                at = copy_string(ctx, b, at);
                at = copy_u32(ctx, b, at);
                i += 1;
            }
        }
        // SC_SPEC_ENTRY_EVENT_V0: doc, lib, name, prefix topics, params (doc,
        // name, type, location), data format.
        5 => {
            at = copy_string(ctx, b, at);
            at = copy_string(ctx, b, at);
            at = def_name(ctx, b, at);
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n && ctx.ok() {
                at = copy_string(ctx, b, at);
                i += 1;
            }
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n && ctx.ok() {
                at = copy_string(ctx, b, at);
                at = copy_string(ctx, b, at);
                at = walk_type(ctx, b, at);
                at = copy_u32(ctx, b, at);
                i += 1;
            }
            at = copy_u32(ctx, b, at);
        }
        _ => ctx.fail(Error::InvalidXdr),
    }
    if ctx.ok() {
        at
    } else {
        b.len()
    }
}

/// An `SCSpecTypeDef`.
const fn walk_type<'a>(ctx: &mut Ctx<'a, '_>, b: &'a [u8], at: usize) -> usize {
    if ctx.depth == DEPTH_LIMIT {
        ctx.fail(Error::InvalidXdr);
        return b.len();
    }
    ctx.depth += 1;
    let ty = read_u32(ctx, b, at);
    let mut at = copy_u32(ctx, b, at);
    match ty {
        // Types without a payload.
        0..=14 | 16 | 17 | 19 | 20 => {}
        // Option and vec: a type.
        1000 | 1002 => at = walk_type(ctx, b, at),
        // Result and map: two types.
        1001 | 1004 => {
            at = walk_type(ctx, b, at);
            at = walk_type(ctx, b, at);
        }
        // Tuple: types.
        1005 => {
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n && ctx.ok() {
                at = walk_type(ctx, b, at);
                i += 1;
            }
        }
        // BytesN: a length.
        1006 => at = copy_u32(ctx, b, at),
        // UDT: a name.
        2000 => at = ref_name(ctx, b, at),
        _ => ctx.fail(Error::InvalidXdr),
    }
    ctx.depth -= 1;
    at
}

#[cfg(all(test, feature = "std"))]
mod test {
    use super::{last_segment, numbered_name, NAME_LIMIT};
    use stellar_xdr::{Limited, Limits, ReadXdr, WriteXdr};
    use stellar_xdr::{
        ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdt, ScSpecUdtStructFieldV0, ScSpecUdtStructV0,
    };

    // The tests check the reduced entries and how each defined name resolved,
    // so they reduce through this adapter, which encodes the entries, reduces
    // the encoding, decodes the result, and pairs each entry's name before
    // and after.

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum Error {
        DuplicateName(Vec<u8>),
        InvalidName(Vec<u8>),
        UndefinedName(Vec<u8>),
        Other(String),
    }

    impl From<super::Error<'_>> for Error {
        fn from(e: super::Error<'_>) -> Self {
            match e {
                super::Error::DuplicateName(n) => Error::DuplicateName(n.to_vec()),
                super::Error::InvalidName(n) => Error::InvalidName(n.to_vec()),
                super::Error::UndefinedName(n) => Error::UndefinedName(n.to_vec()),
                e => Error::Other(e.to_string()),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Rename {
        from: Vec<u8>,
        to: Vec<u8>,
    }

    impl Rename {
        fn renamed(&self) -> bool {
            self.from != self.to
        }

        fn collision(&self) -> bool {
            self.to != last_segment(&self.from)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Reduced(Vec<ScSpecEntry>, Vec<Rename>);

    impl Reduced {
        fn entries(&self) -> impl Iterator<Item = &ScSpecEntry> + '_ {
            self.0.iter()
        }

        fn renames(&self) -> impl Iterator<Item = &Rename> + '_ {
            self.1.iter()
        }
    }

    fn defined_name(entry: &ScSpecEntry) -> Option<Vec<u8>> {
        match entry {
            ScSpecEntry::FunctionV0(_) => None,
            ScSpecEntry::UdtStructV0(e) => Some(e.name.to_vec()),
            ScSpecEntry::UdtUnionV0(e) => Some(e.name.to_vec()),
            ScSpecEntry::UdtEnumV0(e) => Some(e.name.to_vec()),
            ScSpecEntry::UdtErrorEnumV0(e) => Some(e.name.to_vec()),
            ScSpecEntry::EventV0(e) => Some(e.name.to_vec()),
        }
    }

    fn encode(spec: &[ScSpecEntry]) -> Vec<Vec<u8>> {
        spec.iter()
            .map(|e| e.to_xdr(Limits::none()).unwrap())
            .collect()
    }

    fn reduce_xdr(xdr: &[Vec<u8>]) -> Result<Vec<u8>, Error> {
        let slices: Vec<&[u8]> = xdr.iter().map(Vec::as_slice).collect();
        let mut out = vec![0u8; super::reduce(&slices, None)?];
        let len = super::reduce(&slices, Some(&mut out))?;
        assert_eq!(len, out.len());
        Ok(out)
    }

    fn reduce(spec: &[ScSpecEntry]) -> Result<Reduced, Error> {
        let out = reduce_xdr(&encode(spec))?;
        let entries = ScSpecEntry::read_xdr_iter(&mut Limited::new(
            std::io::Cursor::new(out),
            Limits::none(),
        ))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
        assert_eq!(entries.len(), spec.len());
        let renames = spec
            .iter()
            .zip(&entries)
            .filter_map(|(before, after)| {
                Some(Rename {
                    from: defined_name(before)?,
                    to: defined_name(after).unwrap(),
                })
            })
            .collect();
        Ok(Reduced(entries, renames))
    }

    fn numbered(base: &[u8], n: u32) -> Vec<u8> {
        let name = numbered_name(base, n);
        (0..name.len()).map(|i| name.byte(i)).collect()
    }

    fn struct_entry_bytes(name: &[u8], field_type_names: &[&[u8]]) -> ScSpecEntry {
        ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: name.try_into().unwrap(),
            fields: field_type_names
                .iter()
                .map(|n| ScSpecUdtStructFieldV0 {
                    doc: "".try_into().unwrap(),
                    name: "f".try_into().unwrap(),
                    type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                        name: (*n).try_into().unwrap(),
                    }),
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        })
    }

    fn struct_entry(name: &str, field_type_names: &[&str]) -> ScSpecEntry {
        struct_entry_bytes(
            name.as_bytes(),
            &field_type_names
                .iter()
                .map(|n| n.as_bytes())
                .collect::<Vec<_>>(),
        )
    }

    fn event_entry(name: &str, param_type_names: &[&str]) -> ScSpecEntry {
        use stellar_xdr::{
            ScSpecEventDataFormat, ScSpecEventParamLocationV0, ScSpecEventParamV0, ScSpecEventV0,
        };
        ScSpecEntry::EventV0(ScSpecEventV0 {
            doc: "".try_into().unwrap(),
            lib: "".try_into().unwrap(),
            name: name.try_into().unwrap(),
            prefix_topics: [].try_into().unwrap(),
            params: param_type_names
                .iter()
                .map(|n| ScSpecEventParamV0 {
                    doc: "".try_into().unwrap(),
                    name: "p".try_into().unwrap(),
                    type_: ScSpecTypeDef::Udt(ScSpecTypeUdt {
                        name: (*n).try_into().unwrap(),
                    }),
                    location: ScSpecEventParamLocationV0::Data,
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            data_format: ScSpecEventDataFormat::SingleValue,
        })
    }

    fn names<'a>(spec: impl IntoIterator<Item = &'a ScSpecEntry>) -> Vec<(Vec<u8>, Vec<Vec<u8>>)> {
        spec.into_iter()
            .map(|e| match e {
                ScSpecEntry::UdtStructV0(s) => (
                    s.name.to_vec(),
                    s.fields
                        .iter()
                        .map(|f| match &f.type_ {
                            ScSpecTypeDef::Udt(u) => u.name.to_vec(),
                            _ => unreachable!(),
                        })
                        .collect(),
                ),
                _ => unreachable!(),
            })
            .collect()
    }

    #[test]
    fn reduces_a_qualified_name_to_its_last_segment() {
        let spec = [struct_entry("mycrate::mymod::MyType", &[])];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(names(reduced.entries()), [(b"MyType".to_vec(), vec![])]);
        assert_eq!(
            reduced.renames().cloned().collect::<Vec<_>>(),
            [Rename {
                from: b"mycrate::mymod::MyType".to_vec(),
                to: b"MyType".to_vec(),
            }]
        );
        let rename = reduced.renames().next().unwrap();
        assert!(rename.renamed());
        assert!(!rename.collision());
    }

    #[test]
    fn reduces_an_event_name_and_a_type_referenced_in_its_params() {
        let spec = [
            event_entry("mycrate::mymod::MyEvent", &["mycrate::mymod::MyType"]),
            struct_entry("mycrate::mymod::MyType", &[]),
        ];
        let reduced = reduce(&spec).unwrap();
        let entries: Vec<_> = reduced.entries().collect();
        let ScSpecEntry::EventV0(ev) = entries[0] else {
            panic!("first entry should be the event, got {:?}", entries[0]);
        };
        assert_eq!(ev.name.to_vec(), b"MyEvent".to_vec());
        let ScSpecTypeDef::Udt(u) = &ev.params[0].type_ else {
            panic!("param should reference a udt, got {:?}", ev.params[0].type_);
        };
        assert_eq!(u.name.to_vec(), b"MyType".to_vec());
    }

    #[test]
    fn an_event_and_a_type_sharing_a_simple_name_are_numbered() {
        // An event defines a name, so an event and a type that reduce to the
        // same simple name collide: the one whose full name sorts first keeps
        // it, the other is numbered.
        let spec = [
            struct_entry("a::Shared", &[]),
            event_entry("b::Shared", &[]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            reduced
                .renames()
                .map(|r| r.to.as_slice())
                .collect::<Vec<_>>(),
            [b"Shared".as_slice(), b"Shared2"],
        );
    }

    #[test]
    fn numbers_a_simple_name_already_claimed_and_matches_up_references() {
        let spec = [
            struct_entry("mycrate::mymod::MyType", &["mycrate::myothermod::MyType"]),
            struct_entry("mycrate::myothermod::MyType", &["mycrate::mymod::MyType"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"MyType".to_vec(), vec![b"MyType2".to_vec()]),
                (b"MyType2".to_vec(), vec![b"MyType".to_vec()]),
            ]
        );
        let renames: Vec<_> = reduced.renames().collect();
        assert!(!renames[0].collision());
        assert!(renames[1].collision());
    }

    #[test]
    fn colliding_names_are_numbered_in_the_order_of_their_full_names() {
        // The compiler does not guarantee the order it writes spec entries in,
        // so the same types defined in any order are numbered the same: by
        // the order of their full names, not the order they are defined in.
        let spec = [
            struct_entry("c::MyType", &[]),
            struct_entry("a::MyType", &[]),
            struct_entry("b::MyType", &[]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            reduced
                .renames()
                .map(|r| (r.from.as_slice(), r.to.as_slice()))
                .collect::<Vec<_>>(),
            [
                (b"c::MyType".as_slice(), b"MyType3".as_slice()),
                (b"a::MyType", b"MyType"),
                (b"b::MyType", b"MyType2"),
            ],
        );

        let spec = [
            struct_entry("b::MyType", &[]),
            struct_entry("c::MyType", &[]),
            struct_entry("a::MyType", &[]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            reduced
                .renames()
                .map(|r| (r.from.as_slice(), r.to.as_slice()))
                .collect::<Vec<_>>(),
            [
                (b"b::MyType".as_slice(), b"MyType2".as_slice()),
                (b"c::MyType", b"MyType3"),
                (b"a::MyType", b"MyType"),
            ],
        );
    }

    #[test]
    fn a_reference_without_a_leading_separator_follows_a_numbered_type() {
        // The SDK writes names from the crate root, with a leading `::`, but
        // other tools may not. A reference written without it still follows
        // the type it names to that type's numbered name.
        let spec = [
            struct_entry("::a::MyType", &[]),
            struct_entry("::b::MyType", &[]),
            struct_entry("::c::Holder", &["b::MyType", "::a::MyType"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"MyType".to_vec(), vec![]),
                (b"MyType2".to_vec(), vec![]),
                (
                    b"Holder".to_vec(),
                    vec![b"MyType2".to_vec(), b"MyType".to_vec()]
                ),
            ]
        );
    }

    #[test]
    fn a_name_defined_with_and_without_a_leading_separator_is_an_error() {
        let spec = [
            struct_entry("::shared::Meta", &[]),
            struct_entry("shared::Meta", &[]),
        ];
        assert_eq!(
            reduce(&spec),
            Err(Error::DuplicateName(b"shared::Meta".to_vec()))
        );
    }

    #[test]
    fn a_simple_name_and_the_same_name_with_a_leading_separator_is_an_error() {
        let spec = [struct_entry("MyType", &[]), struct_entry("::MyType", &[])];
        assert_eq!(
            reduce(&spec),
            Err(Error::DuplicateName(b"::MyType".to_vec()))
        );
    }

    #[test]
    fn a_reference_with_a_leading_separator_follows_a_simple_name() {
        let spec = [
            struct_entry("MyType", &[]),
            struct_entry("::a::Holder", &["::MyType"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"MyType".to_vec(), vec![]),
                (b"Holder".to_vec(), vec![b"MyType".to_vec()]),
            ]
        );
    }

    #[test]
    fn colliding_names_are_ordered_without_a_leading_separator() {
        // `::b::MyType` sorts before `a::MyType` as bytes, as `:` sorts before
        // letters, but the names are ordered without the leading `::`.
        let spec = [
            struct_entry("::b::MyType", &[]),
            struct_entry("a::MyType", &[]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            reduced
                .renames()
                .map(|r| (r.from.as_slice(), r.to.as_slice()))
                .collect::<Vec<_>>(),
            [
                (b"::b::MyType".as_slice(), b"MyType2".as_slice()),
                (b"a::MyType", b"MyType"),
            ],
        );
    }

    #[test]
    fn steps_over_a_name_claimed_by_a_type_of_that_name() {
        // `MyType2` is a type in its own right, so the numbering steps over it
        // rather than colliding with it in turn.
        let spec = [
            struct_entry("a::MyType", &[]),
            struct_entry("b::MyType2", &[]),
            struct_entry("c::MyType", &[]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            reduced
                .renames()
                .map(|r| r.to.as_slice())
                .collect::<Vec<_>>(),
            [b"MyType".as_slice(), b"MyType2", b"MyType3"],
        );
    }

    #[test]
    fn a_spec_with_simple_names_comes_back_unchanged() {
        let spec = [
            struct_entry("MyType", &["MyOther"]),
            struct_entry("MyOther", &[]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(reduced.entries().cloned().collect::<Vec<_>>(), spec);
        assert!(reduced.renames().all(|r| !r.renamed()));
    }

    #[test]
    fn a_simple_name_keeps_its_claim_over_a_later_qualified_one() {
        let spec = [struct_entry("MyType", &[]), struct_entry("a::MyType", &[])];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            reduced
                .renames()
                .map(|r| r.to.as_slice())
                .collect::<Vec<_>>(),
            [b"MyType".as_slice(), b"MyType2"],
        );
    }

    #[test]
    fn a_qualified_reference_to_an_undefined_type_is_an_error() {
        // Two undefined types sharing a last segment, or an undefined type
        // sharing one with a defined type, could not be told apart once
        // reduced, so a qualified reference must be to a defined type.
        let spec = [
            struct_entry("::a::Config", &[]),
            struct_entry("::a::Holder", &["::a::Config", "::b::Config"]),
        ];
        assert_eq!(
            reduce(&spec),
            Err(Error::UndefinedName(b"::b::Config".to_vec()))
        );
    }

    #[test]
    fn a_simple_reference_to_an_undefined_type_is_unchanged() {
        let spec = [struct_entry("a::MyType", &["Other"])];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [(b"MyType".to_vec(), vec![b"Other".to_vec()])]
        );
        assert_eq!(reduced.renames().count(), 1);
    }

    #[test]
    fn a_name_that_is_not_utf8_is_preserved_byte_for_byte() {
        // Spec names are byte strings with no UTF-8 guarantee. A lossy decode
        // would swap each invalid byte for a multi-byte replacement character,
        // changing the name and potentially overflowing the name limit.
        let name = b"mycrate::\xff\xfeType";
        let spec = [struct_entry_bytes(name, &[name])];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [(b"\xff\xfeType".to_vec(), vec![b"\xff\xfeType".to_vec()])]
        );
    }

    #[test]
    fn a_name_defined_twice_is_an_error() {
        let spec = [
            struct_entry("::shared::Meta", &[]),
            struct_entry("::shared::Meta", &[]),
        ];
        assert_eq!(
            reduce(&spec),
            Err(Error::DuplicateName(b"::shared::Meta".to_vec()))
        );
    }

    #[test]
    fn a_defined_name_with_an_empty_last_segment_is_an_error() {
        let spec = [struct_entry("::mycrate::", &[])];
        assert_eq!(
            reduce(&spec),
            Err(Error::InvalidName(b"::mycrate::".to_vec()))
        );
    }

    #[test]
    fn a_referred_to_name_with_an_empty_last_segment_is_an_error() {
        let spec = [struct_entry("::mycrate::Holder", &["::mycrate::"])];
        assert_eq!(
            reduce(&spec),
            Err(Error::InvalidName(b"::mycrate::".to_vec()))
        );
    }

    #[test]
    fn an_empty_name_is_an_error() {
        let spec = [struct_entry("", &[])];
        assert_eq!(reduce(&spec), Err(Error::InvalidName(b"".to_vec())));
    }

    #[test]
    fn a_simple_name_defined_twice_is_unchanged() {
        let spec = [
            struct_entry("Meta", &[]),
            struct_entry("Meta", &[]),
            struct_entry("Holder", &["Meta"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(reduced.entries().cloned().collect::<Vec<_>>(), spec);
        assert!(reduced.renames().all(|r| !r.renamed()));
    }

    #[test]
    fn a_qualified_name_does_not_take_a_simple_name_defined_after_it() {
        let spec = [
            struct_entry("::a::Meta", &[]),
            struct_entry("Meta", &[]),
            struct_entry("Holder", &["Meta", "::a::Meta"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"Meta2".to_vec(), vec![]),
                (b"Meta".to_vec(), vec![]),
                (
                    b"Holder".to_vec(),
                    vec![b"Meta".to_vec(), b"Meta2".to_vec()]
                ),
            ]
        );
    }

    #[test]
    fn a_qualified_name_that_is_reserved_is_numbered() {
        let spec = [
            struct_entry("::other::Address", &[]),
            struct_entry("Holder", &["::other::Address"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"Address2".to_vec(), vec![]),
                (b"Holder".to_vec(), vec![b"Address2".to_vec()]),
            ]
        );
        assert!(reduced.renames().next().unwrap().collision());
    }

    #[test]
    fn a_qualified_name_that_is_generated_is_numbered() {
        let spec = [
            struct_entry("::other::Contract", &[]),
            struct_entry("::other::Client", &[]),
            struct_entry("::other::Args", &[]),
            struct_entry("::other::WASM", &[]),
            struct_entry(
                "Holder",
                &[
                    "::other::Contract",
                    "::other::Client",
                    "::other::Args",
                    "::other::WASM",
                ],
            ),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"Contract2".to_vec(), vec![]),
                (b"Client2".to_vec(), vec![]),
                (b"Args2".to_vec(), vec![]),
                (b"WASM2".to_vec(), vec![]),
                (
                    b"Holder".to_vec(),
                    vec![
                        b"Contract2".to_vec(),
                        b"Client2".to_vec(),
                        b"Args2".to_vec(),
                        b"WASM2".to_vec(),
                    ]
                ),
            ]
        );
    }

    #[test]
    fn a_simple_name_that_is_generated_is_numbered() {
        let spec = [
            struct_entry("Client", &[]),
            struct_entry("Holder", &["Client"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"Client2".to_vec(), vec![]),
                (b"Holder".to_vec(), vec![b"Client2".to_vec()]),
            ]
        );
    }

    #[test]
    fn a_simple_event_name_that_is_reserved_is_numbered() {
        let spec = [event_entry("Symbol", &[])];
        let reduced = reduce(&spec).unwrap();
        let entries: Vec<_> = reduced.entries().collect();
        let ScSpecEntry::EventV0(ev) = entries[0] else {
            panic!("entry should be the event, got {:?}", entries[0]);
        };
        assert_eq!(ev.name.to_vec(), b"Symbol2".to_vec());
    }

    #[test]
    fn a_simple_name_that_is_reserved_and_defined_twice_is_numbered_once() {
        let spec = [
            struct_entry("Address", &[]),
            struct_entry("Address", &[]),
            struct_entry("Holder", &["Address"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"Address2".to_vec(), vec![]),
                (b"Address2".to_vec(), vec![]),
                (b"Holder".to_vec(), vec![b"Address2".to_vec()]),
            ]
        );
    }

    #[test]
    fn reserved_names_are_numbered_in_the_order_of_their_full_names() {
        let spec = [
            struct_entry("::b::Address", &[]),
            struct_entry("::a::Address", &[]),
            struct_entry("Holder", &["::a::Address", "::b::Address"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"Address3".to_vec(), vec![]),
                (b"Address2".to_vec(), vec![]),
                (
                    b"Holder".to_vec(),
                    vec![b"Address2".to_vec(), b"Address3".to_vec()]
                ),
            ]
        );
    }

    #[test]
    fn a_reserved_name_is_numbered_past_a_defined_name() {
        let spec = [
            struct_entry("::a::Address", &[]),
            struct_entry("Address2", &[]),
            struct_entry("Holder", &["::a::Address", "Address2"]),
        ];
        let reduced = reduce(&spec).unwrap();
        assert_eq!(
            names(reduced.entries()),
            [
                (b"Address3".to_vec(), vec![]),
                (b"Address2".to_vec(), vec![]),
                (
                    b"Holder".to_vec(),
                    vec![b"Address3".to_vec(), b"Address2".to_vec()]
                ),
            ]
        );
    }

    #[test]
    fn a_numbered_name_that_would_overflow_is_trimmed_to_fit() {
        // A 1024-byte base cannot fit a number after it, so the base is
        // trimmed to make room: 1023 bytes of the base followed by "2",
        // exactly at the limit.
        let base = "x".repeat(1024).into_bytes();
        let mut expected = "x".repeat(1023).into_bytes();
        expected.push(b'2');
        assert_eq!(numbered(&base, 2), expected);
    }

    #[test]
    fn a_numbered_name_is_trimmed_at_a_character_boundary() {
        // A 1024-byte base ending in the 2-byte "é" cannot fit a number after
        // it. Trimming to 1023 bytes would split the "é", so the whole "é" is
        // trimmed instead: 1022 bytes of the base followed by "2".
        let base = format!("{}é", "x".repeat(1022)).into_bytes();
        assert_eq!(base.len(), 1024);
        let mut expected = "x".repeat(1022).into_bytes();
        expected.push(b'2');
        assert_eq!(numbered(&base, 2), expected);
    }

    #[test]
    fn the_name_limit_matches_the_xdr() {
        assert_eq!(NAME_LIMIT, stellar_xdr::SC_SPEC_TYPE_NAME_LIMIT as usize);
    }

    #[test]
    fn a_spec_split_across_slices_reduces_as_one() {
        let spec = [
            struct_entry("a::MyType", &["b::MyType"]),
            struct_entry("b::MyType", &[]),
            event_entry("c::MyEvent", &["a::MyType"]),
        ];
        let whole = encode(&spec).concat();
        assert_eq!(reduce_xdr(&encode(&spec)), reduce_xdr(&[whole]));
    }

    #[test]
    fn an_output_of_the_wrong_length_is_an_error() {
        let xdr = encode(&[struct_entry("a::MyType", &[])]);
        let slices: Vec<&[u8]> = xdr.iter().map(Vec::as_slice).collect();
        let len = super::reduce(&slices, None).unwrap();
        for wrong in [0, len - 1, len + 1] {
            let mut out = vec![0u8; wrong];
            assert_eq!(
                super::reduce(&slices, Some(&mut out)),
                Err(super::Error::OutputLength)
            );
        }
    }

    #[test]
    fn invalid_xdr_is_an_error() {
        let xdr = encode(&[
            struct_entry("a::MyType", &["a::MyType"]),
            event_entry("a::MyEvent", &["a::MyType"]),
        ])
        .concat();
        // Every truncation of a valid spec, other than at an entry boundary.
        for len in 1..xdr.len() {
            let truncated = &xdr[..len];
            let result = super::reduce(&[truncated], None);
            let boundary = ScSpecEntry::read_xdr_iter(&mut Limited::new(
                std::io::Cursor::new(truncated),
                Limits::none(),
            ))
            .collect::<Result<Vec<_>, _>>()
            .is_ok();
            if !boundary {
                assert_eq!(result, Err(super::Error::InvalidXdr), "len {len}");
            }
        }
        // An unknown entry kind, type, and union case.
        assert_eq!(
            super::reduce(&[&[0, 0, 0, 6]], None),
            Err(super::Error::InvalidXdr)
        );
        // A length far beyond the input.
        assert_eq!(
            super::reduce(&[&[0, 0, 0, 1, 0xff, 0xff, 0xff, 0xff]], None),
            Err(super::Error::InvalidXdr)
        );
    }

    #[test]
    fn types_nested_too_deeply_are_an_error() {
        // A function whose output is an option of an option of ... of a u32.
        let mut xdr = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, b'f', 0, 0, 0];
        xdr.extend([0, 0, 0, 0, 0, 0, 0, 1]);
        for _ in 0..100_000 {
            xdr.extend(1000u32.to_be_bytes());
        }
        xdr.extend(4u32.to_be_bytes());
        assert_eq!(super::reduce(&[&xdr], None), Err(super::Error::InvalidXdr));
    }

    #[test]
    fn a_spec_can_be_reduced_in_a_const() {
        const SPEC: &[&[u8]] = &[&[
            0, 0, 0, 1, // struct
            0, 0, 0, 0, // doc
            0, 0, 0, 0, // lib
            0, 0, 0, 6, b'a', b':', b':', b'M', b'y', b'T', 0, 0, // name
            0, 0, 0, 0, // fields
        ]];
        const LEN: usize = match super::reduce(SPEC, None) {
            Ok(len) => len,
            Err(_) => panic!(),
        };
        const REDUCED: [u8; LEN] = {
            let mut out = [0u8; LEN];
            if super::reduce(SPEC, Some(&mut out)).is_err() {
                panic!();
            }
            out
        };
        assert_eq!(
            REDUCED,
            [0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, b'M', b'y', b'T', 0, 0, 0, 0, 0]
        );
    }
}
