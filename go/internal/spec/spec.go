// Package spec post-processes contract spec entries the way stellar-cli's
// `contract build` does for the Rust SDK: it shakes out the entries a contract
// does not use, and reduces fully qualified type names to simple names. It is
// a port of soroban-spec's shaking (references model) and reduce modules.
package spec

import (
	"bytes"
	"fmt"
	"sort"
	"strconv"
	"strings"

	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

// NameLimit is the most bytes a spec type or event name can hold.
const NameLimit = 1024

// ReservedNames are the names of SDK types, which a reduced type is never
// given. From soroban-spec names.rs.
var ReservedNames = []string{
	"Val", "bool", "u32", "i32", "u64", "i64", "u128", "i128", "U256", "I256",
	"Timepoint", "Duration", "Bytes", "BytesN", "Hash", "String", "Symbol",
	"Address", "MuxedAddress", "Vec", "Map", "Option", "Result", "Fp", "Fp2",
	"Fr", "G1Affine", "G2Affine", "Bls12381Fp", "Bls12381Fp2", "Bls12381Fr",
	"Bls12381G1Affine", "Bls12381G2Affine", "Bn254Fp", "Bn254Fr",
	"Bn254G1Affine", "Bn254G2Affine", "BnScalar",
}

// GeneratedNames are the names of items generated alongside types by
// binding generators. From soroban-spec names.rs.
var GeneratedNames = []string{"Contract", "Client", "Args", "WASM"}

// Shake keeps the functions, the events and error enums in used, and every
// type a kept entry references transitively. used is keyed by the defined name
// of events and error enums. Order is preserved.
func Shake(entries []xdr.Entry, used map[string]bool) []xdr.Entry {
	defs := map[string][]int{}
	for i, e := range entries {
		if n, ok := typeName(e); ok {
			defs[n] = append(defs[n], i)
		}
	}
	var pending []int
	for i, e := range entries {
		switch e.Kind {
		case xdr.EntryFunction:
			pending = append(pending, i)
		case xdr.EntryEvent, xdr.EntryUDTErrorEnum:
			if used[e.Name] {
				pending = append(pending, i)
			}
		}
	}
	keep := make([]bool, len(entries))
	for len(pending) > 0 {
		i := pending[len(pending)-1]
		pending = pending[:len(pending)-1]
		if keep[i] {
			continue
		}
		keep[i] = true
		for _, n := range ReferencedNames(entries[i]) {
			pending = append(pending, defs[n]...)
		}
	}
	var out []xdr.Entry
	for i, e := range entries {
		if keep[i] {
			out = append(out, e)
		}
	}
	return out
}

// Dedup removes exact duplicate entries, keeping the first.
func Dedup(entries []xdr.Entry) []xdr.Entry {
	var out []xdr.Entry
	var seen [][]byte
outer:
	for _, e := range entries {
		b := e.Encode()
		for _, s := range seen {
			if bytes.Equal(s, b) {
				continue outer
			}
		}
		seen = append(seen, b)
		out = append(out, e)
	}
	return out
}

func typeName(e xdr.Entry) (string, bool) {
	switch e.Kind {
	case xdr.EntryUDTStruct, xdr.EntryUDTUnion, xdr.EntryUDTEnum, xdr.EntryUDTErrorEnum:
		return e.Name, true
	}
	return "", false
}

// ReferencedNames returns the names of every user-defined type the entry
// references.
func ReferencedNames(e xdr.Entry) []string {
	var names []string
	switch e.Kind {
	case xdr.EntryFunction:
		for _, in := range e.Inputs {
			collect(&in.Type, &names)
		}
		for i := range e.Outputs {
			collect(&e.Outputs[i], &names)
		}
	case xdr.EntryUDTStruct:
		for _, f := range e.Fields {
			collect(&f.Type, &names)
		}
	case xdr.EntryUDTUnion:
		for _, c := range e.UnionCases {
			for i := range c.Types {
				collect(&c.Types[i], &names)
			}
		}
	case xdr.EntryEvent:
		for _, p := range e.Params {
			collect(&p.Type, &names)
		}
	}
	return names
}

func collect(t *xdr.TypeDef, names *[]string) {
	walk(t, func(t *xdr.TypeDef) { *names = append(*names, t.UDT) })
}

// walk calls f for every UDT type def within t.
func walk(t *xdr.TypeDef, f func(*xdr.TypeDef)) {
	switch t.Type {
	case xdr.SpecTypeUDT:
		f(t)
	case xdr.SpecTypeOption, xdr.SpecTypeVec:
		walk(t.Elem, f)
	case xdr.SpecTypeResult:
		walk(t.Elem, f)
		walk(t.Err, f)
	case xdr.SpecTypeMap:
		walk(t.Key, f)
		walk(t.Elem, f)
	case xdr.SpecTypeTuple:
		for i := range t.Tuple {
			walk(&t.Tuple[i], f)
		}
	}
}

func isQualified(n string) bool { return strings.Contains(n, "::") }
func unrooted(n string) string  { return strings.TrimPrefix(n, "::") }

func lastSegment(n string) string {
	if i := strings.LastIndex(n, "::"); i >= 0 {
		return n[i+2:]
	}
	return n
}

// Rename records how a defined name was reduced.
type Rename struct {
	From, To string
}

// Collision reports whether the name was numbered to avoid a collision.
func (r Rename) Collision() bool { return r.To != lastSegment(r.From) }

// Reduce reduces every user-defined type and event name from its fully
// qualified form to its simple name, numbering names that collide with each
// other or with reserved names, and rewrites every reference. The entries are
// modified in place.
func Reduce(entries []xdr.Entry) ([]Rename, error) {
	if err := validate(entries); err != nil {
		return nil, err
	}
	var defined []string
	for _, e := range entries {
		if e.Kind != xdr.EntryFunction {
			defined = append(defined, e.Name)
		}
	}
	reservedSet := map[string]bool{}
	for _, r := range append(append([]string{}, ReservedNames...), GeneratedNames...) {
		reservedSet[r] = true
	}
	taken := map[string]bool{}
	for r := range reservedSet {
		taken[r] = true
	}
	for _, n := range defined {
		if !isQualified(n) && !reservedSet[n] {
			taken[n] = true
		}
	}
	var qualified []string
	for _, n := range defined {
		if isQualified(n) {
			qualified = append(qualified, n)
		}
	}
	sort.SliceStable(qualified, func(i, j int) bool { return unrooted(qualified[i]) < unrooted(qualified[j]) })
	var colliding []string
	for _, n := range defined {
		if !isQualified(n) && reservedSet[n] {
			colliding = append(colliding, n)
		}
	}
	for _, n := range qualified {
		ls := lastSegment(n)
		if taken[ls] {
			colliding = append(colliding, n)
		} else {
			taken[ls] = true
		}
	}
	sort.SliceStable(colliding, func(i, j int) bool { return unrooted(colliding[i]) < unrooted(colliding[j]) })
	var dedup []string
	for i, n := range colliding {
		if i == 0 || colliding[i-1] != n {
			dedup = append(dedup, n)
		}
	}
	numbered := map[string]string{}
	for _, n := range dedup {
		base := lastSegment(n)
		for k := 2; ; k++ {
			s := numberedName(base, k, NameLimit)
			if !taken[s] {
				taken[s] = true
				numbered[n] = s
				break
			}
		}
	}
	var renames []Rename
	to := map[string]string{}
	for _, n := range defined {
		r := Rename{From: n, To: lastSegment(n)}
		if s, ok := numbered[n]; ok {
			r.To = s
		}
		renames = append(renames, r)
		to[unrooted(n)] = r.To
	}
	resolve := func(n string) string {
		if t, ok := to[unrooted(n)]; ok {
			return t
		}
		return lastSegment(n)
	}
	rewrite := func(t *xdr.TypeDef) { walk(t, func(t *xdr.TypeDef) { t.UDT = resolve(t.UDT) }) }
	for i := range entries {
		e := &entries[i]
		switch e.Kind {
		case xdr.EntryFunction:
			for j := range e.Inputs {
				rewrite(&e.Inputs[j].Type)
			}
			for j := range e.Outputs {
				rewrite(&e.Outputs[j])
			}
		default:
			e.Name = resolve(e.Name)
			for j := range e.Fields {
				rewrite(&e.Fields[j].Type)
			}
			for j := range e.UnionCases {
				for k := range e.UnionCases[j].Types {
					rewrite(&e.UnionCases[j].Types[k])
				}
			}
			for j := range e.Params {
				rewrite(&e.Params[j].Type)
			}
		}
	}
	return renames, nil
}

func numberedName(base string, n, limit int) string {
	num := strconv.Itoa(n)
	keep := len(base)
	if limit-len(num) < keep {
		keep = limit - len(num)
	}
	return base[:keep] + num
}

func validate(entries []xdr.Entry) error {
	var defined, referred []string
	for _, e := range entries {
		if e.Kind != xdr.EntryFunction {
			defined = append(defined, e.Name)
		}
		referred = append(referred, ReferencedNames(e)...)
	}
	for _, n := range append(append([]string{}, defined...), referred...) {
		if lastSegment(n) == "" {
			return fmt.Errorf("contract spec contains the name `%s`, whose last segment is empty", n)
		}
	}
	simple := map[string]bool{}
	for _, n := range defined {
		if !isQualified(n) {
			simple[n] = true
		}
	}
	qualified := map[string]bool{}
	for _, n := range defined {
		if isQualified(n) {
			u := unrooted(n)
			if simple[u] || qualified[u] {
				return fmt.Errorf("contract spec defines the name `%s` more than once", n)
			}
			qualified[u] = true
		}
	}
	for _, n := range referred {
		if isQualified(n) && !qualified[unrooted(n)] && !simple[unrooted(n)] {
			return fmt.Errorf("contract spec refers to the name `%s`, which it does not define", n)
		}
	}
	return nil
}
