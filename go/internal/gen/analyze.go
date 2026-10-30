package gen

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/parser"
	"go/token"
	"go/types"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strconv"
	"strings"

	"golang.org/x/tools/go/packages"

	"github.com/stellar/rs-soroban-sdk/go/internal/xdr"
)

// GeneratedFile is the name of the file soroban-go generates in a contract
// package.
const GeneratedFile = "soroban_generated.go"

// directive is a parsed //soroban:name args comment.
type directive struct {
	name string
	args map[string]string
	raw  string
}

func directives(groups ...*ast.CommentGroup) []directive {
	var out []directive
	for _, g := range groups {
		if g == nil {
			continue
		}
		for _, c := range g.List {
			text, ok := strings.CutPrefix(c.Text, "//soroban:")
			if !ok {
				continue
			}
			name, rest, _ := strings.Cut(text, " ")
			d := directive{name: name, args: map[string]string{}, raw: strings.TrimSpace(rest)}
			for _, f := range strings.Fields(rest) {
				k, v, _ := strings.Cut(f, "=")
				d.args[k] = v
			}
			out = append(out, d)
		}
	}
	return out
}

func findDirective(ds []directive, name string) (directive, bool) {
	for _, d := range ds {
		if d.name == name {
			return d, true
		}
	}
	return directive{}, false
}

func docText(groups ...*ast.CommentGroup) string {
	for _, g := range groups {
		if g != nil {
			return strings.TrimRight(g.Text(), "\n")
		}
	}
	return ""
}

// Analyze loads the package in dir and extracts the contract model.
func Analyze(dir string) (*Model, error) {
	abs, err := filepath.Abs(dir)
	if err != nil {
		return nil, err
	}
	genPath := filepath.Join(abs, GeneratedFile)
	pkgName, err := packageName(abs)
	if err != nil {
		return nil, err
	}
	cfg := &packages.Config{
		Mode: packages.NeedName | packages.NeedTypes | packages.NeedTypesInfo | packages.NeedSyntax |
			packages.NeedFiles | packages.NeedImports | packages.NeedDeps,
		Dir: abs,
		// Analyze the package without previously generated code, which may be
		// stale.
		Overlay:    map[string][]byte{genPath: []byte("package " + pkgName + "\n")},
		BuildFlags: []string{"-tags=tinygo.wasm"},
	}
	pkgs, err := packages.Load(cfg, ".")
	if err != nil {
		return nil, err
	}
	if len(pkgs) != 1 {
		return nil, fmt.Errorf("expected one package in %s, found %d", dir, len(pkgs))
	}
	pkg := pkgs[0]
	if pkg.Types == nil || pkg.TypesInfo == nil {
		return nil, fmt.Errorf("failed to load package %s: %v", dir, pkg.Errors)
	}
	for _, e := range pkg.Errors {
		// Errors from missing generated methods are expected; others are
		// reported by the compiler later.
		if e.Kind == packages.ListError || e.Kind == packages.ParseError {
			return nil, fmt.Errorf("%s", e)
		}
	}
	a := &analyzer{
		pkg:   pkg,
		model: &Model{Pkg: pkg.Types, PkgName: pkg.Name, Used: map[*types.TypeName]bool{}},
		udts:  map[*types.TypeName]*UDT{},
		evts:  map[*types.TypeName]*Event{},
	}
	if err := a.run(abs); err != nil {
		return nil, err
	}
	return a.model, nil
}

func packageName(dir string) (string, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return "", err
	}
	fset := token.NewFileSet()
	for _, e := range entries {
		if !strings.HasSuffix(e.Name(), ".go") || strings.HasSuffix(e.Name(), "_test.go") || e.Name() == GeneratedFile {
			continue
		}
		f, err := parser.ParseFile(fset, filepath.Join(dir, e.Name()), nil, parser.PackageClauseOnly)
		if err != nil {
			return "", err
		}
		return f.Name.Name, nil
	}
	return "", fmt.Errorf("no Go files in %s", dir)
}

type analyzer struct {
	pkg   *packages.Package
	model *Model
	udts  map[*types.TypeName]*UDT
	evts  map[*types.TypeName]*Event
	errs  []string
}

func (a *analyzer) errorf(pos token.Pos, format string, args ...any) {
	a.errs = append(a.errs, fmt.Sprintf("%s: %s", a.pkg.Fset.Position(pos), fmt.Sprintf(format, args...)))
}

func (a *analyzer) run(dir string) error {
	m := a.model
	m.Crate = crateName(filepath.Base(dir))

	type pending struct {
		spec *ast.TypeSpec
		ds   []directive
		doc  string
	}
	var decls []pending
	var contractTypes []*ast.TypeSpec
	var files []*ast.File
	for _, f := range a.pkg.Syntax {
		if filepath.Base(a.pkg.Fset.Position(f.Pos()).Filename) == GeneratedFile {
			continue
		}
		files = append(files, f)
		// File level directives: crate and meta.
		for _, g := range f.Comments {
			for _, d := range directives(g) {
				switch d.name {
				case "crate":
					m.Crate = d.raw
				case "meta":
					k, v, ok := strings.Cut(d.raw, "=")
					if !ok {
						a.errorf(g.Pos(), "soroban:meta must be of the form key=value")
					}
					m.Meta = append(m.Meta, xdr.MetaEntry{Key: strings.TrimSpace(k), Val: strings.TrimSpace(v)})
				}
			}
		}
		for _, decl := range f.Decls {
			gd, ok := decl.(*ast.GenDecl)
			if !ok || gd.Tok != token.TYPE {
				continue
			}
			for _, s := range gd.Specs {
				ts := s.(*ast.TypeSpec)
				groups := []*ast.CommentGroup{ts.Doc}
				if len(gd.Specs) == 1 {
					groups = append(groups, gd.Doc)
				}
				ds := directives(groups...)
				if len(ds) == 0 {
					continue
				}
				doc := docText(groups...)
				if _, ok := findDirective(ds, "contract"); ok {
					contractTypes = append(contractTypes, ts)
					continue
				}
				decls = append(decls, pending{ts, ds, doc})
			}
		}
	}

	// Declare all UDTs and events first so they can reference each other.
	for _, p := range decls {
		obj := a.pkg.TypesInfo.Defs[p.spec.Name].(*types.TypeName)
		if d, ok := findDirective(p.ds, "contracttype"); ok {
			a.udts[obj] = &UDT{Obj: obj, Doc: p.doc, Kind: a.udtKind(obj, d)}
		}
		if _, ok := findDirective(p.ds, "contracterror"); ok {
			a.udts[obj] = &UDT{Obj: obj, Doc: p.doc, Kind: KindError}
		}
		if _, ok := findDirective(p.ds, "contractevent"); ok {
			a.evts[obj] = &Event{Obj: obj, Doc: p.doc}
		}
	}
	for _, p := range decls {
		obj := a.pkg.TypesInfo.Defs[p.spec.Name].(*types.TypeName)
		if u, ok := a.udts[obj]; ok {
			a.fillUDT(u, p.spec, files)
			m.Types = append(m.Types, u)
		}
		if e, ok := a.evts[obj]; ok {
			d, _ := findDirective(p.ds, "contractevent")
			a.fillEvent(e, p.spec, d)
			m.Events = append(m.Events, e)
		}
		if d, ok := findDirective(p.ds, "contractclient"); ok {
			a.fillClient(obj, p.spec, d)
		}
	}

	seen := map[string]bool{}
	for _, ct := range contractTypes {
		obj := a.pkg.TypesInfo.Defs[ct.Name].(*types.TypeName)
		m.Contracts = append(m.Contracts, obj)
		a.collectFunctions(obj, files)
	}
	for _, f := range m.Functions {
		if seen[f.Name] {
			a.errorf(f.Contract.Pos(), "contract function %s is defined more than once", f.Name)
		}
		seen[f.Name] = true
	}
	a.collectUsed(files)

	if len(a.errs) > 0 {
		return fmt.Errorf("%s", strings.Join(a.errs, "\n"))
	}
	return nil
}

func (a *analyzer) udtKind(obj *types.TypeName, d directive) UDTKind {
	switch obj.Type().Underlying().(type) {
	case *types.Struct:
		if _, ok := d.args["union"]; ok {
			return KindUnion
		}
		if _, ok := d.args["tuple"]; ok {
			return KindTuple
		}
		return KindStruct
	case *types.Basic:
		return KindEnum
	}
	a.errorf(obj.Pos(), "soroban:contracttype %s must be a struct or uint32 type", obj.Name())
	return KindStruct
}

func tagName(tag string) (name string, opts []string) {
	v, ok := reflect.StructTag(tag).Lookup("soroban")
	if !ok {
		return "", nil
	}
	parts := strings.Split(v, ",")
	return parts[0], parts[1:]
}

func (a *analyzer) structFields(spec *ast.TypeSpec) []Field {
	st, ok := spec.Type.(*ast.StructType)
	if !ok {
		return nil
	}
	var out []Field
	for _, f := range st.Fields.List {
		if len(f.Names) == 0 {
			a.errorf(f.Pos(), "embedded fields are not supported in contract types")
			continue
		}
		tag := ""
		if f.Tag != nil {
			tag, _ = strconv.Unquote(f.Tag.Value)
		}
		name, opts := tagName(tag)
		for _, n := range f.Names {
			fl := Field{
				GoName: n.Name,
				Name:   name,
				Doc:    docText(f.Doc),
				Type:   a.pkg.TypesInfo.TypeOf(f.Type),
			}
			for _, o := range opts {
				if o == "topic" {
					fl.Topic = true
				}
			}
			out = append(out, fl)
		}
	}
	return out
}

func (a *analyzer) fillUDT(u *UDT, spec *ast.TypeSpec, files []*ast.File) {
	switch u.Kind {
	case KindStruct:
		u.Fields = a.structFields(spec)
		for i := range u.Fields {
			if u.Fields[i].Name == "" {
				u.Fields[i].Name = snakeCase(u.Fields[i].GoName)
			}
			a.checkSpecable(u.Fields[i].Type, spec.Pos())
		}
		sort.SliceStable(u.Fields, func(i, j int) bool { return u.Fields[i].Name < u.Fields[j].Name })
	case KindTuple:
		u.Fields = a.structFields(spec)
		for i := range u.Fields {
			u.Fields[i].Name = strconv.Itoa(i)
			a.checkSpecable(u.Fields[i].Type, spec.Pos())
		}
	case KindUnion:
		u.Fields = a.structFields(spec)
		for i := range u.Fields {
			f := &u.Fields[i]
			if f.Name == "" {
				f.Name = f.GoName
			}
			switch t := f.Type.(type) {
			case *types.Basic:
				if t.Kind() != types.Bool {
					a.errorf(spec.Pos(), "union %s case %s must be a bool (void case) or a pointer (tuple case)", u.Obj.Name(), f.GoName)
				}
				f.Void = true
			case *types.Pointer:
				if elems, ok := tupleElems(t.Elem()); ok {
					f.Elems = elems
				} else {
					f.Elems = []types.Type{t.Elem()}
				}
				for _, e := range f.Elems {
					a.checkSpecable(e, spec.Pos())
				}
			default:
				a.errorf(spec.Pos(), "union %s case %s must be a bool (void case) or a pointer (tuple case)", u.Obj.Name(), f.GoName)
			}
		}
	case KindEnum, KindError:
		b, ok := u.Obj.Type().Underlying().(*types.Basic)
		if !ok || b.Kind() != types.Uint32 {
			a.errorf(spec.Pos(), "%s must have underlying type uint32", u.Obj.Name())
		}
		u.Cases = a.enumCases(u.Obj, files)
	}
}

// enumCases returns the constants of the type in declaration order.
func (a *analyzer) enumCases(obj *types.TypeName, files []*ast.File) []EnumCase {
	var out []EnumCase
	for _, f := range files {
		for _, decl := range f.Decls {
			gd, ok := decl.(*ast.GenDecl)
			if !ok || gd.Tok != token.CONST {
				continue
			}
			for _, s := range gd.Specs {
				vs := s.(*ast.ValueSpec)
				for _, n := range vs.Names {
					c, ok := a.pkg.TypesInfo.Defs[n].(*types.Const)
					if !ok || !types.Identical(c.Type(), obj.Type()) {
						continue
					}
					v, _ := constant.Uint64Val(c.Val())
					name := n.Name
					if trimmed := strings.TrimPrefix(name, obj.Name()); trimmed != name && trimmed != "" {
						name = trimmed
					}
					doc := docText(vs.Doc)
					if doc == "" && len(gd.Specs) == 1 {
						doc = docText(gd.Doc)
					}
					out = append(out, EnumCase{GoName: n.Name, Name: name, Doc: doc, Value: uint32(v)})
				}
			}
		}
	}
	return out
}

func (a *analyzer) fillEvent(e *Event, spec *ast.TypeSpec, d directive) {
	e.Fields = a.structFields(spec)
	for i := range e.Fields {
		if e.Fields[i].Name == "" {
			e.Fields[i].Name = snakeCase(e.Fields[i].GoName)
		}
		a.checkSpecable(e.Fields[i].Type, spec.Pos())
	}
	if t, ok := d.args["topics"]; ok {
		e.PrefixTopics = strings.Split(t, ",")
	} else {
		e.PrefixTopics = []string{snakeCase(e.Obj.Name())}
	}
	e.DataFormat = xdr.EventDataMap
	switch d.args["data_format"] {
	case "", "map":
	case "vec":
		e.DataFormat = xdr.EventDataVec
	case "single-value":
		e.DataFormat = xdr.EventDataSingleValue
		n := 0
		for _, f := range e.Fields {
			if !f.Topic {
				n++
			}
		}
		if n > 1 {
			a.errorf(spec.Pos(), "data_format=single-value requires 0 or 1 data fields")
		}
	default:
		a.errorf(spec.Pos(), "data_format must be one of single-value, vec, or map")
	}
	e.Sparse = d.args["sparse"] != "false"
}

func (a *analyzer) fillClient(obj *types.TypeName, spec *ast.TypeSpec, d directive) {
	iface, ok := obj.Type().Underlying().(*types.Interface)
	if !ok {
		a.errorf(spec.Pos(), "soroban:contractclient %s must be an interface", obj.Name())
		return
	}
	c := &Client{Name: d.args["name"], Iface: obj}
	if c.Name == "" {
		c.Name = strings.TrimSuffix(obj.Name(), "Interface") + "Client"
	}
	it := spec.Type.(*ast.InterfaceType)
	for _, mf := range it.Methods.List {
		for _, n := range mf.Names {
			fn := iface.Method(indexOfMethod(iface, n.Name))
			f := a.function(fn, directives(mf.Doc), docText(mf.Doc))
			if f != nil {
				c.Methods = append(c.Methods, f)
			}
		}
	}
	a.model.Clients = append(a.model.Clients, c)
}

func indexOfMethod(iface *types.Interface, name string) int {
	for i := 0; i < iface.NumMethods(); i++ {
		if iface.Method(i).Name() == name {
			return i
		}
	}
	return -1
}

func (a *analyzer) collectFunctions(obj *types.TypeName, files []*ast.File) {
	// Methods in source order.
	for _, f := range files {
		for _, decl := range f.Decls {
			fd, ok := decl.(*ast.FuncDecl)
			if !ok || fd.Recv == nil || !fd.Name.IsExported() {
				continue
			}
			fn, ok := a.pkg.TypesInfo.Defs[fd.Name].(*types.Func)
			if !ok {
				continue
			}
			recv := fn.Type().(*types.Signature).Recv().Type()
			if p, ok := recv.(*types.Pointer); ok {
				recv = p.Elem()
			}
			if n, ok := recv.(*types.Named); !ok || n.Obj() != obj {
				continue
			}
			ds := directives(fd.Doc)
			if _, skip := findDirective(ds, "skip"); skip {
				continue
			}
			if f := a.function(fn, ds, docText(fd.Doc)); f != nil {
				f.Contract = obj
				a.model.Functions = append(a.model.Functions, f)
			}
		}
	}
}

func isSDKNamed(t types.Type, name string) bool {
	n, ok := types.Unalias(t).(*types.Named)
	return ok && n.Obj().Pkg() != nil && n.Obj().Pkg().Path() == SDKPath && n.Obj().Name() == name
}

func isErrorInterface(t types.Type) bool {
	return types.Identical(t, types.Universe.Lookup("error").Type())
}

func (a *analyzer) function(fn *types.Func, ds []directive, doc string) *Function {
	sig := fn.Type().(*types.Signature)
	f := &Function{Method: fn.Name(), Doc: doc}
	switch fn.Name() {
	case "Constructor":
		f.Name = "__constructor"
	case "CheckAuth":
		f.Name = "__check_auth"
	default:
		f.Name = snakeCase(fn.Name())
	}
	if d, ok := findDirective(ds, "name"); ok {
		f.Name = d.raw
	}
	if len(f.Name) > 32 {
		a.errorf(fn.Pos(), "contract function name %q is longer than 32 characters", f.Name)
	}
	params := sig.Params()
	for i := 0; i < params.Len(); i++ {
		p := params.At(i)
		if i == 0 && isSDKNamed(p.Type(), "Env") {
			f.HasEnv = true
			continue
		}
		// Leading underscores are trimmed, as the Rust SDK does for unused params.
		name := strings.TrimLeft(snakeCase(p.Name()), "_")
		if len(name) > 30 {
			a.errorf(p.Pos(), "parameter name %q is longer than 30 characters", name)
		}
		a.checkSpecable(p.Type(), p.Pos())
		f.Params = append(f.Params, Param{Name: name, Type: p.Type()})
	}
	res := sig.Results()
	isErr := func(t types.Type) bool {
		if isErrorInterface(t) {
			return true
		}
		if p, ok := t.(*types.Pointer); ok {
			if n, ok := types.Unalias(p.Elem()).(*types.Named); ok {
				if u := a.udts[n.Obj()]; u != nil && u.Kind == KindError {
					return true
				}
				return isSDKNamed(n, "Error")
			}
		}
		return false
	}
	switch res.Len() {
	case 0:
	case 1:
		if isErr(res.At(0).Type()) {
			f.ErrType = res.At(0).Type()
		} else {
			f.Result = res.At(0).Type()
			a.checkSpecable(f.Result, fn.Pos())
		}
	case 2:
		if !isErr(res.At(1).Type()) {
			a.errorf(fn.Pos(), "the second result of %s must be error, *soroban.Error, or a pointer to a soroban:contracterror type", fn.Name())
		}
		f.Result = res.At(0).Type()
		f.ErrType = res.At(1).Type()
		a.checkSpecable(f.Result, fn.Pos())
	default:
		a.errorf(fn.Pos(), "%s has too many results; return a soroban.TupleN for multiple values", fn.Name())
	}
	return f
}

func (a *analyzer) checkSpecable(t types.Type, pos token.Pos) {
	if _, err := a.typeDef(t); err != nil {
		a.errorf(pos, "%v", err)
	}
}

// collectUsed records the error and event types used by value.
func (a *analyzer) collectUsed(files []*ast.File) {
	for _, f := range files {
		ast.Inspect(f, func(n ast.Node) bool {
			e, ok := n.(ast.Expr)
			if !ok {
				return true
			}
			tv, ok := a.pkg.TypesInfo.Types[e]
			if !ok || !tv.IsValue() {
				return true
			}
			t := types.Unalias(tv.Type)
			if p, ok := t.(*types.Pointer); ok {
				t = types.Unalias(p.Elem())
			}
			if n, ok := t.(*types.Named); ok {
				if u := a.udts[n.Obj()]; u != nil && u.Kind == KindError {
					a.model.Used[n.Obj()] = true
				}
				if a.evts[n.Obj()] != nil {
					a.model.Used[n.Obj()] = true
				}
			}
			return true
		})
	}
}
