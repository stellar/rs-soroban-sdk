package soroban

import "github.com/stellar/rs-soroban-sdk/go/soroban/internal/host"

// ErrorType is the type of an Error, corresponding to ScErrorType.
type ErrorType uint32

const (
	ErrorTypeContract ErrorType = 0
	ErrorTypeWasmVm   ErrorType = 1
	ErrorTypeContext  ErrorType = 2
	ErrorTypeStorage  ErrorType = 3
	ErrorTypeObject   ErrorType = 4
	ErrorTypeCrypto   ErrorType = 5
	ErrorTypeEvents   ErrorType = 6
	ErrorTypeBudget   ErrorType = 7
	ErrorTypeValue    ErrorType = 8
	ErrorTypeAuth     ErrorType = 9
)

// ErrorCode is the code of a non-contract Error, corresponding to ScErrorCode.
type ErrorCode uint32

const (
	ErrorCodeArithDomain    ErrorCode = 0
	ErrorCodeIndexBounds    ErrorCode = 1
	ErrorCodeInvalidInput   ErrorCode = 2
	ErrorCodeMissingValue   ErrorCode = 3
	ErrorCodeExistingValue  ErrorCode = 4
	ErrorCodeExceededLimit  ErrorCode = 5
	ErrorCodeInvalidAction  ErrorCode = 6
	ErrorCodeInternalError  ErrorCode = 7
	ErrorCodeUnexpectedType ErrorCode = 8
	ErrorCodeUnexpectedSize ErrorCode = 9
)

// Error is a Soroban error, made of a type and a code. Contract errors have
// type ErrorTypeContract and a contract defined code.
type Error struct {
	Type ErrorType
	Code uint32
}

// ErrorFromContractCode returns a contract error with the code.
func ErrorFromContractCode(code uint32) Error { return Error{Type: ErrorTypeContract, Code: code} }

// ErrorFromTypeAndCode returns an error with the type and code.
func ErrorFromTypeAndCode(t ErrorType, c ErrorCode) Error { return Error{Type: t, Code: uint32(c)} }

// IsContractError reports whether the error is a contract error.
func (e Error) IsContractError() bool { return e.Type == ErrorTypeContract }

// Error implements the error interface.
func (e Error) Error() string { return "soroban error" }

// ToVal implements ToValer.
func (e Error) ToVal() Val { return ValFromMajorMinor(TagError, e.Code, uint32(e.Type)) }

// FromVal implements FromValer.
func (e *Error) FromVal(v Val) {
	v.expectTag(TagError)
	*e = Error{Type: ErrorType(v.Minor()), Code: v.Major()}
}

// ToError implements ErrorValuer.
func (e Error) ToError() Error { return e }

// ErrorValuer is implemented by types that convert to an Error, which includes
// all types declared with the soroban:contracterror directive.
type ErrorValuer interface {
	ToError() Error
}

// PanicWithError fails the current contract invocation with the error. It
// corresponds to panic_with_error! in the Rust SDK.
func PanicWithError(err ErrorValuer) {
	host.FailWithError(uint64(err.ToError().ToVal()))
	trap()
}

// AssertWithError fails the current contract invocation with the error if the
// condition is false. It corresponds to assert_with_error! in the Rust SDK.
func AssertWithError(cond bool, err ErrorValuer) {
	if !cond {
		PanicWithError(err)
	}
}

func conversionError() {
	panicStr("conversion error")
}

func overflow() {
	panicStr("arithmetic overflow")
}

// panicStr traps the contract. When built for a contract with -panic=trap the
// message is discarded.
func panicStr(msg string) {
	panic(msg)
}

func trap() {
	panic("unreachable")
}

// ErrorValOf returns the Error Val for a Go error returned by a contract
// function. Errors that implement ErrorValuer, such as soroban:contracterror
// types, convert to their Error; other errors convert to an internal error.
func ErrorValOf(err error) Val {
	if ev, ok := err.(ErrorValuer); ok {
		return ev.ToError().ToVal()
	}
	return ErrorFromTypeAndCode(ErrorTypeContext, ErrorCodeInternalError).ToVal()
}
