package gen

import (
	"fmt"
	"go/types"

	"github.com/stellar/rs-soroban-sdk/go/internal/gen/sdkspec"
	"github.com/stellar/rs-soroban-sdk/go/internal/spec"
	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

// RawSpec returns the spec entries of every type, event and function with
// fully qualified type names, before shaking and reduction. This matches the
// entries the Rust SDK writes to the contractspecv0 section at compile time.
func (m *Model) RawSpec() ([]xdr.Entry, error) {
	var out []xdr.Entry
	for _, u := range m.Types {
		e, err := m.udtEntry(u)
		if err != nil {
			return nil, err
		}
		out = append(out, e)
	}
	for _, ev := range m.Events {
		e, err := m.eventEntry(ev)
		if err != nil {
			return nil, err
		}
		out = append(out, e)
	}
	for _, f := range m.Functions {
		e, err := m.functionEntry(f)
		if err != nil {
			return nil, err
		}
		out = append(out, e)
	}
	// The SDK's own types, which shaking keeps only if referenced.
	out = append(out, sdkspec.Entries()...)
	for _, e := range out {
		if err := checkLimits(e); err != nil {
			return nil, err
		}
	}
	return out, nil
}

// Spec returns the final spec entries: the raw spec shaken of unused types,
// events and errors, and with type names reduced to simple names. This matches
// the contractspecv0 section stellar-cli's `contract build` writes for a
// contract built with the Rust SDK.
func (m *Model) Spec() ([]xdr.Entry, []spec.Rename, error) {
	raw, err := m.RawSpec()
	if err != nil {
		return nil, nil, err
	}
	used := map[string]bool{}
	for obj := range m.Used {
		used[m.qualifiedName(obj)] = true
	}
	entries := spec.Dedup(spec.Shake(raw, used))
	renames, err := spec.Reduce(entries)
	if err != nil {
		return nil, nil, err
	}
	return entries, renames, nil
}

func checkLimits(e xdr.Entry) error {
	if len(e.Doc) > 1024 {
		return fmt.Errorf("%s: doc is longer than 1024 bytes", e.Name)
	}
	if len(e.Name) > spec.NameLimit {
		return fmt.Errorf("%s: name is longer than %d bytes", e.Name, spec.NameLimit)
	}
	for _, f := range e.Fields {
		if len(f.Name) > 30 {
			return fmt.Errorf("%s: field name %q is longer than 30 characters", e.Name, f.Name)
		}
	}
	for _, c := range e.UnionCases {
		if len(c.Name) > 60 {
			return fmt.Errorf("%s: case name %q is longer than 60 characters", e.Name, c.Name)
		}
	}
	for _, c := range e.EnumCases {
		if len(c.Name) > 60 {
			return fmt.Errorf("%s: case name %q is longer than 60 characters", e.Name, c.Name)
		}
	}
	for _, p := range e.Params {
		if len(p.Name) > 30 {
			return fmt.Errorf("%s: param name %q is longer than 30 characters", e.Name, p.Name)
		}
	}
	for _, t := range e.PrefixTopics {
		if len(t) > 32 {
			return fmt.Errorf("%s: topic %q is longer than 32 characters", e.Name, t)
		}
	}
	return nil
}

func (m *Model) udtEntry(u *UDT) (xdr.Entry, error) {
	e := xdr.Entry{Doc: u.Doc, Name: m.qualifiedName(u.Obj)}
	switch u.Kind {
	case KindStruct, KindTuple:
		e.Kind = xdr.EntryUDTStruct
		for _, f := range u.Fields {
			td, err := m.TypeDef(f.Type)
			if err != nil {
				return e, err
			}
			e.Fields = append(e.Fields, xdr.Field{Doc: f.Doc, Name: f.Name, Type: td})
		}
	case KindUnion:
		e.Kind = xdr.EntryUDTUnion
		for _, f := range u.Fields {
			c := xdr.UnionCase{Doc: f.Doc, Name: f.Name, Tuple: !f.Void}
			for _, t := range f.Elems {
				td, err := m.TypeDef(t)
				if err != nil {
					return e, err
				}
				c.Types = append(c.Types, td)
			}
			e.UnionCases = append(e.UnionCases, c)
		}
	case KindEnum, KindError:
		e.Kind = xdr.EntryUDTEnum
		if u.Kind == KindError {
			e.Kind = xdr.EntryUDTErrorEnum
		}
		for _, c := range u.Cases {
			e.EnumCases = append(e.EnumCases, xdr.EnumCase{Doc: c.Doc, Name: c.Name, Value: c.Value})
		}
	}
	return e, nil
}

func (m *Model) eventEntry(ev *Event) (xdr.Entry, error) {
	e := xdr.Entry{
		Kind:         xdr.EntryEvent,
		Doc:          ev.Doc,
		Name:         m.qualifiedName(ev.Obj),
		PrefixTopics: ev.PrefixTopics,
		DataFormat:   ev.DataFormat,
	}
	for _, f := range ev.Fields {
		td, err := m.TypeDef(f.Type)
		if err != nil {
			return e, err
		}
		loc := xdr.EventParamData
		if f.Topic {
			loc = xdr.EventParamTopicList
		}
		e.Params = append(e.Params, xdr.EventParam{Doc: f.Doc, Name: f.Name, Type: td, Location: loc})
	}
	return e, nil
}

func (m *Model) errTypeDef(t types.Type) (xdr.TypeDef, error) {
	if isErrorInterface(t) {
		return xdr.TypeDef{Type: xdr.SpecTypeError}, nil
	}
	return m.TypeDef(t.(*types.Pointer).Elem())
}

func (m *Model) functionEntry(f *Function) (xdr.Entry, error) {
	e := xdr.Entry{Kind: xdr.EntryFunction, Doc: f.Doc, Name: f.Name}
	for _, p := range f.Params {
		td, err := m.TypeDef(p.Type)
		if err != nil {
			return e, err
		}
		e.Inputs = append(e.Inputs, xdr.Field{Name: p.Name, Type: td})
	}
	switch {
	case f.ErrType != nil:
		ok := xdr.TypeDef{Type: xdr.SpecTypeVoid}
		if f.Result != nil {
			var err error
			if ok, err = m.TypeDef(f.Result); err != nil {
				return e, err
			}
		}
		et, err := m.errTypeDef(f.ErrType)
		if err != nil {
			return e, err
		}
		e.Outputs = []xdr.TypeDef{{Type: xdr.SpecTypeResult, Elem: &ok, Err: &et}}
	case f.Result != nil:
		td, err := m.TypeDef(f.Result)
		if err != nil {
			return e, err
		}
		e.Outputs = []xdr.TypeDef{td}
	}
	return e, nil
}
