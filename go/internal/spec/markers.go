package spec

import (
	"bytes"
	"crypto/sha256"

	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

// markerMagic prefixes the spec markers the Rust SDK embeds in a Wasm's data
// for the events and errors a contract uses.
var markerMagic = []byte("SpEcV1")

// Marker returns the Rust SDK spec marker of an entry: the magic followed by
// the first 8 bytes of the SHA-256 of the entry's XDR.
func Marker(e xdr.Entry) [14]byte {
	var m [14]byte
	copy(m[:], markerMagic)
	h := sha256.Sum256(e.Encode())
	copy(m[6:], h[:8])
	return m
}

// FindMarkers returns the spec markers in the bytes of a Wasm.
func FindMarkers(wasm []byte) map[[14]byte]bool {
	out := map[[14]byte]bool{}
	for i := 0; ; {
		j := bytes.Index(wasm[i:], markerMagic)
		if j < 0 || i+j+14 > len(wasm) {
			return out
		}
		var m [14]byte
		copy(m[:], wasm[i+j:i+j+14])
		out[m] = true
		i += j + 1
	}
}

// UsedByMarkers returns the names of the event and error entries whose markers
// are in the set, for use with Shake.
func UsedByMarkers(entries []xdr.Entry, markers map[[14]byte]bool) map[string]bool {
	used := map[string]bool{}
	for _, e := range entries {
		if (e.Kind == xdr.EntryEvent || e.Kind == xdr.EntryUDTErrorEnum) && markers[Marker(e)] {
			used[e.Name] = true
		}
	}
	return used
}
