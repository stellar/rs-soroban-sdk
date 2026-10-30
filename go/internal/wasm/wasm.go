// Package wasm reads and rewrites the sections of a Wasm module: it replaces
// custom sections, removes exports, and checks that the module only uses the
// Wasm features the Soroban VM supports.
package wasm

import (
	"bytes"
	"errors"
	"fmt"
)

// Section is a Wasm section. For custom sections Name is set and Data excludes
// the name.
type Section struct {
	ID   byte
	Name string
	Data []byte
}

const (
	idCustom   = 0
	idType     = 1
	idImport   = 2
	idFunction = 3
	idGlobal   = 6
	idExport   = 7
	idStart    = 8
	idCode     = 10
)

var magic = []byte{0, 'a', 's', 'm', 1, 0, 0, 0}

// Parse splits a module into its sections.
func Parse(b []byte) ([]Section, error) {
	if !bytes.HasPrefix(b, magic) {
		return nil, errors.New("wasm: not a wasm module")
	}
	r := reader{b: b, off: len(magic)}
	var out []Section
	for !r.done() {
		id := r.byte()
		n := int(r.u32())
		data := r.bytes(n)
		if r.err != nil {
			return nil, r.err
		}
		s := Section{ID: id, Data: data}
		if id == idCustom {
			sr := reader{b: data}
			s.Name = string(sr.bytes(int(sr.u32())))
			if sr.err != nil {
				return nil, sr.err
			}
			s.Data = data[sr.off:]
		}
		out = append(out, s)
	}
	return out, nil
}

// Encode joins sections into a module.
func Encode(sections []Section) []byte {
	var w bytes.Buffer
	w.Write(magic)
	for _, s := range sections {
		data := s.Data
		if s.ID == idCustom {
			var c bytes.Buffer
			writeU32(&c, uint32(len(s.Name)))
			c.WriteString(s.Name)
			c.Write(s.Data)
			data = c.Bytes()
		}
		w.WriteByte(s.ID)
		writeU32(&w, uint32(len(data)))
		w.Write(data)
	}
	return w.Bytes()
}

// CustomSection returns the data of the first custom section with the name.
func CustomSection(b []byte, name string) ([]byte, bool, error) {
	sections, err := Parse(b)
	if err != nil {
		return nil, false, err
	}
	for _, s := range sections {
		if s.ID == idCustom && s.Name == name {
			return s.Data, true, nil
		}
	}
	return nil, false, nil
}

// Finalize removes all custom sections and the named exports, and appends the
// custom sections.
func Finalize(b []byte, removeExports []string, custom []Section) ([]byte, error) {
	sections, err := Parse(b)
	if err != nil {
		return nil, err
	}
	var out []Section
	for _, s := range sections {
		switch s.ID {
		case idCustom:
			continue
		case idExport:
			s.Data, err = removeExportEntries(s.Data, removeExports)
			if err != nil {
				return nil, err
			}
		}
		out = append(out, s)
	}
	for _, c := range custom {
		c.ID = idCustom
		out = append(out, c)
	}
	return Encode(out), nil
}

// Export is an entry in the export section.
type Export struct {
	Name  string
	Kind  byte
	Index uint32
}

// Exports returns the module's exports.
func Exports(b []byte) ([]Export, error) {
	sections, err := Parse(b)
	if err != nil {
		return nil, err
	}
	for _, s := range sections {
		if s.ID == idExport {
			return parseExports(s.Data)
		}
	}
	return nil, nil
}

func parseExports(data []byte) ([]Export, error) {
	r := reader{b: data}
	n := r.u32()
	var out []Export
	for i := uint32(0); i < n && r.err == nil; i++ {
		name := string(r.bytes(int(r.u32())))
		kind := r.byte()
		idx := r.u32()
		out = append(out, Export{Name: name, Kind: kind, Index: idx})
	}
	return out, r.err
}

func removeExportEntries(data []byte, remove []string) ([]byte, error) {
	exports, err := parseExports(data)
	if err != nil {
		return nil, err
	}
	var w bytes.Buffer
	var kept []Export
	for _, e := range exports {
		drop := false
		for _, r := range remove {
			if e.Name == r {
				drop = true
			}
		}
		if !drop {
			kept = append(kept, e)
		}
	}
	writeU32(&w, uint32(len(kept)))
	for _, e := range kept {
		writeU32(&w, uint32(len(e.Name)))
		w.WriteString(e.Name)
		w.WriteByte(e.Kind)
		writeU32(&w, e.Index)
	}
	return w.Bytes(), nil
}

func writeU32(w *bytes.Buffer, v uint32) {
	for {
		b := byte(v & 0x7f)
		v >>= 7
		if v != 0 {
			w.WriteByte(b | 0x80)
		} else {
			w.WriteByte(b)
			return
		}
	}
}

type reader struct {
	b   []byte
	off int
	err error
}

func (r *reader) done() bool { return r.err != nil || r.off >= len(r.b) }

func (r *reader) fail(msg string) {
	if r.err == nil {
		r.err = fmt.Errorf("wasm: %s at offset %d", msg, r.off)
	}
}

func (r *reader) byte() byte {
	if r.off >= len(r.b) {
		r.fail("unexpected end")
		return 0
	}
	c := r.b[r.off]
	r.off++
	return c
}

func (r *reader) bytes(n int) []byte {
	if n < 0 || r.off+n > len(r.b) {
		r.fail("length out of range")
		return nil
	}
	s := r.b[r.off : r.off+n]
	r.off += n
	return s
}

func (r *reader) u32() uint32 {
	var v uint32
	for shift := uint(0); shift < 35; shift += 7 {
		c := r.byte()
		v |= uint32(c&0x7f) << shift
		if c&0x80 == 0 {
			return v
		}
	}
	r.fail("bad leb128")
	return 0
}

// sleb skips a signed LEB128 of up to maxBits bits.
func (r *reader) sleb(maxBits uint) {
	for shift := uint(0); shift < maxBits+7; shift += 7 {
		if r.byte()&0x80 == 0 {
			return
		}
	}
	r.fail("bad sleb128")
}
