package wasm

import "fmt"

// Check returns an error if the module uses Wasm features the Soroban VM does
// not support: floating point types and instructions, saturating float to int
// conversions, multi-value results, reference types and SIMD, or a start
// function.
func Check(b []byte) error {
	sections, err := Parse(b)
	if err != nil {
		return err
	}
	for _, s := range sections {
		switch s.ID {
		case idType:
			if err := checkTypes(s.Data); err != nil {
				return err
			}
		case idGlobal:
			if err := checkGlobals(s.Data); err != nil {
				return err
			}
		case idStart:
			return fmt.Errorf("wasm: start function is not supported")
		case idCode:
			if err := checkCode(s.Data); err != nil {
				return err
			}
		}
	}
	return nil
}

func checkValType(t byte) error {
	switch t {
	case 0x7f, 0x7e: // i32, i64
		return nil
	case 0x7d, 0x7c:
		return fmt.Errorf("wasm: floating point types are not supported by the Soroban VM")
	}
	return fmt.Errorf("wasm: value type 0x%x is not supported by the Soroban VM", t)
}

func checkTypes(data []byte) error {
	r := reader{b: data}
	n := r.u32()
	for i := uint32(0); i < n && r.err == nil; i++ {
		if r.byte() != 0x60 {
			return fmt.Errorf("wasm: unexpected type form")
		}
		params := r.u32()
		for j := uint32(0); j < params; j++ {
			if err := checkValType(r.byte()); err != nil {
				return err
			}
		}
		results := r.u32()
		if results > 1 {
			return fmt.Errorf("wasm: multi-value results are not supported by the Soroban VM")
		}
		for j := uint32(0); j < results; j++ {
			if err := checkValType(r.byte()); err != nil {
				return err
			}
		}
	}
	return r.err
}

func checkGlobals(data []byte) error {
	r := reader{b: data}
	n := r.u32()
	for i := uint32(0); i < n && r.err == nil; i++ {
		if err := checkValType(r.byte()); err != nil {
			return err
		}
		r.byte() // mutability
		if err := checkExpr(&r, -1); err != nil {
			return err
		}
	}
	return r.err
}

func checkCode(data []byte) error {
	r := reader{b: data}
	n := r.u32()
	for i := uint32(0); i < n && r.err == nil; i++ {
		size := int(r.u32())
		body := reader{b: r.bytes(size)}
		groups := body.u32()
		for j := uint32(0); j < groups && body.err == nil; j++ {
			body.u32()
			if err := checkValType(body.byte()); err != nil {
				return fmt.Errorf("%w (in locals of function %d)", err, i)
			}
		}
		if err := checkExpr(&body, int(i)); err != nil {
			return err
		}
		if body.err != nil {
			return body.err
		}
	}
	return r.err
}

// checkExpr scans instructions until the end of the expression.
func checkExpr(r *reader, fn int) error {
	depth := 0
	floatErr := func(op byte) error {
		return fmt.Errorf("wasm: floating point instruction 0x%02x in function %d is not supported by the Soroban VM; avoid float types and operations, such as strconv, fmt or math functions that use floats", op, fn)
	}
	for !r.done() {
		op := r.byte()
		switch {
		case op == 0x02 || op == 0x03 || op == 0x04: // block, loop, if
			r.sleb(33)
			depth++
		case op == 0x0b: // end
			if depth == 0 {
				return r.err
			}
			depth--
		case op == 0x0c || op == 0x0d: // br, br_if
			r.u32()
		case op == 0x0e: // br_table
			n := r.u32()
			for i := uint32(0); i <= n && r.err == nil; i++ {
				r.u32()
			}
		case op == 0x10: // call
			r.u32()
		case op == 0x11: // call_indirect
			r.u32()
			r.u32()
		case op == 0x1c: // select t
			n := r.u32()
			for i := uint32(0); i < n; i++ {
				if err := checkValType(r.byte()); err != nil {
					return err
				}
			}
		case op >= 0x20 && op <= 0x24: // local/global get/set/tee
			r.u32()
		case op >= 0x25 && op <= 0x26: // table.get/set
			return fmt.Errorf("wasm: reference types are not supported by the Soroban VM")
		case op >= 0x28 && op <= 0x3e: // loads and stores
			if op == 0x2a || op == 0x2b || op == 0x38 || op == 0x39 {
				return floatErr(op)
			}
			r.u32()
			r.u32()
		case op == 0x3f || op == 0x40: // memory.size, memory.grow
			r.byte()
		case op == 0x41:
			r.sleb(32)
		case op == 0x42:
			r.sleb(64)
		case op == 0x43 || op == 0x44:
			return floatErr(op)
		case op >= 0x5b && op <= 0x66, op >= 0x8b && op <= 0xa6:
			return floatErr(op)
		case op >= 0xa8 && op <= 0xab, op >= 0xae && op <= 0xbf:
			return floatErr(op)
		case op == 0xfc:
			sub := r.u32()
			switch {
			case sub <= 7:
				return fmt.Errorf("wasm: saturating float to int conversion in function %d is not supported by the Soroban VM", fn)
			case sub == 8: // memory.init
				r.u32()
				r.byte()
			case sub == 9: // data.drop
				r.u32()
			case sub == 10: // memory.copy
				r.byte()
				r.byte()
			case sub == 11: // memory.fill
				r.byte()
			default:
				return fmt.Errorf("wasm: table instruction 0xfc %d is not supported by the Soroban VM", sub)
			}
		case op == 0xfd:
			return fmt.Errorf("wasm: SIMD instructions are not supported by the Soroban VM")
		case op >= 0xd0 && op <= 0xd2:
			return fmt.Errorf("wasm: reference types are not supported by the Soroban VM")
		case op <= 0x01, op == 0x05, op == 0x0f, op == 0x1a, op == 0x1b,
			op >= 0x45 && op <= 0x5a, op >= 0x67 && op <= 0x8a, op == 0xa7, op == 0xac, op == 0xad,
			op >= 0xc0 && op <= 0xc4:
			// No immediates.
		default:
			return fmt.Errorf("wasm: unknown instruction 0x%02x in function %d", op, fn)
		}
	}
	return r.err
}
