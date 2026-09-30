use stellar_xdr::{ScSpecEntry, ScSpecTypeDef, ScSpecUdtUnionCaseV0, SC_SPEC_TYPE_NAME_LIMIT};

/// The most bytes a spec type or event name can hold, which bounds the names
/// the numbering below may produce.
const NAME_LIMIT: usize = SC_SPEC_TYPE_NAME_LIMIT as usize;

/// A spec with its user-defined type names reduced to simple names. Each
/// entry is paired with how its own name resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reduced(pub Vec<Entry>);

/// A reduced spec entry alongside how its name resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The spec entry with every user-defined type name reduced.
    pub entry: ScSpecEntry,
    /// How the entry's own name resolved, or `None` for an entry that defines
    /// no user-defined type or event name (a function).
    pub rename: Option<Rename>,
}

impl Reduced {
    /// The reduced spec entries, without their renames.
    pub fn entries(&self) -> impl Iterator<Item = &ScSpecEntry> + '_ {
        self.0.iter().map(|e| &e.entry)
    }

    /// Consumes into the reduced spec entries, without their renames.
    pub fn into_entries(self) -> impl Iterator<Item = ScSpecEntry> {
        self.0.into_iter().map(|e| e.entry)
    }

    /// How each entry that defines a name resolved it, in entry order.
    pub fn renames(&self) -> impl Iterator<Item = &Rename> + '_ {
        self.0.iter().filter_map(|e| e.rename.as_ref())
    }
}

/// A spec that cannot be reduced.
///
/// The names are byte strings as they appear in the spec.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The spec defines the same fully qualified user-defined type or event
    /// name more than once, which never happens, as a qualified name is the
    /// path of the one Rust type that defines it. Simple names are not
    /// checked, as specs from before qualified names can define the same
    /// simple name more than once.
    #[error("contract spec defines the name `{}` more than once", String::from_utf8_lossy(.0))]
    DuplicateName(Vec<u8>),
    /// The spec defines or refers to a name whose last segment is empty, such
    /// as `mycrate::` or an empty name, which no Rust type has.
    #[error("contract spec contains the name `{}`, whose last segment is empty", String::from_utf8_lossy(.0))]
    InvalidName(Vec<u8>),
}

/// How one user-defined type's name resolved during reduction.
///
/// Spec names are byte strings that are not guaranteed to be valid UTF-8, so
/// the names are byte strings here too, preserved exactly as they appear in
/// the spec. Decode lossily only for display.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rename {
    /// The name as it appears in the input spec.
    pub from: Vec<u8>,
    /// The name the type has in the reduced spec.
    pub to: Vec<u8>,
}

impl Rename {
    /// Whether the type's name changed at all.
    pub fn renamed(&self) -> bool {
        self.from != self.to
    }

    /// Whether the type could not keep the last segment of its name because
    /// another type claimed it first.
    pub fn collision(&self) -> bool {
        self.to != last_segment(&self.from)
    }
}

/// Whether the name is qualified by a path, and so is reduced.
fn is_qualified(name: &[u8]) -> bool {
    name.windows(2).any(|w| w == b"::")
}

/// The last `::`-separated segment of a fully qualified type name.
fn last_segment(name: &[u8]) -> &[u8] {
    name.windows(2)
        .rposition(|w| w == b"::")
        .map_or(name, |i| &name[i + 2..])
}

/// Reduces every user-defined type name in the spec from its fully qualified
/// form (`mycrate::mymod::MyType`) to its simple name (`MyType`), rewriting
/// every reference to a type to follow the type to its new name.
///
/// The first type to claim a simple name keeps it, so two types whose names
/// share a last segment stay distinct: the rest are numbered (`MyType2`,
/// `MyType3`, …), stepping over names claimed by other types. A spec whose
/// type names are already simple comes back unchanged.
///
/// A reference to a type the spec does not define is reduced to its last
/// segment, without claiming a name.
///
/// Names are treated as byte strings throughout and preserved exactly; the
/// spec does not guarantee valid UTF-8. When a numbered name would exceed the
/// spec's name limit, the base is trimmed from its end to make room for the
/// number.
///
/// A simple name, one without `::`, is not reduced: it keeps its name, and
/// claims it ahead of the qualified names, so specs from before qualified
/// names, which can define the same simple name more than once, come back
/// unchanged.
///
/// # Errors
///
/// - If the spec defines or refers to a name whose last segment is empty.
/// - If the spec defines the same qualified name more than once.
pub fn reduce(spec: &[ScSpecEntry]) -> Result<Reduced, Error> {
    validate(spec)?;

    // The names the spec defines, in definition order, each with the most
    // bytes its entry's name field can hold. Events define a name too: the
    // generated bindings declare a type for each event, so an event and a
    // type sharing a simple name are a collision like any other.
    let defined: Vec<(Vec<u8>, usize)> = spec
        .iter()
        .filter_map(|entry| match entry {
            ScSpecEntry::UdtStructV0(s) => Some((s.name.to_vec(), NAME_LIMIT)),
            ScSpecEntry::UdtUnionV0(u) => Some((u.name.to_vec(), NAME_LIMIT)),
            ScSpecEntry::UdtEnumV0(e) => Some((e.name.to_vec(), NAME_LIMIT)),
            ScSpecEntry::UdtErrorEnumV0(e) => Some((e.name.to_vec(), NAME_LIMIT)),
            ScSpecEntry::EventV0(e) => Some((e.name.to_vec(), NAME_LIMIT)),
            _ => None,
        })
        .collect();

    // Simple names keep their names, so they claim them first. Then the first
    // qualified type to claim a last segment keeps it, so a type only ever
    // loses its own name to one defined before it, never to a number handed
    // to a type that collided with something else.
    let mut taken: std::collections::HashSet<Vec<u8>> = defined
        .iter()
        .filter(|(name, _)| !is_qualified(name))
        .map(|(name, _)| name.clone())
        .collect();
    let colliding: Vec<&(Vec<u8>, usize)> = defined
        .iter()
        .filter(|(name, _)| is_qualified(name))
        .filter(|(name, _)| !taken.insert(last_segment(name).to_vec()))
        .collect();

    let mut numbered = std::collections::HashMap::new();
    for (name, limit) in colliding {
        let base = last_segment(name);
        let mut n = 1u32;
        let simple = loop {
            n += 1;
            let simple = numbered_name(base, n, *limit);
            if taken.insert(simple.clone()) {
                break simple;
            }
        };
        numbered.insert(name.clone(), simple);
    }

    let renames: Vec<Rename> = defined
        .iter()
        .map(|(name, _)| Rename {
            from: name.clone(),
            to: numbered
                .get(name)
                .cloned()
                .unwrap_or_else(|| last_segment(name).to_vec()),
        })
        .collect();

    let to: std::collections::HashMap<&[u8], &[u8]> = renames
        .iter()
        .map(|r| (r.from.as_slice(), r.to.as_slice()))
        .collect();
    let resolve = |name: &[u8]| -> Vec<u8> {
        to.get(name)
            .map_or_else(|| last_segment(name).to_vec(), |t| t.to_vec())
    };

    // Every resolved name is a fragment of a name that fit the spec, or was
    // sized to the limit when numbered, so conversion back cannot fail.
    let mut spec = spec.to_vec();
    for entry in spec.iter_mut() {
        match entry {
            ScSpecEntry::FunctionV0(f) => {
                for input in f.inputs.iter_mut() {
                    rewrite_ty(&mut input.type_, &resolve);
                }
                for output in f.outputs.iter_mut() {
                    rewrite_ty(output, &resolve);
                }
            }
            ScSpecEntry::UdtStructV0(s) => {
                s.name = resolve(&s.name).try_into().unwrap();
                for field in s.fields.iter_mut() {
                    rewrite_ty(&mut field.type_, &resolve);
                }
            }
            ScSpecEntry::UdtUnionV0(u) => {
                u.name = resolve(&u.name).try_into().unwrap();
                for case in u.cases.iter_mut() {
                    if let ScSpecUdtUnionCaseV0::TupleV0(t) = case {
                        for ty in t.type_.iter_mut() {
                            rewrite_ty(ty, &resolve);
                        }
                    }
                }
            }
            ScSpecEntry::UdtEnumV0(e) => {
                e.name = resolve(&e.name).try_into().unwrap();
            }
            ScSpecEntry::UdtErrorEnumV0(e) => {
                e.name = resolve(&e.name).try_into().unwrap();
            }
            ScSpecEntry::EventV0(e) => {
                e.name = resolve(&e.name).try_into().unwrap();
                for p in e.params.iter_mut() {
                    rewrite_ty(&mut p.type_, &resolve);
                }
            }
        }
    }

    // Pair each rewritten entry with how its name resolved. Renames were
    // collected over the named entries in definition order, which is the order
    // those entries appear here; a function defines no name and so has none.
    let mut renames = renames.into_iter();
    let entries = spec
        .into_iter()
        .map(|entry| {
            let rename = match &entry {
                ScSpecEntry::UdtStructV0(_)
                | ScSpecEntry::UdtUnionV0(_)
                | ScSpecEntry::UdtEnumV0(_)
                | ScSpecEntry::UdtErrorEnumV0(_)
                | ScSpecEntry::EventV0(_) => renames.next(),
                ScSpecEntry::FunctionV0(_) => None,
            };
            Entry { entry, rename }
        })
        .collect();
    Ok(Reduced(entries))
}

/// The base followed by the number, keeping as much of the base as leaves room
/// for the number within the limit, trimming bytes off its end when the base is
/// too long to hold the number too. A base that is valid UTF-8 is trimmed at a
/// character boundary, so that the name stays valid UTF-8. The limit always
/// exceeds the number's length, so a fitting name always exists.
fn numbered_name(base: &[u8], n: u32, limit: usize) -> Vec<u8> {
    let num = n.to_string();
    let mut keep = base.len().min(limit.saturating_sub(num.len()));
    if let Ok(base) = core::str::from_utf8(base) {
        while !base.is_char_boundary(keep) {
            keep -= 1;
        }
    }
    let mut name = base[..keep].to_vec();
    name.extend_from_slice(num.as_bytes());
    name
}

/// Checks that the spec's names can be reduced.
///
/// The spec can come from an untrusted wasm, so every name is checked before
/// any is reduced, rather than an invalid name reducing to an empty one that
/// only fails when something later uses it.
///
/// # Errors
///
/// - If the spec defines or refers to a name whose last segment is empty,
///   such as `mycrate::` or an empty name, as no Rust type has such a name.
/// - If the spec defines the same qualified name more than once.
fn validate(spec: &[ScSpecEntry]) -> Result<(), Error> {
    let mut defined = Vec::new();
    let mut referred = Vec::new();
    for entry in spec {
        match entry {
            ScSpecEntry::FunctionV0(f) => {
                for input in f.inputs.iter() {
                    udt_names(&input.type_, &mut referred);
                }
                for output in f.outputs.iter() {
                    udt_names(output, &mut referred);
                }
            }
            ScSpecEntry::UdtStructV0(s) => {
                defined.push(s.name.as_ref());
                for field in s.fields.iter() {
                    udt_names(&field.type_, &mut referred);
                }
            }
            ScSpecEntry::UdtUnionV0(u) => {
                defined.push(u.name.as_ref());
                for case in u.cases.iter() {
                    if let ScSpecUdtUnionCaseV0::TupleV0(t) = case {
                        for ty in t.type_.iter() {
                            udt_names(ty, &mut referred);
                        }
                    }
                }
            }
            ScSpecEntry::UdtEnumV0(e) => defined.push(e.name.as_ref()),
            ScSpecEntry::UdtErrorEnumV0(e) => defined.push(e.name.as_ref()),
            ScSpecEntry::EventV0(e) => {
                defined.push(e.name.as_ref());
                for p in e.params.iter() {
                    udt_names(&p.type_, &mut referred);
                }
            }
        }
    }

    if let Some(name) = defined
        .iter()
        .chain(referred.iter())
        .find(|name| last_segment(name).is_empty())
    {
        return Err(Error::InvalidName(name.to_vec()));
    }

    let mut seen = std::collections::HashSet::new();
    if let Some(name) = defined
        .iter()
        .filter(|name| is_qualified(name))
        .find(|name| !seen.insert(**name))
    {
        return Err(Error::DuplicateName(name.to_vec()));
    }

    Ok(())
}

/// Collects the name of every user-defined type reference in the type.
fn udt_names<'a>(t: &'a ScSpecTypeDef, names: &mut Vec<&'a [u8]>) {
    match t {
        ScSpecTypeDef::Udt(u) => names.push(u.name.as_ref()),
        ScSpecTypeDef::Option(o) => udt_names(&o.value_type, names),
        ScSpecTypeDef::Result(r) => {
            udt_names(&r.ok_type, names);
            udt_names(&r.error_type, names);
        }
        ScSpecTypeDef::Vec(v) => udt_names(&v.element_type, names),
        ScSpecTypeDef::Map(m) => {
            udt_names(&m.key_type, names);
            udt_names(&m.value_type, names);
        }
        ScSpecTypeDef::Tuple(tu) => {
            for vt in tu.value_types.iter() {
                udt_names(vt, names);
            }
        }
        _ => {}
    }
}

/// Rewrites the name of every user-defined type reference in the type.
fn rewrite_ty(t: &mut ScSpecTypeDef, resolve: &dyn Fn(&[u8]) -> Vec<u8>) {
    match t {
        ScSpecTypeDef::Udt(u) => {
            u.name = resolve(&u.name).try_into().unwrap();
        }
        ScSpecTypeDef::Option(o) => rewrite_ty(&mut o.value_type, resolve),
        ScSpecTypeDef::Result(r) => {
            rewrite_ty(&mut r.ok_type, resolve);
            rewrite_ty(&mut r.error_type, resolve);
        }
        ScSpecTypeDef::Vec(v) => rewrite_ty(&mut v.element_type, resolve),
        ScSpecTypeDef::Map(m) => {
            rewrite_ty(&mut m.key_type, resolve);
            rewrite_ty(&mut m.value_type, resolve);
        }
        ScSpecTypeDef::Tuple(tu) => {
            for vt in tu.value_types.iter_mut() {
                rewrite_ty(vt, resolve);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod test {
    use super::{numbered_name, reduce, Error, Rename, NAME_LIMIT};
    use stellar_xdr::{
        ScSpecEntry, ScSpecTypeDef, ScSpecTypeUdt, ScSpecUdtStructFieldV0, ScSpecUdtStructV0,
    };

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
        // same simple name collide: the first to claim it keeps it, the later
        // one is numbered.
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
    fn a_reference_to_an_undefined_type_reduces_without_claiming() {
        let spec = [struct_entry("a::MyType", &["elsewhere::Other"])];
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
    fn a_numbered_name_that_would_overflow_is_trimmed_to_fit() {
        // A 1024-byte base cannot fit a number after it, so the base is
        // trimmed to make room: 1023 bytes of the base followed by "2",
        // exactly at the limit.
        let base = "x".repeat(1024).into_bytes();
        let mut expected = "x".repeat(1023).into_bytes();
        expected.push(b'2');
        assert_eq!(numbered_name(&base, 2, NAME_LIMIT), expected);
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
        assert_eq!(numbered_name(&base, 2, NAME_LIMIT), expected);
    }
}
