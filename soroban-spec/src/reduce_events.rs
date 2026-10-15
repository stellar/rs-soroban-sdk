//! Concatenate spec entries at compile time, reducing the names of events to
//! simple names.
//!
//! For crates that publish a spec assembled from the spec entries the SDK's
//! macros emit, such as the token and Stellar Asset contract specs. The macros
//! name events by their fully qualified path (`::mycrate::MyEvent`), which
//! `stellar contract build` reduces to simple names (`MyEvent`) for contracts.
//! A spec assembled in a crate never passes through a contract build, so it is
//! reduced here instead, in a const context so that it remains available
//! without std.
//!
//! Only the names of event entries are reduced. Nothing in a spec refers to an
//! event, so no references need rewriting. Spec entries that define or refer
//! to user-defined types are left unchanged, and so must not be given to these
//! functions if their names are expected to be reduced. Collisions between
//! reduced names are not detected. Crates using these functions should test
//! that the result matches [`crate::reduce::reduce`].

const EVENT_V0: u32 = 5;

/// Returns the length of the spec entries in `spec` concatenated, once the
/// names of their events are reduced.
pub const fn len(spec: &[&[u8]]) -> usize {
    let mut len = 0;
    let mut i = 0;
    while i < spec.len() {
        len += entry_len(spec[i]);
        i += 1;
    }
    len
}

/// Returns the spec entries in `spec` concatenated, with the names of their
/// events reduced. `N` must be [`len`] of `spec`.
pub const fn concat<const N: usize>(spec: &[&[u8]]) -> [u8; N] {
    let mut out = [0u8; N];
    let mut at = 0;
    let mut i = 0;
    while i < spec.len() {
        at = write_entry(spec[i], &mut out, at);
        i += 1;
    }
    if at != N {
        panic!("unexpected output length");
    }
    out
}

/// The location of an event's name in an entry: the offset of its length
/// prefix, the offset of the simple name and its length, and the offset of
/// what follows the name and its padding.
struct Name {
    len_at: usize,
    simple_at: usize,
    simple_len: usize,
    end: usize,
}

const fn padded(len: usize) -> usize {
    len.div_ceil(4) * 4
}

const fn read_u32(b: &[u8], at: usize) -> u32 {
    (b[at] as u32) << 24 | (b[at + 1] as u32) << 16 | (b[at + 2] as u32) << 8 | b[at + 3] as u32
}

/// Returns the location of the name of an event entry, or `None` if the entry
/// is not an event.
const fn event_name(entry: &[u8]) -> Option<Name> {
    if read_u32(entry, 0) != EVENT_V0 {
        return None;
    }
    // ScSpecEventV0: doc, lib, name, ...
    let mut at = 4;
    at += 4 + padded(read_u32(entry, at) as usize);
    at += 4 + padded(read_u32(entry, at) as usize);
    let len_at = at;
    let name_at = at + 4;
    let name_len = read_u32(entry, len_at) as usize;
    // The simple name follows the last `::`.
    let mut simple_at = name_at;
    let mut i = name_at;
    while i + 1 < name_at + name_len {
        if entry[i] == b':' && entry[i + 1] == b':' {
            simple_at = i + 2;
        }
        i += 1;
    }
    Some(Name {
        len_at,
        simple_at,
        simple_len: name_at + name_len - simple_at,
        end: name_at + padded(name_len),
    })
}

const fn entry_len(entry: &[u8]) -> usize {
    match event_name(entry) {
        Some(n) => n.len_at + 4 + padded(n.simple_len) + (entry.len() - n.end),
        None => entry.len(),
    }
}

const fn copy(from: &[u8], start: usize, end: usize, out: &mut [u8], mut at: usize) -> usize {
    let mut i = start;
    while i < end {
        out[at] = from[i];
        at += 1;
        i += 1;
    }
    at
}

const fn write_entry(entry: &[u8], out: &mut [u8], mut at: usize) -> usize {
    match event_name(entry) {
        Some(n) => {
            at = copy(entry, 0, n.len_at, out, at);
            let len = (n.simple_len as u32).to_be_bytes();
            at = copy(&len, 0, 4, out, at);
            at = copy(entry, n.simple_at, n.simple_at + n.simple_len, out, at);
            // Zero padding, which `out` is initialized with.
            at += padded(n.simple_len) - n.simple_len;
            copy(entry, n.end, entry.len(), out, at)
        }
        None => copy(entry, 0, entry.len(), out, at),
    }
}
