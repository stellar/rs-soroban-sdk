package wasm

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"
)

// The Rust SDK test wasms must pass the feature check and round trip.
func TestRustWasms(t *testing.T) {
	files, _ := filepath.Glob("../../../target/wasm32v1-none/release/*.wasm")
	if len(files) == 0 {
		t.Skip("rust test wasms not built")
	}
	for _, f := range files {
		b, err := os.ReadFile(f)
		if err != nil {
			t.Fatal(err)
		}
		if err := Check(b); err != nil {
			t.Errorf("%s: %v", f, err)
		}
		s, err := Parse(b)
		if err != nil {
			t.Fatal(err)
		}
		if !bytes.Equal(Encode(s), b) {
			t.Errorf("%s: round trip mismatch", f)
		}
	}
}

func TestFinalize(t *testing.T) {
	// (module (func (export "a")) (func (export "_initialize")) ) plus a name section.
	m := []byte{0, 'a', 's', 'm', 1, 0, 0, 0,
		1, 4, 1, 0x60, 0, 0, // type
		3, 3, 2, 0, 0, // funcs
		7, 19, 2, 1, 'a', 0, 0, 11, '_', 'i', 'n', 'i', 't', 'i', 'a', 'l', 'i', 'z', 'e', 0, 1,
		10, 7, 2, 2, 0, 0x0b, 2, 0, 0x0b,
		0, 5, 4, 'n', 'a', 'm', 'e'}
	out, err := Finalize(m, []string{"_initialize"}, []Section{{Name: "contractspecv0", Data: []byte{1, 2}}})
	if err != nil {
		t.Fatal(err)
	}
	exports, _ := Exports(out)
	if len(exports) != 1 || exports[0].Name != "a" {
		t.Fatalf("exports %v", exports)
	}
	d, ok, _ := CustomSection(out, "contractspecv0")
	if !ok || !bytes.Equal(d, []byte{1, 2}) {
		t.Fatal("missing spec section")
	}
	if _, ok, _ := CustomSection(out, "name"); ok {
		t.Fatal("name section not removed")
	}
	if err := Check(out); err != nil {
		t.Fatal(err)
	}
}
