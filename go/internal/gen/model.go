package gen

import (
	"go/ast"
	"go/types"

	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

// SDKPath is the import path of the soroban SDK package.
const SDKPath = "github.com/stellar/rs-soroban-sdk/go/soroban"

// Model is the contract model extracted from a Go package.
type Model struct {
	Pkg       *types.Package
	PkgName   string
	Crate     string
	Contracts []*types.TypeName
	Functions []*Function
	Types     []*UDT
	Events    []*Event
	Clients   []*Client
	Meta      []xdr.MetaEntry
	// Used is the set of error and event types used by value in the package.
	Used map[*types.TypeName]bool
}

// Function is a contract function.
type Function struct {
	Name     string // contract function name
	Contract *types.TypeName
	Method   string // Go method name
	Doc      string
	HasEnv   bool
	Params   []Param
	Result   types.Type // nil if none
	ErrType  types.Type // nil if none; *E or error
}

// Param is a function parameter.
type Param struct {
	Name string // spec name
	Type types.Type
}

// UDTKind is the kind of a user defined type.
type UDTKind int

const (
	KindStruct UDTKind = iota
	KindTuple
	KindEnum
	KindError
	KindUnion
)

// UDT is a user defined contract type.
type UDT struct {
	Kind   UDTKind
	Obj    *types.TypeName
	Doc    string
	Fields []Field    // struct, tuple, union
	Cases  []EnumCase // enum, error
}

// Field is a field of a struct, tuple struct, or union, or an event param.
type Field struct {
	GoName string
	Name   string // spec name; union case name
	Doc    string
	Type   types.Type
	Topic  bool // event params only
	// Union cases: Void is true for void cases. Elems is the tuple element
	// types: one for *T, or the elements of a soroban.TupleN.
	Void  bool
	Elems []types.Type
}

// EnumCase is a case of an integer or error enum.
type EnumCase struct {
	GoName string
	Name   string
	Doc    string
	Value  uint32
}

// Event is a contract event type.
type Event struct {
	Obj          *types.TypeName
	Doc          string
	PrefixTopics []string
	Fields       []Field
	DataFormat   xdr.EventDataFormat
	Sparse       bool
}

// Client is a contract client generated from an interface.
type Client struct {
	Name    string
	Iface   *types.TypeName
	Methods []*Function
}

// file is a parsed source file.
type file struct {
	ast  *ast.File
	path string
}
