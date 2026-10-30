package gen

import (
	"fmt"
	"go/types"
	"strconv"
	"strings"

	"github.com/stellar/rs-soroban-sdk/go/internal/gen/sdkspec"
	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

// qualifiedName returns the fully qualified spec name of a type, in the form
// the Rust SDK uses: ::crate::Type.
func (m *Model) qualifiedName(obj *types.TypeName) string {
	return "::" + m.Crate + "::" + obj.Name()
}

func tupleElems(t types.Type) ([]types.Type, bool) {
	n, ok := types.Unalias(t).(*types.Named)
	if !ok || n.Obj().Pkg() == nil || n.Obj().Pkg().Path() != SDKPath || !strings.HasPrefix(n.Obj().Name(), "Tuple") {
		return nil, false
	}
	args := n.TypeArgs()
	var out []types.Type
	for i := 0; i < args.Len(); i++ {
		out = append(out, args.At(i))
	}
	return out, true
}

// bytesNLen returns the length of a BytesN length marker type.
func bytesNLen(t types.Type) (uint32, error) {
	n, ok := types.Unalias(t).(*types.Named)
	if !ok {
		return 0, fmt.Errorf("unsupported BytesN length type %s", t)
	}
	name := n.Obj().Name()
	if strings.HasPrefix(name, "N") {
		if v, err := strconv.ParseUint(name[1:], 10, 32); err == nil {
			return uint32(v), nil
		}
	}
	return 0, fmt.Errorf("BytesN length type %s must be named N<length>, e.g. N32", name)
}

func (a *analyzer) typeDef(t types.Type) (xdr.TypeDef, error) {
	return a.model.typeDefWith(t, func(obj *types.TypeName) bool {
		return a.udts[obj] != nil
	})
}

// TypeDef returns the spec type of a Go type.
func (m *Model) TypeDef(t types.Type) (xdr.TypeDef, error) {
	return m.typeDefWith(t, func(obj *types.TypeName) bool {
		for _, u := range m.Types {
			if u.Obj == obj {
				return true
			}
		}
		return false
	})
}

func (m *Model) typeDefWith(t types.Type, isUDT func(*types.TypeName) bool) (xdr.TypeDef, error) {
	prim := func(s xdr.SpecType) (xdr.TypeDef, error) { return xdr.TypeDef{Type: s}, nil }
	sub := func(t types.Type) (*xdr.TypeDef, error) {
		d, err := m.typeDefWith(t, isUDT)
		return &d, err
	}
	switch t := types.Unalias(t).(type) {
	case *types.Basic:
		switch t.Kind() {
		case types.Bool:
			return prim(xdr.SpecTypeBool)
		case types.Uint32:
			return prim(xdr.SpecTypeU32)
		case types.Int32:
			return prim(xdr.SpecTypeI32)
		case types.Uint64:
			return prim(xdr.SpecTypeU64)
		case types.Int64:
			return prim(xdr.SpecTypeI64)
		}
	case *types.Named:
		obj := t.Obj()
		if obj.Pkg() != nil && obj.Pkg().Path() == SDKPath {
			args := t.TypeArgs()
			switch obj.Name() {
			case "Val":
				return prim(xdr.SpecTypeVal)
			case "Void":
				return prim(xdr.SpecTypeVoid)
			case "Error":
				return prim(xdr.SpecTypeError)
			case "Timepoint":
				return prim(xdr.SpecTypeTimepoint)
			case "Duration":
				return prim(xdr.SpecTypeDuration)
			case "U128":
				return prim(xdr.SpecTypeU128)
			case "I128":
				return prim(xdr.SpecTypeI128)
			case "U256":
				return prim(xdr.SpecTypeU256)
			case "I256":
				return prim(xdr.SpecTypeI256)
			case "Bytes":
				return prim(xdr.SpecTypeBytes)
			case "String":
				return prim(xdr.SpecTypeString)
			case "Symbol":
				return prim(xdr.SpecTypeSymbol)
			case "Address":
				return prim(xdr.SpecTypeAddress)
			case "MuxedAddress":
				return prim(xdr.SpecTypeMuxedAddress)
			case "Option":
				e, err := sub(args.At(0))
				return xdr.TypeDef{Type: xdr.SpecTypeOption, Elem: e}, err
			case "Vec":
				e, err := sub(args.At(0))
				return xdr.TypeDef{Type: xdr.SpecTypeVec, Elem: e}, err
			case "Map":
				k, err := sub(args.At(0))
				if err != nil {
					return xdr.TypeDef{}, err
				}
				v, err := sub(args.At(1))
				return xdr.TypeDef{Type: xdr.SpecTypeMap, Key: k, Elem: v}, err
			case "BytesN":
				n, err := bytesNLen(args.At(0))
				return xdr.TypeDef{Type: xdr.SpecTypeBytesN, N: n}, err
			}
			if name, ok := sdkspec.Names[obj.Name()]; ok {
				return xdr.TypeDef{Type: xdr.SpecTypeUDT, UDT: name}, nil
			}
			if elems, ok := tupleElems(t); ok {
				d := xdr.TypeDef{Type: xdr.SpecTypeTuple}
				for _, e := range elems {
					x, err := m.typeDefWith(e, isUDT)
					if err != nil {
						return d, err
					}
					d.Tuple = append(d.Tuple, x)
				}
				return d, nil
			}
		}
		if isUDT(obj) {
			return xdr.TypeDef{Type: xdr.SpecTypeUDT, UDT: m.qualifiedName(obj)}, nil
		}
	}
	return xdr.TypeDef{}, fmt.Errorf("type %s is not a contract type; use a soroban SDK type, bool, uint32, int32, uint64, int64, or a type declared with soroban:contracttype or soroban:contracterror", t)
}
