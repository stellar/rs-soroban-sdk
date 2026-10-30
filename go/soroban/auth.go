package soroban

// This file contains the SDK's built-in contract types for authorization. Their
// encodings match the Rust SDK's auth module.

// ContractContext is the context of a contract invocation being authorized.
type ContractContext struct {
	Contract Address
	FnName   Symbol
	Args     Vec[Val]
}

var contractContextKeys = []string{"args", "contract", "fn_name"}

// ToVal implements ToValer.
func (c ContractContext) ToVal() Val {
	return StructToVal(contractContextKeys, []Val{c.Args.ToVal(), c.Contract.ToVal(), c.FnName.ToVal()})
}

// FromVal implements FromValer.
func (c *ContractContext) FromVal(v Val) {
	var vals [3]Val
	StructFromVal(v, contractContextKeys, vals[:])
	c.Args.FromVal(vals[0])
	c.Contract.FromVal(vals[1])
	c.FnName.FromVal(vals[2])
}

// ContractExecutable is the executable of a contract being created: Wasm with
// a hash, or a reference to an executable owned by a contract. Exactly one
// field is set.
type ContractExecutable struct {
	Wasm        *BytesN[N32]
	ExternalRef *ContractExecutableRef
}

var contractExecutableCases = []string{"Wasm", "ExternalRef"}

// ToVal implements ToValer.
func (e ContractExecutable) ToVal() Val {
	switch {
	case e.Wasm != nil:
		return UnionToVal("Wasm", e.Wasm.ToVal())
	case e.ExternalRef != nil:
		return UnionToVal("ExternalRef", e.ExternalRef.ToVal())
	}
	conversionError()
	return ValVoid
}

// FromVal implements FromValer.
func (e *ContractExecutable) FromVal(v Val) {
	idx, vec := UnionFromVal(v, contractExecutableCases)
	var f [1]Val
	UnionCaseFromVal(vec, f[:])
	*e = ContractExecutable{}
	switch idx {
	case 0:
		h := FromVal[BytesN[N32]](f[0])
		e.Wasm = &h
	case 1:
		r := FromVal[ContractExecutableRef](f[0])
		e.ExternalRef = &r
	default:
		conversionError()
	}
}

// ContractExecutableRef references an executable by a persistent storage
// entry owned by a contract, with the tag as its key.
type ContractExecutableRef struct {
	Owner Address
	Tag   String
}

var contractExecutableRefKeys = []string{"owner", "tag"}

// ToVal implements ToValer.
func (r ContractExecutableRef) ToVal() Val {
	return StructToVal(contractExecutableRefKeys, []Val{r.Owner.ToVal(), r.Tag.ToVal()})
}

// FromVal implements FromValer.
func (r *ContractExecutableRef) FromVal(v Val) {
	var vals [2]Val
	StructFromVal(v, contractExecutableRefKeys, vals[:])
	r.Owner.FromVal(vals[0])
	r.Tag.FromVal(vals[1])
}

// CreateContractHostFnContext is the context of a contract creation.
type CreateContractHostFnContext struct {
	Executable ContractExecutable
	Salt       BytesN[N32]
}

var createContractKeys = []string{"executable", "salt"}

// ToVal implements ToValer.
func (c CreateContractHostFnContext) ToVal() Val {
	return StructToVal(createContractKeys, []Val{c.Executable.ToVal(), c.Salt.ToVal()})
}

// FromVal implements FromValer.
func (c *CreateContractHostFnContext) FromVal(v Val) {
	var vals [2]Val
	StructFromVal(v, createContractKeys, vals[:])
	c.Executable.FromVal(vals[0])
	c.Salt.FromVal(vals[1])
}

// CreateContractWithConstructorHostFnContext is the context of a contract
// creation with constructor arguments.
type CreateContractWithConstructorHostFnContext struct {
	ConstructorArgs Vec[Val]
	Executable      ContractExecutable
	Salt            BytesN[N32]
}

var createContractCtorKeys = []string{"constructor_args", "executable", "salt"}

// ToVal implements ToValer.
func (c CreateContractWithConstructorHostFnContext) ToVal() Val {
	return StructToVal(createContractCtorKeys, []Val{c.ConstructorArgs.ToVal(), c.Executable.ToVal(), c.Salt.ToVal()})
}

// FromVal implements FromValer.
func (c *CreateContractWithConstructorHostFnContext) FromVal(v Val) {
	var vals [3]Val
	StructFromVal(v, createContractCtorKeys, vals[:])
	c.ConstructorArgs.FromVal(vals[0])
	c.Executable.FromVal(vals[1])
	c.Salt.FromVal(vals[2])
}

// Context is the context of an authorized invocation, passed to a custom
// account's __check_auth. Exactly one field is set.
type Context struct {
	Contract                     *ContractContext
	CreateContractHostFn         *CreateContractHostFnContext
	CreateContractWithCtorHostFn *CreateContractWithConstructorHostFnContext
}

var contextCases = []string{"Contract", "CreateContractHostFn", "CreateContractWithCtorHostFn"}

// ToVal implements ToValer.
func (c Context) ToVal() Val {
	switch {
	case c.Contract != nil:
		return UnionToVal("Contract", c.Contract.ToVal())
	case c.CreateContractHostFn != nil:
		return UnionToVal("CreateContractHostFn", c.CreateContractHostFn.ToVal())
	case c.CreateContractWithCtorHostFn != nil:
		return UnionToVal("CreateContractWithCtorHostFn", c.CreateContractWithCtorHostFn.ToVal())
	}
	conversionError()
	return ValVoid
}

// FromVal implements FromValer.
func (c *Context) FromVal(v Val) {
	idx, vec := UnionFromVal(v, contextCases)
	var f [1]Val
	UnionCaseFromVal(vec, f[:])
	*c = Context{}
	switch idx {
	case 0:
		c.Contract = new(ContractContext)
		c.Contract.FromVal(f[0])
	case 1:
		c.CreateContractHostFn = new(CreateContractHostFnContext)
		c.CreateContractHostFn.FromVal(f[0])
	case 2:
		c.CreateContractWithCtorHostFn = new(CreateContractWithConstructorHostFnContext)
		c.CreateContractWithCtorHostFn.FromVal(f[0])
	default:
		conversionError()
	}
}

// SubContractInvocation is an invocation authorized by the current contract,
// with the sub-invocations it makes.
type SubContractInvocation struct {
	Context        ContractContext
	SubInvocations Vec[InvokerContractAuthEntry]
}

var subContractInvocationKeys = []string{"context", "sub_invocations"}

// ToVal implements ToValer.
func (s SubContractInvocation) ToVal() Val {
	return StructToVal(subContractInvocationKeys, []Val{s.Context.ToVal(), s.SubInvocations.ToVal()})
}

// FromVal implements FromValer.
func (s *SubContractInvocation) FromVal(v Val) {
	var vals [2]Val
	StructFromVal(v, subContractInvocationKeys, vals[:])
	s.Context.FromVal(vals[0])
	s.SubInvocations.FromVal(vals[1])
}

// InvokerContractAuthEntry is an authorization entry for
// Env.AuthorizeAsCurrentContract. Exactly one field is set.
type InvokerContractAuthEntry struct {
	Contract                     *SubContractInvocation
	CreateContractHostFn         *CreateContractHostFnContext
	CreateContractWithCtorHostFn *CreateContractWithConstructorHostFnContext
}

// ToVal implements ToValer.
func (e InvokerContractAuthEntry) ToVal() Val {
	return Context{
		CreateContractHostFn:         e.CreateContractHostFn,
		CreateContractWithCtorHostFn: e.CreateContractWithCtorHostFn,
	}.toValWithContract(e.Contract)
}

func (c Context) toValWithContract(s *SubContractInvocation) Val {
	if s != nil {
		return UnionToVal("Contract", s.ToVal())
	}
	return c.ToVal()
}

// FromVal implements FromValer.
func (e *InvokerContractAuthEntry) FromVal(v Val) {
	idx, vec := UnionFromVal(v, contextCases)
	var f [1]Val
	UnionCaseFromVal(vec, f[:])
	*e = InvokerContractAuthEntry{}
	switch idx {
	case 0:
		e.Contract = new(SubContractInvocation)
		e.Contract.FromVal(f[0])
	case 1:
		e.CreateContractHostFn = new(CreateContractHostFnContext)
		e.CreateContractHostFn.FromVal(f[0])
	case 2:
		e.CreateContractWithCtorHostFn = new(CreateContractWithConstructorHostFnContext)
		e.CreateContractWithCtorHostFn.FromVal(f[0])
	default:
		conversionError()
	}
}
