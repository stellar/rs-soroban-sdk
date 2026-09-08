use stellar_xdr::{ScSpecEntry, ScSpecTypeDef, ScSpecUdtUnionCaseV0};

/// The most bytes a spec type name can hold (`SC_SPEC_TYPE_NAME_LIMIT`), which
/// bounds the names the numbering below may produce.
const TYPE_NAME_LIMIT: usize = 1024;

/// The most bytes an event name can hold (`SC_SPEC_TYPE_NAME_LIMIT`).
const EVENT_NAME_LIMIT: usize = 1024;

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
pub fn reduce(spec: &[ScSpecEntry]) -> Reduced {
    // The names the spec defines, in definition order, each with the most
    // bytes its entry's name field can hold. Events define a name too: the
    // generated bindings declare a type for each event, so an event and a
    // type sharing a simple name are a collision like any other.
    let defined: Vec<(Vec<u8>, usize)> = spec
        .iter()
        .filter_map(|entry| match entry {
            ScSpecEntry::UdtStructV0(s) => Some((s.name.to_vec(), TYPE_NAME_LIMIT)),
            ScSpecEntry::UdtUnionV0(u) => Some((u.name.to_vec(), TYPE_NAME_LIMIT)),
            ScSpecEntry::UdtEnumV0(e) => Some((e.name.to_vec(), TYPE_NAME_LIMIT)),
            ScSpecEntry::UdtErrorEnumV0(e) => Some((e.name.to_vec(), TYPE_NAME_LIMIT)),
            ScSpecEntry::EventV0(e) => Some((e.name.to_vec(), EVENT_NAME_LIMIT)),
            _ => None,
        })
        .collect();

    // The first type to claim a last segment keeps it, so a type only ever
    // loses its own name to one defined before it, never to a number handed
    // to a type that collided with something else.
    let mut taken = std::collections::HashSet::new();
    let colliding: Vec<&(Vec<u8>, usize)> = defined
        .iter()
        .filter(|(name, _)| !taken.insert(last_segment(name).to_vec()))
        .collect();

    let mut numbered = std::collections::HashMap::new();
    for (name, limit) in colliding {
        let base = last_segment(name);
        let mut n = 1u32;
        let simple = loop {
            n += 1;
            let num = n.to_string();
            // Keep as much of the base as leaves room for the number within
            // the limit, trimming bytes off its end when the base is too long
            // to hold the number too. The limit always exceeds the number's
            // length, so a fitting name always exists.
            let keep = limit.saturating_sub(num.len());
            let mut simple = base[..base.len().min(keep)].to_vec();
            simple.extend_from_slice(num.as_bytes());
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
    Reduced(entries)
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
    use super::{reduce, Rename};
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
        let reduced = reduce(&spec);
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
        let reduced = reduce(&spec);
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
        let reduced = reduce(&spec);
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
        let reduced = reduce(&spec);
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
        let reduced = reduce(&spec);
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
        let reduced = reduce(&spec);
        assert_eq!(reduced.entries().cloned().collect::<Vec<_>>(), spec);
        assert!(reduced.renames().all(|r| !r.renamed()));
    }

    #[test]
    fn a_simple_name_keeps_its_claim_over_a_later_qualified_one() {
        let spec = [struct_entry("MyType", &[]), struct_entry("a::MyType", &[])];
        let reduced = reduce(&spec);
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
        let reduced = reduce(&spec);
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
        let reduced = reduce(&spec);
        assert_eq!(
            names(reduced.entries()),
            [(b"\xff\xfeType".to_vec(), vec![b"\xff\xfeType".to_vec()])]
        );
    }

    #[test]
    fn a_name_whose_numbered_form_would_overflow_is_trimmed_to_fit() {
        let long = "x".repeat(1024);
        let spec = [
            struct_entry(&long, &[]),
            struct_entry_bytes(long.as_bytes(), &[]),
        ];
        let reduced = reduce(&spec);
        // The first claims the full 1024-byte name; the second cannot fit a
        // number after it, so the base is trimmed to make room: 1023 bytes of
        // the name followed by "2", exactly at the limit.
        let mut expected = "x".repeat(1023).into_bytes();
        expected.push(b'2');
        let to = reduced.renames().nth(1).unwrap().to.as_slice();
        assert_eq!(to, expected.as_slice());
        assert_eq!(to.len(), 1024);
    }
}
