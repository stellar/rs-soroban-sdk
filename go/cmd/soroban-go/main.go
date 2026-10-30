// Command soroban-go builds Soroban smart contracts written in Go.
//
// Usage:
//
//	soroban-go build [flags] [dir]     build a contract package to Wasm
//	soroban-go generate [dir]          generate the contract glue code only
//	soroban-go spec [-json] file.wasm  print the contract spec of a Wasm
//	soroban-go version                 print the SDK version
package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"strings"

	"github.com/stellar/rs-soroban-sdk/go/internal/build"
	"github.com/stellar/rs-soroban-sdk/go/internal/wasm"
	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
	"github.com/stellar/rs-soroban-sdk/go/soroban"
)

func main() {
	if len(os.Args) < 2 {
		usage()
	}
	var err error
	switch os.Args[1] {
	case "build":
		err = cmdBuild(os.Args[2:], false)
	case "generate":
		err = cmdBuild(os.Args[2:], true)
	case "spec":
		err = cmdSpec(os.Args[2:])
	case "version":
		fmt.Printf("soroban-go %s (protocol %d)\n", soroban.Version, soroban.InterfaceVersion)
	default:
		usage()
	}
	if err != nil {
		fmt.Fprintln(os.Stderr, "error:", err)
		os.Exit(1)
	}
}

func usage() {
	fmt.Fprint(os.Stderr, `soroban-go builds Soroban smart contracts written in Go.

Usage:
  soroban-go build [flags] [dir]     build a contract package to Wasm
  soroban-go generate [dir]          generate the contract glue code only
  soroban-go spec [-json] file.wasm  print the contract spec of a Wasm
  soroban-go version                 print the SDK version

Run 'soroban-go build -h' for build flags.
`)
	os.Exit(2)
}

type metaFlag []xdr.MetaEntry

func (m *metaFlag) String() string { return "" }
func (m *metaFlag) Set(s string) error {
	k, v, ok := strings.Cut(s, "=")
	if !ok {
		return fmt.Errorf("meta must be key=value")
	}
	*m = append(*m, xdr.MetaEntry{Key: k, Val: v})
	return nil
}

func cmdBuild(args []string, generateOnly bool) error {
	fs := flag.NewFlagSet("build", flag.ExitOnError)
	out := fs.String("o", "", "output wasm `file` (default <dir>/<name>.wasm)")
	tinygo := fs.String("tinygo", "", "path to the tinygo binary")
	protocol := fs.Uint("protocol", soroban.InterfaceVersion, "ledger protocol version to record in the contract env meta")
	raw := fs.Bool("raw", false, "write the raw spec with every entry and fully qualified names, without shaking or reducing")
	verbose := fs.Bool("v", false, "print commands")
	opt := fs.String("opt", "z", "TinyGo optimization `level` (0, 1, 2, s, z)")
	var meta metaFlag
	fs.Var(&meta, "meta", "add `key=value` to the contract meta (repeatable)")
	fs.Parse(args)
	dir := "."
	if fs.NArg() > 0 {
		dir = fs.Arg(0)
	}
	res, err := build.Build(build.Options{
		Dir:          dir,
		Out:          *out,
		TinyGo:       *tinygo,
		Protocol:     uint32(*protocol),
		Meta:         meta,
		Raw:          *raw,
		GenerateOnly: generateOnly,
		Verbose:      *verbose,
		Opt:          *opt,
		Log:          func(f string, a ...any) { fmt.Fprintf(os.Stderr, f+"\n", a...) },
	})
	if err != nil {
		return err
	}
	if !generateOnly {
		st, _ := os.Stat(res.Wasm)
		fmt.Fprintf(os.Stderr, "built %s (%d bytes, %d spec entries)\n", res.Wasm, st.Size(), len(res.Spec))
	}
	return nil
}

func cmdSpec(args []string) error {
	fs := flag.NewFlagSet("spec", flag.ExitOnError)
	asJSON := fs.Bool("json", false, "print as JSON")
	fs.Parse(args)
	if fs.NArg() != 1 {
		return fmt.Errorf("usage: soroban-go spec [-json] file.wasm")
	}
	b, err := os.ReadFile(fs.Arg(0))
	if err != nil {
		return err
	}
	data, _, err := wasm.CustomSection(b, "contractspecv0")
	if err != nil {
		return err
	}
	entries, err := xdr.DecodeEntries(data)
	if err != nil {
		return err
	}
	metaData, _, _ := wasm.CustomSection(b, "contractmetav0")
	meta, _ := xdr.DecodeMeta(metaData)
	if *asJSON {
		e := json.NewEncoder(os.Stdout)
		e.SetIndent("", "  ")
		return e.Encode(map[string]any{"spec": entries, "meta": meta})
	}
	for _, m := range meta {
		fmt.Printf("meta %s = %s\n", m.Key, m.Val)
	}
	for _, e := range entries {
		fmt.Println(xdr.Describe(e))
	}
	return nil
}
