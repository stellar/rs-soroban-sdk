package soroban

import (
	"math/big"
	"testing"
)

func bigU(x U128) *big.Int {
	b := new(big.Int).SetUint64(x.Hi)
	return b.Lsh(b, 64).Or(b, new(big.Int).SetUint64(x.Lo))
}

func bigI(x I128) *big.Int {
	b := bigU(U128{Hi: uint64(x.Hi), Lo: x.Lo})
	if x.Hi < 0 {
		b.Sub(b, new(big.Int).Lsh(big.NewInt(1), 128))
	}
	return b
}

func TestI128Arith(t *testing.T) {
	vals := []I128{I128FromI64(0), I128FromI64(1), I128FromI64(-1), I128FromI64(7), I128FromI64(-13),
		I128FromI64(1 << 62), I128FromI64(-1 << 62), {Hi: 3, Lo: 5}, {Hi: -4, Lo: 9}, {Hi: 1, Lo: 0}}
	for _, a := range vals {
		for _, b := range vals {
			if got, want := bigI(a.Add(b)), new(big.Int).Add(bigI(a), bigI(b)); got.Cmp(want) != 0 {
				t.Errorf("%v+%v = %v want %v", a, b, got, want)
			}
			if got, want := bigI(a.Sub(b)), new(big.Int).Sub(bigI(a), bigI(b)); got.Cmp(want) != 0 {
				t.Errorf("%v-%v = %v want %v", a, b, got, want)
			}
			want := new(big.Int).Mul(bigI(a), bigI(b))
			if want.BitLen() < 127 {
				if got := bigI(a.Mul(b)); got.Cmp(want) != 0 {
					t.Errorf("%v*%v = %v want %v", a, b, got, want)
				}
			}
			if bigI(b).Sign() != 0 {
				q, r := new(big.Int).QuoRem(bigI(a), bigI(b), new(big.Int))
				if got := bigI(a.Div(b)); got.Cmp(q) != 0 {
					t.Errorf("%v/%v = %v want %v", a, b, got, q)
				}
				if got := bigI(a.Rem(b)); got.Cmp(r) != 0 {
					t.Errorf("%v%%%v = %v want %v", a, b, got, r)
				}
			}
		}
	}
}

func TestI128Overflow(t *testing.T) {
	max := I128{Hi: 1<<63 - 1, Lo: ^uint64(0)}
	defer func() {
		if recover() == nil {
			t.Fatal("expected overflow")
		}
	}()
	max.Add(I128FromI64(1))
}

func TestSmallVals(t *testing.T) {
	if v := valFromU64(5); v.Tag() != TagU64Small || valToU64(v) != 5 {
		t.Fatal(v)
	}
	if v := valFromI64(-5); v.Tag() != TagI64Small || valToI64(v) != -5 {
		t.Fatal(v)
	}
	if v := I128FromI64(-7).ToVal(); v.Tag() != TagI128Small {
		t.Fatal(v)
	} else {
		var x I128
		x.FromVal(v)
		if x != I128FromI64(-7) {
			t.Fatal(x)
		}
	}
	e := ErrorFromContractCode(3).ToVal()
	if e != Val(3<<32|uint64(TagError)) {
		t.Fatalf("%x", e)
	}
	if U32FromVal(ValFromU32(9)) != 9 || I32FromVal(ValFromI32(-9)) != -9 || !BoolFromVal(ValFromBool(true)) {
		t.Fatal("primitive round trip")
	}
}

func TestSymbolShort(t *testing.T) {
	// Values from soroban-env-common's symbol encoding.
	for _, s := range []string{"", "a", "hello", "Hello_9", "abcdefghi"} {
		sym := SymbolShort(s)
		if sym.ToVal().Tag() != TagSymbolSmall || sym.String() != s {
			t.Errorf("%q: %v %q", s, sym.ToVal(), sym.String())
		}
	}
	// 'a' is code 38.
	if SymbolShort("a").ToVal() != ValFromBody(TagSymbolSmall, 38) {
		t.Fatal("encoding of a")
	}
	if _, ok := smallSymbolVal("abcdefghij"); ok {
		t.Fatal("10 chars should not be small")
	}
	if _, ok := smallSymbolVal("a-b"); ok {
		t.Fatal("invalid char")
	}
}
