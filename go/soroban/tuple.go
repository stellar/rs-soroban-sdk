package soroban

// Tuples correspond to Rust tuples, and are encoded as a Vec of their
// elements. The contract interface type of a TupleN is a tuple of its
// element types.

// Tuple1 is a tuple of 1 value.
type Tuple1[A any] struct {
	V0 A
}

// NewTuple1 returns a Tuple1 of the value.
func NewTuple1[A any](v0 A) Tuple1[A] { return Tuple1[A]{v0} }

// ToVal implements ToValer.
func (t Tuple1[A]) ToVal() Val { return TupleToVal([]Val{ToVal(any(t.V0))}) }

// FromVal implements FromValer.
func (t *Tuple1[A]) FromVal(v Val) {
	var vals [1]Val
	TupleFromVal(v, vals[:])
	FromValInto(&t.V0, vals[0])
}

// Tuple2 is a tuple of 2 values.
type Tuple2[A, B any] struct {
	V0 A
	V1 B
}

// NewTuple2 returns a Tuple2 of the values.
func NewTuple2[A, B any](v0 A, v1 B) Tuple2[A, B] { return Tuple2[A, B]{v0, v1} }

// ToVal implements ToValer.
func (t Tuple2[A, B]) ToVal() Val {
	return TupleToVal([]Val{ToVal(any(t.V0)), ToVal(any(t.V1))})
}

// FromVal implements FromValer.
func (t *Tuple2[A, B]) FromVal(v Val) {
	var vals [2]Val
	TupleFromVal(v, vals[:])
	FromValInto(&t.V0, vals[0])
	FromValInto(&t.V1, vals[1])
}

// Tuple3 is a tuple of 3 values.
type Tuple3[A, B, C any] struct {
	V0 A
	V1 B
	V2 C
}

// NewTuple3 returns a Tuple3 of the values.
func NewTuple3[A, B, C any](v0 A, v1 B, v2 C) Tuple3[A, B, C] { return Tuple3[A, B, C]{v0, v1, v2} }

// ToVal implements ToValer.
func (t Tuple3[A, B, C]) ToVal() Val {
	return TupleToVal([]Val{ToVal(any(t.V0)), ToVal(any(t.V1)), ToVal(any(t.V2))})
}

// FromVal implements FromValer.
func (t *Tuple3[A, B, C]) FromVal(v Val) {
	var vals [3]Val
	TupleFromVal(v, vals[:])
	FromValInto(&t.V0, vals[0])
	FromValInto(&t.V1, vals[1])
	FromValInto(&t.V2, vals[2])
}

// Tuple4 is a tuple of 4 values.
type Tuple4[A, B, C, D any] struct {
	V0 A
	V1 B
	V2 C
	V3 D
}

// NewTuple4 returns a Tuple4 of the values.
func NewTuple4[A, B, C, D any](v0 A, v1 B, v2 C, v3 D) Tuple4[A, B, C, D] {
	return Tuple4[A, B, C, D]{v0, v1, v2, v3}
}

// ToVal implements ToValer.
func (t Tuple4[A, B, C, D]) ToVal() Val {
	return TupleToVal([]Val{ToVal(any(t.V0)), ToVal(any(t.V1)), ToVal(any(t.V2)), ToVal(any(t.V3))})
}

// FromVal implements FromValer.
func (t *Tuple4[A, B, C, D]) FromVal(v Val) {
	var vals [4]Val
	TupleFromVal(v, vals[:])
	FromValInto(&t.V0, vals[0])
	FromValInto(&t.V1, vals[1])
	FromValInto(&t.V2, vals[2])
	FromValInto(&t.V3, vals[3])
}

// Tuple5 is a tuple of 5 values.
type Tuple5[A, B, C, D, E any] struct {
	V0 A
	V1 B
	V2 C
	V3 D
	V4 E
}

// NewTuple5 returns a Tuple5 of the values.
func NewTuple5[A, B, C, D, E any](v0 A, v1 B, v2 C, v3 D, v4 E) Tuple5[A, B, C, D, E] {
	return Tuple5[A, B, C, D, E]{v0, v1, v2, v3, v4}
}

// ToVal implements ToValer.
func (t Tuple5[A, B, C, D, E]) ToVal() Val {
	return TupleToVal([]Val{ToVal(any(t.V0)), ToVal(any(t.V1)), ToVal(any(t.V2)), ToVal(any(t.V3)), ToVal(any(t.V4))})
}

// FromVal implements FromValer.
func (t *Tuple5[A, B, C, D, E]) FromVal(v Val) {
	var vals [5]Val
	TupleFromVal(v, vals[:])
	FromValInto(&t.V0, vals[0])
	FromValInto(&t.V1, vals[1])
	FromValInto(&t.V2, vals[2])
	FromValInto(&t.V3, vals[3])
	FromValInto(&t.V4, vals[4])
}

// Tuple6 is a tuple of 6 values.
type Tuple6[A, B, C, D, E, F any] struct {
	V0 A
	V1 B
	V2 C
	V3 D
	V4 E
	V5 F
}

// NewTuple6 returns a Tuple6 of the values.
func NewTuple6[A, B, C, D, E, F any](v0 A, v1 B, v2 C, v3 D, v4 E, v5 F) Tuple6[A, B, C, D, E, F] {
	return Tuple6[A, B, C, D, E, F]{v0, v1, v2, v3, v4, v5}
}

// ToVal implements ToValer.
func (t Tuple6[A, B, C, D, E, F]) ToVal() Val {
	return TupleToVal([]Val{ToVal(any(t.V0)), ToVal(any(t.V1)), ToVal(any(t.V2)), ToVal(any(t.V3)), ToVal(any(t.V4)), ToVal(any(t.V5))})
}

// FromVal implements FromValer.
func (t *Tuple6[A, B, C, D, E, F]) FromVal(v Val) {
	var vals [6]Val
	TupleFromVal(v, vals[:])
	FromValInto(&t.V0, vals[0])
	FromValInto(&t.V1, vals[1])
	FromValInto(&t.V2, vals[2])
	FromValInto(&t.V3, vals[3])
	FromValInto(&t.V4, vals[4])
	FromValInto(&t.V5, vals[5])
}
