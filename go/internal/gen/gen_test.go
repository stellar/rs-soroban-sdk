package gen

import (
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"testing"

	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

// TestGenerate analyzes and generates code for every test contract, and
// checks that the generated code is up to date and compiles natively.
func TestGenerate(t *testing.T) {
	dirs, _ := filepath.Glob("../../tests/test_*")
	if len(dirs) == 0 {
		t.Fatal("no test contracts")
	}
	for _, dir := range dirs {
		t.Run(filepath.Base(dir), func(t *testing.T) {
			m, err := Analyze(dir)
			if err != nil {
				t.Fatal(err)
			}
			src, err := m.Generate()
			if err != nil {
				t.Fatal(err)
			}
			if _, _, err := m.Spec(); err != nil {
				t.Fatal(err)
			}
			existing, err := os.ReadFile(filepath.Join(dir, GeneratedFile))
			if err == nil && string(existing) != string(src) {
				t.Errorf("%s is stale; run soroban-go generate %s", GeneratedFile, dir)
			}
		})
	}
	out, err := exec.Command("go", "vet", "../../tests/...").CombinedOutput()
	if err != nil {
		t.Fatalf("go vet: %v\n%s", err, out)
	}
}

func TestAnalyzeErrors(t *testing.T) {
	dir := t.TempDir()
	write := func(src string) {
		os.WriteFile(filepath.Join(dir, "go.mod"), []byte("module example.com/bad\n\ngo 1.24\n\nrequire github.com/stellar/rs-soroban-sdk/go v0.0.0\n\nreplace github.com/stellar/rs-soroban-sdk/go => "+mustAbs(t, "../..")+"\n"), 0o644)
		os.WriteFile(filepath.Join(dir, "main.go"), []byte(src), 0o644)
	}
	write(`package main

//soroban:contract
type Contract struct{}

func (Contract) Bad(x float64) {}

func main() {}
`)
	_, err := Analyze(dir)
	if err == nil || !strings.Contains(err.Error(), "not a contract type") {
		t.Fatalf("expected unsupported type error, got %v", err)
	}
}

func mustAbs(t *testing.T, p string) string {
	a, err := filepath.Abs(p)
	if err != nil {
		t.Fatal(err)
	}
	return a
}

// TestUnionAndStructSpec checks the spec of the udt test contract in detail.
func TestUnionAndStructSpec(t *testing.T) {
	m, err := Analyze("../../tests/test_udt")
	if err != nil {
		t.Fatal(err)
	}
	raw, err := m.RawSpec()
	if err != nil {
		t.Fatal(err)
	}
	var names []string
	for _, e := range raw {
		names = append(names, xdr.Describe(e))
	}
	want := []string{
		"enum ::test_udt::UdtEnum2 { A = 10, B = 15 }",
		"union ::test_udt::UdtEnum { UdtA, UdtB(::test_udt::UdtStruct), UdtC(::test_udt::UdtEnum2), UdtD(::test_udt::UdtTuple) }",
		"struct ::test_udt::UdtTuple { 0: i64, 1: Vec<i64> }",
		"struct ::test_udt::UdtStruct { a: i64, b: i64, c: Vec<i64> }",
	}
	for i, w := range want {
		if names[i] != w {
			t.Errorf("entry %d:\n got  %s\n want %s", i, names[i], w)
		}
	}
}
