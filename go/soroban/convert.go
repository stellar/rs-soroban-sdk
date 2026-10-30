package soroban

// ToValer is implemented by types that convert to a Val.
type ToValer interface {
	ToVal() Val
}

// FromValer is implemented by pointers to types that can be set from a Val.
// Implementations trap if the Val is not the expected type.
type FromValer interface {
	FromVal(Val)
}

// ToVal converts a Go value to a Val. Supported values are the Go primitives
// bool, uint32, int32, uint64, int64, string (converted to a Symbol if it is a
// valid symbol, otherwise a String), and any value that implements ToValer,
// which includes all SDK types and all generated contract types.
func ToVal(x any) Val {
	switch v := x.(type) {
	case nil:
		return ValVoid
	case Val:
		return v
	case ToValer:
		return v.ToVal()
	case bool:
		return valFromBool(v)
	case uint32:
		return valFromU32(v)
	case int32:
		return valFromI32(v)
	case uint64:
		return valFromU64(v)
	case int64:
		return valFromI64(v)
	case uint:
		return valFromU64(uint64(v))
	case int:
		return valFromI64(int64(v))
	case uint8:
		return valFromU32(uint32(v))
	case []byte:
		return BytesFromSlice(v).ToVal()
	case string:
		return SymbolFromStr(v).ToVal()
	}
	conversionError()
	return ValVoid
}

// FromVal converts a Val to a Go value of type T, trapping if the Val is not of
// the type.
func FromVal[T any](v Val) T {
	var t T
	FromValInto(&t, v)
	return t
}

// FromValInto sets the value pointed to by p from the Val, trapping if the Val
// is not of the type.
func FromValInto(p any, v Val) {
	switch x := p.(type) {
	case FromValer:
		x.FromVal(v)
	case *bool:
		switch v {
		case ValTrue:
			*x = true
		case ValFalse:
			*x = false
		default:
			conversionError()
		}
	case *uint32:
		v.expectTag(TagU32Val)
		*x = v.Major()
	case *int32:
		v.expectTag(TagI32Val)
		*x = int32(v.Major())
	case *uint64:
		*x = valToU64(v)
	case *int64:
		*x = valToI64(v)
	case *uint:
		*x = uint(valToU64(v))
	case *int:
		*x = int(valToI64(v))
	case *uint8:
		v.expectTag(TagU32Val)
		m := v.Major()
		if m > 255 {
			conversionError()
		}
		*x = uint8(m)
	case *struct{}:
		v.expectTag(TagVoid)
	default:
		conversionError()
	}
}

// Void is the unit type, and converts to and from the void Val.
type Void struct{}

// ToVal implements ToValer.
func (Void) ToVal() Val { return ValVoid }

// FromVal implements FromValer.
func (*Void) FromVal(v Val) { v.expectTag(TagVoid) }

// Option is an optional value. It converts to void when not present, otherwise
// to the value. It corresponds to Option<T> in the Rust SDK.
type Option[T any] struct {
	Value T
	Some  bool
}

// Some returns an Option containing the value.
func Some[T any](v T) Option[T] { return Option[T]{Value: v, Some: true} }

// None returns an empty Option.
func None[T any]() Option[T] { return Option[T]{} }

// IsSome reports whether the option has a value.
func (o Option[T]) IsSome() bool { return o.Some }

// IsNone reports whether the option has no value.
func (o Option[T]) IsNone() bool { return !o.Some }

// Unwrap returns the value, trapping if there is no value.
func (o Option[T]) Unwrap() T {
	if !o.Some {
		panicStr("called Unwrap on a None Option")
	}
	return o.Value
}

// UnwrapOr returns the value, or def if there is no value.
func (o Option[T]) UnwrapOr(def T) T {
	if !o.Some {
		return def
	}
	return o.Value
}

// ToVal implements ToValer.
func (o Option[T]) ToVal() Val {
	if !o.Some {
		return ValVoid
	}
	return ToVal(any(o.Value))
}

// FromVal implements FromValer.
func (o *Option[T]) FromVal(v Val) {
	if v.IsVoid() {
		*o = Option[T]{}
		return
	}
	o.Some = true
	FromValInto(&o.Value, v)
}
