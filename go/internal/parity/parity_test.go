// Package parity checks that the contract specs soroban-go emits for the Go
// ports in go/tests match the specs the Rust SDK emits for the Rust test
// contracts in tests/, after the post-processing stellar-cli's contract build
// applies (shaking and name reduction).
//
// The Rust wasms must be built first:
//
//	cargo build --release --target wasm32v1-none -p test_udt ...
//
// and the Go wasms with soroban-go build.
package parity

import (
	"os"
	"path/filepath"
	"sort"
	"strings"
	"testing"

	"github.com/stellar/rs-soroban-sdk/go/internal/spec"
	"github.com/stellar/rs-soroban-sdk/go/internal/wasm"
	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

const (
	rustDir = "../../../target/wasm32v1-none/release"
	goDir   = "../../tests"
)

var contracts = []string{
	"test_empty", "test_add_u64", "test_add_i128", "test_udt", "test_errors",
	"test_events", "test_auth", "test_constructor", "test_logging",
	"test_contract_data", "test_invoke_contract", "test_tuples", "test_account",
}

func readSpec(t *testing.T, path string) ([]xdr.Entry, []byte) {
	t.Helper()
	b, err := os.ReadFile(path)
	if err != nil {
		t.Skipf("%s not built: %v", path, err)
	}
	data, _, err := wasm.CustomSection(b, "contractspecv0")
	if err != nil {
		t.Fatal(err)
	}
	entries, err := xdr.DecodeEntries(data)
	if err != nil {
		t.Fatal(err)
	}
	return entries, b
}

// rustFinalSpec applies stellar-cli's contract build post-processing to a
// Rust SDK wasm spec.
func rustFinalSpec(t *testing.T, path string) []xdr.Entry {
	entries, b := readSpec(t, path)
	used := spec.UsedByMarkers(entries, spec.FindMarkers(b))
	entries = spec.Dedup(spec.Shake(entries, used))
	if _, err := spec.Reduce(entries); err != nil {
		t.Fatal(err)
	}
	return entries
}

func describeAll(entries []xdr.Entry) []string {
	var out []string
	for _, e := range entries {
		out = append(out, xdr.Describe(e)+" doc="+strings.ReplaceAll(e.Doc, "\n", "\\n"))
	}
	sort.Strings(out)
	return out
}

func encodeAll(entries []xdr.Entry) []string {
	var out []string
	for _, e := range entries {
		out = append(out, string(e.Encode()))
	}
	sort.Strings(out)
	return out
}

// TestSpecParityStellarCLI compares against Rust wasms built and
// post-processed by stellar-cli's contract build, in the directory named by
// the STELLAR_CLI_BUILT_WASMS environment variable.
func TestSpecParityStellarCLI(t *testing.T) {
	dir := os.Getenv("STELLAR_CLI_BUILT_WASMS")
	if dir == "" {
		t.Skip("STELLAR_CLI_BUILT_WASMS not set")
	}
	for _, c := range contracts {
		t.Run(c, func(t *testing.T) {
			rust, _ := readSpec(t, filepath.Join(dir, c+".wasm"))
			goSpec, _ := readSpec(t, filepath.Join(goDir, c, c+".wasm"))
			compare(t, rust, goSpec)
		})
	}
}

func compare(t *testing.T, rust, goSpec []xdr.Entry) {
	t.Helper()
	re, ge := encodeAll(rust), encodeAll(goSpec)
	if strings.Join(re, "\x00") == strings.Join(ge, "\x00") {
		return
	}
	rd, gd := describeAll(rust), describeAll(goSpec)
	t.Errorf("spec mismatch\nrust:\n  %s\ngo:\n  %s", strings.Join(rd, "\n  "), strings.Join(gd, "\n  "))
}

func TestSpecParity(t *testing.T) {
	for _, c := range contracts {
		t.Run(c, func(t *testing.T) {
			rust := rustFinalSpec(t, filepath.Join(rustDir, c+".wasm"))
			goSpec, _ := readSpec(t, filepath.Join(goDir, c, c+".wasm"))
			compare(t, rust, goSpec)
		})
	}
}
