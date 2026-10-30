package xdr

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"
)

// TestRoundTrip decodes and re-encodes spec sections emitted by the Rust SDK.
func TestRoundTrip(t *testing.T) {
	files, _ := filepath.Glob("testdata/*.spec.xdr")
	if len(files) == 0 {
		t.Fatal("no testdata")
	}
	for _, f := range files {
		b, err := os.ReadFile(f)
		if err != nil {
			t.Fatal(err)
		}
		entries, err := DecodeEntries(b)
		if err != nil {
			t.Fatalf("%s: %v", f, err)
		}
		var out []byte
		for _, e := range entries {
			out = append(out, e.Encode()...)
		}
		if !bytes.Equal(out, b) {
			t.Errorf("%s: round trip mismatch", f)
		}
	}
}

func TestMetaRoundTrip(t *testing.T) {
	m := []MetaEntry{{"rsver", "1.99.1"}, {"k", "value"}}
	got, err := DecodeMeta(EncodeMeta(m))
	if err != nil || len(got) != 2 || got[1] != m[1] {
		t.Fatalf("got %v %v", got, err)
	}
}
