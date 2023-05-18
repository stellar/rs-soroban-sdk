import * as SorobanClient from 'soroban-client';
let xdr = SorobanClient.xdr;

export function scvalToBigInt(scval: SorobanClient.xdr.ScVal | undefined): BigInt {
    switch (scval?.switch()) {
        case undefined: {
            return BigInt(0);
        }
        case xdr.ScValType.scvU64(): {
            const { high, low } = scval.u64();
            return bigIntFromBytes(false, high, low);
        }
        case xdr.ScValType.scvI64(): {
            const { high, low } = scval.i64();
            return bigIntFromBytes(true, high, low);
        }
        case xdr.ScValType.scvU128(): {
            const parts = scval.u128();
            const a = parts.hi();
            const b = parts.lo();
            return bigIntFromBytes(false, a.high, a.low, b.high, b.low);
        }
        case xdr.ScValType.scvI128(): {
            const parts = scval.i128();
            const a = parts.hi();
            const b = parts.lo();
            return bigIntFromBytes(true, a.high, a.low, b.high, b.low);
        }
        case xdr.ScValType.scvU256(): {
            return bigIntFromBytes(false, ...scval.u256());
        }
        case xdr.ScValType.scvI256(): {
            return bigIntFromBytes(true, ...scval.i256());
        }
        default: {
            throw new Error(`Invalid type for scvalToBigInt: ${scval?.switch().name}`);
        }
    };
}

export function scValToJs<T>(scval: SorobanClient.xdr.ScVal): T {
    switch (scval?.switch()) {
        case undefined: {
            return 0 as T;
        }
        case xdr.ScValType.scvU32(): {
            return scval.u32() as T;
        }
        case xdr.ScValType.scvI32(): {
            return scval.i32() as T;
        }
        case xdr.ScValType.scvU64():
        case xdr.ScValType.scvI64():
        case xdr.ScValType.scvU128():
        case xdr.ScValType.scvI128():
        case xdr.ScValType.scvU256():
        case xdr.ScValType.scvI256():
            return scvalToBigInt(scval) as T;
        case xdr.ScValType.scvAddress():
            return scval.address().value.toString() as T;
        case xdr.ScValType.scvBytes():
            return scval.bytes() as T;
        default: {
            throw new Error(`type not implemented yet: ${scval?.switch().name}`);
        }
    };

}

function bigIntFromBytes(signed: boolean, ...bytes: (string | number | bigint)[]): BigInt {
    let sign = 1;
    if (signed && bytes[0] === 0x80) {
        // top bit is set, negative number.
        sign = -1;
        bytes[0] &= 0x7f;
    }
    let b = BigInt(0);
    for (let byte of bytes) {
        b <<= BigInt(8);
        b |= BigInt(byte);
    }
    return BigInt(b.toString()) * BigInt(sign);
}

// export function bigIntToI128(value: BigInt): SorobanClient.xdr.ScVal {
//   const buf = bigintToBuf(value);
//   if (buf.length > 16) {
//     throw new Error("BigInt overflows i128");
//   }
//
//   if (value < BigInt(0)) {
//     // Clear the top bit
//     buf[0] &= 0x7f;
//   }
//
//   // left-pad with zeros up to 16 bytes
//   let padded = Buffer.alloc(16);
//   buf.copy(padded, padded.length-buf.length);
//   console.debug({value: value.toString(), padded});
//
//   if (value < BigInt(0)) {
//     // Set the top bit
//     padded[0] |= 0x80;
//   }
//
//   const hi = new xdr.Uint64(
//     bigIntFromBytes(false, ...padded.slice(4, 8)).toNumber(),
//     bigIntFromBytes(false, ...padded.slice(0, 4)).toNumber()
//   );
//   const lo = new xdr.Uint64(
//     bigIntFromBytes(false, ...padded.slice(12, 16)).toNumber(),
//     bigIntFromBytes(false, ...padded.slice(8, 12)).toNumber()
//   );
//
//   return xdr.ScVal.scvI128(new xdr.Int128Parts({lo, hi}));
// }

// function bigintToBuf(bn: BigInt): Buffer {
//   var hex = bn.toString(16).replace(/^-/, '');
//   if (hex.length % 2) { hex = '0' + hex; }
//
//   var len = hex.length / 2;
//   var u8 = new Uint8Array(len);
//
//   var i = 0;
//   var j = 0;
//   while (i < len) {
//     u8[i] = parseInt(hex.slice(j, j+2), 16);
//     i += 1;
//     j += 2;
//   }
//
//   if (bn < BigInt(0)) {
//     // Set the top bit
//     u8[0] |= 0x80;
//   }
//
//   return Buffer.from(u8);
// }

export function xdrUint64ToNumber(value: SorobanClient.xdr.Uint64): number {
    let b = 0;
    b |= value.high;
    b <<= 8;
    b |= value.low;
    return b;
}

export function scvalToString(value: SorobanClient.xdr.ScVal): string | undefined {
    return value.bytes().toString();
}

