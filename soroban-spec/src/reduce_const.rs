//! Reduce the user-defined type and event names in a spec at compile time.
//!
//! The same reduction as [`reduce`](crate::reduce::reduce), over the XDR
//! encoding of the spec, in const fns that need neither std nor an allocator.
//! For crates that publish a spec assembled from the spec entries the SDK's
//! macros emit, such as the token and Stellar Asset contract specs, whose
//! specs never pass through `stellar contract build`, so that they can publish
//! the reduced spec as a const.
//!
//! ```ignore
//! const SPEC: &[&[u8]] = &[&MyType::spec_xdr(), &MyEvent::spec_xdr()];
//! const LEN: usize = soroban_spec::reduce_const::len(SPEC);
//! const REDUCED: [u8; LEN] = soroban_spec::reduce_const::reduce(SPEC);
//! ```
//!
//! A spec that [`reduce`](crate::reduce::reduce) would return an error for
//! panics instead, which in a const fails the build.

use crate::names::{GENERATED_NAMES, RESERVED_NAMES};

/// The most bytes a spec type or event name can hold
/// (`SC_SPEC_TYPE_NAME_LIMIT`), which bounds the names the numbering may
/// produce.
const NAME_LIMIT: usize = 1024;

/// The most type and event names a spec can define for it to be reduced here.
const MAX_NAMES: usize = 256;

/// Returns the length of the spec once reduced.
///
/// The spec is the XDR encoding of its entries, split across any number of
/// slices, each holding zero or more whole entries.
///
/// # Panics
///
/// If the spec cannot be reduced, for the reasons
/// [`reduce`](crate::reduce::reduce) returns an error, or is not a valid
/// encoding of spec entries.
pub const fn len(spec: &[&[u8]]) -> usize {
    let mut out = [0u8; 0];
    let mut ctx = Ctx::new(&mut out, Mode::Measure);
    run(&mut ctx, spec);
    ctx.at
}

/// Returns the spec with its user-defined type and event names reduced to
/// simple names, and every reference to a type rewritten to follow it, as
/// [`reduce`](crate::reduce::reduce) reduces them. `N` must be [`len`] of
/// `spec`.
///
/// The spec is the XDR encoding of its entries, split across any number of
/// slices, each holding zero or more whole entries. The result is the XDR
/// encoding of the reduced entries, concatenated.
///
/// # Panics
///
/// If the spec cannot be reduced, for the reasons
/// [`reduce`](crate::reduce::reduce) returns an error, if it is not a valid
/// encoding of spec entries, or if `N` is not its reduced length.
pub const fn reduce<const N: usize>(spec: &[&[u8]]) -> [u8; N] {
    let mut out = [0u8; N];
    reduce_into(spec, &mut out);
    out
}

/// Writes the reduced spec into `out`, which must be exactly its length.
const fn reduce_into(spec: &[&[u8]], out: &mut [u8]) {
    let mut ctx = Ctx::new(out, Mode::Write);
    run(&mut ctx, spec);
    if ctx.at != ctx.out.len() {
        panic!("reduced spec length does not match the output length");
    }
}

/// Collects the names the spec defines, resolves what each reduces to, and
/// then measures or writes the reduced spec.
const fn run<'a>(ctx: &mut Ctx<'a, '_>, spec: &[&'a [u8]]) {
    let mode = ctx.mode;
    ctx.mode = Mode::Collect;
    walk_spec(ctx, spec);
    validate(ctx);
    assign(ctx);
    ctx.mode = mode;
    ctx.at = 0;
    walk_spec(ctx, spec);
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
        if self.n == 0 {
            return (digits, 0);
        }
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

    const fn eq_name(&self, other: &Name) -> bool {
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

    const fn eq(&self, name: &[u8]) -> bool {
        let (digits, count) = self.digits();
        if self.len() != name.len() {
            return false;
        }
        let (base, number) = name.split_at(self.base.len());
        eq(self.base, base) && eq(digits.split_at(digits.len() - count).1, number)
    }
}

struct Ctx<'a, 'o> {
    mode: Mode,
    /// The names the spec defines, in definition order.
    defined: [&'a [u8]; MAX_NAMES],
    count: usize,
    /// What each defined name reduces to.
    assigned: [Name<'a>; MAX_NAMES],
    out: &'o mut [u8],
    at: usize,
}

impl<'a, 'o> Ctx<'a, 'o> {
    const fn new(out: &'o mut [u8], mode: Mode) -> Self {
        Ctx {
            mode,
            defined: [&[]; MAX_NAMES],
            count: 0,
            assigned: [Name::EMPTY; MAX_NAMES],
            out,
            at: 0,
        }
    }
}

// Name helpers, mirroring those of `reduce`.

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

/// The name without a leading `::`.
const fn unrooted(name: &[u8]) -> &[u8] {
    if name.len() >= 2 && name[0] == b':' && name[1] == b':' {
        name.split_at(2).1
    } else {
        name
    }
}

/// The last `::`-separated segment of the name.
const fn last_segment(name: &[u8]) -> &[u8] {
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

/// The base followed by the number, trimming the base from its end to fit
/// the number within the name limit, at a character boundary when the base is
/// valid UTF-8, as `reduce` numbers names.
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

/// Checks that the defined names can be reduced, as `reduce` does. References
/// are checked as they are resolved.
const fn validate(ctx: &Ctx) {
    let mut i = 0;
    while i < ctx.count {
        let d = ctx.defined[i];
        if last_segment(d).is_empty() {
            panic!("contract spec contains a name whose last segment is empty");
        }
        if is_qualified(d) {
            let mut j = 0;
            while j < ctx.count {
                let e = ctx.defined[j];
                let same = if is_qualified(e) {
                    j != i && eq(unrooted(e), unrooted(d))
                } else {
                    eq(e, unrooted(d))
                };
                if same {
                    panic!("contract spec defines a name more than once");
                }
                j += 1;
            }
        }
        i += 1;
    }
}

/// Resolves what each defined name reduces to, as `reduce` does: reserved
/// names are taken first, then simple names keep their names, then qualified
/// names claim their last segments in the order of their full names, and the
/// names that collided are numbered, in the order of their full names.
const fn assign(ctx: &mut Ctx<'_, '_>) {
    // Whether each defined name collided and so is numbered.
    let mut colliding = [false; MAX_NAMES];
    let mut i = 0;
    while i < ctx.count {
        let d = ctx.defined[i];
        colliding[i] = if is_qualified(d) {
            let base = last_segment(d);
            let mut lost = is_reserved(base) || is_simple_name(ctx, base);
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
        // The colliding name that sorts next.
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
        // A simple name can be defined more than once, and each definition is
        // numbered the same.
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
const fn is_simple_name(ctx: &Ctx, name: &[u8]) -> bool {
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
        if name.eq(RESERVED_NAMES[i].as_bytes()) {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < GENERATED_NAMES.len() {
        if name.eq(GENERATED_NAMES[i].as_bytes()) {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < ctx.count {
        let d = ctx.defined[i];
        let kept = if is_qualified(d) {
            !colliding[i] && name.eq(last_segment(d))
        } else {
            !is_reserved(d) && name.eq(d)
        };
        if kept {
            return true;
        }
        i += 1;
    }
    let mut i = 0;
    while i < numbered_count {
        if numbered[i].eq_name(name) {
            return true;
        }
        i += 1;
    }
    false
}

/// What a defined or referenced name reduces to.
const fn resolve<'a>(ctx: &Ctx<'a, '_>, name: &'a [u8]) -> Name<'a> {
    if last_segment(name).is_empty() {
        panic!("contract spec contains a name whose last segment is empty");
    }
    let mut i = 0;
    while i < ctx.count {
        if eq(unrooted(ctx.defined[i]), unrooted(name)) {
            return ctx.assigned[i];
        }
        i += 1;
    }
    if is_qualified(name) {
        panic!("contract spec refers to a name that it does not define");
    }
    Name::new(name)
}

// XDR walking. Each fn takes the offset of the value in `b` and returns the
// offset after it, copying the value to the output, with names rewritten.

const fn read_u32(b: &[u8], at: usize) -> u32 {
    u32::from_be_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

const fn padded(len: usize) -> usize {
    len.div_ceil(4) * 4
}

const fn copy(ctx: &mut Ctx, b: &[u8], at: usize, n: usize) -> usize {
    if let Mode::Write = ctx.mode {
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
const fn string(b: &[u8], at: usize) -> (&[u8], usize) {
    let len = read_u32(b, at) as usize;
    let s = b.split_at(at + 4).1.split_at(len).0;
    (s, at + 4 + padded(len))
}

const fn copy_u32(ctx: &mut Ctx, b: &[u8], at: usize) -> usize {
    copy(ctx, b, at, 4)
}

const fn copy_string(ctx: &mut Ctx, b: &[u8], at: usize) -> usize {
    let len = read_u32(b, at) as usize;
    copy(ctx, b, at, 4 + padded(len))
}

/// Copies the length of an array, returning it and the offset after it.
const fn copy_len(ctx: &mut Ctx, b: &[u8], at: usize) -> (u32, usize) {
    (read_u32(b, at), copy_u32(ctx, b, at))
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
    let (name, next) = string(b, at);
    if let Mode::Collect = ctx.mode {
        if ctx.count == MAX_NAMES {
            panic!("contract spec defines more names than can be reduced at compile time");
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
    let (name, next) = string(b, at);
    // References are resolved once every definition is collected.
    if !matches!(ctx.mode, Mode::Collect) {
        let name = resolve(ctx, name);
        emit(ctx, &name);
    }
    next
}

const fn walk_spec<'a>(ctx: &mut Ctx<'a, '_>, spec: &[&'a [u8]]) {
    let mut i = 0;
    while i < spec.len() {
        let b = spec[i];
        let mut at = 0;
        while at < b.len() {
            at = walk_entry(ctx, b, at);
        }
        i += 1;
    }
}

/// An `SCSpecEntry`.
const fn walk_entry<'a>(ctx: &mut Ctx<'a, '_>, b: &'a [u8], at: usize) -> usize {
    let kind = read_u32(b, at);
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
            while i < n {
                at = copy_string(ctx, b, at);
                at = copy_string(ctx, b, at);
                at = walk_type(ctx, b, at);
                i += 1;
            }
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n {
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
            while i < n {
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
            while i < n {
                let case = read_u32(b, at);
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
                        while j < n {
                            at = walk_type(ctx, b, at);
                            j += 1;
                        }
                    }
                    _ => panic!("contract spec contains an unsupported union case"),
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
            while i < n {
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
            while i < n {
                at = copy_string(ctx, b, at);
                i += 1;
            }
            let (n, next) = copy_len(ctx, b, at);
            at = next;
            let mut i = 0;
            while i < n {
                at = copy_string(ctx, b, at);
                at = copy_string(ctx, b, at);
                at = walk_type(ctx, b, at);
                at = copy_u32(ctx, b, at);
                i += 1;
            }
            at = copy_u32(ctx, b, at);
        }
        _ => panic!("contract spec contains an unsupported entry"),
    }
    at
}

/// An `SCSpecTypeDef`.
const fn walk_type<'a>(ctx: &mut Ctx<'a, '_>, b: &'a [u8], at: usize) -> usize {
    let ty = read_u32(b, at);
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
            while i < n {
                at = walk_type(ctx, b, at);
                i += 1;
            }
        }
        // BytesN: a length.
        1006 => at = copy_u32(ctx, b, at),
        // UDT: a name.
        2000 => at = ref_name(ctx, b, at),
        _ => panic!("contract spec contains an unsupported type"),
    }
    at
}

#[cfg(all(test, feature = "std"))]
mod test {
    use super::{len, reduce_into, NAME_LIMIT};
    use stellar_xdr::{
        Limits, ScSpecEntry, ScSpecEventDataFormat, ScSpecEventParamLocationV0, ScSpecEventParamV0,
        ScSpecEventV0, ScSpecFunctionInputV0, ScSpecFunctionV0, ScSpecTypeDef, ScSpecTypeMap,
        ScSpecTypeOption, ScSpecTypeResult, ScSpecTypeTuple, ScSpecTypeUdt, ScSpecTypeVec,
        ScSpecUdtEnumCaseV0, ScSpecUdtEnumV0, ScSpecUdtErrorEnumCaseV0, ScSpecUdtErrorEnumV0,
        ScSpecUdtStructFieldV0, ScSpecUdtStructV0, ScSpecUdtUnionCaseTupleV0, ScSpecUdtUnionCaseV0,
        ScSpecUdtUnionCaseVoidV0, ScSpecUdtUnionV0, WriteXdr, SC_SPEC_TYPE_NAME_LIMIT,
    };

    #[test]
    fn name_limit_matches_xdr() {
        assert_eq!(NAME_LIMIT, SC_SPEC_TYPE_NAME_LIMIT as usize);
    }

    struct Rng(u64);
    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }
        fn below(&mut self, n: usize) -> usize {
            (self.next() % n as u64) as usize
        }
        fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
            &items[self.below(items.len())]
        }
    }

    /// Names chosen to collide with each other, with reserved and generated
    /// names, with numbered names, and with the name limit.
    fn pool() -> Vec<Vec<u8>> {
        // Names at the limit, so that numbering them trims them, one at a
        // character boundary.
        let long = "L".repeat(NAME_LIMIT - 5);
        let wide = "é".repeat((NAME_LIMIT - 5) / 2);
        [
            "::a::X",
            "::b::X",
            "::c::X",
            "X",
            "X2",
            "::d::X2",
            "::a::Y",
            "Y",
            "Address",
            "::a::Address",
            "::m::Client",
            "Client2",
            "::a::m::X3",
            "a::X",
            "::Z",
            "Z",
            "::a::",
            "",
            "::a::Undefined",
            &format!("::a::{long}"),
            &format!("::b::{long}"),
            &format!("::a::{wide}"),
            &format!("::b::{wide}"),
        ]
        .iter()
        .map(|s| s.as_bytes().to_vec())
        .collect()
    }

    fn ty(rng: &mut Rng, names: &[Vec<u8>], depth: u32) -> ScSpecTypeDef {
        match if depth == 0 {
            rng.below(3)
        } else {
            rng.below(9)
        } {
            0 => ScSpecTypeDef::U32,
            1 | 2 => ScSpecTypeDef::Udt(ScSpecTypeUdt {
                name: rng.pick(names).clone().try_into().unwrap(),
            }),
            3 => ScSpecTypeDef::Option(Box::new(ScSpecTypeOption {
                value_type: Box::new(ty(rng, names, depth - 1)),
            })),
            4 => ScSpecTypeDef::Vec(Box::new(ScSpecTypeVec {
                element_type: Box::new(ty(rng, names, depth - 1)),
            })),
            5 => ScSpecTypeDef::Map(Box::new(ScSpecTypeMap {
                key_type: Box::new(ty(rng, names, depth - 1)),
                value_type: Box::new(ty(rng, names, depth - 1)),
            })),
            6 => ScSpecTypeDef::Result(Box::new(ScSpecTypeResult {
                ok_type: Box::new(ty(rng, names, depth - 1)),
                error_type: Box::new(ty(rng, names, depth - 1)),
            })),
            7 => ScSpecTypeDef::Tuple(Box::new(ScSpecTypeTuple {
                value_types: (0..rng.below(3))
                    .map(|_| ty(rng, names, depth - 1))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            })),
            _ => ScSpecTypeDef::BytesN(stellar_xdr::ScSpecTypeBytesN { n: 32 }),
        }
    }

    fn entry(rng: &mut Rng, name: &[u8], refs: &[Vec<u8>]) -> ScSpecEntry {
        let name = name.to_vec();
        match rng.below(6) {
            0 => ScSpecEntry::FunctionV0(ScSpecFunctionV0 {
                doc: "doc".try_into().unwrap(),
                name: "f".try_into().unwrap(),
                inputs: (0..rng.below(3))
                    .map(|_| ScSpecFunctionInputV0 {
                        doc: "".try_into().unwrap(),
                        name: "i".try_into().unwrap(),
                        type_: ty(rng, refs, 2),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                outputs: [ty(rng, refs, 2)].try_into().unwrap(),
            }),
            1 => ScSpecEntry::UdtStructV0(ScSpecUdtStructV0 {
                doc: "a doc".try_into().unwrap(),
                lib: "lib".try_into().unwrap(),
                name: name.try_into().unwrap(),
                fields: (0..rng.below(3))
                    .map(|_| ScSpecUdtStructFieldV0 {
                        doc: "".try_into().unwrap(),
                        name: "field".try_into().unwrap(),
                        type_: ty(rng, refs, 2),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            }),
            2 => ScSpecEntry::UdtUnionV0(ScSpecUdtUnionV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: name.try_into().unwrap(),
                cases: (0..rng.below(3))
                    .map(|i| {
                        if i % 2 == 0 {
                            ScSpecUdtUnionCaseV0::VoidV0(ScSpecUdtUnionCaseVoidV0 {
                                doc: "".try_into().unwrap(),
                                name: "V".try_into().unwrap(),
                            })
                        } else {
                            ScSpecUdtUnionCaseV0::TupleV0(ScSpecUdtUnionCaseTupleV0 {
                                doc: "d".try_into().unwrap(),
                                name: "T".try_into().unwrap(),
                                type_: (0..rng.below(3))
                                    .map(|_| ty(rng, refs, 2))
                                    .collect::<Vec<_>>()
                                    .try_into()
                                    .unwrap(),
                            })
                        }
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            }),
            3 => ScSpecEntry::UdtEnumV0(ScSpecUdtEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: name.try_into().unwrap(),
                cases: [ScSpecUdtEnumCaseV0 {
                    doc: "".try_into().unwrap(),
                    name: "A".try_into().unwrap(),
                    value: 1,
                }]
                .try_into()
                .unwrap(),
            }),
            4 => ScSpecEntry::UdtErrorEnumV0(ScSpecUdtErrorEnumV0 {
                doc: "".try_into().unwrap(),
                lib: "".try_into().unwrap(),
                name: name.try_into().unwrap(),
                cases: [ScSpecUdtErrorEnumCaseV0 {
                    doc: "e".try_into().unwrap(),
                    name: "E".try_into().unwrap(),
                    value: 2,
                }]
                .try_into()
                .unwrap(),
            }),
            _ => ScSpecEntry::EventV0(ScSpecEventV0 {
                doc: "".try_into().unwrap(),
                lib: "l".try_into().unwrap(),
                name: name.try_into().unwrap(),
                prefix_topics: ["topic".try_into().unwrap()].try_into().unwrap(),
                params: (0..rng.below(3))
                    .map(|_| ScSpecEventParamV0 {
                        doc: "".try_into().unwrap(),
                        name: "p".try_into().unwrap(),
                        type_: ty(rng, refs, 2),
                        location: ScSpecEventParamLocationV0::TopicList,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                data_format: ScSpecEventDataFormat::Map,
            }),
        }
    }

    fn encode(entries: impl IntoIterator<Item = ScSpecEntry>) -> Vec<u8> {
        entries
            .into_iter()
            .flat_map(|e| e.to_xdr(Limits::none()).unwrap())
            .collect()
    }

    /// Reduces the spec with the const fns, returning `None` if they panic.
    fn reduce_const(entries: &[Vec<u8>]) -> Option<Vec<u8>> {
        let spec: Vec<&[u8]> = entries.iter().map(Vec::as_slice).collect();
        std::panic::catch_unwind(|| {
            let mut out = vec![0u8; len(&spec)];
            reduce_into(&spec, &mut out);
            out
        })
        .ok()
    }

    #[test]
    fn matches_reduce() {
        let pool = pool();
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        let (mut ok, mut err) = (0, 0);
        for _ in 0..4000 {
            // Mostly names that are defined, sometimes any name in the pool.
            let defined: Vec<Vec<u8>> = (0..1 + rng.below(8))
                .map(|_| rng.pick(&pool).clone())
                .collect();
            let refs: Vec<Vec<u8>> = if rng.below(4) == 0 {
                pool.clone()
            } else {
                defined.clone()
            };
            let spec: Vec<ScSpecEntry> = defined
                .iter()
                .map(|name| entry(&mut rng, name, &refs))
                .collect();
            // Split the encoding across slices of one or more entries.
            let mut chunks = Vec::new();
            for entries in spec.chunks(1 + rng.below(3)) {
                chunks.push(encode(entries.iter().cloned()));
            }

            let expected = crate::reduce::reduce(&spec).map(|r| encode(r.into_entries()));
            let actual = reduce_const(&chunks);
            match expected {
                Ok(expected) => {
                    ok += 1;
                    assert_eq!(actual.as_ref(), Some(&expected), "spec: {spec:?}");
                }
                Err(_) => {
                    err += 1;
                    assert_eq!(actual, None, "spec: {spec:?}");
                }
            }
        }
        // Both outcomes are exercised.
        assert!(ok > 1000 && err > 100, "ok: {ok}, err: {err}");
    }
}
