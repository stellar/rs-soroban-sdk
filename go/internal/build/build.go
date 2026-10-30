// Package build compiles a Go contract package to a Soroban contract Wasm:
// it generates the contract glue code, compiles the package with TinyGo for a
// target restricted to the Wasm features the Soroban VM supports, and writes
// the contract spec and meta custom sections.
package build

import (
	"bytes"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"runtime/debug"
	"strings"

	"github.com/stellar/rs-soroban-sdk/go/internal/gen"
	"github.com/stellar/rs-soroban-sdk/go/internal/spec"
	"github.com/stellar/rs-soroban-sdk/go/internal/wasm"
	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
	"github.com/stellar/rs-soroban-sdk/go/soroban"
)

// Options configures a build.
type Options struct {
	// Dir is the contract package directory.
	Dir string
	// Out is the output Wasm path. Defaults to <dir>/<crate>.wasm.
	Out string
	// TinyGo is the tinygo binary. Defaults to tinygo on PATH, or
	// $TINYGOROOT/bin/tinygo.
	TinyGo string
	// Protocol is the ledger protocol version recorded in the env meta.
	Protocol uint32
	// PreRelease is the pre-release version recorded in the env meta.
	PreRelease uint32
	// Meta is additional contract meta.
	Meta []xdr.MetaEntry
	// Raw keeps the raw spec with fully qualified names and every entry,
	// instead of shaking and reducing it.
	Raw bool
	// GenerateOnly generates the glue code without compiling.
	GenerateOnly bool
	// Opt is the TinyGo optimization level. Defaults to z.
	Opt string
	// Verbose prints the commands run.
	Verbose bool
	// Log receives progress and warnings.
	Log func(format string, args ...any)
}

// Result describes a built contract.
type Result struct {
	Wasm    string
	Spec    []xdr.Entry
	Renames []spec.Rename
}

// Build builds the contract.
func Build(o Options) (*Result, error) {
	if o.Log == nil {
		o.Log = func(string, ...any) {}
	}
	if o.Protocol == 0 {
		o.Protocol = soroban.InterfaceVersion
	}
	m, err := gen.Analyze(o.Dir)
	if err != nil {
		return nil, err
	}
	src, err := m.Generate()
	if err != nil {
		return nil, err
	}
	genPath := filepath.Join(o.Dir, gen.GeneratedFile)
	if err := os.WriteFile(genPath, src, 0o644); err != nil {
		return nil, err
	}
	res := &Result{}
	if o.Raw {
		res.Spec, err = m.RawSpec()
	} else {
		res.Spec, res.Renames, err = m.Spec()
	}
	if err != nil {
		return nil, err
	}
	for _, r := range res.Renames {
		if r.Collision() {
			o.Log("warning: type name %s was renamed to %s to avoid a collision", r.From, r.To)
		}
	}
	if o.GenerateOnly {
		return res, nil
	}
	if len(m.Contracts) == 0 {
		return nil, fmt.Errorf("no type with the soroban:contract directive in %s", o.Dir)
	}

	out := o.Out
	if out == "" {
		out = filepath.Join(o.Dir, m.Crate+".wasm")
	}
	res.Wasm = out

	tinygo, err := findTinyGo(o.TinyGo)
	if err != nil {
		return nil, err
	}
	tmp, err := os.MkdirTemp("", "soroban-go-")
	if err != nil {
		return nil, err
	}
	defer os.RemoveAll(tmp)
	target := filepath.Join(tmp, "soroban.json")
	if err := os.WriteFile(target, targetJSON(), 0o644); err != nil {
		return nil, err
	}
	rawWasm := filepath.Join(tmp, "contract.wasm")
	if o.Opt == "" {
		o.Opt = "z"
	}
	args := []string{"build", "-target", target, "-opt", o.Opt, "-no-debug", "-panic", "trap", "-o", rawWasm, "."}
	cmd := exec.Command(tinygo, args...)
	cmd.Dir = o.Dir
	var stderr bytes.Buffer
	cmd.Stdout = os.Stdout
	cmd.Stderr = &stderr
	if o.Verbose {
		o.Log("%s %s", tinygo, strings.Join(args, " "))
	}
	if err := cmd.Run(); err != nil {
		return nil, fmt.Errorf("tinygo build failed: %w\n%s", err, stderr.String())
	}
	b, err := os.ReadFile(rawWasm)
	if err != nil {
		return nil, err
	}
	if err := wasm.Check(b); err != nil {
		return nil, err
	}

	var specXDR []byte
	for _, e := range res.Spec {
		specXDR = append(specXDR, e.Encode()...)
	}
	meta := []xdr.MetaEntry{
		{Key: "gover", Val: tinygoVersion(tinygo)},
		{Key: "gosdkver", Val: sdkVersion()},
	}
	meta = append(meta, m.Meta...)
	meta = append(meta, o.Meta...)
	b, err = wasm.Finalize(b, []string{"_initialize"}, []wasm.Section{
		{Name: "contractspecv0", Data: specXDR},
		{Name: "contractenvmetav0", Data: xdr.EncodeEnvMetaInterfaceVersion(o.Protocol, o.PreRelease)},
		{Name: "contractmetav0", Data: xdr.EncodeMeta(meta)},
	})
	if err != nil {
		return nil, err
	}
	if err := os.WriteFile(out, b, 0o644); err != nil {
		return nil, err
	}
	return res, nil
}

func findTinyGo(explicit string) (string, error) {
	if explicit != "" {
		return explicit, nil
	}
	if p, err := exec.LookPath("tinygo"); err == nil {
		return p, nil
	}
	if root := os.Getenv("TINYGOROOT"); root != "" {
		return filepath.Join(root, "bin", "tinygo"), nil
	}
	return "", fmt.Errorf("tinygo not found: install TinyGo (https://tinygo.org) and add it to PATH, or set TINYGOROOT")
}

func tinygoVersion(tinygo string) string {
	out, err := exec.Command(tinygo, "version").Output()
	if err != nil {
		return "unknown"
	}
	// "tinygo version 0.40.1 linux/amd64 (using go version go1.24.7 and LLVM version 20.1.1)"
	f := strings.Fields(string(out))
	v := "tinygo"
	if len(f) >= 3 {
		v += f[2]
	}
	for i := 0; i+2 < len(f); i++ {
		if f[i] == "go" && f[i+1] == "version" {
			v += "+" + f[i+2]
		}
	}
	return v
}

// sdkVersion returns the SDK version, with the VCS revision when known, in the
// same form as the Rust SDK's rssdkver meta: <version>#<revision>.
func sdkVersion() string {
	v := soroban.Version
	if info, ok := debug.ReadBuildInfo(); ok {
		for _, s := range info.Settings {
			if s.Key == "vcs.revision" {
				return v + "#" + s.Value
			}
		}
	}
	return v
}

// targetJSON returns the TinyGo target: wasm-unknown restricted to the Wasm
// features the Soroban VM supports. The VM supports MVP Wasm with mutable
// globals, sign extension, and bulk memory, and rejects floating point and
// saturating float to int conversions.
func targetJSON() []byte {
	t := map[string]any{
		"llvm-target":        "wasm32-unknown-unknown",
		"cpu":                "mvp",
		"features":           "+mutable-globals,+sign-ext,-nontrapping-fptoint,-bulk-memory,-multivalue,-reference-types",
		"build-tags":         []string{"tinygo.wasm", "wasm_unknown", "soroban"},
		"buildmode":          "c-shared",
		"goos":               "linux",
		"goarch":             "arm",
		"linker":             "wasm-ld",
		"rtlib":              "compiler-rt",
		"libc":               "wasmbuiltins",
		"scheduler":          "none",
		"gc":                 "leaking",
		"default-stack-size": 32768,
		"cflags":             []string{"-mno-nontrapping-fptoint", "-mno-bulk-memory", "-mno-multivalue", "-mno-reference-types", "-msign-ext"},
		"ldflags":            []string{"--stack-first", "--no-demangle", "--no-entry"},
		"extra-files":        []string{"src/runtime/asm_tinygowasm.S"},
	}
	b, _ := json.MarshalIndent(t, "", "\t")
	return b
}
