package xdr

import (
	"fmt"
	"strings"
)

// String returns a Rust-like rendering of the type.
func (t TypeDef) String() string {
	switch t.Type {
	case SpecTypeVal:
		return "Val"
	case SpecTypeBool:
		return "bool"
	case SpecTypeVoid:
		return "()"
	case SpecTypeError:
		return "Error"
	case SpecTypeU32:
		return "u32"
	case SpecTypeI32:
		return "i32"
	case SpecTypeU64:
		return "u64"
	case SpecTypeI64:
		return "i64"
	case SpecTypeTimepoint:
		return "Timepoint"
	case SpecTypeDuration:
		return "Duration"
	case SpecTypeU128:
		return "u128"
	case SpecTypeI128:
		return "i128"
	case SpecTypeU256:
		return "U256"
	case SpecTypeI256:
		return "I256"
	case SpecTypeBytes:
		return "Bytes"
	case SpecTypeString:
		return "String"
	case SpecTypeSymbol:
		return "Symbol"
	case SpecTypeAddress:
		return "Address"
	case SpecTypeMuxedAddress:
		return "MuxedAddress"
	case SpecTypeOption:
		return "Option<" + t.Elem.String() + ">"
	case SpecTypeResult:
		return "Result<" + t.Elem.String() + ", " + t.Err.String() + ">"
	case SpecTypeVec:
		return "Vec<" + t.Elem.String() + ">"
	case SpecTypeMap:
		return "Map<" + t.Key.String() + ", " + t.Elem.String() + ">"
	case SpecTypeTuple:
		var s []string
		for _, x := range t.Tuple {
			s = append(s, x.String())
		}
		return "(" + strings.Join(s, ", ") + ")"
	case SpecTypeBytesN:
		return fmt.Sprintf("BytesN<%d>", t.N)
	case SpecTypeUDT:
		return t.UDT
	}
	return fmt.Sprintf("<unknown %d>", t.Type)
}

// Describe returns a one line Rust-like rendering of the entry.
func Describe(e Entry) string {
	switch e.Kind {
	case EntryFunction:
		var in []string
		for _, f := range e.Inputs {
			in = append(in, f.Name+": "+f.Type.String())
		}
		s := "fn " + e.Name + "(" + strings.Join(in, ", ") + ")"
		if len(e.Outputs) > 0 {
			s += " -> " + e.Outputs[0].String()
		}
		return s
	case EntryUDTStruct:
		var f []string
		for _, x := range e.Fields {
			f = append(f, x.Name+": "+x.Type.String())
		}
		return "struct " + e.Name + " { " + strings.Join(f, ", ") + " }"
	case EntryUDTUnion:
		var c []string
		for _, x := range e.UnionCases {
			if x.Tuple {
				var t []string
				for _, y := range x.Types {
					t = append(t, y.String())
				}
				c = append(c, x.Name+"("+strings.Join(t, ", ")+")")
			} else {
				c = append(c, x.Name)
			}
		}
		return "union " + e.Name + " { " + strings.Join(c, ", ") + " }"
	case EntryUDTEnum, EntryUDTErrorEnum:
		kw := "enum "
		if e.Kind == EntryUDTErrorEnum {
			kw = "error "
		}
		var c []string
		for _, x := range e.EnumCases {
			c = append(c, fmt.Sprintf("%s = %d", x.Name, x.Value))
		}
		return kw + e.Name + " { " + strings.Join(c, ", ") + " }"
	case EntryEvent:
		var p []string
		for _, x := range e.Params {
			loc := ""
			if x.Location == EventParamTopicList {
				loc = "#[topic] "
			}
			p = append(p, loc+x.Name+": "+x.Type.String())
		}
		return fmt.Sprintf("event %s topics=%v format=%d { %s }", e.Name, e.PrefixTopics, e.DataFormat, strings.Join(p, ", "))
	}
	return "?"
}
