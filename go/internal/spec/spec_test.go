package spec

import (
	"testing"

	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

func udt(name string) xdr.TypeDef { return xdr.TypeDef{Type: xdr.SpecTypeUDT, UDT: name} }

func TestReduceCollisions(t *testing.T) {
	entries := []xdr.Entry{
		{Kind: xdr.EntryUDTStruct, Name: "::b::State", Fields: []xdr.Field{{Name: "x", Type: udt("::a::State")}}},
		{Kind: xdr.EntryUDTStruct, Name: "::a::State"},
		{Kind: xdr.EntryUDTStruct, Name: "::a::Address"},
		{Kind: xdr.EntryFunction, Name: "f", Inputs: []xdr.Field{{Name: "s", Type: udt("::b::State")}}},
	}
	renames, err := Reduce(entries)
	if err != nil {
		t.Fatal(err)
	}
	got := map[string]string{}
	for _, r := range renames {
		got[r.From] = r.To
	}
	// ::a::State sorts first and keeps the simple name; Address is reserved.
	if got["::a::State"] != "State" || got["::b::State"] != "State2" || got["::a::Address"] != "Address2" {
		t.Fatalf("renames %v", got)
	}
	if entries[0].Fields[0].Type.UDT != "State" || entries[3].Inputs[0].Type.UDT != "State2" {
		t.Fatalf("references not rewritten: %+v", entries)
	}
}

func TestReduceUndefined(t *testing.T) {
	entries := []xdr.Entry{{Kind: xdr.EntryFunction, Name: "f", Inputs: []xdr.Field{{Name: "s", Type: udt("::a::Missing")}}}}
	if _, err := Reduce(entries); err == nil {
		t.Fatal("expected error")
	}
}

func TestShake(t *testing.T) {
	entries := []xdr.Entry{
		{Kind: xdr.EntryUDTStruct, Name: "Used", Fields: []xdr.Field{{Name: "x", Type: udt("Nested")}}},
		{Kind: xdr.EntryUDTStruct, Name: "Nested"},
		{Kind: xdr.EntryUDTStruct, Name: "Unused"},
		{Kind: xdr.EntryUDTErrorEnum, Name: "Err"},
		{Kind: xdr.EntryUDTErrorEnum, Name: "UnusedErr"},
		{Kind: xdr.EntryEvent, Name: "Ev"},
		{Kind: xdr.EntryFunction, Name: "f", Outputs: []xdr.TypeDef{{Type: xdr.SpecTypeVec, Elem: &xdr.TypeDef{Type: xdr.SpecTypeUDT, UDT: "Used"}}}},
	}
	out := Shake(entries, map[string]bool{"Err": true, "Ev": true})
	var names []string
	for _, e := range out {
		names = append(names, e.Name)
	}
	want := "Used Nested Err Ev f"
	if got := join(names); got != want {
		t.Fatalf("got %s want %s", got, want)
	}
}

func TestMarkers(t *testing.T) {
	e := xdr.Entry{Kind: xdr.EntryEvent, Name: "::x::Ev"}
	m := Marker(e)
	wasm := append([]byte("junk"), m[:]...)
	used := UsedByMarkers([]xdr.Entry{e}, FindMarkers(wasm))
	if !used["::x::Ev"] {
		t.Fatal("marker not found")
	}
}

func join(s []string) string {
	out := ""
	for i, x := range s {
		if i > 0 {
			out += " "
		}
		out += x
	}
	return out
}
