// Package xdr implements XDR encoding and decoding of the contract spec and
// meta types used in a contract's custom sections: ScSpecEntry, ScMetaEntry
// and ScEnvMetaEntry, as defined in Stellar-contract-spec.x and
// Stellar-contract-meta.x / Stellar-contract-env-meta.x.
package xdr

import (
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
)

// Encoder writes XDR.
type Encoder struct {
	bytes.Buffer
}

func (e *Encoder) U32(v uint32) {
	var b [4]byte
	binary.BigEndian.PutUint32(b[:], v)
	e.Write(b[:])
}

func (e *Encoder) Str(s string) {
	e.U32(uint32(len(s)))
	e.WriteString(s)
	if pad := (4 - len(s)%4) % 4; pad > 0 {
		e.Write(make([]byte, pad))
	}
}

func (e *Encoder) Bool(v bool) {
	if v {
		e.U32(1)
	} else {
		e.U32(0)
	}
}

// Decoder reads XDR.
type Decoder struct {
	b   []byte
	off int
	err error
}

// NewDecoder returns a decoder of the bytes.
func NewDecoder(b []byte) *Decoder { return &Decoder{b: b} }

// Done reports whether all input has been consumed.
func (d *Decoder) Done() bool { return d.off >= len(d.b) }

// Err returns the first error encountered.
func (d *Decoder) Err() error { return d.err }

func (d *Decoder) U32() uint32 {
	if d.err != nil {
		return 0
	}
	if d.off+4 > len(d.b) {
		d.err = errors.New("xdr: unexpected end of input")
		return 0
	}
	v := binary.BigEndian.Uint32(d.b[d.off:])
	d.off += 4
	return v
}

func (d *Decoder) Str() string {
	n := int(d.U32())
	if d.err != nil {
		return ""
	}
	padded := n + (4-n%4)%4
	if n < 0 || d.off+padded > len(d.b) {
		d.err = errors.New("xdr: string length exceeds input")
		return ""
	}
	s := string(d.b[d.off : d.off+n])
	d.off += padded
	return s
}

func (d *Decoder) Bool() bool { return d.U32() != 0 }

func (d *Decoder) fail(format string, args ...any) {
	if d.err == nil {
		d.err = fmt.Errorf(format, args...)
	}
}
