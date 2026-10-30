package xdr

// SpecType is SCSpecType.
type SpecType uint32

const (
	SpecTypeVal          SpecType = 0
	SpecTypeBool         SpecType = 1
	SpecTypeVoid         SpecType = 2
	SpecTypeError        SpecType = 3
	SpecTypeU32          SpecType = 4
	SpecTypeI32          SpecType = 5
	SpecTypeU64          SpecType = 6
	SpecTypeI64          SpecType = 7
	SpecTypeTimepoint    SpecType = 8
	SpecTypeDuration     SpecType = 9
	SpecTypeU128         SpecType = 10
	SpecTypeI128         SpecType = 11
	SpecTypeU256         SpecType = 12
	SpecTypeI256         SpecType = 13
	SpecTypeBytes        SpecType = 14
	SpecTypeString       SpecType = 16
	SpecTypeSymbol       SpecType = 17
	SpecTypeAddress      SpecType = 19
	SpecTypeMuxedAddress SpecType = 20
	SpecTypeOption       SpecType = 1000
	SpecTypeResult       SpecType = 1001
	SpecTypeVec          SpecType = 1002
	SpecTypeMap          SpecType = 1004
	SpecTypeTuple        SpecType = 1005
	SpecTypeBytesN       SpecType = 1006
	SpecTypeUDT          SpecType = 2000
)

// TypeDef is SCSpecTypeDef. Only the fields relevant to Type are set.
type TypeDef struct {
	Type SpecType
	// Option, Vec: Elem. Result: Elem (ok) and Err. Map: Key and Elem.
	Elem  *TypeDef
	Err   *TypeDef
	Key   *TypeDef
	Tuple []TypeDef
	N     uint32 // BytesN
	UDT   string // UDT name
}

func (t TypeDef) encode(e *Encoder) {
	e.U32(uint32(t.Type))
	switch t.Type {
	case SpecTypeOption, SpecTypeVec:
		t.Elem.encode(e)
	case SpecTypeResult:
		t.Elem.encode(e)
		t.Err.encode(e)
	case SpecTypeMap:
		t.Key.encode(e)
		t.Elem.encode(e)
	case SpecTypeTuple:
		e.U32(uint32(len(t.Tuple)))
		for _, x := range t.Tuple {
			x.encode(e)
		}
	case SpecTypeBytesN:
		e.U32(t.N)
	case SpecTypeUDT:
		e.Str(t.UDT)
	}
}

func decodeTypeDef(d *Decoder, depth int) TypeDef {
	if depth > 64 {
		d.fail("xdr: type def nested too deeply")
		return TypeDef{}
	}
	t := TypeDef{Type: SpecType(d.U32())}
	sub := func() *TypeDef { x := decodeTypeDef(d, depth+1); return &x }
	switch t.Type {
	case SpecTypeOption, SpecTypeVec:
		t.Elem = sub()
	case SpecTypeResult:
		t.Elem = sub()
		t.Err = sub()
	case SpecTypeMap:
		t.Key = sub()
		t.Elem = sub()
	case SpecTypeTuple:
		n := d.U32()
		if n > 12 {
			d.fail("xdr: tuple too long")
			return t
		}
		for i := uint32(0); i < n; i++ {
			t.Tuple = append(t.Tuple, decodeTypeDef(d, depth+1))
		}
	case SpecTypeBytesN:
		t.N = d.U32()
	case SpecTypeUDT:
		t.UDT = d.Str()
	case SpecTypeVal, SpecTypeBool, SpecTypeVoid, SpecTypeError, SpecTypeU32, SpecTypeI32,
		SpecTypeU64, SpecTypeI64, SpecTypeTimepoint, SpecTypeDuration, SpecTypeU128,
		SpecTypeI128, SpecTypeU256, SpecTypeI256, SpecTypeBytes, SpecTypeString,
		SpecTypeSymbol, SpecTypeAddress, SpecTypeMuxedAddress:
	default:
		d.fail("xdr: unknown spec type %d", t.Type)
	}
	return t
}

// EntryKind is SCSpecEntryKind.
type EntryKind uint32

const (
	EntryFunction     EntryKind = 0
	EntryUDTStruct    EntryKind = 1
	EntryUDTUnion     EntryKind = 2
	EntryUDTEnum      EntryKind = 3
	EntryUDTErrorEnum EntryKind = 4
	EntryEvent        EntryKind = 5
)

// Field is a named, documented type: a struct field (SCSpecUDTStructFieldV0)
// or function input (SCSpecFunctionInputV0).
type Field struct {
	Doc  string
	Name string
	Type TypeDef
}

// UnionCase is SCSpecUDTUnionCaseV0. A case with no types is a void case.
type UnionCase struct {
	Doc   string
	Name  string
	Tuple bool
	Types []TypeDef
}

// EnumCase is SCSpecUDTEnumCaseV0 or SCSpecUDTErrorEnumCaseV0.
type EnumCase struct {
	Doc   string
	Name  string
	Value uint32
}

// EventParamLocation is SCSpecEventParamLocationV0.
type EventParamLocation uint32

const (
	EventParamData      EventParamLocation = 0
	EventParamTopicList EventParamLocation = 1
)

// EventParam is SCSpecEventParamV0.
type EventParam struct {
	Doc      string
	Name     string
	Type     TypeDef
	Location EventParamLocation
}

// EventDataFormat is SCSpecEventDataFormat.
type EventDataFormat uint32

const (
	EventDataSingleValue EventDataFormat = 0
	EventDataVec         EventDataFormat = 1
	EventDataMap         EventDataFormat = 2
)

// Entry is SCSpecEntry. Only the fields relevant to Kind are set.
type Entry struct {
	Kind EntryKind
	Doc  string
	Lib  string // not used by functions
	Name string

	// Function.
	Inputs  []Field
	Outputs []TypeDef

	// Struct.
	Fields []Field

	// Union.
	UnionCases []UnionCase

	// Enum and error enum.
	EnumCases []EnumCase

	// Event.
	PrefixTopics []string
	Params       []EventParam
	DataFormat   EventDataFormat
}

// Encode returns the XDR of the entry.
func (x Entry) Encode() []byte {
	var e Encoder
	x.encode(&e)
	return e.Bytes()
}

func (x Entry) encode(e *Encoder) {
	e.U32(uint32(x.Kind))
	switch x.Kind {
	case EntryFunction:
		e.Str(x.Doc)
		e.Str(x.Name)
		e.U32(uint32(len(x.Inputs)))
		for _, f := range x.Inputs {
			e.Str(f.Doc)
			e.Str(f.Name)
			f.Type.encode(e)
		}
		e.U32(uint32(len(x.Outputs)))
		for _, o := range x.Outputs {
			o.encode(e)
		}
	case EntryUDTStruct:
		e.Str(x.Doc)
		e.Str(x.Lib)
		e.Str(x.Name)
		e.U32(uint32(len(x.Fields)))
		for _, f := range x.Fields {
			e.Str(f.Doc)
			e.Str(f.Name)
			f.Type.encode(e)
		}
	case EntryUDTUnion:
		e.Str(x.Doc)
		e.Str(x.Lib)
		e.Str(x.Name)
		e.U32(uint32(len(x.UnionCases)))
		for _, c := range x.UnionCases {
			if c.Tuple {
				e.U32(1)
				e.Str(c.Doc)
				e.Str(c.Name)
				e.U32(uint32(len(c.Types)))
				for _, t := range c.Types {
					t.encode(e)
				}
			} else {
				e.U32(0)
				e.Str(c.Doc)
				e.Str(c.Name)
			}
		}
	case EntryUDTEnum, EntryUDTErrorEnum:
		e.Str(x.Doc)
		e.Str(x.Lib)
		e.Str(x.Name)
		e.U32(uint32(len(x.EnumCases)))
		for _, c := range x.EnumCases {
			e.Str(c.Doc)
			e.Str(c.Name)
			e.U32(c.Value)
		}
	case EntryEvent:
		e.Str(x.Doc)
		e.Str(x.Lib)
		e.Str(x.Name)
		e.U32(uint32(len(x.PrefixTopics)))
		for _, t := range x.PrefixTopics {
			e.Str(t)
		}
		e.U32(uint32(len(x.Params)))
		for _, p := range x.Params {
			e.Str(p.Doc)
			e.Str(p.Name)
			p.Type.encode(e)
			e.U32(uint32(p.Location))
		}
		e.U32(uint32(x.DataFormat))
	}
}

// DecodeEntries decodes a sequence of concatenated ScSpecEntry XDR, as stored
// in the contractspecv0 custom section.
func DecodeEntries(b []byte) ([]Entry, error) {
	d := NewDecoder(b)
	var out []Entry
	for !d.Done() {
		x := decodeEntry(d)
		if d.err != nil {
			return nil, d.err
		}
		out = append(out, x)
	}
	return out, nil
}

func decodeEntry(d *Decoder) Entry {
	x := Entry{Kind: EntryKind(d.U32())}
	field := func() Field {
		return Field{Doc: d.Str(), Name: d.Str(), Type: decodeTypeDef(d, 0)}
	}
	switch x.Kind {
	case EntryFunction:
		x.Doc = d.Str()
		x.Name = d.Str()
		n := d.U32()
		for i := uint32(0); i < n && d.err == nil; i++ {
			x.Inputs = append(x.Inputs, field())
		}
		n = d.U32()
		for i := uint32(0); i < n && d.err == nil; i++ {
			x.Outputs = append(x.Outputs, decodeTypeDef(d, 0))
		}
	case EntryUDTStruct:
		x.Doc, x.Lib, x.Name = d.Str(), d.Str(), d.Str()
		n := d.U32()
		for i := uint32(0); i < n && d.err == nil; i++ {
			x.Fields = append(x.Fields, field())
		}
	case EntryUDTUnion:
		x.Doc, x.Lib, x.Name = d.Str(), d.Str(), d.Str()
		n := d.U32()
		for i := uint32(0); i < n && d.err == nil; i++ {
			c := UnionCase{Tuple: d.U32() == 1}
			c.Doc, c.Name = d.Str(), d.Str()
			if c.Tuple {
				m := d.U32()
				for j := uint32(0); j < m && d.err == nil; j++ {
					c.Types = append(c.Types, decodeTypeDef(d, 0))
				}
			}
			x.UnionCases = append(x.UnionCases, c)
		}
	case EntryUDTEnum, EntryUDTErrorEnum:
		x.Doc, x.Lib, x.Name = d.Str(), d.Str(), d.Str()
		n := d.U32()
		for i := uint32(0); i < n && d.err == nil; i++ {
			x.EnumCases = append(x.EnumCases, EnumCase{Doc: d.Str(), Name: d.Str(), Value: d.U32()})
		}
	case EntryEvent:
		x.Doc, x.Lib, x.Name = d.Str(), d.Str(), d.Str()
		n := d.U32()
		for i := uint32(0); i < n && d.err == nil; i++ {
			x.PrefixTopics = append(x.PrefixTopics, d.Str())
		}
		n = d.U32()
		for i := uint32(0); i < n && d.err == nil; i++ {
			p := EventParam{Doc: d.Str(), Name: d.Str(), Type: decodeTypeDef(d, 0)}
			p.Location = EventParamLocation(d.U32())
			x.Params = append(x.Params, p)
		}
		x.DataFormat = EventDataFormat(d.U32())
	default:
		d.fail("xdr: unknown spec entry kind %d", x.Kind)
	}
	return x
}

// MetaEntry is SCMetaEntry with kind SC_META_V0.
type MetaEntry struct {
	Key, Val string
}

// EncodeMeta returns the concatenated XDR of the meta entries, as stored in
// the contractmetav0 custom section.
func EncodeMeta(entries []MetaEntry) []byte {
	var e Encoder
	for _, m := range entries {
		e.U32(0)
		e.Str(m.Key)
		e.Str(m.Val)
	}
	return e.Bytes()
}

// DecodeMeta decodes the contractmetav0 custom section.
func DecodeMeta(b []byte) ([]MetaEntry, error) {
	d := NewDecoder(b)
	var out []MetaEntry
	for !d.Done() {
		if k := d.U32(); k != 0 {
			d.fail("xdr: unknown meta kind %d", k)
		}
		out = append(out, MetaEntry{Key: d.Str(), Val: d.Str()})
		if d.err != nil {
			return nil, d.err
		}
	}
	return out, nil
}

// EncodeEnvMetaInterfaceVersion returns the XDR of an ScEnvMetaEntry
// interface version entry, as stored in the contractenvmetav0 custom section.
func EncodeEnvMetaInterfaceVersion(protocol, preRelease uint32) []byte {
	var e Encoder
	e.U32(0) // SC_ENV_META_KIND_INTERFACE_VERSION
	e.U32(protocol)
	e.U32(preRelease)
	return e.Bytes()
}
